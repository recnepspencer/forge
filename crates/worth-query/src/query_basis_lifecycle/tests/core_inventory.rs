use super::{
    admit_observation_basis, basis_compatibility_debt_registry, basis_inventory,
    evaluate_basis_eligibility, lower_runtime_api_reuse_matrix, normalize_raw_basis,
    target_dx_transcript_inventory, BasisCapabilityAdmission, BasisCompatibilityDebtPosture,
    BasisInventoryDisposition, BasisLifecyclePosture, BasisOperationLaneRequest, BasisVisibility,
    DeniedBasisCapabilityKind, LowerRuntimeApiReuseClass, RawBasisIntent, TargetDxTranscriptKind,
};

#[test]
fn observation_capability_exposes_visibility_lifecycle_and_placeholders() {
    let normalized = normalize_raw_basis(RawBasisIntent::current_head(
        BasisOperationLaneRequest::Observation,
    ))
    .expect("current head observation should normalize");
    let eligibility = evaluate_basis_eligibility(normalized)
        .expect("current head observation should be eligible");
    let capability = admit_observation_basis(eligibility)
        .expect("current head observation should admit observation capability");

    match capability.admission() {
        BasisCapabilityAdmission::Admitted(admitted) => {
            assert_eq!(admitted.visibility(), BasisVisibility::CurrentHead);
            assert_eq!(
                admitted.lifecycle_posture(),
                BasisLifecyclePosture::Authoritative
            );
            assert!(admitted
                .lower_runtime_evidence_placeholders()
                .iter()
                .any(|placeholder| placeholder.as_str() == "bridge_truth_view_authority"));
            assert!(admitted
                .permitted_lanes()
                .contains(&BasisOperationLaneRequest::MutationPreparation));
        }
        other => panic!("unexpected capability admission: {other:?}"),
    }
}

#[test]
fn policy_mask_tenant_mismatch_schema_incompatible_and_missing_binding_deny_at_eligibility() {
    let policy_masked = evaluate_basis_eligibility(
        normalize_raw_basis(
            RawBasisIntent::branch_head(
                super::test_branch_identity("branch:main"),
                BasisOperationLaneRequest::Observation,
            )
            .with_policy_scope("policy:masked:redacted"),
        )
        .expect("policy-masked intent should normalize"),
    )
    .expect_err("policy-masked intent should deny at eligibility");
    assert!(matches!(
        policy_masked.kind(),
        DeniedBasisCapabilityKind::PolicyMasked { .. }
    ));

    let tenant_mismatched = evaluate_basis_eligibility(
        normalize_raw_basis(
            RawBasisIntent::branch_head(
                super::test_branch_identity("branch:main"),
                BasisOperationLaneRequest::Observation,
            )
            .with_tenant_scope("tenant:mismatch:beta"),
        )
        .expect("tenant-mismatched intent should normalize"),
    )
    .expect_err("tenant-mismatched intent should deny at eligibility");
    assert!(matches!(
        tenant_mismatched.kind(),
        DeniedBasisCapabilityKind::TenantMismatched { .. }
    ));

    let schema_incompatible = evaluate_basis_eligibility(
        normalize_raw_basis(
            RawBasisIntent::branch_head(
                super::test_branch_identity("branch:main"),
                BasisOperationLaneRequest::Observation,
            )
            .with_schema_scope("schema:incompatible:v3"),
        )
        .expect("schema-incompatible intent should normalize"),
    )
    .expect_err("schema-incompatible intent should deny at eligibility");
    assert!(matches!(
        schema_incompatible.kind(),
        DeniedBasisCapabilityKind::SchemaIncompatible { .. }
    ));

    let missing_binding = evaluate_basis_eligibility(
        normalize_raw_basis(RawBasisIntent::branch_head(
            super::test_branch_identity("branch:missing_binding"),
            BasisOperationLaneRequest::Observation,
        ))
        .expect("missing-binding intent should normalize"),
    )
    .expect_err("missing-binding intent should deny at eligibility");
    assert!(matches!(
        missing_binding.kind(),
        DeniedBasisCapabilityKind::LowerRuntimeBindingMissing { .. }
    ));
}

#[test]
fn basis_inventory_covers_required_owner_surfaces() {
    let inventory = basis_inventory();

    assert!(inventory.rows().iter().any(|row| {
        row.surface_label() == "query_basis_lifecycle::*"
            && row.disposition() == BasisInventoryDisposition::ConsolidatedLifecycleHome
    }));
    assert!(inventory.rows().iter().any(|row| {
        row.surface_label() == "RuntimeBridge::deliver_continuity"
            && row.disposition() == BasisInventoryDisposition::ReusedAuthority
    }));
    assert!(inventory.rows().iter().any(|row| {
        row.surface_label() == "RuntimeBridgeRelationalSource"
            && row.disposition() == BasisInventoryDisposition::ReusedAuthority
    }));
    assert!(inventory.rows().iter().any(|row| {
        row.surface_label() == "future async/store/durable basis neighbors"
            && row.disposition() == BasisInventoryDisposition::DeferredNeighbor
    }));
    assert!(inventory.rows().iter().any(|row| {
        row.surface_label() == "fresh Query-side branch/snapshot/writeback/causal authority objects"
            && row.disposition() == BasisInventoryDisposition::ForbiddenDuplicate
    }));
}

#[test]
fn lower_runtime_reuse_matrix_names_phase_four_bridge_rows() {
    let matrix = lower_runtime_api_reuse_matrix();

    for required in [
        "RuntimeBridge::evaluate",
        "RuntimeBridge::evaluate_current",
        "RuntimeBridge::plan_truth_view_packet",
        "RuntimeBridge::plan_source_packet_set",
        "RuntimeBridge::materialize_source_packet",
        "RuntimeBridge::admit_subscription",
        "RuntimeBridge::admit_subscription_preview_basis",
        "RuntimeBridge::deliver_continuity",
    ] {
        assert!(
            matrix.rows().iter().any(|row| row.api_label() == required),
            "missing reuse row for {required}"
        );
    }
    assert!(matrix.rows().iter().any(|row| {
        row.api_label() == "fresh Query-owned lower-runtime authority clones"
            && row.reuse_class() == LowerRuntimeApiReuseClass::ForbiddenDuplicate
    }));
}

#[test]
fn target_dx_transcript_inventory_covers_phase_one_common_paths() {
    let inventory = target_dx_transcript_inventory();

    for required in [
        TargetDxTranscriptKind::CurrentHeadObservation,
        TargetDxTranscriptKind::BranchHeadMutationPreparation,
        TargetDxTranscriptKind::PreviewDenial,
        TargetDxTranscriptKind::CausalInspection,
        TargetDxTranscriptKind::LowerRuntimeEvidenceMaterialization,
        TargetDxTranscriptKind::SupportDiscovery,
    ] {
        assert!(
            inventory.rows().iter().any(|row| row.kind() == required),
            "missing DX transcript row for {required:?}"
        );
    }
}

#[test]
fn compatibility_debt_registry_covers_named_phase_three_migration_surfaces() {
    let registry = basis_compatibility_debt_registry();

    assert!(registry.rows().iter().any(|row| {
        row.surface_label()
            == "query_context::{bind_query_basis_context,admit_query_basis_context,execute_query_basis_context}"
            && row.posture() == BasisCompatibilityDebtPosture::ScopedMigrationPending
    }));
    assert!(registry.rows().iter().any(|row| {
        row.surface_label()
            == "preview::{assess_preview_live_drift,PreviewLiveExecutionEnvelope::preview_live}"
            && row.posture() == BasisCompatibilityDebtPosture::CompatibilityAdapterPending
    }));
    assert!(registry.rows().iter().any(|row| {
        row.surface_label() == "subscription::{declaration,activation,support,diagnostic}::*"
            && row.posture() == BasisCompatibilityDebtPosture::ScopedMigrationPending
    }));
}
