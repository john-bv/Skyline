use binary_util::{types::varu32, BinaryIo};

#[derive(Debug, Clone, Copy, BinaryIo)]
pub struct Shard {
    pub id: varu32
}