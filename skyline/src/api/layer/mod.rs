use async_trait::async_trait;
use protocol::skyline::{SkylinePacket, channel::packets::ChannelPackets};
use anyhow::Result;
use super::channel::ChannelPool;

#[async_trait]
pub trait ApiLayer {
    /// Process a skyline packet
    async fn process_packet(&mut self, packet: SkylinePacket) -> Result<()>;
    /// Process a Channel Packet from skyline
    async fn process_channel_packet(&mut self, packet: ChannelPackets) -> Result<()>;
    /// Get the Connection Adapter for the protocol layer.
    async fn get_conn(&self) -> Box<&mut dyn crate::net::ConnAdapter>;
    /// The channel pool for the protocol layer.
    fn get_channel_pool(&self) -> Box<&mut ChannelPool>;
    /// The name of the protocol layer.
    /// Usually used for debugging purposes, to identify the type of layer.
    fn get_name() -> &'static str;
}