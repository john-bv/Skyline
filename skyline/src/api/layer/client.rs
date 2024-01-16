use std::sync::Arc;

use async_trait::async_trait;
use protocol::skyline::{SkylinePacket, channel::packets::ChannelPackets};
use anyhow::Result;
use crate::{client::ConnectionNetworkMode, net::ConnAdapter};

use crate::api::channel::client::ChannelPool;

#[derive(Clone)]
pub struct ClientLayer {
    network_mode: ConnectionNetworkMode,
    channel_pool: ChannelPool,
    connection: Arc<Box<dyn ConnAdapter>>,
}