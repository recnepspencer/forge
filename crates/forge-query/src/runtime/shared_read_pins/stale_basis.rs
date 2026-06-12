use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::ForgeQueryRuntimeError;

pub(in crate::runtime) fn forge_query_shared_read_stale_basis_error(
    snapshot_identity: ForgeQuerySnapshotIdentity,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::SharedReadStaleBasis { snapshot_identity }
}
