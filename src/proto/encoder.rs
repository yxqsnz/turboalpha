use std::io::Result;

use monoio::io::AsyncWriteRent;

pub async fn put_short<W: AsyncWriteRent>(w: &mut W, value: u16) -> Result<()> {
    let data = value.to_be_bytes().to_vec();
    let (res, _) = w.write(data).await;
    res?;
    Ok(())
}

pub async fn put_string<W: AsyncWriteRent>(w: &mut W, data: &str) -> Result<()> {
    put_short(w, data.len() as u16).await?;

    let str_buf = data.as_bytes().to_vec();

    let (res, _wrote) = w.write(str_buf).await;
    res?;

    Ok(())
}
