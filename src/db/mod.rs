use std::sync::Arc;

use anyhow::{bail, Context, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

use schema::{repos, tracked_items};

use crate::domain::NewTrackedItem;

mod schema;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub type Conn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<SqlitePool> {
    Pool::new(ConnectionManager::new(database_url))
        .with_context(|| format!("failed to access db: {}", database_url))
}

pub trait Db {
    fn insert_tracked_items(
        &self,
        repo_name: &StoredRepo,
        items: Vec<NewTrackedItem>,
    ) -> Result<()>;
    fn all(&self) -> Result<Vec<FullStoredRepo>>;
    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo>;
    fn delete(&self, r: i32) -> Result<()>;
}

pub struct SqliteDB {
    pub(crate) conn: Arc<SqlitePool>,
}

impl Db for SqliteDB {
    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo> {
        let conn = self.conn.get()?;
        insert_new_repo(&conn, repo_name)
    }

    fn insert_tracked_items(&self, repo: &StoredRepo, items: Vec<NewTrackedItem>) -> Result<()> {
        let conn = self.conn.get()?;
        insert_tracked_items(&conn, repo, items)
    }

    fn all(&self) -> Result<Vec<FullStoredRepo>> {
        let conn = self.conn.get()?;
        all(&conn)
    }

    fn delete(&self, r: i32) -> Result<()> {
        let conn = self.conn.get()?;
        delete(&conn, r)
    }
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "repos"]
pub struct StoredRepo {
    pub id: i32,
    pub title: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct FullStoredRepo {
    pub id: i32,
    pub title: String,
    pub issues: Vec<StoredIssue>,
    pub prs: Vec<StoredPullRequest>,
}

#[derive(Insertable)]
#[table_name = "repos"]
pub struct NewRepo<'a> {
    pub title: &'a str,
}

#[derive(Debug)]
pub struct StoredPullRequest {
    id: i32,
    repo_id: i32,
    pub nr: i32,
    pub title: String,
    pub by: String,
    pub link: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct StoredIssue {
    id: i32,
    repo_id: i32,
    pub nr: i32,
    pub title: String,
    pub by: String,
    pub link: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
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

pub fn all(conn: &Conn) -> Result<Vec<FullStoredRepo>> {
    use schema::repos::dsl::*;
    let rs: Vec<StoredRepo> = repos.load(conn).with_context(|| "getting all repos")?;

    let ids: Vec<i32> = rs.iter().map(|r| r.id).collect();

    let items: Vec<Vec<StoredTrackedItem>> = tracked_items::table
        .filter(tracked_items::columns::repo_id.eq_any(ids))
        .load(conn)
        .context("loading tracked items")?
        .grouped_by(&rs[..]);

    Result::Ok(
        rs.into_iter()
            .zip(items)
            .map(|(repo, tracked)| {
                let prs = tracked
                    .iter()
                    .filter(|t| t.kind == "pr")
                    .map(|item| StoredPullRequest {
                        id: item.id,
                        repo_id: item.repo_id,
                        title: item.title.clone(),
                        by: item.by.clone(),
                        nr: item.number,
                        link: item.link.clone(),
                        created_at: item.created_at,
                        updated_at: item.updated_at,
                    })
                    .collect();
                let issues = tracked
                    .iter()
                    .filter(|t| t.kind == "issue")
                    .map(|item| StoredIssue {
                        id: item.id,
                        repo_id: item.repo_id,
                        title: item.title.clone(),
                        by: item.by.clone(),
                        nr: item.number,
                        link: item.link.clone(),
                        created_at: item.created_at,
                        updated_at: item.updated_at,
                    })
                    .collect();
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
struct StoredTrackedItem {
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
                labels: "",
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

    pub fn find_repo(conn: &Conn, n: i32) -> Option<StoredRepo> {
        use schema::repos::dsl::*;
        repos.find(n).first(conn).ok()
    }

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
