use crate::proto::decoder::fetch_string;

use super::Packet;

#[derive(Debug, Default)]
pub struct Handshake {
    pub nick: String,
}

impl Packet for Handshake {
    async fn decode<R: monoio::io::AsyncReadRentExt>(r: &mut R) -> color_eyre::Result<Self> {
        Ok(Handshake {
            nick: fetch_string(r).await?,
        })
    }
}
