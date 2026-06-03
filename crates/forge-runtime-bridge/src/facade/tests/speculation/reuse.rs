use crate::facade::tests::runtime;
use crate::facade::tests::speculation::{
    preview_declaration, structural_basis, StructuralBasisInput, StructuralSemanticsVersion,
};
use crate::facade::{
    BridgePreviewLifecycleStateKind, BridgePreviewSessionIdentity, BridgeRuntimePolicy,
    StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, TruthBranchIdentity,
    TruthSnapshotIdentity,
};

#[test]
fn runtime_admits_and_activates_reused_preview_session_only_for_exact_equivalence() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let source_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:reuse-source"),
            preview_declaration(),
        )
        .expect("source preview declaration should admit");
    let (source_active, source_execution_record) =
        runtime.activate_preview_session(source_admitted, 4, 2, 2);

    let target_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:reuse-target"),
            preview_declaration(),
        )
        .expect("target preview declaration should admit");
    let reuse_equivalence = runtime
        .admit_preview_reuse(&source_active, &source_execution_record, &target_admitted)
        .expect("exactly equivalent preview declarations should admit reuse");

    let (target_active, target_execution_record) = runtime
        .activate_preview_session_with_reuse(
            target_admitted,
            &source_active,
            &source_execution_record,
            &reuse_equivalence,
        )
        .expect("reused preview activation should succeed");

    assert_eq!(
        target_active.lifecycle_state_kind(),
        BridgePreviewLifecycleStateKind::Active
    );
    assert_eq!(
        target_execution_record.counters().preview_artifact_count(),
        source_execution_record.counters().preview_artifact_count()
    );
    assert_eq!(runtime.diagnostics().preview_execution_records().len(), 2);
}

#[test]
fn runtime_rejects_preview_reuse_when_target_basis_drifts() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let source_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:reuse-source-drift"),
            preview_declaration(),
        )
        .expect("source preview declaration should admit");
    let (source_active, source_execution_record) =
        runtime.activate_preview_session(source_admitted, 2, 1, 1);

    let drifted_target = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:reuse-target-drift"),
            preview_declaration().with_structural_basis(structural_basis(StructuralBasisInput {
                schema_identity: StructuralSchemaIdentity::new("schema:drift"),
                declaration_identity: StructuralIdentityDeclarationIdentity::new(
                    "structural:drift",
                ),
                truth_branch_identity: TruthBranchIdentity::new("truth:drift"),
                snapshot_identity: TruthSnapshotIdentity::new("snapshot:drift"),
                semantics_version: StructuralSemanticsVersion::Drift,
            })),
        )
        .expect("drifted preview declaration should still admit");
    let error = runtime
        .admit_preview_reuse(&source_active, &source_execution_record, &drifted_target)
        .expect_err("drifted declarations must fail reuse admission");

    assert_eq!(
        error.kind(),
        crate::error::BridgeSpeculationErrorKind::PreviewReuseEquivalenceMismatch
    );
}
