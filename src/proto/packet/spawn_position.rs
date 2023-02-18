use crate::proto::encoder::{put_int, put_short};

use super::Packet;

pub struct SpawnPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Packet for SpawnPosition {
    async fn encode<W: monoio::io::AsyncWriteRent>(&self, w: &mut W) -> color_eyre::Result<()>
    where
        Self: Sized,
    {
        put_short(w, 0x06).await?;
        put_int(w, self.x).await?;
        put_int(w, self.y).await?;
        put_int(w, self.z).await?;

        Ok(())
    }

    async fn decode<R: monoio::io::AsyncReadRentExt>(_r: &mut R) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
