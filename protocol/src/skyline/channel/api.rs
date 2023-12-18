use binary_util::{BinaryIo, types::varu32};

use crate::skyline::api::value::ValueIds;

/// This packet will fetch all available endpoints within the underlying channel.
/// Not all channels support this.
///
/// The exact response to this packet is the ApiInfo packet; which will contain
/// all the packets that are available to the client.
#[derive(Debug, Clone, BinaryIo)]
pub struct FetchApi {}

/// The response to the FetchApiPackets packet.
/// This packet contains all the packets that are available to the client.
///
///
/// The following code:
/// ```ignore
/// pub struct GetPlayerPacket {
///    pub player_id: varu32
/// }
///
/// pub struct Player {
///    pub id: varu32,
///    pub name: String,
///    pub age: u8,
///    pub server: Option<String>,
/// }
///
/// pub struct GetPlayerResponse {
///    pub player: Player
/// }
///
/// let api_layer = ApiLayer::build()
///     .with_packet("GetPlayerPacket", GetPlayerPacket::default())
///     .with_packet("GetPlayerResponse", GetPlayerResponse::default())
///     .with_type("Player", Player::default())
///     .finish();
/// ```
/// Can be visualized as the following in JSON over the wire:
/// ```json
/// {
///     "name": "database-proto-skyline",
///     "types": {
///         "Player": {
///             "$typeId": 0,
///             "fields": [
///                 {
///                     "name": "id",
///                     "type": "varu32"
///                 },
///                 {
///                     "name": "name",
///                     "type": "string"
///                 },
///                 {
///                     "name": "age",
///                     "type": "u8"
///                 },
///                 {
///                     "name": "server",
///                     "type": "string?"
///                 }
///             ]
///         }
///     },
///     "packets": [
///         {
///             "$id": 0,
///             "name": "GetPlayerPacket",
///             "fields": [
///                 {
///                     "name": "playerId",
///                     "type": "varu32"
///                 }
///             ]
///         },
///         {
///             "$id": 1,
///             "name": "GetPlayerResponse",
///             "fields": [
///                 {
///                     "name": "player",
///                     "type": "Player"
///                 }
///             ]
///         }
///     ]
/// }
/// ```
#[derive(Debug, Clone, BinaryIo)]
pub struct ApiInfo {
    /// This is the version the service gave this api,
    /// and is used to determine if the client is compatible with the server.
    pub version: u16,
    /// All possible types that a packet field can be.
    pub types: Vec<ApiTypeDefinition>,
    /// These are all the packets that the service has mapped to their types.
    pub packets: Vec<ApiPacket>,
    /// These are permissions the service has mapped.
    pub permissions: Vec<ApiPermission>,
}

/// This is a SINGLE packet that the service has.
#[derive(Debug, Clone, BinaryIo)]
pub struct ApiPacket {
    /// This is the internal ID of the packet assigned by the service.
    /// You should rely on this ID to identify the packet.
    id: u16,
    /// This is the name that the service assigned the packet.
    name: String,
    /// This is a list of all properties that the packet has mapped to their types.
    fields: Vec<ApiField>,
    /// If the packet requires a permission or permissions to be sent,
    /// this will be a list of all the permissions required.
    permissions: Vec<u16>,
}

/// A simple struct for finding and removing Packets from an Api.
#[derive(Debug, Clone, BinaryIo)]
pub struct ApiField {
    pub name: String,
    /// The internal ID for this type,
    /// This could be a custom type, defined by the service.
    pub value: u16,
    /// Whether or not this field is optional.
    pub optional: bool
}

#[derive(Debug, Clone, BinaryIo)]
pub struct ApiTypeDefinition {
    /// The name of the type.
    pub name: String,
    /// The ID of the type.
    pub id: varu32,
    /// The underlying fields of the type
    pub fields: Vec<ApiField>,
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