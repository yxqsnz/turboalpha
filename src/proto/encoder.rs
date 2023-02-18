use std::io::Result;

use monoio::io::AsyncWriteRent;

macro_rules! impl_num {
    ($($name:tt => $ty:ty,)*) => {
        $(
        pub async fn $name<W: AsyncWriteRent>(w: &mut W, value: $ty) -> Result<()> {
            let data = value.to_be_bytes().to_vec();
            let (res, _) = w.write(data).await;
            res?;
            Ok(())
        }
        )*
    };
}

impl_num! {
    put_short => u16,
    put_int => i32,
    put_long => i64,
    put_byte => i8,
}

pub async fn put_string<W: AsyncWriteRent>(w: &mut W, data: &str) -> Result<()> {
    put_short(w, data.len() as u16).await?;

    let str_buf = data.as_bytes().to_vec();

    let (res, _wrote) = w.write(str_buf).await;
    res?;

    Ok(())
}
