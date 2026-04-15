use crate::facade::{
    resolve_snapshot_basis, BasisAuthorityFamily, BasisResolutionMode, ExecutionBasisIntent,
    ResolvedSnapshotBasis, ResolvedSnapshotIdentity, SnapshotLineageClass, ValidatedQueryBundle,
};

pub fn runtime_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

pub fn runtime_basis(bundle: &ValidatedQueryBundle, snapshot_token: &str) -> ResolvedSnapshotBasis {
    resolve_snapshot_basis(
        runtime_basis_intent(),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Runtime,
            Some("workspace-main".to_string()),
            snapshot_token,
            bundle.query().schema_basis().clone(),
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap()
}
