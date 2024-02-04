use std::sync::Arc;

use tokio::sync::RwLock;

use crate::client::Client;


pub mod client;
pub mod server;

pub const CX_FIXED: usize = 1048;