use color_eyre::eyre::format_err;

use crate::proto::encoder::{put_bool, put_int, put_short};

use super::Packet;

pub struct PreChunk {
    pub x: i32,
    pub z: i32,
    pub mode: bool,
}

impl Packet for PreChunk {
    async fn decode<R: monoio::io::AsyncReadRentExt>(_r: &mut R) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        Err(format_err!("cant decode"))
    }

    async fn encode<W: monoio::io::AsyncWriteRent>(&self, w: &mut W) -> color_eyre::Result<()>
    where
        Self: Sized,
    {
        put_short(w, 0x32).await?;
        put_int(w, self.x).await?;
        put_int(w, self.z).await?;
        put_bool(w, self.mode).await?;

        Ok(())
    }
}
