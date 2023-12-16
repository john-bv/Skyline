use binary_util::BinaryIo;

use crate::skyline::api::value::ValueIds;

/// This packet will fetch all available endpoints within the underlying channel.
/// Not all channels support this.
///
/// The exact response to this packet is the ApiInfo packet; which will contain
/// all the packets that are available to the client.
#[derive(Debug, Clone, BinaryIo)]
pub struct FetchApi {}

/// The response to the FetchApiPackets packet.
#[derive(Debug, Clone, BinaryIo)]
pub struct ApiInfo {
    /// This is the version the service gave this api,
    /// and is used to determine if the client is compatible with the server.
    pub version: u16,
    /// These are all the packets that the service has mapped to their types.
    pub packets: Vec<ApiPacket>,
    /// These are permissions the service has mapped.
    pub permissions: Vec<ApiPermission>,
}

#[derive(Debug, Clone, BinaryIo)]
pub struct ApiPacket {
    /// This is the internal ID of the packet assigned by the service.
    /// You should rely on this ID to identify the packet.
    id: u16,
    /// This is the name that the service assigned the packet.
    name: String,
    /// This is a list of all properties that the packet has mapped to their types.
    properties: Vec<ApiPacketPropertyId>,
    /// If the packet requires a permission or permissions to be sent,
    /// this will be a list of all the permissions required.
    permissions: Vec<u16>,
}

/// A simple struct for finding and removing Packets from an Api.
#[derive(Debug, Clone, BinaryIo)]
pub struct ApiPacketPropertyId {
    pub name: String,
    pub value: ValueIds,
}

/// This is a dynamic permission that the service has mapped
/// to a packet. This is used to restrict access to certain packets
/// on a channel.
///
/// !> It is important to note that at the current point in time,
/// !> skyline will not check these permissions.
/// todo: In the future skyline will check these permissions by identifying the client to the service
/// todo: and the service will then check the permissions before sending the packet.
#[derive(Debug, Clone, BinaryIo)]
pub struct ApiPermission {
    /// The internal ID of this permission.
    /// You should rely on this ID to identify the permission.
    pub id: u16,
    /// The name of the permission.
    pub name: String,
}