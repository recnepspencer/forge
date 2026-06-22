use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQueryMutationDelta,
    ForgeQueryMutationKind, ForgeQueryMutationReceipt, ForgeQuerySnapshotIdentity,
};
use crate::runtime::ForgeQueryAspectTouch;
use forge_runtime_bridge::facade::BridgeMutationAuthorityBundle;
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

pub(in crate::runtime::tests) fn test_mutation_receipt(
    commit_identity: ForgeQueryCommitIdentity,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    collection: impl Into<String>,
    entity_identity: ForgeQueryEntityIdentity,
    kind: ForgeQueryMutationKind,
    aspect_touches: Vec<ForgeQueryAspectTouch>,
) -> ForgeQueryMutationReceipt {
    ForgeQueryMutationReceipt::from_authoritative_parts(
        commit_identity,
        snapshot_identity,
        vec![ForgeQueryMutationDelta::from_touched_aspects(
            collection,
            entity_identity,
            kind,
            aspect_touches,
        )],
    )
}

pub(in crate::runtime::tests) fn test_mutation_receipt_with_bridge_authority(
    commit_identity: ForgeQueryCommitIdentity,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    collection: impl Into<String>,
    entity_identity: ForgeQueryEntityIdentity,
    kind: ForgeQueryMutationKind,
    aspect_touches: Vec<ForgeQueryAspectTouch>,
    bridge_authority: BridgeMutationAuthorityBundle,
) -> ForgeQueryMutationReceipt {
    ForgeQueryMutationReceipt::from_bridge_authoritative_parts(
        commit_identity,
        snapshot_identity,
        vec![ForgeQueryMutationDelta::from_touched_aspects(
            collection,
            entity_identity,
            kind,
            aspect_touches,
        )],
        bridge_authority,
    )
}

#[allow(dead_code)]
pub(in crate::runtime::tests) fn test_relational_snapshot_identity(
    branch_id: u64,
    snapshot_id: u64,
) -> ForgeQuerySnapshotIdentity {
    ForgeQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(branch_id, snapshot_id),
    )
}

pub(in crate::runtime::tests) fn test_empty_mutation_receipt(
    commit_identity: ForgeQueryCommitIdentity,
    snapshot_identity: ForgeQuerySnapshotIdentity,
) -> ForgeQueryMutationReceipt {
    ForgeQueryMutationReceipt::from_authoritative_parts(
        commit_identity,
        snapshot_identity,
        Vec::new(),
    )
}
