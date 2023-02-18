use color_eyre::Result;
use monoio::io::{AsyncReadRentExt, AsyncWriteRent};

mod handshake;
mod kick;
mod login_request;
pub use handshake::*;
mod login_response;
mod map_chunk;
mod player_inventory;
mod pre_chunk;
mod spawn_position;

pub use kick::*;
pub use login_request::*;
pub use login_response::*;
pub use map_chunk::*;
pub use player_inventory::{Item, Kind as InventoryKind, PlayerInventory};
pub use pre_chunk::*;
pub use spawn_position::*;

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
