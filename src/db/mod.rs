use std::sync::Arc;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};

// TODO: this needs to move away
use schema::repos;

use crate::domain::NewTrackedItem;

mod schema;
mod in_memory;
mod json_storage;
pub mod sqlite;

pub fn in_memory() -> Arc<impl Db> {
    Arc::new(in_memory::new())
}

pub fn json_backend() -> Arc<impl Db> {
    let path = std::env::current_dir().unwrap();
    Arc::new(json_storage::new(path))
}

pub trait Db: Send + Sync {
    fn find_repo(&self, repo_name: &str) -> Option<StoredRepo>;
    fn insert_tracked_items(&self, repo: &StoredRepo, items: Vec<NewTrackedItem>) -> Result<()>;
    fn update_tracked_item(&self, repo: &StoredRepo, item: NewTrackedItem) -> Result<()>;
    fn remove_tracked_item(&self, repo: &StoredRepo, item: NewTrackedItem) -> Result<()>;
    fn all(&self) -> Result<Vec<FullStoredRepo>>;
    fn insert_new_repo(&self, repo_name: &str) -> Result<StoredRepo>;
    fn delete(&self, repo: StoredRepo) -> Result<()>;
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
    pub fn stored(&self) -> StoredRepo {
        StoredRepo {
            id: self.id,
            title: self.title.clone(),
            // TODO: get rid of some of these...
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
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
