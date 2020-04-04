use async_trait::async_trait;
use std::fmt;
use std::{collections::HashMap, sync::Arc};

use anyhow::{Context, Error, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::domain::{Author, ItemKind, Label, NewTrackedItem, State};

use sqlx::sqlite::SqliteQueryAs;
use sqlx::{Pool, SqlitePool};

use super::{Db, FullStoredRepo, StoredRepo};

use itertools::Itertools;

pub fn placeholders(rows: usize, columns: usize) -> String {
    (0..rows)
        .format_with(",", |i, f| {
            f(&format_args!(
                "({})",
                (1..=columns).format_with(",", |j, f| f(&format_args!("${}", j + (i * columns))))
            ))
        })
        .to_string()
}

struct SqliteDB {
    conn: SqlitePool,
}

pub fn new(database_url: &str, run_migrations: bool) -> Result<Arc<dyn Db>> {
    let pool =
        async_std::task::block_on(
            async move { Pool::builder().max_size(10).build(database_url).await },
        );

    let db = SqliteDB {
        conn: pool.unwrap(),
    };
    Ok(Arc::new(db))
}

impl fmt::Debug for SqliteDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SqliteConnection")
    }
}

#[async_trait]
impl Db for SqliteDB {
    async fn find_repo(&self, repo_name: &str) -> Option<StoredRepo> {
        let conn = self.conn.clone();
        sqlx::query_as::<_, StoredRepo>("SELECT * FROM Repos WHERE Title = ?")
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
        let conn = self.conn.clone();

        let sql = format!("INSERT INTO TrackedItems (repo_id, title, link, by, labels, kind, foreign_id, number, last_updated) VALUES {}", placeholders(items.len(), 9));
        let mut insert = sqlx::query(&sql);

        for item in items.iter() {
            insert = insert
                .bind(repo.id)
                .bind(item.title.clone())
                .bind(item.link.clone())
                .bind(Label::join(&item.labels[..]))
                .bind(item.kind.to_string())
                .bind(item.foreign_id.clone())
                .bind(item.number)
                .bind(item.last_updated.naive_utc());
        }

        insert.execute(&conn).await?;

        Result::Ok(())
    }

    async fn update_tracked_item(&self, _repo: &StoredRepo, item: NewTrackedItem) -> Result<()> {
        let conn = self.conn.clone();

        sqlx::query(
            "UPDATE TrackedItems SET last_updated = ?, labels = ?, title = ? WHERE foreign_id = ?",
        )
        .bind(item.last_updated.naive_utc())
        .bind(Label::join(&item.labels[..]))
        .bind(item.title.clone())
        .bind(item.foreign_id)
        .execute(&conn)
        .await
        .map(|_affected| ())
        .context(format!("failed to update item {}", item.title))
    }

    async fn remove_tracked_item(&self, _repo: &StoredRepo, item: NewTrackedItem) -> Result<()> {
        let conn = self.conn.clone();

        sqlx::query("DELETE FROM TrackedItems WHERE foreign_id = ?")
            .bind(item.foreign_id)
            .execute(&conn)
            .await
            .map(|_| ())
            .map_err(Error::msg)
    }

    async fn all(&self) -> Result<Vec<FullStoredRepo>> {
        let conn = self.conn.clone();

        let rs: Vec<StoredRepo> = sqlx::query_as::<_, StoredRepo>("SELECT * FROM Repos")
            .fetch_all(&conn)
            .await?;

        let mut repos_by_id = HashMap::new();

        for repo in rs.iter() {
            repos_by_id.insert(repo.id, repo);
        }

        let inner = (0..repos_by_id.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            "SELECT * FROM TrackedItems WHERE repo_id IN ({}) GROUP BY repo_id",
            inner
        );
        let mut q = sqlx::query_as::<_, RawTrackedItem>(&sql);
        for (k, _) in repos_by_id.iter() {
            q = q.bind(k);
        }

        let all_tracked_items = q.fetch_all(&conn).await?;

        let mut tracked_items_per_repo: HashMap<i32, Vec<NewTrackedItem>> = HashMap::new();

        for tracked in all_tracked_items.into_iter() {
            let id = tracked.repo_id.clone();
            let x = convert(tracked);
            let items = tracked_items_per_repo
                .entry(id)
                .or_insert_with(|| Vec::new());
            items.push(x)
        }

        let mut result = Vec::new();
        for (repo_id, repo) in repos_by_id {
            let items = tracked_items_per_repo
                .remove(&repo_id)
                .unwrap_or_else(|| Vec::new());
            let (prs, issues) = items.into_iter().partition(|i| i.kind == ItemKind::PR);

            result.push(FullStoredRepo {
                id: repo_id,
                title: repo.title.clone(),
                issues,
                prs,
            });
        }

        Ok(result)
    }

    async fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo> {
        let conn = self.conn.clone();

        // let mut tx = conn.begin().await.with_context(|| "unable to get tx!")?;

        sqlx::query("INSERT INTO repos (title) VALUES (?)")
            .bind(repo_name)
            .execute(&conn)
            .await
            .with_context(|| format!("failed to insert the initial repo: {}", repo_name))?;

        let repo = sqlx::query_as::<_, StoredRepo>("SELECT id, title FROM repos ORDER BY Id DESC")
            .fetch_one(&conn)
            .await
            .with_context(|| {
                format!(
                    "failed to retrieve ID of recently stored repo: {}",
                    repo_name
                )
            })?;

        // tx.commit().await.with_context(|| "unable to commit tx")?;

        Ok(repo)
    }

    async fn delete(&self, repo: StoredRepo) -> Result<()> {
        let conn = self.conn.clone();

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

#[derive(sqlx::FromRow)]
struct RawTrackedItem {
    repo_id: i32,
    foreign_id: String,
    number: i32,
    title: String,
    by: String,
    link: String,
    labels: String,
    kind: String,
    last_updated: NaiveDateTime,
}

fn convert(item: RawTrackedItem) -> NewTrackedItem {
    NewTrackedItem {
        state: State::Open, // TODO: Need to derive this, or should it be assumed to be always open?
        title: item.title.clone(),
        by: Author::from(item.by.clone()),
        number: item.number,
        link: item.link.clone(),
        labels: Label::split(&item.labels),
        kind: ItemKind::from(item.kind.clone()),
        foreign_id: item.foreign_id.clone(),
        last_updated: DateTime::from_utc(item.last_updated, Utc),
    }
}

#[cfg(test)]
mod test {
    use crate::config::{Backing, DatabaseConfig};

    use super::*;

    fn test_db() -> Arc<dyn Db> {
        let config = DatabaseConfig {
            backing: Backing::Sqlite,
            file: "sqlite://test.db".into(),
            run_migrations: Some(false),
        };

        config.get().unwrap()
    }

    crate::behaves_like_a_db!(test_db);
}
