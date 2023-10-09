use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterOpts {
    enabled: bool,
    #[serde(rename(serialize = "allowUnverified", deserialize = "allowUnverified"))]
    allow_unverified: bool,
    #[serde(rename(serialize = "maxPeers", deserialize = "maxPeers"))]
    max_peers: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthOpts {
    enabled: bool,
    database: DbOpts,
    kind: TokenStrategy,
    #[serde(rename(serialize = "maxAttempts", deserialize = "maxAttempts"))]
    max_attempts: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbOpts {
    /// The provider to use
    provider: DbStrategy,
    /// The address of the database
    host: String,
    /// The port of the database
    port: u16,
    /// The username of the database **if required**
    username: String,
    /// The password of the database **if required**
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbStrategy {
    #[serde(rename = "postgres")]
    Postgres,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "mongo")]
    Mongo,
    #[serde(rename = "redis")]
    Redis
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenStrategy {
    Skyline,
    JWT,
    UUID
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    port: u16,
    cluster: ClusterOpts,
    authorization: AuthOpts,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 24833,
            cluster: ClusterOpts {
                enabled: false,
                allow_unverified: false,
                max_peers: 0,
            },
            authorization: AuthOpts {
                enabled: false,
                database: DbOpts {
                    provider: DbStrategy::Local,
                    host: String::from("localhost"),
                    port: 5432,
                    username: String::from("postgres"),
                    password: String::from("postgres"),
                },
                kind: TokenStrategy::Skyline,
                max_attempts: 0,
            },
        }
    }
}