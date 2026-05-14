use super::{
    admit_basis_capability, admit_observation_basis, basis_compatibility_debt_registry,
    basis_inventory, evaluate_basis_eligibility, lower_runtime_api_reuse_matrix,
    normalize_query_context_request, normalize_raw_basis, target_dx_transcript_inventory,
    BasisCapabilityAdmission, BasisCompatibilityDebtPosture, BasisEligibilityDisposition,
    BasisIntentDenialKind, BasisInventoryDisposition, BasisLifecyclePosture,
    BasisOperationLaneRequest, BasisTenantSchemaPosture, BasisVisibility,
    DeniedBasisCapabilityKind, LowerRuntimeApiReuseClass, RawBasisIntent,
    RawFutureBasisNeighborFamily, TargetDxTranscriptKind,
};
use crate::query_context::QueryBasisContextRequest;

#[test]
fn equivalent_current_head_paths_normalize_to_same_digest() {
    let direct = normalize_raw_basis(RawBasisIntent::current_head(
        BasisOperationLaneRequest::Observation,
    ))
    .expect("direct current-head intent should normalize");
    let compatibility = normalize_query_context_request(
        &QueryBasisContextRequest::current_branch_head(),
        BasisOperationLaneRequest::Observation,
    )
    .expect("compatibility current-head intent should normalize");

    assert_eq!(direct.family(), compatibility.family());
    assert_eq!(
        direct.authority_posture(),
        compatibility.authority_posture()
    );
    assert_eq!(direct.canonical_digest(), compatibility.canonical_digest());
    assert_ne!(
        direct.raw_basis_intent_digest(),
        compatibility.raw_basis_intent_digest()
    );
    assert_ne!(direct.source_path(), compatibility.source_path());
    assert_eq!(direct.counters().raw_intent_width(), 1);
    assert_eq!(direct.counters().normalized_family_count(), 1);
    assert_eq!(direct.counters().source_path_count(), 1);
    assert_eq!(direct.counters().rejection_width(), 0);
}

#[test]
fn tenant_and_policy_scopes_participate_in_normalized_digest() {
    let tenant_scoped = normalize_raw_basis(
        RawBasisIntent::branch_head("branch:main", BasisOperationLaneRequest::Observation)
            .with_tenant_scope("tenant:alpha"),
    )
    .expect("tenant-scoped intent should normalize");
    let policy_scoped = normalize_raw_basis(
        RawBasisIntent::branch_head("branch:main", BasisOperationLaneRequest::Observation)
            .with_policy_scope("policy:redacted"),
    )
    .expect("policy-scoped intent should normalize");

    assert_ne!(
        tenant_scoped.canonical_digest(),
        policy_scoped.canonical_digest()
    );
}

#[test]
fn schema_scope_participates_in_posture_and_digest() {
    let schema_scoped = normalize_raw_basis(
        RawBasisIntent::branch_head("branch:main", BasisOperationLaneRequest::Observation)
            .with_schema_scope("schema:v2"),
    )
    .expect("schema-scoped intent should normalize");

    assert_eq!(
        schema_scoped.tenant_schema_posture(),
        &BasisTenantSchemaPosture::SchemaScoped
    );
    assert_eq!(schema_scoped.schema_scope(), Some("schema:v2"));
}

#[test]
fn preview_derived_and_historical_snapshot_remain_distinct() {
    let historical = normalize_raw_basis(RawBasisIntent::historical_snapshot(
        "history:snapshot-1",
        BasisOperationLaneRequest::Observation,
    ))
    .expect("historical snapshot should normalize");
    let preview = normalize_raw_basis(RawBasisIntent::preview_derived_historical(
        "preview:session-1",
        BasisOperationLaneRequest::Observation,
    ))
    .expect("preview-derived historical should normalize");

    assert_ne!(historical.family(), preview.family());
    assert_ne!(historical.canonical_digest(), preview.canonical_digest());
}

#[test]
fn future_neighbor_denies_before_normalization() {
    let denial = normalize_raw_basis(RawBasisIntent::future_neighbor(
        RawFutureBasisNeighborFamily::DurableReload,
        BasisOperationLaneRequest::Replay,
    ))
    .expect_err("future durable reload should deny during normalization");

    match denial.kind() {
        BasisIntentDenialKind::UnsupportedFutureNeighbor { family, owner } => {
            assert_eq!(family, &RawFutureBasisNeighborFamily::DurableReload);
            assert_eq!(owner, &"forge_store");
        }
        other => panic!("unexpected denial kind: {other:?}"),
    }
    assert_eq!(denial.counters().rejection_width(), 1);
}

#[test]
fn blank_scope_denies_typed_before_normalization() {
    let denial = normalize_raw_basis(
        RawBasisIntent::branch_head("branch:main", BasisOperationLaneRequest::Observation)
            .with_tenant_scope("   "),
    )
    .expect_err("blank tenant scope should deny");

    match denial.kind() {
        BasisIntentDenialKind::MalformedIdentifier { field } => {
            assert_eq!(field, &"tenant_scope");
        }
        other => panic!("unexpected denial kind: {other:?}"),
    }
    assert_eq!(denial.counters().normalized_family_count(), 0);
}

#[test]
fn current_head_observation_eligibility_admits_with_success_disposition() {
    let normalized = normalize_raw_basis(RawBasisIntent::current_head(
        BasisOperationLaneRequest::Observation,
    ))
    .expect("current head observation should normalize");
    let eligibility = evaluate_basis_eligibility(normalized)
        .expect("current head observation should be eligible");

    assert_eq!(
        eligibility.disposition(),
        &BasisEligibilityDisposition::Success
    );
    assert_eq!(eligibility.counters().consulted_row_count(), 1);
    assert_eq!(eligibility.counters().tenant_check_count(), 0);
    assert_eq!(eligibility.counters().policy_check_count(), 0);
    assert_eq!(eligibility.counters().schema_check_count(), 0);
}

#[test]
fn preview_observation_admits_as_advisory_capability() {
    let normalized = normalize_raw_basis(RawBasisIntent::preview(
        "preview:session-1",
        BasisOperationLaneRequest::Observation,
    ))
    .expect("preview observation should normalize");
    let eligibility = evaluate_basis_eligibility(normalized)
        .expect("preview observation should remain eligibility-admitted as advisory");
    let capability = admit_observation_basis(eligibility)
        .expect("preview observation should wrap advisory capability for observation");

    match capability.admission() {
        BasisCapabilityAdmission::Advisory(advisory) => {
            assert_eq!(
                advisory.operation_lane(),
                &BasisOperationLaneRequest::Observation
            );
        }
        other => panic!("unexpected capability admission: {other:?}"),
    }
}

#[test]
fn branch_snapshot_mutation_preparation_denies_as_operation_ineligible() {
    let normalized = normalize_raw_basis(RawBasisIntent::branch_snapshot(
        "branch:main",
        "snapshot:1",
        BasisOperationLaneRequest::MutationPreparation,
    ))
    .expect("branch snapshot mutation-preparation should normalize");
    let denial = evaluate_basis_eligibility(normalized)
        .expect_err("branch snapshot should deny mutation preparation during eligibility");

    match denial.kind() {
        DeniedBasisCapabilityKind::OperationIneligible {
            family,
            operation_lane,
        } => {
            assert_eq!(family.as_str(), "branch_snapshot");
            assert_eq!(
                operation_lane,
                &BasisOperationLaneRequest::MutationPreparation
            );
        }
        other => panic!("unexpected denial kind: {other:?}"),
    }
    assert_eq!(denial.counters().denied_residue_count(), 1);
}

#[test]
fn runtime_backed_replay_denies_as_historical_replay_unsupported() {
    let normalized = normalize_raw_basis(RawBasisIntent::current_head(
        BasisOperationLaneRequest::Replay,
    ))
    .expect("current head replay should normalize");
    let denial = evaluate_basis_eligibility(normalized)
        .expect_err("current head replay should deny before any replay artifact exists");

    match denial.kind() {
        DeniedBasisCapabilityKind::HistoricalReplayUnsupported { family } => {
            assert_eq!(family.as_str(), "current_head");
        }
        other => panic!("unexpected denial kind: {other:?}"),
    }
}

#[test]
fn lane_specific_admission_rejects_mismatched_normalized_lane() {
    let normalized = normalize_raw_basis(RawBasisIntent::current_head(
        BasisOperationLaneRequest::Inspection,
    ))
    .expect("current head inspection should normalize");
    let eligibility = evaluate_basis_eligibility(normalized)
        .expect("inspection basis should be eligible for its own lane");
    let denial = admit_observation_basis(eligibility)
        .expect_err("observation admission should reject inspection-eligible basis");

    assert_eq!(
        denial.trace().rule_label(),
        "lane_specific_admission_requires_matching_eligible_lane"
    );
    assert_eq!(denial.counters().denied_residue_count(), 1);
}

#[test]
fn preview_eligibility_does_not_promote_into_success_capability() {
    let normalized = normalize_raw_basis(RawBasisIntent::preview(
        "preview:session-2",
        BasisOperationLaneRequest::Observation,
    ))
    .expect("preview observation should normalize");
    let eligibility = evaluate_basis_eligibility(normalized)
        .expect("preview observation should stay eligible as advisory");
    let admission = admit_basis_capability(eligibility);

    match admission {
        BasisCapabilityAdmission::Advisory(_) => {}
        other => panic!("unexpected capability admission: {other:?}"),
    }
}

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
            RawBasisIntent::branch_head("branch:main", BasisOperationLaneRequest::Observation)
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
            RawBasisIntent::branch_head("branch:main", BasisOperationLaneRequest::Observation)
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
            RawBasisIntent::branch_head("branch:main", BasisOperationLaneRequest::Observation)
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
            "branch:missing_binding",
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
