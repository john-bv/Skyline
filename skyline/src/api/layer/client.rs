use std::sync::Arc;

use crate::{client::ConnectionNetworkMode, net::ConnAdapter};
use anyhow::Result;
use async_trait::async_trait;
use protocol::skyline::{channel::packets::ChannelPackets, SkylinePacket};

use crate::api::channel::client::ChannelPool;

#[derive(Clone)]
pub struct ClientLayer {
    network_mode: ConnectionNetworkMode,
    channel_pool: ChannelPool,
    connection: Arc<Box<dyn ConnAdapter>>,
}
