use crate::proto::{
    decoder::fetch_string,
    encoder::{put_short, put_string},
};

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

    async fn encode<W: monoio::io::AsyncWriteRent>(&self, w: &mut W) -> color_eyre::Result<()>
    where
        Self: Sized,
    {
        put_short(w, 0x02).await?;
        put_string(w, &self.nick).await?;

        Ok(())
    }
}
