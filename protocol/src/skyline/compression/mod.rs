use binary_util::BinaryIo;

#[derive(Debug, BinaryIo)]
#[repr(u8)]
pub enum CompressionAlgorithm {
    None,
    Zlib,
    Gzip,
}

#[derive(Debug, BinaryIo)]
pub struct CompressedMessage {
    pub algorithm: CompressionAlgorithm,
    pub message: Vec<u8>,
}
