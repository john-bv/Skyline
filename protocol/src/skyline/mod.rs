use binary_util::BinaryIo;

/// This is the binary api interface for Skyline.
/// This module contains things like the value types, and the api packets.
/// Think of this as a BSON but for Skyline.
pub mod api;
/// This is the skyline protocol
/// This includes things like channels, api info etc...
pub mod channel;
pub mod compression;
pub mod connection;

/// The packets enum for Skyline
#[derive(BinaryIo)]
#[repr(u16)]
pub enum SkylinePacket {
    /// Compressed messages can be sent intermixed with other packets.
    /// This is because some packets are extremely large.
    /// If you send this packet, it is expected that the inner packet is a regular
    /// SkylinePacket.
    CompressedMessage(compression::CompressedMessage) = 0,
    Disconnect(connection::Disconnect) = 1,
    LoginPacket(connection::LoginPacket),
    LoginResponse(connection::LoginResponse),
}
