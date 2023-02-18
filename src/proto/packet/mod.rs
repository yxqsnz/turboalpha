use color_eyre::Result;
use monoio::io::{AsyncReadRentExt, AsyncWriteRent};

mod handshake;
mod kick;
mod login_request;

pub use handshake::*;
pub use kick::*;
pub use login_request::*;

pub enum PacketKind {
    Handshake(Handshake),
    LoginRequest(LoginRequest),
}

pub trait Packet {
    async fn decode<R: AsyncReadRentExt>(r: &mut R) -> Result<Self>
    where
        Self: Sized;

    async fn encode<W: AsyncWriteRent>(&self, w: &mut W) -> Result<()>
    where
        Self: Sized;
}
