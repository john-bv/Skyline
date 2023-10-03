//! This module contains the networking protocol for the server.
//! It doesn't contain the packets that are added by zeqa, ie: Party, Guild, etc.
use binary_util::BinaryIo;

use self::{offline::OfflinePackets, online::OnlinePackets};

pub const GUEST_UUID: &str = "00000000-0000-0000-0000-000000000000";
// proto magic: IP + UDP + ZEQA_DISPATCH:1.0.0 + DatasetOverhead
// datasetoverhead = 1 (flags) + 4 (seq) + 2 (sid) + 4 (stotal) + 4 (sindex) + 2 (oid) + 4 (oindex) + 4 (payload length) = 25
pub const MAX_PROTO_OVERHEAD: u16 = 20 + 8 + 4 + 25;

/// Protocol designed to communicate with clients that are not connected
/// to the server.
pub mod offline;
/// This is protocol designed as the bare minimum for communication
/// It does not contain the actual Skyline binary api implementation
pub mod online;
/// Various ambiguous types used by the protocol.
pub mod types;

/// These are the packets strictly related to protocol handling.
/// THIS DOES NOT INCLUDE PACKETS DESIGNED FOR THE CLIENT/SERVER.
///
/// <!-- docs -->
/// # PacketWrapper
/// This is the base packet for all packets.
#[derive(BinaryIo)]
#[repr(u8)]
pub enum Packets {
    Disconnect(offline::Disconnect) = 1,
    Ping(offline::Ping),
    Pong(offline::Pong),
    ConnectRequest(offline::ConnectRequest),
    ConnectResponse(offline::ConnectResponse),
    OnlinePacket(online::OnlinePackets),
}

impl Packets {
    pub fn is_online(&self) -> bool {
        match self {
            Packets::OnlinePacket(_) => true,
            _ => false,
        }
    }

    pub fn is_offline(&self) -> bool {
        match self {
            Packets::OnlinePacket(_) => false,
            _ => true,
        }
    }
}

impl From<OfflinePackets> for Packets {
    fn from(packet: OfflinePackets) -> Self {
        match packet {
            OfflinePackets::Disconnect(packet) => Packets::Disconnect(packet),
            OfflinePackets::Ping(packet) => Packets::Ping(packet),
            OfflinePackets::Pong(packet) => Packets::Pong(packet),
            OfflinePackets::ConnectRequest(packet) => Packets::ConnectRequest(packet),
            OfflinePackets::ConnectResponse(packet) => Packets::ConnectResponse(packet),
            _ => panic!("Invalid packet type!"),
        }
    }
}

impl From<OnlinePackets> for Packets {
    fn from(packet: OnlinePackets) -> Self {
        Packets::OnlinePacket(packet)
    }
}

impl From<Packets> for OfflinePackets {
    fn from(packet: Packets) -> Self {
        match packet {
            Packets::Disconnect(packet) => OfflinePackets::Disconnect(packet),
            Packets::Ping(packet) => OfflinePackets::Ping(packet),
            Packets::Pong(packet) => OfflinePackets::Pong(packet),
            Packets::ConnectRequest(packet) => OfflinePackets::ConnectRequest(packet),
            Packets::ConnectResponse(packet) => OfflinePackets::ConnectResponse(packet),
            _ => panic!("Invalid packet type!"),
        }
    }
}

impl From<Packets> for OnlinePackets {
    fn from(packet: Packets) -> Self {
        match packet {
            Packets::OnlinePacket(packet) => packet,
            _ => panic!("Invalid packet type!"),
        }
    }
}
