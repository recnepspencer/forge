use crate::capabilities::{CommitEnvelopeSource, SnapshotSource};
use crate::history::data::CommitId;
use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;
use worth_runtime_bridge::facade::{RelationalBridgeSourceError, TruthSnapshotIdentity};

use crate::presentation::bridge::identities::parse_bridge_snapshot_identity;

pub(super) fn resolve_snapshot_version(
    runtime: &RelationalRuntime,
    identity: &TruthSnapshotIdentity,
) -> Result<VersionId, RelationalBridgeSourceError> {
    let (snapshot_id, expected_version_id) = parse_bridge_snapshot_identity(identity)?;
    let active_version_id = runtime
        .active_snapshot_binding(snapshot_id)
        .map(|(_, version_id, _)| version_id);
    let execution_version_id = runtime
        .execution_basis_binding(snapshot_id)
        .map(|(_, version_id, _)| version_id);
    let observed_version_id = active_version_id
        .or(execution_version_id)
        .or_else(|| runtime.published_snapshot_version(snapshot_id))
        .or_else(|| {
            runtime
                .commit_envelope(CommitId(snapshot_id.0))
                .map(|envelope| envelope.commit.version_id)
        })
        .ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "relational bridge snapshot `{}` has no active, execution, published, or canonical commit authority",
                snapshot_id.0
            ))
        })?;
    if observed_version_id != expected_version_id {
        return Err(RelationalBridgeSourceError::new(format!(
            "relational bridge snapshot `{}` expected version `{}` but authoritative binding resolved to version `{}`",
            snapshot_id.0,
            expected_version_id.0,
            observed_version_id.0
        )));
    }

    Ok(observed_version_id)
}
