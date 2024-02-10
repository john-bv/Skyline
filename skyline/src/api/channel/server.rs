use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::net::ConnAdapter;

pub struct Channel {
    pub peers: Vec<Box<dyn ConnAdapter>>,
    pub name: String,
}

impl Channel {
    pub fn new(name: String) -> Self {
        Self {
            peers: Vec::new(),
            name,
        }
    }
}

/// ## Server Channel Pool
/// A channel Pool is a collection of channels.
/// You can think of it as Multiple channels on a single server.
///
/// This struct is designed to be used both on a client and a server.
///
/// You can digest a message inside a pool using the `digest` method.
#[derive(Clone)]
pub struct ChannelPool {
    // cursed lmao
    pub channels: Arc<RefCell<HashMap<String, Box<Channel>>>>,
}

impl ChannelPool {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Digest a message inside the pool.
    /// This will send the message to all channels inside the pool for processing.
    pub fn digest(&mut self, message: Vec<u8>) {}

    pub fn add_channel(&mut self, channel: Box<Channel>) {
        let mut chans = self.channels.borrow_mut();
        chans.insert(channel.name.clone(), channel);
    }
}
