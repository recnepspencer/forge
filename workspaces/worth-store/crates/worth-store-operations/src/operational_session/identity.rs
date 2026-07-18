use sha2::{Digest, Sha256};

use crate::OperationalOperationId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationalSessionIdentity([u8; 32]);

impl OperationalSessionIdentity {
    pub fn from_operation(operation: &OperationalOperationId) -> Self {
        let mut digest = Sha256::new();
        digest.update(b"worth-store-operational-session-v1");
        digest.update(operation.as_str().as_bytes());
        Self(digest.finalize().into())
    }

    pub const fn fingerprint(self) -> [u8; 32] {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationalSessionKind {
    Backup,
    Restore,
    PointInTimeRecovery,
    Rollback,
    Repair,
    ReplicaBootstrap,
    ReplicaPromotion,
    ForensicAcquisition,
    OfflineVerification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationalSessionDisposition {
    Completed,
    Abandoned,
}
