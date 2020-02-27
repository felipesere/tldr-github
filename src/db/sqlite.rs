use std::fmt;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use tracing::{event, Level};

use crate::domain::{Author, ItemKind, Label, NewTrackedItem, State};

use super::schema::{repos, tracked_items};
use super::{Db, FullStoredRepo, NewRepo, StoredRepo};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<SqlitePool> {
    Pool::new(ConnectionManager::new(database_url))
        .with_context(|| format!("failed to access db: {}", database_url))
}

pub struct SqliteDB {
    pub(crate) conn: Arc<SqlitePool>,
}

embed_migrations!("./migrations");

pub fn new(database_url: &str, run_migrations: bool) -> Result<Arc<dyn Db>> {
    let pool = establish_connection(database_url)?;

    if run_migrations {
        embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout())?;
    }

    Ok(Arc::new(with_pool(pool)))
}

pub fn with_pool(conn: SqlitePool) -> impl Db {
    SqliteDB {
        conn: Arc::new(conn),
    }
}

impl fmt::Debug for SqliteDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SqliteConnection")
    }
}

impl Db for SqliteDB {
    fn find_repo(&self, repo_name: &str) -> Option<StoredRepo> {
        use super::schema::repos::dsl::*;
        let conn = self.conn.get().unwrap();

        repos.filter(title.eq(repo_name)).first(&conn).ok()
    }

    fn insert_tracked_items(&self, repo: &StoredRepo, items: Vec<NewTrackedItem>) -> Result<()> {
        let conn = self.conn.get()?;

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
                    .execute(&conn)?;
            }

            Result::Ok(())
        })
    }

    fn update_tracked_item(&self, _repo: &StoredRepo, item: NewTrackedItem) -> Result<()> {
        use super::schema::tracked_items::dsl::*;

        diesel::update(tracked_items.filter(foreign_id.eq(item.foreign_id)))
            .set((
                last_updated.eq(item.last_updated.naive_utc()),
                labels.eq(Label::join(&item.labels)),
            ))
            .execute(&self.conn.get().unwrap())
            .map(|_affected| ())
            .context(format!("failed to update item {}", item.title))
    }

    fn remove_tracked_item(&self, _repo: &StoredRepo, item: NewTrackedItem) -> Result<()> {
        use super::schema::tracked_items::dsl::*;

        diesel::delete(tracked_items.filter(foreign_id.eq(item.foreign_id)))
            .execute(&self.conn.get().unwrap())
            .map(|_affected| ())
            .context(format!("failed to delete item {}", item.title))
    }

    fn all(&self) -> Result<Vec<FullStoredRepo>> {
        let conn = self.conn.get()?;

        use super::schema::repos::dsl::*;

        let rs: Vec<StoredRepo> = repos.load(&conn).with_context(|| "getting all repos")?;
        event!(Level::INFO, "Found {} repos", rs.len());

        let ids: Vec<i32> = rs.iter().map(|r| r.id).collect();

        let items: Vec<Vec<RawTrackedItem>> = tracked_items::table
            .filter(tracked_items::columns::repo_id.eq_any(ids))
            .load(&conn)
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

    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo> {
        let conn = self.conn.get()?;

        use super::schema::repos::dsl::*;
        let new_repo = NewRepo { title: repo_name };

        conn.transaction::<_, anyhow::Error, _>(|| {
            diesel::insert_into(repos)
                .values(&new_repo)
                .execute(&conn)
                .with_context(|| format!("failed to insert '{}'", repo_name))?;

            // this is kinda meh, but there is no 'RETURNING'
            repos
                .order(id.desc())
                .first::<StoredRepo>(&conn)
                .with_context(|| "retrieving stored repo")
        })
    }

    fn delete(&self, repo: StoredRepo) -> Result<()> {
        let conn = self.conn.get()?;

        match diesel::delete(repos::table.filter(repos::id.eq(repo.id))).execute(&conn) {
            Ok(size) if size == 1 => {}
            Ok(_) => bail!("{} not found", repo.title),
            Err(m) => bail!("could not delete repo: {}", m),
        };

        match diesel::delete(tracked_items::table.filter(tracked_items::repo_id.eq(repo.id)))
            .execute(&conn)
        {
            Ok(_) => {}
            Err(m) => bail!("could not delete tracked for repo repo: {}", m),
        };

        Ok(())
    }
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

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};

    use crate::config::{DatabaseConfig, Backing};
    use crate::domain::*;

    use super::*;

    fn test_db() -> Arc<dyn Db> {
        let config = DatabaseConfig {
            backing: Backing::Sqlite,
            file: ":memory:".into(),
            run_migrations: Some(true),
        };

        config.get().unwrap()
    }

    #[test]
    fn can_find_repos_it_just_stored() {
        let db = test_db();

        let repo = db.insert_new_repo("felipesere/test").unwrap();

        assert!(
            db.find_repo(&repo.title).is_some(),
            "did not find stored repo"
        );
    }

    #[test]
    fn can_insert_tracked_items() {
        let db = test_db();

        let repo = db.insert_new_repo("felipesere/test").unwrap();

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

        db.insert_tracked_items(&repo, vec![item1, item2]).expect("should have inserted items");

        let repos: Vec<FullStoredRepo> = db.all().unwrap();

        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].issues.len(), 1);
        assert_eq!(repos[0].prs.len(), 1);
    }
}
