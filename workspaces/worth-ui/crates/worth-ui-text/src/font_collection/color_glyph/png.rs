use std::io::Cursor;

use crate::font_collection::UiFontCollectionAdmissionDenial;

use super::malformed;

pub(super) fn validate_png(bytes: &[u8]) -> Result<(), UiFontCollectionAdmissionDenial> {
    validate_chunks(bytes)?;
    let decoder = png::Decoder::new_with_limits(
        Cursor::new(bytes),
        png::Limits {
            bytes: 2 * 1024 * 1024,
        },
    );
    let mut reader = decoder.read_info().map_err(|_| malformed())?;
    let width = reader.info().width;
    let height = reader.info().height;
    if width == 0
        || height == 0
        || width > 512
        || height > 512
        || reader.info().animation_control.is_some()
    {
        return Err(malformed());
    }
    let output_size = reader.output_buffer_size().ok_or_else(malformed)?;
    if output_size > 512 * 512 * 4 {
        return Err(malformed());
    }
    let mut output = vec![0; output_size];
    let decoded = reader.next_frame(&mut output).map_err(|_| malformed())?;
    if decoded.width != width || decoded.height != height {
        return Err(malformed());
    }
    reader.finish().map_err(|_| malformed())
}

fn validate_chunks(bytes: &[u8]) -> Result<(), UiFontCollectionAdmissionDenial> {
    if bytes.get(..8) != Some(b"\x89PNG\r\n\x1a\n") {
        return Err(malformed());
    }
    let mut cursor = 8;
    let mut ihdr = false;
    let mut idat = false;
    let mut iend = false;
    while cursor < bytes.len() {
        let length = usize::try_from(be_u32(bytes, cursor).ok_or_else(malformed)?)
            .map_err(|_| malformed())?;
        let kind = bytes.get(cursor + 4..cursor + 8).ok_or_else(malformed)?;
        let data_start = cursor.checked_add(8).ok_or_else(malformed)?;
        let data_end = data_start.checked_add(length).ok_or_else(malformed)?;
        let crc_end = data_end.checked_add(4).ok_or_else(malformed)?;
        let data = bytes.get(data_start..data_end).ok_or_else(malformed)?;
        let expected = be_u32(bytes, data_end).ok_or_else(malformed)?;
        if png_crc(kind, data) != expected {
            return Err(malformed());
        }
        match kind {
            b"IHDR" if !ihdr && cursor == 8 && length == 13 => {
                let width = be_u32(data, 0).ok_or_else(malformed)?;
                let height = be_u32(data, 4).ok_or_else(malformed)?;
                if width == 0 || height == 0 || width > 512 || height > 512 {
                    return Err(malformed());
                }
                ihdr = true;
            }
            b"IDAT" if ihdr && !iend => idat = true,
            b"IEND" if ihdr && idat && length == 0 => iend = true,
            _ if iend => return Err(malformed()),
            _ => {}
        }
        cursor = crc_end;
    }
    if ihdr && idat && iend && cursor == bytes.len() {
        Ok(())
    } else {
        Err(malformed())
    }
}

pub(super) fn png_crc(kind: &[u8], data: &[u8]) -> u32 {
    kind.iter().chain(data).fold(u32::MAX, |crc, byte| {
        (0..8).fold(crc ^ u32::from(*byte), |value, _| {
            (value >> 1) ^ (0xEDB8_8320 & 0_u32.wrapping_sub(value & 1))
        })
    }) ^ u32::MAX
}

fn be_u32(bytes: &[u8], start: usize) -> Option<u32> {
    Some(u32::from_be_bytes(
        bytes.get(start..start.checked_add(4)?)?.try_into().ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::validate_png;

    #[test]
    fn crc_correct_but_undecodable_png_is_rejected() {
        let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
        append_chunk(
            &mut bytes,
            b"IHDR",
            &[0, 0, 0, 1, 0, 0, 0, 1, 8, 99, 0, 0, 0],
        );
        append_chunk(&mut bytes, b"IDAT", b"not a zlib stream");
        append_chunk(&mut bytes, b"IEND", &[]);
        assert!(validate_png(&bytes).is_err());
    }

    fn append_chunk(output: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
        output.extend_from_slice(&(data.len() as u32).to_be_bytes());
        output.extend_from_slice(kind);
        output.extend_from_slice(data);
        output.extend_from_slice(&super::png_crc(kind, data).to_be_bytes());
    }
}
