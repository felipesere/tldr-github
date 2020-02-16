use std::fmt;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use tracing::{event, instrument, Level};

use schema::{repos, tracked_items};

use crate::domain::{Author, ItemKind, Label, NewTrackedItem, State};

// TODO: could possibly just re-export some simple types and functions here
pub mod in_memory;
pub mod json_storage;
mod schema;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub type Conn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<SqlitePool> {
    Pool::new(ConnectionManager::new(database_url))
        .with_context(|| format!("failed to access db: {}", database_url))
}

pub trait Db: Send + Sync {
    fn find_repo(&self, id: i32) -> Option<StoredRepo>;
    fn insert_tracked_items(
        &self,
        repo_name: &StoredRepo,
        items: Vec<NewTrackedItem>,
    ) -> Result<()>;
    fn update_tracked_item(&self, item: NewTrackedItem) -> Result<()>;
    fn remove_tracked_item(&self, item: NewTrackedItem) -> Result<()>;
    fn all(&self) -> Result<Vec<FullStoredRepo>>;
    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo>;
    fn delete(&self, repo: i32) -> Result<()>;
}

pub struct SqliteDB {
    pub(crate) conn: Arc<SqlitePool>,
}

impl fmt::Debug for SqliteDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SqliteConnection")
    }
}

impl Db for SqliteDB {
    fn find_repo(&self, id: i32) -> Option<StoredRepo> {
        // TODO improve
        let conn = self.conn.get().unwrap();

        find_repo(&conn, id)
    }

    fn insert_tracked_items(&self, repo: &StoredRepo, items: Vec<NewTrackedItem>) -> Result<()> {
        let conn = self.conn.get()?;
        insert_tracked_items(&conn, repo, items)
    }

    fn update_tracked_item(&self, item: NewTrackedItem) -> Result<()> {
        use schema::tracked_items::dsl::*;

        diesel::update(tracked_items.filter(foreign_id.eq(item.foreign_id)))
            .set((
                last_updated.eq(item.last_updated.naive_utc()),
                labels.eq(Label::join(&item.labels)),
            ))
            .execute(&self.conn.get().unwrap())
            .map(|_affected| ())
            .context(format!("failed to update item {}", item.title))
    }

    fn remove_tracked_item(&self, item: NewTrackedItem) -> Result<()> {
        use schema::tracked_items::dsl::*;

        diesel::delete(tracked_items.filter(foreign_id.eq(item.foreign_id)))
            .execute(&self.conn.get().unwrap())
            .map(|_affected| ())
            .context(format!("failed to delete item {}", item.title))
    }

    fn all(&self) -> Result<Vec<FullStoredRepo>> {
        let conn = self.conn.get()?;
        all(&conn)
    }

    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo> {
        let conn = self.conn.get()?;
        insert_new_repo(&conn, repo_name)
    }

    fn delete(&self, r: i32) -> Result<()> {
        let conn = self.conn.get()?;
        delete(&conn, r)
    }
}

#[derive(Identifiable, Queryable, Debug, Clone)]
#[table_name = "repos"]
pub struct StoredRepo {
    pub id: i32,
    pub title: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl StoredRepo {
    pub fn name(&self) -> crate::domain::RepoName {
        crate::domain::RepoName::from(&self.title).unwrap()
    }
}

impl StoredRepo {
    pub fn new<S: Into<String>>(id: i32, title: S) -> Self {
        StoredRepo {
            id,
            title: title.into(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug)]
pub struct FullStoredRepo {
    pub id: i32,
    pub title: String,
    pub issues: Vec<NewTrackedItem>,
    pub prs: Vec<NewTrackedItem>,
}

impl FullStoredRepo {
    pub fn name(&self) -> crate::domain::RepoName {
        crate::domain::RepoName::from(&self.title).unwrap()
    }

    pub fn items(&self) -> Vec<NewTrackedItem> {
        let mut res = Vec::new();

        res.append(&mut self.issues.clone());
        res.append(&mut self.prs.clone());

        res
    }
}

#[derive(Insertable)]
#[table_name = "repos"]
pub struct NewRepo<'a> {
    pub title: &'a str,
}

pub fn find_repo(conn: &Conn, n: i32) -> Option<StoredRepo> {
    use schema::repos::dsl::*;
    repos.find(n).first(conn).ok()
}

// this needs to be made transactional
pub fn delete(conn: &Conn, r: i32) -> Result<()> {
    match diesel::delete(repos::table.filter(repos::id.eq(r))).execute(conn) {
        Ok(size) if size == 1 => {}
        Ok(_) => bail!("{} not found", r),
        Err(m) => bail!("could not delete repo: {}", m),
    };

    match diesel::delete(tracked_items::table.filter(tracked_items::repo_id.eq(r))).execute(conn) {
        Ok(_) => {}
        Err(m) => bail!("could not delete tracked for repo repo: {}", m),
    };

    Ok(())
}

pub fn insert_new_repo(conn: &Conn, repo_name: &str) -> Result<StoredRepo> {
    use schema::repos::dsl::*;
    let new_repo = NewRepo { title: repo_name };

    conn.transaction::<_, anyhow::Error, _>(|| {
        diesel::insert_into(repos)
            .values(&new_repo)
            .execute(conn)
            .with_context(|| format!("failed to insert '{}'", repo_name))?;

        // this is kinda meh, but there is no 'RETURNING'
        repos
            .order(id.desc())
            .first::<StoredRepo>(conn)
            .with_context(|| "retrieving stored repo")
    })
}

#[instrument(skip(conn))]
pub fn all(conn: &Conn) -> Result<Vec<FullStoredRepo>> {
    use schema::repos::dsl::*;

    let rs: Vec<StoredRepo> = repos.load(conn).with_context(|| "getting all repos")?;
    event!(Level::INFO, "Found {} repos", rs.len());

    let ids: Vec<i32> = rs.iter().map(|r| r.id).collect();

    let items: Vec<Vec<RawTrackedItem>> = tracked_items::table
        .filter(tracked_items::columns::repo_id.eq_any(ids))
        .load(conn)
        .context("loading tracked items")?
        .grouped_by(&rs[..]);
    event!(Level::INFO, "Read tracked items from DB");

    Result::Ok(
        rs.into_iter()
            .zip(items)
            .map(|(repo, tracked)| {
                let (prs, issues) = tracked
                    .iter()
                    .map(|item| NewTrackedItem {
                        state: State::Open, // TODO: Need to derive this, or should it be assumed to be always open?
                        title: item.title.clone(),
                        by: Author::from(item.by.clone()),
                        number: item.number,
                        link: item.link.clone(),
                        labels: Label::split(&item.labels),
                        kind: ItemKind::from(item.kind.clone()),
                        foreign_id: item.foreign_id.clone(),
                        last_updated: DateTime::from_utc(item.last_updated, Utc),
                    })
                    .partition(|item| item.kind == ItemKind::PR);

                FullStoredRepo {
                    id: repo.id,
                    title: repo.title,
                    prs,
                    issues,
                }
            })
            .collect(),
    )
}

#[derive(Insertable)]
#[table_name = "tracked_items"]
struct InsertableTrackedItem<'a> {
    repo_id: i32,
    foreign_id: &'a str,
    number: i32,
    title: &'a str,
    link: &'a str,
    by: &'a str,
    labels: &'a str,
    kind: String,
    last_updated: NaiveDateTime,
}

#[derive(Associations, Identifiable, Queryable, Debug)]
#[belongs_to(StoredRepo, foreign_key = "repo_id")]
#[table_name = "tracked_items"]
struct RawTrackedItem {
    id: i32,
    repo_id: i32,
    foreign_id: String,
    number: i32,
    title: String,
    by: String,
    link: String,
    labels: String,
    kind: String,
    last_updated: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub fn insert_tracked_items(
    conn: &Conn,
    repo: &StoredRepo,
    items: Vec<NewTrackedItem>,
) -> Result<()> {
    conn.transaction::<_, anyhow::Error, _>(|| {
        for i in items.iter() {
            let item = InsertableTrackedItem {
                repo_id: repo.id,
                title: &i.title,
                link: &i.link,
                by: &i.by.name,
                labels: &Label::join(&i.labels),
                kind: i.kind.to_string(),
                foreign_id: &i.foreign_id,
                number: i.number,
                last_updated: i.last_updated.naive_utc(),
            };

            diesel::insert_into(tracked_items::table)
                .values(&item)
                .execute(conn)?;
        }

        Result::Ok(())
    })
}

#[cfg(test)]
mod test {
    use crate::config::DatabaseConfig;
    use crate::domain::*;
    use chrono::{TimeZone, Utc};

    use super::*;

    fn test_pool(
    ) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> {
        let config = DatabaseConfig {
            file: ":memory:".into(),
            run_migrations: Some(true),
        };
        let pool = config.setup().expect("was not able to create test pool");

        pool.get().unwrap()
    }

    fn in_test_transaction<T, F>(conn: &Conn, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let mut user_result = None;

        let _ = conn.transaction::<(), _, _>(|| {
            user_result = Some(f());
            Err(diesel::result::Error::RollbackTransaction)
        });
        user_result.expect("this didn't work")
    }

    #[test]
    fn can_find_repos_it_just_stored() {
        let conn = test_pool();
        in_test_transaction(&conn, || {
            let repo = insert_new_repo(&conn, "felipesere/test")?;

            assert!(
                find_repo(&conn, repo.id).is_some(),
                "did not find stored repo"
            );

            Result::<StoredRepo, anyhow::Error>::Ok(repo)
        })
        .unwrap();
    }

    #[test]
    fn can_insert_tracked_items() {
        let conn = test_pool();
        in_test_transaction(&conn, || {
            let repo = insert_new_repo(&conn, "felipesere/test")?;

            let item1 = NewTrackedItem {
                state: State::Open,
                title: "pr".into(),
                link: "something".into(),
                by: "felipe".into(),
                labels: vec!["foo".into(), "bar".into()],
                kind: ItemKind::PR,
                foreign_id: "abc123".into(),
                last_updated: Utc.ymd(2019, 4, 22).and_hms(15, 37, 18),
                number: 7,
            };

            let item2 = NewTrackedItem {
                state: State::Open,
                title: "an issue".into(),
                link: "something".into(),
                by: "felipe".into(),
                labels: vec!["foo".into(), "bar".into()],
                kind: ItemKind::Issue,
                foreign_id: "abc123".into(),
                last_updated: Utc.ymd(2019, 4, 22).and_hms(15, 37, 18),
                number: 1,
            };

            insert_tracked_items(&conn, &repo, vec![item1, item2])?;

            let repos: Vec<FullStoredRepo> = all(&conn)?;

            let repos = dbg!(repos);

            assert_eq!(repos.len(), 1);
            assert_eq!(repos[0].issues.len(), 1);
            assert_eq!(repos[0].prs.len(), 1);

            Result::<StoredRepo, anyhow::Error>::Ok(repo)
        })
        .unwrap();
    }
}
