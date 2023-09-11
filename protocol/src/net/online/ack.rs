use binary_util::BinaryIo;

#[derive(BinaryIo)]
#[repr(u8)]
pub enum AckVariant {
    /// Acknowledge a packet.
    Ack(Acknowledgement) = 1,
    /// Request acknowledgement for a packet.
    /// No Acknowledgement
    Nack(Acknowledgement) = 0,
}

#[derive(BinaryIo, Clone)]
pub struct Acknowledgement {
    /// A list of sequences we are missing.
    /// (In no particular order)
    pub seqs: Vec<u32>
}

impl Acknowledgement {
    pub fn new() -> Self {
        Self {
            seqs: Vec::new()
        }
    }

    pub fn add(&mut self, seq: u32) {
        self.seqs.push(seq);
    }

    pub fn to_nack(&self) -> AckVariant {
        AckVariant::Nack(self.clone())
    }

    pub fn to_ack(&self) -> AckVariant {
        AckVariant::Ack(self.clone())
    }
}