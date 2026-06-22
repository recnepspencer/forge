use crate::facade::{
    resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode, ExecutionBasisIntent,
    ResolvedSnapshotBasis, ResolvedSnapshotIdentity, SnapshotLineageClass, ValidatedQueryBundle,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

pub fn runtime_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

pub fn store_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Store,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

pub fn primary_snapshot_identity() -> ForgeQuerySnapshotIdentity {
    relational_snapshot_identity(1, 1)
}

pub fn alternate_snapshot_identity() -> ForgeQuerySnapshotIdentity {
    relational_snapshot_identity(2, 1)
}

pub fn relational_snapshot_identity(
    snapshot_id: u64,
    version_id: u64,
) -> ForgeQuerySnapshotIdentity {
    ForgeQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(snapshot_id, version_id),
    )
}

pub fn runtime_basis(
    bundle: &ValidatedQueryBundle,
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) -> ResolvedSnapshotBasis {
    resolve_snapshot_basis(
        runtime_basis_intent(),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Runtime,
            Some("workspace-main".to_string()),
            snapshot_identity.evidence_identity(),
            bundle.query().schema_basis().clone(),
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap()
}

pub fn store_basis(
    bundle: &ValidatedQueryBundle,
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) -> ResolvedSnapshotBasis {
    resolve_snapshot_basis(
        store_basis_intent(),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Store,
            Some("workspace-main".to_string()),
            snapshot_identity.evidence_identity(),
            bundle.query().schema_basis().clone(),
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::StoreDirect,
    )
    .unwrap()
}
