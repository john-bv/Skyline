//! This module contains the networking protocol for the server.
//! It doesn't contain the packets that are added by zeqa, ie: Party, Guild, etc.
use binary_util::BinaryIo;

pub const GUEST_UUID: &str = "00000000-0000-0000-0000-000000000000";
// proto magic: IP + UDP + ZEQA_DISPATCH:1.0.0 + DatasetOverhead
// datasetoverhead = 1 (flags) + 4 (seq) + 2 (sid) + 4 (stotal) + 4 (sindex) + 2 (oid) + 4 (oindex) + 4 (payload length) = 25
pub const MAX_PROTO_OVERHEAD: u16 = 20 + 8 + 4 + 25;

pub mod online;
pub mod offline;
pub mod types;


/// These are the packets strictly related to protocol handling.
/// THIS DOES NOT INCLUDE PACKETS DESIGNED FOR THE CLIENT/SERVER.
#[derive(BinaryIo)]
#[repr(u8)]
pub enum Packets {
    Disconnect(offline::Disconnect),
    Ping(offline::Ping),
    Pong(offline::Pong),
    ConnectRequest(offline::ConnectRequest),
    ConnectResponse(offline::ConnectResponse),
    OnlinePacket(online::OnlinePackets)
}