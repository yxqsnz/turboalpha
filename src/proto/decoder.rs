use color_eyre::Result;

use monoio::io::AsyncReadRentExt;

pub async fn fetch_string<R: AsyncReadRentExt>(reader: &mut R) -> Result<String> {
    let size = reader.read_i16().await?;

    tracing::trace!("Trying to read string with size: {size}");

    let mut buf = Vec::with_capacity(size as _);
    let (res, new_buf) = reader.read_exact(buf).await;
    res?;
    buf = new_buf;

    Ok(String::from_utf8(buf)?)
}
