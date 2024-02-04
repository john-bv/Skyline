use std::sync::Arc;

use protocol::skyline::channel::{ChannelInfo, ChannelTopic};
use tokio::sync::RwLock;

use crate::client::Client;

pub struct Channel {
    channel_info: Option<Box<ChannelInfo>>,
    selected_topic: Option<u16>,
    client: Arc<RwLock<Client>>
}

impl Channel {
    pub fn new(client: Arc<RwLock<Client>>) -> Self {
        Self {
            channel_info: None,
            selected_topic: None,
            client
        }
    }
}

/// The client channel pool,
/// this will house all channels the client is currently connected to.
/// 
/// Disconnected channels will be removed from the pool.
#[derive(Debug, Copy, Clone)]
pub struct ChannelPool {

}