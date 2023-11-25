use tokio::sync::Notify;

use crate::config::NetworkMode;

/// This is the main struct responsible for managing the server.
/// It will handle all the connections, and will be responsible for
/// managing the database.
///
/// The server will initialize depending on the network mode specified
/// in the config. IE, the TCP interface will be used if TCP is specified.
pub struct Server {
    mode: NetworkMode,
    close: Notify,
    config: crate::config::Config,
}

impl Server {
    pub fn new(
        address: &str,
        config: &crate::config::Config,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mode = config.network.mode.clone();
        let close = Notify::new();
        Ok(Self {
            mode,
            close,
            config: config.clone(),
        })
    }
}
