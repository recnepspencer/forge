use std::path::Path;

use sha2::{Digest, Sha256};

const PREFIX_BYTES: usize = 16;
const COMPACTION_DOMAIN: &[u8] =
    worth_store_physical_format::PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN;

struct CompletedRecordList {
    frame_start: usize,
    start: usize,
    end: usize,
}

/// Exchange two completed terminal record lists in an already-published
/// checkpoint. Each binding record remains canonically encoded and the
/// checkpoint footer is rehashed, while the operation-to-record association
/// no longer agrees with the persisted root.
pub(super) fn swap_completed_record_lists(root: &Path) {
    let path = root.join("families/checkpoint.current");
    let mut bytes = std::fs::read(&path).expect("read checkpoint for tuple mutation");
    let mut completed = Vec::new();
    let mut footer = None;
    let mut offset = 0;
    while offset < bytes.len() {
        let (kind, payload_start, payload_end, total) = checkpoint_frame(&bytes, offset);
        if kind == 4 {
            if let Some((start, end)) = completed_record_list(&bytes[payload_start..payload_end]) {
                completed.push(CompletedRecordList {
                    frame_start: offset,
                    start: payload_start + start,
                    end: payload_start + end,
                });
            }
        } else if kind == 5 {
            footer = Some(offset);
        }
        offset += total;
    }
    assert!(
        completed.len() >= 2,
        "checkpoint tuple mutation requires at least two completed terminal bindings"
    );
    let first = &completed[0];
    let second = &completed[1];
    assert_eq!(
        first.end - first.start,
        second.end - second.start,
        "checkpoint tuple mutation requires equal persisted record-list widths"
    );
    let first_list = bytes[first.start..first.end].to_vec();
    let second_list = bytes[second.start..second.end].to_vec();
    bytes[first.start..first.end].copy_from_slice(&second_list);
    bytes[second.start..second.end].copy_from_slice(&first_list);
    repair_frame_crc(&mut bytes, first.frame_start);
    repair_frame_crc(&mut bytes, second.frame_start);
    let footer = footer.expect("checkpoint tuple mutation requires a footer");
    repair_binding_digest(&mut bytes, footer);
    assert_frame_checksums(&bytes);
    worth_store_physical_format::inspect_checkpoint_stream(&bytes, u64::MAX, u64::MAX)
        .expect("rehashed checkpoint tuple mutation must retain physical checkpoint integrity");
    std::fs::write(path, bytes).expect("write checkpoint tuple mutation");
}

fn checkpoint_frame(bytes: &[u8], offset: usize) -> (u8, usize, usize, usize) {
    let prefix = bytes
        .get(offset..offset + PREFIX_BYTES)
        .expect("checkpoint mutation frame prefix");
    assert_eq!(&prefix[..8], b"WCP7REC\0");
    assert_eq!(prefix[8], 1);
    let payload_bytes = u32::from_le_bytes(prefix[12..16].try_into().unwrap()) as usize;
    let payload_start = offset + PREFIX_BYTES;
    let payload_end = payload_start + payload_bytes;
    let total = payload_bytes + PREFIX_BYTES + 4;
    assert!(
        offset + total <= bytes.len(),
        "checkpoint mutation frame bounds"
    );
    (prefix[9], payload_start, payload_end, total)
}

fn completed_record_list(payload: &[u8]) -> Option<(usize, usize)> {
    let mut cursor = Cursor::new(payload);
    if cursor.field()? != COMPACTION_DOMAIN || cursor.byte()? != 3 {
        return None;
    }
    for _ in 0..3 {
        cursor.field()?;
    }
    cursor.u64()?;
    cursor.u64()?;
    cursor.field()?;
    cursor.field()?;
    cursor.field()?;
    cursor.u64()?;
    cursor.u64()?;
    cursor.u64()?;
    if cursor.byte()? != 2 {
        return None;
    }
    cursor.field()?;
    cursor.u32()?;
    cursor.u64()?;
    let count = cursor.u32()?;
    if count == 0 {
        return None;
    }
    let start = cursor.offset;
    for _ in 0..count {
        if cursor.field()?.len() != 16 {
            return None;
        }
        cursor.u64()?;
    }
    let end = cursor.offset;
    for _ in 0..13 {
        cursor.u64()?;
    }
    cursor.is_empty().then_some((start, end))
}

fn repair_frame_crc(bytes: &mut [u8], frame_start: usize) {
    let payload_bytes = u32::from_le_bytes(
        bytes[frame_start + 12..frame_start + 16]
            .try_into()
            .unwrap(),
    ) as usize;
    let crc_offset = frame_start + PREFIX_BYTES + payload_bytes;
    let crc = crc32c(&bytes[frame_start..crc_offset]);
    bytes[crc_offset..crc_offset + 4].copy_from_slice(&crc.to_le_bytes());
}

fn assert_frame_checksums(bytes: &[u8]) {
    let mut offset = 0;
    while offset < bytes.len() {
        let (_, _, payload_end, total) = checkpoint_frame(bytes, offset);
        let expected = u32::from_le_bytes(bytes[payload_end..payload_end + 4].try_into().unwrap());
        let actual = crc32c(&bytes[offset..payload_end]);
        assert_eq!(actual, expected, "checkpoint mutation checksum at {offset}");
        offset += total;
    }
}

fn repair_binding_digest(bytes: &mut [u8], footer_start: usize) {
    let mut digest = Sha256::new();
    let mut offset = 0;
    while offset < footer_start {
        let (kind, _, _, total) = checkpoint_frame(bytes, offset);
        if kind == 4 {
            digest.update(&bytes[offset..offset + total]);
        }
        offset += total;
    }
    let footer_payload = footer_start + PREFIX_BYTES;
    bytes[footer_payload + 104..footer_payload + 136].copy_from_slice(&digest.finalize());
    repair_frame_crc(bytes, footer_start);
}

struct Cursor<'bytes> {
    bytes: &'bytes [u8],
    offset: usize,
}

impl<'bytes> Cursor<'bytes> {
    const fn new(bytes: &'bytes [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn field(&mut self) -> Option<&'bytes [u8]> {
        let length = usize::try_from(self.u64()?).ok()?;
        self.take(length)
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }

    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }

    fn byte(&mut self) -> Option<u8> {
        Some(*self.take(1)?.first()?)
    }

    fn take(&mut self, length: usize) -> Option<&'bytes [u8]> {
        let end = self.offset.checked_add(length)?;
        let value = self.bytes.get(self.offset..end)?;
        self.offset = end;
        Some(value)
    }

    const fn is_empty(&self) -> bool {
        self.offset == self.bytes.len()
    }
}

fn crc32c(bytes: &[u8]) -> u32 {
    let mut value = !0_u32;
    for byte in bytes {
        value ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(value & 1);
            value = (value >> 1) ^ (0x82f6_3b78 & mask);
        }
    }
    !value
}
