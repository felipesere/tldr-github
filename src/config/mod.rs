use anyhow::Result;
use serde::Deserialize;
use crate::db;

embed_migrations!("./migrations");

#[derive(Deserialize)]
pub struct DatabaseConfig {
    file: String,
    run_migrations: Option<bool>,
}

impl DatabaseConfig {
    pub fn setup(&self) -> Result<db::SqlitePool> {
        let pool = db::establish_connection(&self.file)?;
        match self.run_migrations {
            Some(true) | None => {
                embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout())?;
            },
            _ => {},
        }

        Ok(pool)
    }
}

#[derive(Deserialize)]
enum AutoEnum{
    #[serde(rename="auto")]
    Auto,
}


#[derive(Deserialize)]
#[serde(untagged)]
enum ServerPort {
    Auto(AutoEnum),
    Fixed(i32),
}

#[derive(Deserialize)]
pub struct ServerConfig {
    port: ServerPort,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        let port = match self.port {
            ServerPort::Fixed(val) => val,
            ServerPort::Auto(_) => 9000,
        };

        format!("127.0.0.1:{}", port)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}
