//! Independent durable-frame geometry used by integration evidence.

use std::path::Path;

pub(super) const HEADER_BYTES: usize = 48;
const CHECKSUM_OFFSET: usize = 44;

pub(super) fn payload(bytes: &[u8]) -> &[u8] {
    &bytes[HEADER_BYTES..]
}

pub(super) fn payload_mut(bytes: &mut [u8]) -> &mut [u8] {
    &mut bytes[HEADER_BYTES..]
}

pub(super) fn reseal(bytes: &mut [u8]) {
    let checksum = independent_crc32c(&[&bytes[..CHECKSUM_OFFSET], &bytes[HEADER_BYTES..]]);
    bytes[CHECKSUM_OFFSET..HEADER_BYTES].copy_from_slice(&checksum.to_le_bytes());
}

pub(super) fn artifact_bytes(root: &Path, relative_paths: &[&str]) -> u64 {
    relative_paths
        .iter()
        .map(|path| std::fs::metadata(root.join(path)).unwrap().len())
        .sum()
}

pub(super) fn independent_crc32c(parts: &[&[u8]]) -> u32 {
    let mut crc = !0_u32;
    for part in parts {
        for byte in *part {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0x82f6_3b78 & 0_u32.wrapping_sub(crc & 1));
            }
        }
    }
    !crc
}
