use color_eyre::eyre::format_err;

use crate::proto::encoder::{put_byte, put_int, put_long, put_short, put_string};

use super::Packet;

pub struct LoginResponse {
    pub entity_id: i32,
    // Not used.
    pub server_name: String,
    // Not used.
    pub server_motd: String,
    pub map_seed: i64,
    pub dimension: i8,
}

impl Packet for LoginResponse {
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
        put_short(w, 0x01).await?;
        put_int(w, self.entity_id).await?;
        put_string(w, &self.server_name).await?;
        put_string(w, &self.server_motd).await?;
        put_long(w, self.map_seed).await?;
        put_byte(w, self.dimension).await?;

        Ok(())
    }
}
