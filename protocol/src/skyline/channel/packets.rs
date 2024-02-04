use binary_util::{
    types::{varu32, varu64},
    BinaryIo,
};

use super::{ChannelInfo, ChannelPermission, ChannelResponseStatus};

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum ChannelPackets {
    ChannelJoinRequest(ChannelJoinRequest),
    ChannelJoinResponse(ChannelJoinResponse),
    ChannelPermissionUpdate(ChannelPermissionUpdate),
    ChannelMessage(ChannelMessage),
}

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
    pub channel: Option<ChannelInfo>,
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

/// This packet is sent either by a peer or the server.
/// This is a message sent to a specific peer on a channel
#[derive(Debug, Clone, BinaryIo)]
pub struct ChannelMessage {
    /// The ID of the channel.
    pub channel_id: u16,
    /// The ID of the topic.
    pub topic_id: u16,
    /// The ID of the peer that sent the message.
    pub peer_id: varu32,
    /// Whether or not this message was queued.
    /// If this is true, the message was queued.
    /// If this is false, the message was sent immediately.
    pub queued: bool,
    /// If queued, the time the message was queued.
    #[satisfy(self.queued)]
    pub queued_time: Option<varu64>,
    /// The message sent.
    pub message: Vec<u8>,
}
