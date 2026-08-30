pub(super) const MANIFEST_PAYLOAD_BYTES: u32 = 104;
pub(crate) const MANIFEST_FRAME_BYTES: u64 = 8 + 2 + 4 + MANIFEST_PAYLOAD_BYTES as u64;
