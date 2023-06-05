use binary_util::BinaryIo;

pub mod handshake;
pub mod types;

#[derive(BinaryIo)]
#[repr(u8)]
pub enum Packets {
    Handshake(handshake::LoginPacket),
    Disconnect(handshake::Disconnect)
}