use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterOpts {
    pub enabled: bool,
    #[serde(rename(serialize = "allowUnverified", deserialize = "allowUnverified"))]
    pub allow_unverified: bool,
    #[serde(rename(serialize = "maxPeers", deserialize = "maxPeers"))]
    pub max_peers: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthOpts {
    pub enabled: bool,
    pub database: DbOpts,
    pub kind: TokenStrategy,
    #[serde(rename(serialize = "maxAttempts", deserialize = "maxAttempts"))]
    pub max_attempts: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbOpts {
    /// The provider to use
    pub provider: DbStrategy,
    /// The address of the database
    pub host: String,
    /// The port of the database
    pub port: u16,
    /// The username of the database **if required**
    pub username: String,
    /// The password of the database **if required**
    pub password: String,
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
    Redis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenStrategy {
    Skyline,
    JWT,
    UUID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOpts {
    pub mode: NetworkMode,
    #[serde(rename(serialize = "maxConnections", deserialize = "maxConnections"))]
    pub max_connections: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkMode {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub cluster: ClusterOpts,
    pub authorization: AuthOpts,
    pub network: NetworkOpts,
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
            network: NetworkOpts {
                mode: NetworkMode::Tcp,
                max_connections: 0,
            },
        }
    }
}

pub trait ConfigParser {
    fn parse(&self, config: &str) -> Config;
}

impl ConfigParser for Config {
    fn parse(&self, config: &str) -> Config {
        let config: Config = serde_yaml::from_str(config).unwrap();
        config
    }
}
