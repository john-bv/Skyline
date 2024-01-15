/// Skyline Specific API helpers.
/// This includes base `Channel` implementation as well as a generic `ProtocolLayer` implementation.
pub mod api;
/// The skyline client implementation.
/// Used as a non-server peer. A connection that can come and go without affecting any services.
pub mod client;
/// Network specific helpers for interfacing the Skyline Handshake Protocol
/// with the underlying network adapter.
pub mod net;
pub mod queue;
/// Utilities to assist with the implementation of the Skyline.
pub mod utils;
