//! Length-prefixed JSON framing shared by the rail server and its clients.

use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const MAX_FRAME_BYTES: u32 = 64 * 1024;

/// Writes one length-prefixed frame.
pub async fn write_frame<S, T>(stream: &mut S, value: &T) -> std::io::Result<()>
where
    S: AsyncWrite + Unpin,
    T: Serialize,
{
    let payload = serde_json::to_vec(value).expect("rail protocol values always serialize");
    let length = u32::try_from(payload.len()).expect("rail protocol frames stay well under u32");
    stream.write_all(&length.to_le_bytes()).await?;
    stream.write_all(&payload).await?;
    stream.flush().await
}

/// One frame read outcome: a decoded value, or a clean disconnect with zero
/// bytes read for this frame attempt.
pub enum FrameRead<T> {
    Frame(T),
    Disconnected,
}

/// Reads one length-prefixed frame, or observes disconnection.
///
/// A partial frame followed by EOF is treated as an I/O error, not a clean
/// disconnect: a real network boundary can still distinguish "nothing sent
/// yet" from "a write started and never finished".
pub async fn read_frame<S, T>(stream: &mut S) -> std::io::Result<FrameRead<T>>
where
    S: AsyncRead + Unpin,
    T: DeserializeOwned,
{
    let mut length_bytes = [0u8; 4];
    if !read_exact_or_eof(stream, &mut length_bytes).await? {
        return Ok(FrameRead::Disconnected);
    }
    let length = u32::from_le_bytes(length_bytes);
    if length > MAX_FRAME_BYTES {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "rail frame exceeds maximum size",
        ));
    }
    let mut payload = vec![0u8; length as usize];
    if !read_exact_or_eof(stream, &mut payload).await? {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "rail connection closed mid-frame",
        ));
    }
    let value = serde_json::from_slice(&payload)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    Ok(FrameRead::Frame(value))
}

/// Reads exactly `buffer.len()` bytes, or returns `Ok(false)` if the peer
/// disconnects before the first byte of this read lands.
async fn read_exact_or_eof<S: AsyncRead + Unpin>(
    stream: &mut S,
    buffer: &mut [u8],
) -> std::io::Result<bool> {
    let mut filled = 0;
    while filled < buffer.len() {
        let read = stream.read(&mut buffer[filled..]).await?;
        if read == 0 {
            if filled == 0 {
                return Ok(false);
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "rail connection closed mid-frame",
            ));
        }
        filled += read;
    }
    Ok(true)
}
