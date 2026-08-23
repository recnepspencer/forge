use crate::font_collection::UiFontCollectionAdmissionDenial;

#[derive(Clone, Copy)]
struct TableRecord {
    tag: [u8; 4],
    checksum: u32,
    offset: usize,
    length: usize,
}

pub(super) fn validate(
    bytes: &[u8],
    face_index: u32,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    use UiFontCollectionAdmissionDenial as Denial;
    match bytes.get(..4) {
        Some(b"wOFF" | b"wOF2") => Err(Denial::UnsupportedFontContainer),
        Some(b"ttcf") => validate_collection(bytes, face_index),
        Some(b"\0\x01\0\0" | b"OTTO" | b"true") if face_index == 0 => validate_face(bytes, 0, 0),
        _ => Err(Denial::MalformedFont),
    }
}

fn validate_collection(
    bytes: &[u8],
    face_index: u32,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    let version = be_u32(bytes, 4)?;
    if !matches!(version, 0x0001_0000 | 0x0002_0000) {
        return malformed();
    }
    let count = usize::try_from(be_u32(bytes, 8)?).map_err(|_| denial())?;
    let selected = usize::try_from(face_index).map_err(|_| denial())?;
    if count == 0
        || count > crate::UiGlobalTextProfile::MAX_APPLICATION_FONT_FACES
        || selected >= count
    {
        return malformed();
    }
    let offsets_end = 12usize
        .checked_add(count.checked_mul(4).ok_or_else(denial)?)
        .ok_or_else(denial)?;
    if offsets_end > bytes.len() {
        return malformed();
    }
    let offset = usize::try_from(be_u32(bytes, 12 + selected * 4)?).map_err(|_| denial())?;
    if !offset.is_multiple_of(4) {
        return malformed();
    }
    validate_face(bytes, offset, offsets_end)
}

fn validate_face(
    bytes: &[u8],
    face_offset: usize,
    container_header_end: usize,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    let scaler = bytes.get(face_offset..face_offset + 4).ok_or_else(denial)?;
    if !matches!(scaler, b"\0\x01\0\0" | b"OTTO" | b"true") {
        return malformed();
    }
    let table_count = usize::from(be_u16(bytes, face_offset + 4)?);
    if table_count == 0 {
        return malformed();
    }
    validate_search_fields(bytes, face_offset, table_count)?;
    let directory_end = face_offset
        .checked_add(12)
        .and_then(|start| start.checked_add(table_count.checked_mul(16)?))
        .ok_or_else(denial)?;
    if directory_end > bytes.len() {
        return malformed();
    }
    let mut records = Vec::with_capacity(table_count);
    for index in 0..table_count {
        let start = face_offset + 12 + index * 16;
        let record = TableRecord {
            tag: bytes[start..start + 4].try_into().map_err(|_| denial())?,
            checksum: be_u32(bytes, start + 4)?,
            offset: usize::try_from(be_u32(bytes, start + 8)?).map_err(|_| denial())?,
            length: usize::try_from(be_u32(bytes, start + 12)?).map_err(|_| denial())?,
        };
        if !record.offset.is_multiple_of(4)
            || record
                .offset
                .checked_add(record.length)
                .is_none_or(|end| end > bytes.len())
            || overlaps(record.offset, record.length, 0, container_header_end)
            || overlaps(
                record.offset,
                record.length,
                face_offset,
                directory_end - face_offset,
            )
            || records
                .last()
                .is_some_and(|prior: &TableRecord| prior.tag >= record.tag)
        {
            return malformed();
        }
        records.push(record);
    }
    let mut ranges = records
        .iter()
        .filter(|record| record.length != 0)
        .map(|record| (record.offset, record.offset + record.length))
        .collect::<Vec<_>>();
    ranges.sort_unstable();
    if ranges.windows(2).any(|pair| pair[0].1 > pair[1].0) {
        return malformed();
    }
    for record in records {
        if table_checksum(bytes, record) != record.checksum {
            return malformed();
        }
    }
    Ok(())
}

fn overlaps(left: usize, left_len: usize, right: usize, right_len: usize) -> bool {
    left_len != 0
        && right_len != 0
        && left < right.saturating_add(right_len)
        && right < left.saturating_add(left_len)
}

fn validate_search_fields(
    bytes: &[u8],
    face_offset: usize,
    table_count: usize,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    let power = 1usize << table_count.ilog2();
    let search_range = power.checked_mul(16).ok_or_else(denial)?;
    let range_shift = table_count
        .checked_mul(16)
        .and_then(|total| total.checked_sub(search_range))
        .ok_or_else(denial)?;
    if usize::from(be_u16(bytes, face_offset + 6)?) != search_range
        || usize::from(be_u16(bytes, face_offset + 8)?) != power.ilog2() as usize
        || usize::from(be_u16(bytes, face_offset + 10)?) != range_shift
    {
        return malformed();
    }
    Ok(())
}

fn table_checksum(bytes: &[u8], record: TableRecord) -> u32 {
    let mut sum = 0u32;
    let mut word = [0u8; 4];
    for relative in (0..record.length).step_by(4) {
        word.fill(0);
        let available = (record.length - relative).min(4);
        word[..available].copy_from_slice(
            &bytes[record.offset + relative..record.offset + relative + available],
        );
        if record.tag == *b"head" && relative == 8 {
            word.fill(0);
        }
        sum = sum.wrapping_add(u32::from_be_bytes(word));
    }
    sum
}

fn be_u16(bytes: &[u8], start: usize) -> Result<u16, UiFontCollectionAdmissionDenial> {
    bytes
        .get(start..start + 2)
        .and_then(|value| value.try_into().ok())
        .map(u16::from_be_bytes)
        .ok_or_else(denial)
}

fn be_u32(bytes: &[u8], start: usize) -> Result<u32, UiFontCollectionAdmissionDenial> {
    bytes
        .get(start..start + 4)
        .and_then(|value| value.try_into().ok())
        .map(u32::from_be_bytes)
        .ok_or_else(denial)
}

const fn denial() -> UiFontCollectionAdmissionDenial {
    UiFontCollectionAdmissionDenial::MalformedFont
}

fn malformed<T>() -> Result<T, UiFontCollectionAdmissionDenial> {
    Err(denial())
}
