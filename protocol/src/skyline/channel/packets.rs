use binary_util::{BinaryIo, types::{varu32, varu64}};

use super::{Channel, ChannelPermission, ChannelResponseStatus};

#[derive(Debug, Clone, BinaryIo)]
pub struct ChannelJoinRequest {
    /// The ID of the channel.
    pub channel_id: u16,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct ChannelJoinResponse {
    /// The ID of the channel.
    pub status: ChannelResponseStatus,
    #[satisfy(self.status == ChannelResponseStatus::Ok)]
    pub channel: Option<Channel>,
    /// The permissions you have on the channel.
    /// Not really important for the client, but is sent by the server
    /// as a way to tell the client what permissions it has on the channel.
    #[satisfy(self.status == ChannelResponseStatus::Ok)]
    pub permissions: Option<ChannelPermission>,
}

/// This packet updates the permissions of the peer on a channel.
/// This packet is sent by the server to the peer.
#[derive(Debug, Clone, BinaryIo)]
pub struct ChannelPermissionUpdate {
    /// The ID of the channel.
    pub channel_id: u16,
    /// The ID of the topic.
    pub topic_id: u16,
    /// The permissions of the topic.
    pub permissions: ChannelPermission,
}