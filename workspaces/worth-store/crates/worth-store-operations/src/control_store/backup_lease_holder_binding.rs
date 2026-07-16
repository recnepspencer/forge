use sha2::{Digest, Sha256};
use worth_store_physical_isolation::BackupReachabilityLeaseHolderId;

use super::OperationalOperationId;

pub(crate) fn backup_lease_holder_id(
    operation: &OperationalOperationId,
) -> BackupReachabilityLeaseHolderId {
    let mut digest = Sha256::new();
    digest.update(b"worth-store:backup-reachability-holder:v1\0");
    digest.update(operation.as_str().as_bytes());
    BackupReachabilityLeaseHolderId::from_stable_identity(digest.finalize().into())
}
