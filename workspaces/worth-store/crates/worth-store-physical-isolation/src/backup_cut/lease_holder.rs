#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BackupReachabilityLeaseHolderId([u8; 32]);

impl BackupReachabilityLeaseHolderId {
    pub const fn from_stable_identity(identity: [u8; 32]) -> Self {
        Self(identity)
    }

    pub const fn fingerprint(self) -> [u8; 32] {
        self.0
    }
}
