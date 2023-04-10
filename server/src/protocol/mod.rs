pub mod handshake;

pub trait PacketId {
    const ID: u8;
}

#[macro_export]
macro_rules! packet_id {
    ($name: ident, $id: expr) => {
        impl PacketId for $name {
            const ID: u8 = $id;
        }
    };
}