use anyhow::Result;
use serde::Deserialize;
use crate::db;

embed_migrations!("./migrations");

#[derive(Deserialize, Clone)]
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

#[derive(Deserialize, Clone)]
enum AutoEnum{
    #[serde(rename="auto")]
    Auto,
}


#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum ServerPort {
    Auto(AutoEnum),
    Fixed(i32),
}

#[derive(Deserialize, Clone)]
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

#[derive(Deserialize, Clone)]
pub struct UiConfig {
    pub local_files: String,
    pub hosted_on: String,
    pub entry_point: String,
}

impl UiConfig {
    pub fn entry(&self) -> String {
        format!("{}/{}", self.local_files, self.entry_point)
    }

    pub fn hosted(&self) -> String {
        format!("{}/*filename", self.local_files)
    }
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub ui: UiConfig,
}
