/// These are constant values that CAN NOT be modified or changed
/// without breaking the protocol.
pub const MAX_SPLIT_SIZE: u16 = 1024;
pub const MAX_ORDER_CHANNELS: u8 = 16;

pub mod net;
pub mod util;

/// The binary protocol used to communicate with the server.
/// THIS IS THE RAW PROTOCOL, DO NOT USE THIS DIRECTLY.
pub mod skyline;
