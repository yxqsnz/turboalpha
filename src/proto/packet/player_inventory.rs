use crate::proto::encoder::{put_byte, put_int, put_short};

use super::Packet;

#[derive(Clone, Copy)]
pub enum Kind {
    MainInventory = -1,
    EquippedArmor = -2,
    CrafitingSlots = -3,
}

#[derive(Clone)]
pub struct Item {
    pub id: u16,
    pub count: i8,
    pub used: u16,
}

pub struct PlayerInventory {
    pub kind: Kind,
    pub count: u16,
    pub payload: Vec<Item>,
}

impl Packet for PlayerInventory {
    async fn encode<W: monoio::io::AsyncWriteRent>(&self, w: &mut W) -> color_eyre::Result<()>
    where
        Self: Sized,
    {
        put_short(w, 0x05).await?;

        put_int(w, self.kind as _).await?;
        put_short(w, self.count).await?;

        for item in &self.payload {
            put_short(w, item.id).await?;
            put_byte(w, item.count).await?;
            put_short(w, item.used).await?;
        }

        Ok(())
    }

    async fn decode<R: monoio::io::AsyncReadRentExt>(_r: &mut R) -> color_eyre::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
