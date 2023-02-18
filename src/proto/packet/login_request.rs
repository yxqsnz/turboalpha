use color_eyre::eyre::format_err;

use crate::proto::decoder::fetch_string;

use super::Packet;

pub struct LoginRequest {
    pub protocol_version: i32,
    pub username: String,
    pub password: String,
    pub map_seed: i64,
    pub dimension: i8,
}

impl Packet for LoginRequest {
    async fn decode<R: monoio::io::AsyncReadRentExt>(r: &mut R) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            protocol_version: r.read_i32_le().await?,
            username: fetch_string(r).await?,
            password: fetch_string(r).await?,
            map_seed: r.read_i64_le().await?,
            dimension: r.read_i8_le().await?,
        })
    }

    async fn encode<W: monoio::io::AsyncWriteRent>(&self, _w: &mut W) -> color_eyre::Result<()>
    where
        Self: Sized,
    {
        Err(format_err!("cant encode"))
    }
}
