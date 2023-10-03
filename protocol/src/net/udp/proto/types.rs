use binary_util::*;

#[derive(Debug, Clone, Copy, BinaryIo, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LoginResponseCode {
    /// An unknown issue occurred during packet processing, or the
    /// proxy is refusing the session
    Disconnect,
    /// The token provided was valid but either banned or expired.
    /// The server will stop processing packets from the client.
    DisconnectToken,
    /// The name provided was invalid, or overrides a reserved name.
    DisconnectName,
    /// The session is not allowed because the name or an identifier is not unique
    /// and already connected. (this is returned if the server is in "strict" mode)
    DisconnectDuplicate,
    /// The server allowed the session to connect, and the token is verified.
    AccessGranted,
    /// The server allowed the session to connect,
    /// but the token was invalid and the session is in guest mode.
    AccessLimited,
}
