use super::{
    admit_basis_capability, basis_lifecycle_support_matrix, discover_basis_lifecycle_support,
    evaluate_basis_inspection_advisory_eligibility, evaluate_basis_inspection_eligibility,
    evaluate_basis_materialization_eligibility, evaluate_basis_mutation_preparation_eligibility,
    evaluate_basis_observation_eligibility, evaluate_basis_preview_closeout_eligibility,
    evaluate_basis_replay_eligibility, evaluate_basis_subscription_declaration_eligibility,
    normalize_raw_basis_intent, scope_basis_for_mutation_preparation, scope_basis_for_observation,
    scope_basis_for_subscription_declaration, BasisIntentDenialKind, BasisOperationLane,
    BasisSupportPosture, DeniedBasisCapabilityKind, InspectionLaneWitness, RawBasisIntent,
};

#[test]
fn equivalent_current_head_intents_normalize_to_the_same_operation_digest() {
    let first = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head should normalize");
    let second = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head should normalize again");

    assert_eq!(first.normalized_digest(), second.normalized_digest());
    assert_eq!(first.counters().raw_intent_width(), 1);
    assert_eq!(first.counters().normalized_family_count(), 1);
    assert_eq!(first.counters().source_path_count(), 1);
}

#[test]
fn different_operation_lanes_change_normalized_basis_digest() {
    let observation = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: true,
        },
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch observation should normalize");
    let mutation = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: true,
        },
        <super::MutationPreparationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch mutation should normalize");

    assert_ne!(
        observation.normalized_digest(),
        mutation.normalized_digest()
    );
}

#[test]
fn observation_eligibility_admits_and_scopes_current_head_basis() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head should normalize");
    let eligibility =
        evaluate_basis_observation_eligibility(normalized).expect("observation should admit");
    let trace_digest = eligibility.decision_trace().trace_digest().to_string();
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_observation(capability);

    assert!(!trace_digest.is_empty());
    assert!(!scoped.scoped_basis_digest().is_empty());
    assert_eq!(scoped.counters().scoped_capability_construction_count(), 1);
}

#[test]
fn branch_head_mutation_preparation_gets_a_scoped_capability() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: true,
        },
        <super::MutationPreparationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch head should normalize");
    let eligibility = evaluate_basis_mutation_preparation_eligibility(normalized)
        .expect("branch mutation preparation should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_mutation_preparation(capability);

    assert!(!scoped.capability_digest().is_empty());
    assert_eq!(scoped.counters().scoped_capability_construction_count(), 1);
}

#[test]
fn stale_preview_denies_before_closeout_artifacts_exist() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::Preview {
            preview_identity: "preview-a".to_string(),
            stale: true,
        },
        <super::PreviewCloseoutLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("preview should normalize");
    let denial = evaluate_basis_preview_closeout_eligibility(normalized)
        .expect_err("stale preview must deny");

    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::PreviewDrifted
    );
    assert_eq!(denial.counters().denied_residue_count(), 0);
}

#[test]
fn policy_mask_and_tenant_schema_mismatch_stop_at_eligibility() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::PolicyScoped {
            policy_digest: "policy-a".to_string(),
            tenant_identity: "tenant-a".to_string(),
            branch_identity: "branch-a".to_string(),
            schema_identity: "schema-a".to_string(),
            tenant_schema_matches: false,
            policy_masks_operation: false,
            advisory_visibility: false,
        },
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("policy scoped basis should normalize");
    let denial =
        evaluate_basis_observation_eligibility(normalized).expect_err("mismatch must deny");

    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::TenantMismatched
    );
    assert_eq!(denial.counters().tenant_schema_check_count(), 1);
}

#[test]
fn policy_mask_and_tenant_mismatch_remain_distinct_denial_families() {
    let policy_masked = normalize_raw_basis_intent(
        RawBasisIntent::PolicyScoped {
            policy_digest: "policy-a".to_string(),
            tenant_identity: "tenant-a".to_string(),
            branch_identity: "branch-a".to_string(),
            schema_identity: "schema-a".to_string(),
            tenant_schema_matches: true,
            policy_masks_operation: true,
            advisory_visibility: false,
        },
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("policy scoped basis should normalize");
    let policy_denial =
        evaluate_basis_observation_eligibility(policy_masked).expect_err("policy mask must deny");

    assert_eq!(
        policy_denial.denial_kind(),
        DeniedBasisCapabilityKind::PolicyMasked
    );

    let inaccessible_branch = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: false,
        },
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("inaccessible branch should normalize into denial posture");
    let branch_denial = evaluate_basis_observation_eligibility(inaccessible_branch)
        .expect_err("inaccessible branch must deny");

    assert_eq!(
        branch_denial.denial_kind(),
        DeniedBasisCapabilityKind::Inaccessible
    );
}

#[test]
fn current_head_subscription_declaration_gets_a_scoped_capability() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <super::SubscriptionDeclarationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head subscription should normalize");
    let eligibility = evaluate_basis_subscription_declaration_eligibility(normalized)
        .expect("current head subscription declaration should admit");
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_subscription_declaration(capability);

    assert!(!scoped.scoped_basis_digest().is_empty());
    assert_eq!(scoped.counters().scoped_capability_construction_count(), 1);
}

#[test]
fn historical_replay_unsupported_stops_at_eligibility() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::HistoricalSnapshot {
            snapshot_identity: "snapshot-a".to_string(),
            replay_supported: false,
        },
        <super::ReplayLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("historical basis should normalize");
    let denial = evaluate_basis_replay_eligibility(normalized).expect_err("replay must deny");

    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::HistoricalReplayUnsupported
    );
}

#[test]
fn missing_lower_runtime_binding_stops_before_use_receipts() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::RuntimeSnapshot {
            snapshot_identity: "snapshot-a".to_string(),
            lower_runtime_binding_digest: None,
        },
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("runtime snapshot should normalize");
    let denial = evaluate_basis_observation_eligibility(normalized)
        .expect_err("missing lower runtime binding must deny");

    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::LowerRuntimeBindingMissing
    );
}

#[test]
fn future_neighbor_raw_intents_fail_before_eligibility_or_defer_with_zero_residue() {
    let temporal = normalize_raw_basis_intent(
        RawBasisIntent::TemporalFuture {
            temporal_identity: "clock-a".to_string(),
        },
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect_err("temporal basis is not normalized in 9.3.2");
    assert_eq!(
        temporal.denial_kind(),
        BasisIntentDenialKind::TemporalDeferred
    );

    let durable = normalize_raw_basis_intent(
        RawBasisIntent::DurableReload {
            reload_identity: "reload-a".to_string(),
        },
        <super::CertificationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("durable overclaim should normalize into deferred posture");
    let denial = super::evaluate_basis_certification_eligibility(durable)
        .expect_err("durable reload must deny");
    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::DurableOverclaim
    );
    assert_eq!(denial.counters().denied_residue_count(), 0);
}

#[test]
fn advisory_basis_shape_cannot_be_silently_promoted_to_admitted_capability() {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::PreviewDerived {
            preview_identity: "preview-a".to_string(),
            source_basis_identity: "branch-a".to_string(),
        },
        <InspectionLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("preview-derived basis should normalize");
    let advisory = evaluate_basis_inspection_advisory_eligibility(normalized.clone())
        .expect("preview-derived inspection is advisory");
    let denial = evaluate_basis_inspection_eligibility(normalized)
        .expect_err("advisory support must not become admitted automatically");

    assert!(!advisory.decision_trace().trace_digest().is_empty());
    assert_eq!(
        denial.denial_kind(),
        DeniedBasisCapabilityKind::OperationIneligible
    );
}

#[test]
fn support_matrix_is_derived_from_executable_lane_registry() {
    let matrix = basis_lifecycle_support_matrix();

    assert!(matrix.rows().iter().any(|row| {
        row.operation_lane() == "observation"
            && row.posture() == BasisSupportPosture::Admitted
            && row.family() == super::BasisFamily::CurrentHead
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.operation_lane() == "inspection"
            && row.posture() == BasisSupportPosture::Advisory
            && row.family() == super::BasisFamily::PreviewDerived
    }));
    assert!(!matrix.matrix_digest().is_empty());
}

#[test]
fn support_discovery_reports_admitted_lane_before_execution() {
    let discovery = discover_basis_lifecycle_support(
        super::BasisFamily::CurrentHead,
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    );
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <super::ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head should normalize");

    evaluate_basis_observation_eligibility(normalized).expect("admitted support should execute");

    assert_eq!(discovery.posture(), BasisSupportPosture::Admitted);
    assert!(discovery.matched_row_digest().is_some());
    assert_eq!(discovery.counters().basis_support_lookup_count(), 1);
    assert!(!discovery.discovery_digest().is_empty());
}

#[test]
fn support_discovery_reports_advisory_without_admitted_promotion() {
    let discovery = discover_basis_lifecycle_support(
        super::BasisFamily::PreviewDerived,
        <InspectionLaneWitness as BasisOperationLane>::lane_name(),
    );
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::PreviewDerived {
            preview_identity: "preview-a".to_string(),
            source_basis_identity: "branch-a".to_string(),
        },
        <InspectionLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("preview-derived basis should normalize");

    evaluate_basis_inspection_advisory_eligibility(normalized.clone())
        .expect("advisory discovery should match advisory eligibility");
    let admitted_denial = evaluate_basis_inspection_eligibility(normalized)
        .expect_err("advisory support must not become admitted");

    assert_eq!(discovery.posture(), BasisSupportPosture::Advisory);
    assert_eq!(
        admitted_denial.denial_kind(),
        DeniedBasisCapabilityKind::OperationIneligible
    );
}

#[test]
fn support_discovery_reports_deferred_and_unsupported_lanes() {
    let deferred = discover_basis_lifecycle_support(
        super::BasisFamily::DurableReload,
        <super::CertificationLaneWitness as BasisOperationLane>::lane_name(),
    );
    let durable = normalize_raw_basis_intent(
        RawBasisIntent::DurableReload {
            reload_identity: "reload-a".to_string(),
        },
        <super::CertificationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("durable reload should normalize into deferred posture");
    let durable_denial = super::evaluate_basis_certification_eligibility(durable)
        .expect_err("deferred support must deny execution");

    assert_eq!(deferred.posture(), BasisSupportPosture::Deferred);
    assert_eq!(
        durable_denial.denial_kind(),
        DeniedBasisCapabilityKind::DurableOverclaim
    );

    let unsupported = discover_basis_lifecycle_support(
        super::BasisFamily::BranchHead,
        <super::MaterializationLaneWitness as BasisOperationLane>::lane_name(),
    );
    let branch = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: true,
        },
        <super::MaterializationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch head should normalize");
    let unsupported_denial = evaluate_basis_materialization_eligibility(branch)
        .expect_err("unsupported support must deny execution");

    assert_eq!(unsupported.posture(), BasisSupportPosture::Unsupported);
    assert!(unsupported.matched_row_digest().is_none());
    assert_eq!(
        unsupported_denial.denial_kind(),
        DeniedBasisCapabilityKind::OperationIneligible
    );
}
