use sha2::{Digest, Sha256};

use super::super::WalFacts;

const HEADER_BYTES: usize = 116;
const FOOTER_BYTES: usize = 32;

pub(super) struct WalTailSuffix {
    pub(super) bytes: Vec<u8>,
    pub(super) first_lsn: u64,
    pub(super) last_lsn: u64,
}

pub(super) fn select(
    bytes: &[u8],
    facts: WalFacts,
    frontier: u64,
) -> Result<Option<WalTailSuffix>, String> {
    let valid = usize::try_from(facts.valid_bytes)
        .map_err(|_| "selected-basis WAL prefix is too large".to_owned())?;
    if valid > bytes.len() {
        return Err("selected-basis WAL prefix exceeds artifact".to_owned());
    }
    let mut offset = 0;
    let mut selected_offset = None;
    let mut first_lsn = None;
    let mut last_lsn = None;
    while offset < valid {
        let frame = read_frame(&bytes[offset..valid])?;
        first_lsn.get_or_insert(frame.start_lsn);
        last_lsn = Some(frame.end_lsn);
        if frame.start_lsn == frontier {
            selected_offset = Some(offset);
        } else if frame.start_lsn > frontier && selected_offset.is_none() {
            return Err("selected-basis WAL frontier is not a complete frame start".to_owned());
        }
        offset = offset
            .checked_add(frame.length)
            .ok_or_else(|| "selected-basis WAL frame offset overflowed".to_owned())?;
    }
    if offset != valid {
        return Err("selected-basis WAL valid prefix ended inside a frame".to_owned());
    }
    let first_lsn =
        first_lsn.ok_or_else(|| "selected-basis WAL omitted its first LSN".to_owned())?;
    let last_lsn = last_lsn.ok_or_else(|| "selected-basis WAL omitted its last LSN".to_owned())?;
    if last_lsn <= frontier {
        return Ok(None);
    }
    let start = selected_offset.ok_or_else(|| {
        "selected-basis WAL frontier is absent from the complete frame sequence".to_owned()
    })?;
    Ok(Some(WalTailSuffix {
        bytes: bytes[start..valid].to_vec(),
        first_lsn: if start == 0 { first_lsn } else { frontier },
        last_lsn,
    }))
}

struct Frame {
    length: usize,
    start_lsn: u64,
    end_lsn: u64,
}

fn read_frame(bytes: &[u8]) -> Result<Frame, String> {
    let header = bytes
        .get(..HEADER_BYTES)
        .ok_or_else(|| "selected-basis WAL frame header is truncated".to_owned())?;
    if header.get(..8) != Some(b"WORTHWAL")
        || read_u16(header, 8) != Some(1)
        || read_u16(header, 10) != Some(116)
    {
        return Err("selected-basis WAL frame header is invalid".to_owned());
    }
    let payload_bytes = usize::try_from(
        read_u64(header, 44)
            .ok_or_else(|| "selected-basis WAL frame omitted payload length".to_owned())?,
    )
    .map_err(|_| "selected-basis WAL payload length overflowed".to_owned())?;
    let length = HEADER_BYTES
        .checked_add(payload_bytes)
        .and_then(|value| value.checked_add(FOOTER_BYTES))
        .ok_or_else(|| "selected-basis WAL frame length overflowed".to_owned())?;
    let frame = bytes
        .get(..length)
        .ok_or_else(|| "selected-basis WAL frame is truncated".to_owned())?;
    let start_lsn = read_u64(header, 28)
        .ok_or_else(|| "selected-basis WAL frame omitted start LSN".to_owned())?;
    let end_lsn = read_u64(header, 36)
        .ok_or_else(|| "selected-basis WAL frame omitted end LSN".to_owned())?;
    let payload = &frame[HEADER_BYTES..HEADER_BYTES + payload_bytes];
    let payload_digest: [u8; 32] = Sha256::digest(payload).into();
    let frame_digest: [u8; 32] = Sha256::digest(&frame[..HEADER_BYTES + payload_bytes]).into();
    if read_u64(header, 12).is_none()
        || read_u64(header, 20).is_none()
        || payload_bytes == 0
        || start_lsn >= end_lsn
        || header[84..116] != payload_digest
        || frame[HEADER_BYTES + payload_bytes..] != frame_digest
    {
        return Err("selected-basis WAL frame integrity is invalid".to_owned());
    }
    Ok(Frame {
        length,
        start_lsn,
        end_lsn,
    })
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    bytes
        .get(offset..offset + 2)?
        .try_into()
        .ok()
        .map(u16::from_le_bytes)
}

fn read_u64(bytes: &[u8], offset: usize) -> Option<u64> {
    bytes
        .get(offset..offset + 8)?
        .try_into()
        .ok()
        .map(u64::from_le_bytes)
}

#[cfg(test)]
mod tests {
    use super::{select, WalFacts};
    use sha2::{Digest, Sha256};

    #[test]
    fn checkpoint_straddling_segment_selects_complete_suffix() {
        let first = frame(10, b"covered");
        let second = frame(11, b"selected");
        let mut bytes = first.clone();
        bytes.extend_from_slice(&second);
        let suffix = select(&bytes, facts(&bytes), 11)
            .unwrap_or_else(|error| {
                panic!(
                    "MUTANT_PREDICATE:c8-parent-selector-rejects-checkpoint-straddling-wal\n{error}"
                )
            })
            .expect("post-checkpoint suffix");
        assert_eq!(suffix.first_lsn, 11);
        assert_eq!(suffix.last_lsn, 12);
        assert_eq!(suffix.bytes, second);
        assert_ne!(suffix.bytes, bytes);
    }

    #[test]
    fn frontier_inside_frame_is_rejected() {
        let bytes = frame_with_end(10, 12, b"wide");
        assert!(select(&bytes, facts(&bytes), 11).is_err());
    }

    #[test]
    fn torn_terminal_suffix_is_excluded_from_selected_bytes() {
        let first = frame(10, b"covered");
        let second = frame(11, b"selected");
        let mut bytes = first.clone();
        bytes.extend_from_slice(&second);
        bytes.extend_from_slice(b"torn-terminal-residue");
        let valid = first.len() + second.len();
        let suffix = select(&bytes, facts_with_valid_bytes(&bytes, valid), 11)
            .unwrap()
            .unwrap();
        assert_eq!(suffix.bytes, second);
        assert_eq!(suffix.bytes.len(), valid - first.len());
    }

    #[test]
    fn covered_segment_is_omitted() {
        let bytes = frame(10, b"covered");
        assert!(select(&bytes, facts(&bytes), 11).unwrap().is_none());
    }

    #[test]
    fn trimmed_first_segment_remains_contiguous_with_physical_successor() {
        assert!(super::super::validate_wal_continuation((1, 1), 1, 11, 11, 12, 11, None).is_ok());
        assert!(
            super::super::validate_wal_continuation((2, 1), 1, 11, 12, 13, 12, Some(1)).is_ok()
        );
    }

    fn facts(bytes: &[u8]) -> WalFacts {
        facts_with_valid_bytes(bytes, bytes.len())
    }

    fn facts_with_valid_bytes(bytes: &[u8], valid_bytes: usize) -> WalFacts {
        WalFacts {
            segment: Some(1),
            generation: Some(1),
            valid_bytes: valid_bytes as u64,
            observed_bytes: bytes.len() as u64,
            frames: 2,
            first: Some(10),
            last: Some(12),
            digest: [0; 32],
        }
    }

    fn frame(start: u64, payload: &[u8]) -> Vec<u8> {
        frame_with_end(start, start + 1, payload)
    }

    fn frame_with_end(start: u64, end: u64, payload: &[u8]) -> Vec<u8> {
        let mut header = [0; 116];
        header[..8].copy_from_slice(b"WORTHWAL");
        header[8..10].copy_from_slice(&1u16.to_le_bytes());
        header[10..12].copy_from_slice(&116u16.to_le_bytes());
        header[12..20].copy_from_slice(&1u64.to_le_bytes());
        header[20..28].copy_from_slice(&1u64.to_le_bytes());
        header[28..36].copy_from_slice(&start.to_le_bytes());
        header[36..44].copy_from_slice(&end.to_le_bytes());
        header[44..52].copy_from_slice(&(payload.len() as u64).to_le_bytes());
        header[84..116].copy_from_slice(&Sha256::digest(payload));
        let mut frame = header.to_vec();
        frame.extend_from_slice(payload);
        let digest: [u8; 32] = Sha256::digest(&frame).into();
        frame.extend_from_slice(&digest);
        frame
    }
}
