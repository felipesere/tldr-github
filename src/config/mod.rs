use anyhow::Result;
use core::fmt;
use serde::de::{Unexpected, Visitor};
use serde::{de, Deserialize, Deserializer};

// TODO: this need sto be done better, not pointing directly at sqlite
use crate::db::sqlite;

embed_migrations!("./migrations");

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub file: String,
    pub run_migrations: Option<bool>,
}

impl DatabaseConfig {
    pub fn setup(&self) -> Result<sqlite::SqlitePool> {
        let pool = sqlite::establish_connection(&self.file)?;
        match self.run_migrations {
            Some(true) | None => {
                embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout())?;
            }
            _ => {}
        }

        Ok(pool)
    }
}

#[derive(Clone, Debug)]
enum ServerPort {
    Auto,
    Fixed(u64),
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    #[serde(deserialize_with = "port_or_auto")]
    port: ServerPort,
}

fn port_or_auto<'de, D>(deserializer: D) -> Result<ServerPort, D::Error>
where
    D: Deserializer<'de>,
{
    struct ServerPortVisitor {}

    impl<'de> Visitor<'de> for ServerPortVisitor {
        type Value = ServerPort;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("either \"auto\" or a port number")
        }

        fn visit_u64<E>(self, value: u64) -> Result<ServerPort, E>
        where
            E: de::Error,
        {
            Ok(ServerPort::Fixed(value))
        }

        fn visit_str<E>(self, value: &str) -> Result<ServerPort, E>
        where
            E: de::Error,
        {
            if value == "auto" {
                Ok(ServerPort::Auto)
            } else {
                Err(de::Error::invalid_value(Unexpected::Str(value), &self))
            }
        }
    }

    deserializer.deserialize_any(ServerPortVisitor {})
}

impl ServerConfig {
    pub fn address(&self) -> String {
        let port = match self.port {
            ServerPort::Fixed(val) => val,
            ServerPort::Auto => 9000,
        };

        format!("127.0.0.1:{}", port)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct GithubConfig {
    pub token: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdaterConfig {
    pub run: bool,
}

impl std::default::Default for UpdaterConfig {
    fn default() -> UpdaterConfig {
        UpdaterConfig { run: true }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub github: GithubConfig,
    pub updater: UpdaterConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn it_can_read_the_config_with_automatic_port_selection() {
        let sample_config = r#"
{
  "database": {
    "file": "./repos.db",
    "run_migrations": true
  },
  "server": {
    "port": "auto"
  },
  "github": {
    "token": "some-token"
  },
  "updater": {
     "run": true
  }
}
"#;
        let result = serde_json::from_str::<Config>(sample_config);
        assert!(result.is_ok())
    }

    #[test]
    fn it_can_read_the_config_with_specific_port() {
        let sample_config = r#"
{
  "database": {
    "file": "./repos.db",
    "run_migrations": true
  },
  "server": {
    "port": 8080
  },
  "github": {
    "token": "some-token"
  },
  "updater": {
     "run": true
  }
}
"#;

        let result = serde_json::from_str::<Config>(sample_config);
        assert!(result.is_ok())
    }
}
