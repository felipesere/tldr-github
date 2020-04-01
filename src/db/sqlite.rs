use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;

use anyhow::{bail, Context, Error, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::domain::{Author, ItemKind, Label, NewTrackedItem, State};

use sqlx::sqlite::SqliteQueryAs;
use sqlx::SqlitePool;

use super::schema::{repos, tracked_items};
use super::{Db, FullStoredRepo, NewRepo, StoredRepo};

struct SqliteDB {
    conn: SqlitePool,
}

embed_migrations!("./migrations");

pub async fn new(database_url: &str, run_migrations: bool) -> Result<Arc<dyn Db>> {
    let pool = SqlitePool::new(database_url).await?;

    Ok(Arc::new(SqliteDB { conn: pool }))
}

impl fmt::Debug for SqliteDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SqliteConnection")
    }
}

#[async_trait]
impl Db for SqliteDB {
    async fn find_repo(&self, repo_name: &str) -> Option<StoredRepo> {
        let mut conn = self.conn.clone();
        sqlx::query_as::<_, StoredRepo>("SELECT * FROM Repos WHERE Title = ? LIMIT 1")
            .bind(repo_name)
            .fetch_one(&conn)
            .await
            .ok()
    }

    async fn insert_tracked_items(
        &self,
        repo: &StoredRepo,
        items: Vec<NewTrackedItem>,
    ) -> Result<()> {
        let mut conn = self.conn.clone();

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

    async fn update_tracked_item(&self, _repo: &StoredRepo, item: NewTrackedItem) -> Result<()> {
        let mut conn = self.conn.clone();

        sqlx::query(
            "UPDATE TrackedItems SET last_updated = ?, labels = ?, title = ? WHERE foreign_id = ?",
        )
        .bind(item.last_updated.naive_utc())
        .bind(Label::join(&item.labels[..]))
        .bind(item.title)
        .bind(item.foreign_id)
        .execute(&conn)
        .await
        .map(|_affected| ())
        .context(format!("failed to update item {}", item.title))
    }

    async fn remove_tracked_item(&self, _repo: &StoredRepo, item: NewTrackedItem) -> Result<()> {
        let mut conn = self.conn.clone();

        sqlx::query("DELETE FROM TrackedItems WHERE foreign_id = ?")
            .bind(item.foreign_id)
            .execute(&conn)
            .await
            .map(|_| ())
            .map_err(Error::msg)
    }

    async fn all(&self) -> Result<Vec<FullStoredRepo>> {
        let conn = self.conn.get()?;

        use super::schema::repos::dsl::*;

        let rs: Vec<StoredRepo> = repos.load(&conn).with_context(|| "getting all repos")?;

        let ids: Vec<i32> = rs.iter().map(|r| r.id).collect();

        let items: Vec<Vec<RawTrackedItem>> = tracked_items::table
            .filter(tracked_items::columns::repo_id.eq_any(ids))
            .load(&conn)
            .context("loading tracked items")?
            .grouped_by(&rs[..]);

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

    async fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo> {
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

    async fn delete(&self, repo: StoredRepo) -> Result<()> {
        let mut conn = self.conn.clone();

        sqlx::query("DELETE FROM Repos WHERE Id = ?")
            .bind(repo.id)
            .execute(&conn)
            .await?;

        sqlx::query("DELETE FROM TrackedItems WHERE repo_id = ?")
            .bind(repo.id)
            .execute(&conn)
            .await
            .map(|_| ())
            .map_err(Error::msg)
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
    use crate::config::{Backing, DatabaseConfig};

    use super::*;

    fn test_db() -> Arc<dyn Db> {
        let config = DatabaseConfig {
            backing: Backing::Sqlite,
            file: ":memory:".into(),
            run_migrations: Some(true),
        };

        config.get().unwrap()
    }

    crate::behaves_like_a_db!(test_db);
}
