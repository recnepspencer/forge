#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoringSnapshotDigest(u64);

impl WorthUiAuthoringSnapshotDigest {
    pub(crate) fn from_basis(parts: &[String]) -> Self {
        let mut digest = 0xcbf2_9ce4_8422_2325;
        for part in parts {
            digest = fold_bytes(digest, part.as_bytes());
            digest = fold_bytes(digest, b"\n");
        }
        Self(digest)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

pub(crate) fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
