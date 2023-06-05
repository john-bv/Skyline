use binary_util::*;

pub const GUEST_UUID: &str = "00000000-0000-0000-0000-000000000000";

/// A token that is unique for each Share Session on the proxy.
#[derive(Debug, Clone, BinaryIo)]
pub struct ShareToken {
    /// The date the token was created.
    pub date: u64,
    /// The name/id the token was created for.
    pub name: String,
    /// The tokens unique identifier
    pub uuid: String,
}

/// The origin of a Session.
#[derive(Debug, Clone, BinaryIo)]
pub struct ExternalSessionIdentity {
    /// The name of the client.
    pub name: String,
    /// The unique identifier of the client.
    pub identifiers: Vec<String>,
    /// The id that the session is assigned to.
    pub session_id: u32,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct AllowDispatch {
    /// The Id of the session the share trip is for.
    pub session_id: u32,
    /// Whether or not the session allowed the request.
    pub allowing: bool,
    /// The permissions the session will be allowing for this share id.
    /// This is in regards to what the session will be contributing to the
    /// session.
    pub permission: SessionDispatchPermission,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct Channel {
    pub name: String,
    pub listeners: Vec<u32>,
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum ProxyStatusCode {
    /// The proxy is online and healthy.
    Healthy = 0,
    /// The proxy is online but has a problem.
    Unhealthy = 1,
    /// The proxy is going offline.
    ShuttingDown = 2,
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum SessionResponseCode {
    /// An unknown issue occurred or the Proxy is refusing to allow
    /// The session to be established.
    Disconnect = 0,
    /// The session is allowed to connect and the token is valid. (If present)
    /// If the token was not present, this code is still returned.
    Access = 1,
    /// The session is allowed to connect but the token was invalid.
    /// This is only returned if the token was present and the token was invalid.
    AccessLimited = 2,
    /// The session is not allowed because the name or identifier is **not**
    /// unique and is already connected under a different address.
    DisconnectUnique = 3,
    /// The session is not allowed to connect because the share token is invalid.
    /// And the token is banned or expired.
    DisconnectToken = 4,
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum SessionDisconnectReason {
    /// The session has become invalid for any reason.
    /// This is usually caused by the proxy either:
    /// - Limiting the number of sessions allowed to connect.
    /// - The session has been banned.
    /// - The proxy is shutting down.
    Invalid = 0,

    /// The session has been disconnected because the proxy is shutting down.
    ShutDown = 1,

    /// Share token is expired
    TokenExpired = 2,

    /// The proxy encountered an error while processing the session.
    /// And the session needs to reconnect.
    ProxyError = 3,
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum SessionDispatchPermission {
    /// The session is allowing other sessions to Read it's output.
    /// But not respond to it.
    ReadOnly,
    /// The session is allowing other sessions to Write to it, but not
    /// read it's output.
    ///
    /// EG: Session B is allowing Write.
    ///
    /// Steps (If B is sending to Proxy):
    /// 1. Session B sends ShareDataPacket to proxy.
    /// 2. Proxy terminates the ShareDataPacket here.
    ///
    /// Steps (If A is sending to Proxy)
    /// 1. Session A sends ShareDataPacket to proxy.
    /// 2. Proxy allows the share packet to be sent to `Session B` because write is allowed.
    /// 3. Session B recieves the share data packet, but does not respond.
    WriteOnly,
    /// The session is allowing other sessions to Read and Write to it.
    ///
    /// EG: Session B is allowing Read/Write.
    ///
    /// Steps:
    /// 1. Session B sends ShareDataPacket to proxy.
    /// 2. Proxy checks if the recipient is allowed to read/write to the session.
    /// 3. Proxy then sends ShareDataPacket to Session A and C
    /// 4. Session A then sends ShareDataPacket back to the proxy which then sends the share back to Session B
    ReadWrite,
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum StatusCode {
    /// The request was successful.
    Success,
    /// The request is has not completed.
    Incomplete,
    /// The request was not successful due to an error on the client.
    Malformed,
    /// The request was not successful due to the proxy.
    Failure,
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum SessionCommand {
    /// List all the sessions on the proxy.
    ListAllSessions,
    /// Allow a session to connect to the share.
    AllowShare,
    /// Deny a session to connect to a share.
    DenyShare,
    /// Stop sharing with the given share session.
    StopShare,
    /// Leave the sharing session.
    LeaveShare,
    /// Elevate a session to a higher level.
    ElevateSession,
    /// Generate a new token.
    CreateToken,
    /// Delete a token.
    DeleteToken,
}

#[derive(Debug, Clone, BinaryIo)]
#[repr(u8)]
pub enum ChannelAction {
    /// Subscribes to an open channel.
    Subscribe,
    /// Unsubscribes from an open channel.
    Unsubscribe,
    /// Create a new channel.
    Create,
    /// Delete a channel
    Delete,
    /// List all existing channels.
    List,
    /// Broadcast data to all subscribers of the channel.
    Broadcast,
}
