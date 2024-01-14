use tokio::sync::mpsc::Sender;

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