use binary_util::{
    types::{varu32, varu64},
    BinaryIo,
};

pub mod api;
pub mod packets;

/// A channel in Skyline is like an api "endpoint"
/// It is a way to isolate packets from each other,
/// while using a unique pub/sub system.
///
/// There is a lot of complexity to channels but the idea is simple.
/// You can subscribe to a channel, and you will receive packets from that channel.
/// This channel optionally, may have a "topic" which is a way to further isolate packets.
///
/// For example, if you subscribe to the "chat" channel, you will receive all chat packets.
/// however if the channel has a "topic" of "guild", you will only receive guild chat packets.
#[derive(Debug, Clone, BinaryIo)]
pub struct Channel {
    /// This is the ID of the channel,
    /// It's used to identify the channel.
    pub id: u16,
    /// The amount of subscribers to the channel.
    /// You need to request the peers to get that information
    pub subscribers: varu32,
    /// The different kinds of Topics that this channel has.
    /// Topics are like sub-channels, they are a way to further isolate packets.
    /// IE: If you want to join a chat channel, you can choose to only join the guild chat topic.
    pub topics: Vec<ChannelTopic>,
    /// A boolean to tell the client whether or not this channel has a api-layer.
    /// If true, the client will attempt to fetch all available endpoints for this channel.
    ///
    /// If this this false, you are assumed to know the endpoints for this channel.
    /// False is less overhead, but less user friendly.
    pub api_enabled: bool,
    /// Whether or not the api-layer for this channel is enforced.
    pub api_enforced: bool,
    /// The type of messages that will be sent on this channel.
    /// This is used to determine how the client should handle the messages.
    pub message_type: ChannelMessageType,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct ChannelTopic {
    /// The ID of the topic.
    /// This is used to identify the topic.
    pub id: u16,
    /// The name of the topic.
    /// This is used to identify the topic.
    /// This is typically a UUID.
    pub name: String,
    /// The permissions of the topic.
    /// This is used to restrict access to the topic.
    pub permissions: ChannelPermission,
}

/// These are permissions that can be used to restrict access to channels.
/// Global channels automatically give all authenticated clients the following permissions:
/// - Recv
/// - RecvAll
/// - SendOne
/// - SendAll
#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum ChannelPermission {
    /// This permission allows the user to subscribe to the channel.
    /// This is the default permission.
    Recv,
    /// This permission allows the user to subscribe to the channel with a topic.
    RecvAll,
    /// This permission allows the user to publish to the channel.
    SendOne,
    /// This permission allows the user to send messages to more than just the server
    SendAll,
    /// Whether or not the user can use the api-layer for this channel.
    UseApi,
    /// This permission allows you to listen to when people subscribe
    ListenSub,
    /// This permission allows you to listen to when people unsubscribe
    ListenUnsub,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, BinaryIo)]
#[repr(u8)]
pub enum ChannelResponseStatus {
    /// You should disconnect.
    /// The server has already terminated the channel.
    Disconnect,
    /// You are not allowed to join the channel.
    /// This can happen if the
    /// The channel was not found.
    /// This is typically sent when the channel is not found.
    NotFound,
    /// The channel was found, and you are allowed to connect
    Ok,
    /// The channel is migrating to another server soon,
    /// and you should reconnect to the new server.
    Migrate(String),
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum ChannelMessageType {
    /// Messages are live, and will never be queued.
    /// This is the default message type, and is the most common.
    Broadcast,
    /// Messages are peer to peer, and will never be queued.
    /// This is the second most common message type.
    Direct,
    /// Messages are queued, and will be sent when the peer is online.
    /// However after the peer is online, the message will be sent immediately, and will not be queued.
    Propagate,
    /// All messages are queued, regardless of the state of the peer.
    /// This is useful for messages like store updates or state updates.
    Queue,
}
