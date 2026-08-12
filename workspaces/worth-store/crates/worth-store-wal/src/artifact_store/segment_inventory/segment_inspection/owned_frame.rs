use crate::WalLsnRange;

/// Owned payload copied only from a frame admitted by the bounded WAL scanner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedWalFrame {
    pub(super) lsn_range: WalLsnRange,
    pub(super) payload: Box<[u8]>,
    pub(super) encoded_bytes: u64,
}

impl VerifiedWalFrame {
    pub const fn lsn_range(&self) -> WalLsnRange {
        self.lsn_range
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub const fn encoded_bytes(&self) -> u64 {
        self.encoded_bytes
    }
}
