use color_eyre::{eyre::format_err, Result};
use monoio::io::{AsyncReadRent, AsyncReadRentExt};

use crate::proto::packet::LoginRequest;

use self::packet::{Handshake, Packet, PacketKind};

pub mod decoder;
pub mod encoder;

pub mod packet;

pub struct Packets;

impl Packets {
    pub async fn decode<R: AsyncReadRent + AsyncReadRentExt>(r: &mut R) -> Result<PacketKind> {
        tracing::trace!("Reading packet");
        let packet_id = r.read_i8_le().await?;

        tracing::debug!("Trying to decode packet: {packet_id} ({packet_id:X})");
        match packet_id {
            0x02 => Ok(PacketKind::Handshake(Handshake::decode(r).await?)),
            0x01 => Ok(PacketKind::LoginRequest(LoginRequest::decode(r).await?)),
            _ => Err(format_err!("unknown packet: {packet_id}")),
        }
    }
}
