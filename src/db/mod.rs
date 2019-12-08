mod schema;

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

use schema::repos;
use schema::repos::dsl::*;
use serde::Serialize;

use chrono::{NaiveDateTime, Utc};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

pub type Conn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<SqlitePool> {
    Pool::new(ConnectionManager::new(database_url))
        .with_context(|| format!("failed to access db: {}", database_url))
}

#[derive(Debug, Serialize, Queryable)]
pub struct StoredRepo {
    id: i32,
    pub name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "repos"]
pub struct NewRepo<'a> {
    pub name: &'a str,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn insert_new(conn: &Conn, repo_name: &str) -> Result<StoredRepo> {
    let now = Utc::now().naive_utc();
    let new_repo = NewRepo {
        name: repo_name,
        created_at: now,
        updated_at: now,
    };

    conn.transaction::<_, anyhow::Error, _>(|| {
        diesel::insert_into(repos::table)
            .values(&new_repo)
            .execute(conn)
            .with_context(|| format!("failed to insert {}", repo_name))?;

        // this is kinda meh, but there is no 'RETURNING'
        repos
            .order(id.desc())
            .first(conn)
            .with_context(|| "retrieving stored repo")
    })
}

pub fn all_repos(conn: Conn) -> Result<Vec<StoredRepo>> {
    repos.load(&*conn).with_context(|| "getting all repos")
}

pub fn find_repo(conn: &Conn, n: i32) -> Option<StoredRepo> {
    repos.find(n).first(conn).ok()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::DatabaseConfig;
    use once_cell::sync::Lazy;

    static TEST_POOL: Lazy<SqlitePool> = Lazy::new(|| {
        let config = DatabaseConfig {
            file: ":memory:".into(),
            run_migrations: Some(true),
        };
        config.setup().expect("was not able to create test pool")
    });

    fn test_pool(
    ) -> r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>> {
        TEST_POOL.get().unwrap()
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
    fn it_does_something() {
        let le_pool = test_pool();
        in_test_transaction(&le_pool, || {
            let repo: Result<StoredRepo, anyhow::Error> = insert_new(&le_pool, "felipesere/test");

            let r = repo.unwrap();

            assert!(
                find_repo(&le_pool, r.id).is_some(),
                "did not find stored repo"
            );
            Result::<StoredRepo, anyhow::Error>::Ok(r)
        });
    }
}
