use crate::runtime::ForgeQueryRuntimeError;

pub(in crate::runtime) fn forge_query_shared_read_stale_basis_error(
    snapshot_token: impl Into<String>,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::SharedReadStaleBasis {
        snapshot_token: snapshot_token.into(),
    }
}
