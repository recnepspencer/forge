use sha2::{Digest, Sha256};

use super::{
    directory_durability::sync_directory, RecoverySourceLeaseDenial,
    RecoverySourceReachabilityLease,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoverySourceLeaseReleaseReceipt {
    lease_identity: [u8; 32],
    source_identity: [u8; 32],
    source_evidence_identity: [u8; 32],
}

impl RecoverySourceLeaseReleaseReceipt {
    pub const fn lease_identity(self) -> [u8; 32] {
        self.lease_identity
    }

    pub const fn source_identity(self) -> [u8; 32] {
        self.source_identity
    }

    pub const fn source_evidence_identity(self) -> [u8; 32] {
        self.source_evidence_identity
    }
}

pub(super) fn release_lease(
    lease: RecoverySourceReachabilityLease,
) -> Result<RecoverySourceLeaseReleaseReceipt, RecoverySourceLeaseDenial> {
    if lease.durable_record.exists() {
        let content = std::fs::read(&lease.durable_record)?;
        if Sha256::digest(&content).as_slice() != lease.identity {
            return Err(RecoverySourceLeaseDenial::LeaseConflict);
        }
        std::fs::remove_file(&lease.durable_record)?;
        sync_directory(
            lease
                .durable_record
                .parent()
                .ok_or(RecoverySourceLeaseDenial::LeaseConflict)?,
        )?;
    }
    Ok(RecoverySourceLeaseReleaseReceipt {
        lease_identity: lease.identity,
        source_identity: lease.source_identity,
        source_evidence_identity: lease.source_evidence_identity,
    })
}
