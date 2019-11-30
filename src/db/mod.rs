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

pub fn establish_connection() -> Result<SqlitePool> {
    let database_url = "repos.db";
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).with_context(|| format!("failed to access db: {}", database_url))
}

#[derive(Serialize, Queryable)]
pub struct Repo {
    id: i32,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}


pub fn all_repos(conn: Conn) -> Result<Vec<Repo>> {
    repos.load(&*conn).with_context(|| "getting all repos")
}
