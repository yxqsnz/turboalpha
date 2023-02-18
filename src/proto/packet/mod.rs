use color_eyre::Result;
use monoio::io::AsyncReadRentExt;

mod handshake;
pub use handshake::*;

pub enum PacketKind {
    Handshake(Handshake),
}

pub trait Packet {
    async fn decode<R: AsyncReadRentExt>(r: &mut R) -> Result<Self>
    where
        Self: Sized;
}
