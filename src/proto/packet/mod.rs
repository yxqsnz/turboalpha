use color_eyre::Result;
use monoio::io::{AsyncReadRentExt, AsyncWriteRent};

mod handshake;
mod kick;
mod login_request;
pub use handshake::*;
mod login_response;
mod map_chunk;
mod pre_chunk;
pub use kick::*;
pub use login_request::*;
pub use login_response::*;
pub use map_chunk::*;
pub use pre_chunk::*;

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
