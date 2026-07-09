use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::runtime::WorthQueryRuntimeError;

pub(in crate::runtime) fn worth_query_shared_read_stale_basis_error(
    snapshot_identity: WorthQuerySnapshotIdentity,
) -> WorthQueryRuntimeError {
    WorthQueryRuntimeError::SharedReadStaleBasis { snapshot_identity }
}
