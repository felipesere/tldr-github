mod schema;

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{Pool, ConnectionManager};

use schema::repos::dsl::*;
use serde::Serialize;

use chrono::NaiveDateTime;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub type Conn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<SqlitePool> {
    Pool::new(ConnectionManager::new(database_url)).with_context(|| format!("failed to access db: {}", database_url))
}

#[derive(Serialize, Queryable)]
pub struct StoredRepo {
    id: i32,
    pub name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}


pub fn all_repos(conn: Conn) -> Result<Vec<StoredRepo>> {
    repos.load(&*conn).with_context(|| "getting all repos")
}

pub fn find_repo(conn: Conn, n: i32) -> Option<StoredRepo> {
    repos.find(n).first(&*conn).ok()
}
