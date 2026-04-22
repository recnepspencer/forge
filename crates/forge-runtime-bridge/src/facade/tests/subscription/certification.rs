use super::support::*;
use crate::facade::RuntimeBridge;

fn product_ids() -> Vec<String> {
    (0..128).map(|slot| format!("product-{slot:03}")).collect()
}

fn component_ids() -> Vec<String> {
    ["steel", "rubber", "copper", "glass", "labor"]
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn lane_ids() -> Vec<String> {
    [
        "authoritative-live",
        "historical-replay",
        "branch-local",
        "preview-discard",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn sealed_certification_bundle(
    runtime: &RuntimeBridge,
    source_inputs: Vec<crate::facade::BridgeSubscriptionSourceArtifactInput>,
    rich_diagnostics_admitted: bool,
) -> crate::facade::BridgeSubscriptionCertificationBundleSealed {
    let manifest = runtime
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("manifest should seal");
    let index = runtime.build_subscription_certification_source_index(source_inputs);
    let plan = runtime.plan_subscription_certification_bundle(&manifest, &index);
    let cost_profile = runtime
        .admit_subscription_certification_cost_profile(
            crate::facade::BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
            16,
            16,
            32,
            rich_diagnostics_admitted,
        )
        .expect("sparse certification profile should admit");
    let scratch = runtime.prepare_subscription_certification_scratch(&cost_profile);
    let draft = runtime
        .assemble_subscription_certification_bundle(plan, cost_profile, scratch)
        .expect("admitted certification bundle should assemble");
    runtime.seal_subscription_certification_bundle(draft)
}

fn active_source_inputs(
    active: &crate::facade::BridgeActiveSubscription,
    declaration_digest: &str,
    strategy_digest: &str,
) -> Vec<crate::facade::BridgeSubscriptionSourceArtifactInput> {
    vec![
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::Declaration,
            "detail-declaration",
            declaration_digest,
        ),
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            active
                .activation_ready()
                .admitted()
                .admitted_subscription_identity()
                .as_str(),
            active.activation_ready().digest(),
        ),
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            active.active_subscription_identity().as_str(),
            active.digest(),
        ),
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::StrategyLowering,
            "detail-strategy",
            strategy_digest,
        ),
    ]
}

#[test]
fn reference_workload_manifest_is_canonicalized_before_execution() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let manifest = runtime
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("canonical manifest should seal");

    let mut reversed_products = product_ids();
    reversed_products.reverse();
    let mut reversed_components = component_ids();
    reversed_components.reverse();
    let mut reversed_lanes = lane_ids();
    reversed_lanes.reverse();

    let reordered = runtime
        .declare_subscription_reference_workload_manifest(
            reversed_products,
            reversed_components,
            reversed_lanes,
        )
        .expect("reordered manifest should seal");

    assert_eq!(manifest.digest(), reordered.digest());
    assert_eq!(manifest.product_ids().len(), 128);
    assert_eq!(manifest.component_ids().len(), 5);
}

#[test]
fn reference_workload_manifest_rejects_incomplete_fixture_contract() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let rejection = runtime
        .declare_subscription_reference_workload_manifest(
            vec!["product-001".to_owned()],
            component_ids(),
            lane_ids(),
        )
        .expect_err("manifest must require the canonical 128-product fixture");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionReferenceWorkloadManifestRejectionKind::ProductCountMismatch
    );
}

#[test]
fn certification_source_index_is_canonical_and_scan_bounded() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let inputs = vec![
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            active.active_subscription_identity().as_str(),
            active.digest(),
        ),
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            active.active_subscription_identity().as_str(),
            active.digest(),
        ),
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            active
                .activation_ready()
                .admitted()
                .admitted_subscription_identity()
                .as_str(),
            active.activation_ready().digest(),
        ),
    ];

    let index = runtime.build_subscription_certification_source_index(inputs);

    assert_eq!(index.records().len(), 2);
    assert_eq!(index.counters().source_artifact_index_entry_count(), 2);
    assert_eq!(index.counters().source_artifact_index_scan_count(), 3);
    assert_eq!(index.counters().global_history_scan_count(), 0);
    assert_eq!(index.counters().global_subscription_scan_count(), 0);
}

#[test]
fn certification_cost_profile_rejects_over_budget_before_assembly() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let rejection = runtime
        .admit_subscription_certification_cost_profile(
            crate::facade::BridgeSubscriptionCertificationDensityPosture::RejectedOverBudgetCertification,
            4,
            8,
            8,
            false,
        )
        .expect_err("over-budget certification posture should reject before assembly");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionCertificationCostProfileRejectionKind::OverBudgetPostureRejected
    );
    assert_eq!(rejection.counters().over_budget_rejection_count(), 1);
}

#[test]
fn certification_cost_profile_empty_budget_rejection_does_not_claim_over_budget_work() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let rejection = runtime
        .admit_subscription_certification_cost_profile(
            crate::facade::BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
            0,
            8,
            8,
            false,
        )
        .expect_err("empty source artifact budgets must reject before posture work");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionCertificationCostProfileRejectionKind::EmptySourceArtifactBudget
    );
    assert_eq!(rejection.counters().over_budget_rejection_count(), 0);
}

#[test]
fn certification_cost_posture_report_proves_dense_over_budget_and_scratch_lifecycle() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_cost_posture();

    assert!(report.dense_selected_before_assembly());
    assert!(report.over_budget_rejected_before_assembly());
    assert!(report.scratch_lifecycle_reuse_visible());
    assert!(report
        .sparse_cost_profile_digest()
        .starts_with("bridge-subscription-certification-cost-profile:sha256:"));
    assert!(report
        .dense_cost_profile_digest()
        .starts_with("bridge-subscription-certification-cost-profile:sha256:"));
    assert_ne!(
        report.sparse_cost_profile_digest(),
        report.dense_cost_profile_digest()
    );
    assert!(report
        .over_budget_rejection_digest()
        .starts_with("bridge-subscription-certification-cost-profile-rejection:sha256:"));
    assert_eq!(
        report.first_scratch_digest(),
        report.repeated_scratch_digest()
    );
    assert_eq!(report.counters().bundle_cost_profile_count(), 2);
    assert_eq!(report.counters().dense_rebuild_count(), 1);
    assert_eq!(report.counters().over_budget_rejection_count(), 1);
    assert_eq!(report.counters().scratch_allocation_count(), 1);
    assert_eq!(report.counters().scratch_reuse_count(), 2);
    assert_eq!(report.counters().cost_posture_report_count(), 1);
    assert_eq!(report.counters().certification_bundle_count(), 0);
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
    assert!(report
        .digest()
        .starts_with("bridge-subscription-certification-cost-posture-report:sha256:"));
}

#[test]
fn certification_schema_compatibility_report_preempts_semantic_drift() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_schema_compatibility();

    assert_ne!(
        report.compatible_bundle_digest(),
        report.incompatible_bundle_digest()
    );
    assert_eq!(
        report.compatible_schema_version(),
        "bridge-subscription-certification-bundle-v1"
    );
    assert_eq!(
        report.incompatible_schema_version(),
        "bridge-subscription-certification-bundle-v999"
    );
    assert_eq!(report.compatible_digest_algorithm(), "sha256");
    assert_eq!(report.incompatible_digest_algorithm(), "sha512");
    assert_eq!(
        report.primary_failure_boundary(),
        crate::facade::BridgeSubscriptionCertificationFailureBoundary::BundleSchemaOrDigestIncompatibility
    );
    assert_eq!(
        report.primary_failure_precedence_stage(),
        crate::facade::BridgeSubscriptionCertificationFailurePrecedenceStage::BundleCompatibility
    );
    assert_eq!(report.suppressed_failure_boundary_count(), 0);
    assert!(report.semantic_drift_hidden_by_schema_incompatibility());
    assert_eq!(report.counters().comparison_plan_count(), 1);
    assert_eq!(report.counters().bundle_comparison_count(), 1);
    assert_eq!(report.counters().bundle_comparison_mismatch_count(), 1);
    assert_eq!(report.counters().failure_localization_count(), 1);
    assert_eq!(report.counters().schema_compatibility_report_count(), 1);
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
    assert!(report
        .comparison_report_digest()
        .starts_with("bridge-subscription-certification-comparison-report:sha256:"));
    assert!(report
        .digest()
        .starts_with("bridge-subscription-certification-schema-compatibility-report:sha256:"));
}

#[test]
fn certification_multi_failure_report_promotes_highest_precedence_boundary() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_multi_failure_precedence();

    assert_ne!(
        report.control_bundle_digest(),
        report.hostile_bundle_digest()
    );
    assert_eq!(
        report.primary_failure_boundary(),
        crate::facade::BridgeSubscriptionCertificationFailureBoundary::BasisDrift
    );
    assert_eq!(
        report.primary_failure_precedence_stage(),
        crate::facade::BridgeSubscriptionCertificationFailurePrecedenceStage::BasisBinding
    );
    assert!(report.basis_drift_is_primary_without_registry_drift());
    assert!(report.suppressed_checkpoint_replay_and_diagnostics());
    assert_eq!(report.suppressed_failure_boundaries().len(), 3);
    assert!(report.suppressed_failure_boundaries().contains(
        &crate::facade::BridgeSubscriptionCertificationFailureBoundary::CheckpointIncompatibility
    ));
    assert!(report
        .suppressed_failure_boundaries()
        .contains(&crate::facade::BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch));
    assert!(report.suppressed_failure_boundaries().contains(
        &crate::facade::BridgeSubscriptionCertificationFailureBoundary::DiagnosticsInfluence
    ));
    assert!(!report
        .suppressed_failure_boundaries()
        .contains(&crate::facade::BridgeSubscriptionCertificationFailureBoundary::RegistryDrift));
    assert!(!report.suppressed_failure_boundaries().contains(
        &crate::facade::BridgeSubscriptionCertificationFailureBoundary::DeclarationEquivalenceDrift
    ));
    assert_eq!(report.counters().comparison_plan_count(), 1);
    assert_eq!(report.counters().bundle_comparison_count(), 1);
    assert_eq!(report.counters().bundle_comparison_mismatch_count(), 4);
    assert_eq!(report.counters().failure_localization_count(), 1);
    assert_eq!(report.counters().multi_failure_precedence_report_count(), 1);
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
    assert!(report
        .comparison_report_digest()
        .starts_with("bridge-subscription-certification-comparison-report:sha256:"));
    assert!(report
        .digest()
        .starts_with("bridge-subscription-certification-multi-failure-precedence-report:sha256:"));
}

#[test]
fn certification_ordering_hostility_preserves_canonical_bundle_meaning() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let report = runtime.certify_subscription_certification_ordering_hostility();

    assert_eq!(
        report.control_source_artifact_index_digest(),
        report.hostile_source_artifact_index_digest()
    );
    assert_eq!(
        report.control_bundle_digest(),
        report.hostile_bundle_digest()
    );
    assert!(report.canonical_source_order_preserved());
    assert!(report.semantic_digest_preserved());
    assert!(report.sealed_bundle_digest_preserved());
    assert!(report.field_order_preserved());
    assert!(report
        .comparison_report_digest()
        .starts_with("bridge-subscription-certification-comparison-report:sha256:"));
    assert_eq!(
        report.comparison_outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::Equivalent
    );
    assert_eq!(report.counters().source_artifact_index_entry_count(), 28);
    assert_eq!(report.counters().source_artifact_index_scan_count(), 28);
    assert_eq!(report.counters().bundle_assembly_plan_count(), 2);
    assert_eq!(report.counters().bundle_cost_profile_count(), 2);
    assert_eq!(report.counters().certification_bundle_count(), 2);
    assert_eq!(report.counters().scratch_allocation_count(), 2);
    assert_eq!(report.counters().scratch_reuse_count(), 2);
    assert_eq!(report.counters().comparison_plan_count(), 1);
    assert_eq!(report.counters().bundle_comparison_count(), 1);
    assert_eq!(report.counters().bundle_comparison_mismatch_count(), 0);
    assert_eq!(report.counters().failure_localization_count(), 0);
    assert_eq!(report.counters().ordering_hostility_report_count(), 1);
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
    assert!(report
        .digest()
        .starts_with("bridge-subscription-certification-ordering-hostility-report:sha256:"));
}

#[test]
fn certification_bundle_assembly_consumes_plan_cost_profile_and_scratch() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let manifest = runtime
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("manifest should seal");
    let index = runtime.build_subscription_certification_source_index(vec![
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            active.active_subscription_identity().as_str(),
            active.digest(),
        ),
    ]);
    let plan = runtime.plan_subscription_certification_bundle(&manifest, &index);
    let cost_profile = runtime
        .admit_subscription_certification_cost_profile(
            crate::facade::BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
            8,
            16,
            32,
            false,
        )
        .expect("sparse certification profile should admit");
    let scratch = runtime.prepare_subscription_certification_scratch(&cost_profile);

    let draft = runtime
        .assemble_subscription_certification_bundle(plan, cost_profile, scratch)
        .expect("admitted sparse plan should assemble");

    assert_eq!(draft.counters().bundle_assembly_plan_count(), 1);
    assert_eq!(draft.counters().bundle_cost_profile_count(), 1);
    assert_eq!(draft.counters().certification_bundle_count(), 1);
    assert_eq!(draft.counters().source_artifact_index_scan_count(), 1);
    assert_eq!(draft.counters().scratch_allocation_count(), 1);
    assert!(draft.fields().iter().any(|field| field.field_state()
        == crate::facade::BridgeSubscriptionBundleFieldState::NotExercised));
    let comparison_inputs = draft
        .fields()
        .iter()
        .find(|field| field.field_name() == "comparison_inputs")
        .expect("comparison input field should be present for phase 2 comparison");
    assert_eq!(
        comparison_inputs.field_state(),
        crate::facade::BridgeSubscriptionBundleFieldState::Present
    );
    assert_eq!(
        comparison_inputs.field_digest(),
        draft.semantic_digests().digest()
    );
    assert_eq!(draft.completeness_report().required_field_count(), 8);
    assert_eq!(draft.completeness_report().present_field_count(), 7);
    assert_eq!(draft.completeness_report().not_exercised_field_count(), 1);
    assert_eq!(
        draft
            .completeness_report()
            .rejected_before_produced_field_count(),
        0
    );
    assert!(draft
        .semantic_digests()
        .subscription_digest()
        .starts_with("bridge-subscription-certification-subscription-digest:sha256:"));
    assert_ne!(
        draft.semantic_digests().subscription_digest(),
        draft.semantic_digests().subscription_delivery_digest()
    );
    assert!(
        !draft
            .semantic_digests()
            .strategy_lowering_digest()
            .eq(draft.semantic_digests().subscription_delivery_digest()),
        "strategy lowering must not collapse onto the delivery digest surface"
    );
    assert!(draft
        .semantic_digests()
        .counter_snapshot_digest()
        .starts_with("bridge-subscription-certification-counters:sha256:"));

    let draft_subscription_digest = draft.semantic_digests().subscription_digest().to_owned();
    let draft_strategy_lowering_digest = draft
        .semantic_digests()
        .strategy_lowering_digest()
        .to_owned();
    let draft_completeness_digest = draft.completeness_report().digest().to_owned();

    let sealed = runtime.seal_subscription_certification_bundle(draft);

    assert_eq!(
        sealed.schema_version(),
        "bridge-subscription-certification-bundle-v1"
    );
    assert_eq!(sealed.digest_algorithm(), "sha256");
    assert_eq!(sealed.counters().certification_bundle_count(), 1);
    assert_eq!(sealed.completeness_report().required_field_count(), 8);
    assert_eq!(
        sealed.semantic_digests().subscription_digest(),
        draft_subscription_digest
    );
    assert_eq!(
        sealed.semantic_digests().strategy_lowering_digest(),
        draft_strategy_lowering_digest
    );
    assert_eq!(
        sealed.completeness_report().digest(),
        draft_completeness_digest
    );
}

#[test]
fn certification_bundle_assembly_rejects_budget_mismatch_before_draft_exists() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let manifest = runtime
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("manifest should seal");
    let index = runtime.build_subscription_certification_source_index(vec![
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
            active.active_subscription_identity().as_str(),
            active.digest(),
        ),
        crate::facade::BridgeSubscriptionSourceArtifactInput::new(
            crate::facade::BridgeSubscriptionSourceArtifactKind::AdmittedSubscription,
            active
                .activation_ready()
                .admitted()
                .admitted_subscription_identity()
                .as_str(),
            active.activation_ready().digest(),
        ),
    ]);
    let plan = runtime.plan_subscription_certification_bundle(&manifest, &index);
    let cost_profile = runtime
        .admit_subscription_certification_cost_profile(
            crate::facade::BridgeSubscriptionCertificationDensityPosture::SparseCertificationWindow,
            1,
            16,
            32,
            false,
        )
        .expect("sparse certification profile should admit");
    let scratch = runtime.prepare_subscription_certification_scratch(&cost_profile);

    let rejection = runtime
        .assemble_subscription_certification_bundle(plan, cost_profile, scratch)
        .expect_err("assembly must reject when indexed source artifacts exceed budget");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionCertificationAssemblyRejectionKind::SourceArtifactBudgetExceeded
    );
    assert_eq!(rejection.counters().certification_bundle_count(), 1);
    assert_eq!(rejection.counters().source_artifact_index_scan_count(), 2);
}

#[test]
fn certification_comparison_reports_semantic_equivalence_from_sealed_bundles() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let left = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1"),
        false,
    );
    let right = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1"),
        false,
    );
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::SemanticEquivalence,
            None,
            None,
        )
        .expect("semantic equivalence plan should admit");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(left.digest(), right.digest());
    assert_eq!(report.left_bundle_digest(), left.digest());
    assert_eq!(report.right_bundle_digest(), right.digest());
    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::Equivalent
    );
    assert_eq!(report.primary_failure_boundary(), None);
    assert_eq!(report.counters().comparison_plan_count(), 1);
    assert_eq!(report.counters().bundle_comparison_count(), 1);
    assert_eq!(report.counters().bundle_comparison_mismatch_count(), 0);
}

#[test]
fn certification_comparison_reports_diagnostics_only_variation() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let left = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1"),
        false,
    );
    let right = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1"),
        true,
    );
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::DiagnosticsOnlyVariation,
            None,
            None,
        )
        .expect("diagnostics variation plan should admit");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(
        left.semantic_digests().subscription_digest(),
        right.semantic_digests().subscription_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_registry_digest(),
        right.semantic_digests().subscription_registry_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_basis_digest(),
        right.semantic_digests().subscription_basis_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_lifecycle_digest(),
        right.semantic_digests().subscription_lifecycle_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_delivery_digest(),
        right.semantic_digests().subscription_delivery_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_share_digest(),
        right.semantic_digests().subscription_share_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_continuation_digest(),
        right.semantic_digests().subscription_continuation_digest()
    );
    assert_eq!(
        left.semantic_digests().consumer_contract_digest(),
        right.semantic_digests().consumer_contract_digest()
    );
    assert_eq!(
        left.semantic_digests().checkpoint_digest(),
        right.semantic_digests().checkpoint_digest()
    );
    assert_eq!(
        left.semantic_digests().routing_digest(),
        right.semantic_digests().routing_digest()
    );
    assert_eq!(
        left.semantic_digests().replay_digest(),
        right.semantic_digests().replay_digest()
    );
    assert_eq!(
        left.semantic_digests().failure_digest(),
        right.semantic_digests().failure_digest()
    );
    assert_eq!(
        left.semantic_digests().residue_digest(),
        right.semantic_digests().residue_digest()
    );
    assert_eq!(
        left.semantic_digests().strategy_lowering_digest(),
        right.semantic_digests().strategy_lowering_digest()
    );
    assert_ne!(
        left.semantic_digests().diagnostics_digest(),
        right.semantic_digests().diagnostics_digest()
    );
    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::DiagnosticsOnlyDifference
    );
    assert_eq!(
        report.primary_failure_boundary(),
        Some(crate::facade::BridgeSubscriptionCertificationFailureBoundary::DiagnosticsInfluence)
    );
    assert_eq!(report.mismatch_count(), 1);
    assert!(report.suppressed_failure_boundaries().is_empty());
}

#[test]
fn certification_comparison_reports_intentional_strategy_divergence() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let left = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1"),
        false,
    );
    let right = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v2"),
        false,
    );
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::IntentionalDivergence,
            None,
            Some(crate::facade::BridgeSubscriptionCertificationDivergenceAxis::StrategyLowering),
        )
        .expect("intentional strategy divergence plan should admit");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::IntentionallyDivergent
    );
    assert_eq!(
        report.primary_failure_boundary(),
        Some(
            crate::facade::BridgeSubscriptionCertificationFailureBoundary::StrategyLoweringProvenanceMismatch
        )
    );
}

#[test]
fn certification_comparison_reports_replay_mismatch_distinctly() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let mut left_inputs =
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    left_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::RetainedReplay,
        "replay-lane",
        "replay-digest-v1",
    ));
    let mut right_inputs =
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    right_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::RetainedReplay,
        "replay-lane",
        "replay-digest-v2",
    ));
    let left = sealed_certification_bundle(&runtime, left_inputs, false);
    let right = sealed_certification_bundle(&runtime, right_inputs, false);
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::ReplayEquivalence,
            None,
            None,
        )
        .expect("replay equivalence plan should admit");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::ReplayMismatch
    );
    assert_eq!(
        report.primary_failure_boundary(),
        Some(crate::facade::BridgeSubscriptionCertificationFailureBoundary::ReplayMismatch)
    );
    assert_eq!(report.mismatch_count(), 1);
}

#[test]
fn certification_comparison_reports_residue_mismatch_distinctly() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let mut left_inputs =
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    left_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::Preview,
        "preview-residue",
        "residue-digest-v1",
    ));
    let mut right_inputs =
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    right_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::Preview,
        "preview-residue",
        "residue-digest-v2",
    ));
    let left = sealed_certification_bundle(&runtime, left_inputs, false);
    let right = sealed_certification_bundle(&runtime, right_inputs, false);
    for bundle in [&left, &right] {
        let preview_records = bundle
            .fields()
            .iter()
            .find(|field| field.field_name() == "preview_records")
            .expect("preview residue source must produce an inspectable preview field");
        assert_eq!(
            preview_records.field_state(),
            crate::facade::BridgeSubscriptionBundleFieldState::Present
        );
        assert_eq!(
            preview_records.field_digest(),
            bundle.semantic_digests().residue_digest()
        );
    }
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::ResidueAbsence,
            None,
            None,
        )
        .expect("residue absence plan should admit");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::ResidueMismatch
    );
    assert_eq!(
        report.primary_failure_boundary(),
        Some(crate::facade::BridgeSubscriptionCertificationFailureBoundary::PreviewResidueMismatch)
    );
    assert_eq!(report.mismatch_count(), 1);
}

#[test]
fn certification_comparison_localizes_failure_record_drift_before_counters() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let left_inputs = active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    let mut right_inputs =
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    right_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::Failure,
        "retained-failure-record",
        "typed-failure-digest-v1",
    ));
    let left = sealed_certification_bundle(&runtime, left_inputs, false);
    let right = sealed_certification_bundle(&runtime, right_inputs, false);
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(
                crate::facade::BridgeSubscriptionCertificationFailureBoundary::MissingRequiredRetainedArtifact,
            ),
            None,
        )
        .expect("expected missing artifact comparison should admit");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary
    );
    assert_eq!(
        report.primary_failure_boundary(),
        Some(
            crate::facade::BridgeSubscriptionCertificationFailureBoundary::MissingRequiredRetainedArtifact
        )
    );
    assert!(report.suppressed_failure_boundaries().contains(
        &crate::facade::BridgeSubscriptionCertificationFailureBoundary::CounterContractViolation
    ));
}

#[test]
fn certification_comparison_reports_expected_rejection_with_precedence() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let mut left_inputs =
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    left_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::BasisBinding,
        "detail-basis",
        "basis-digest-v1",
    ));
    let left = sealed_certification_bundle(&runtime, left_inputs, false);
    let mut right_inputs =
        active_source_inputs(&active, "declaration-digest-v2", "strategy-digest-v2");
    right_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::BasisBinding,
        "detail-basis",
        "basis-digest-v2",
    ));
    right_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
        active.active_subscription_identity().as_str(),
        active.digest(),
    ));
    let right = sealed_certification_bundle(&runtime, right_inputs, true);
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(crate::facade::BridgeSubscriptionCertificationFailureBoundary::RegistryDrift),
            None,
        )
        .expect("expected rejection plan should admit with boundary");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::RejectedAtExpectedBoundary
    );
    assert_eq!(
        report.primary_failure_boundary(),
        Some(crate::facade::BridgeSubscriptionCertificationFailureBoundary::RegistryDrift)
    );
    assert_eq!(
        report.primary_failure_precedence_stage(),
        Some(
            crate::facade::BridgeSubscriptionCertificationFailurePrecedenceStage::DeclarationOrRegistry
        )
    );
    assert!(report
        .suppressed_failure_boundaries()
        .contains(&crate::facade::BridgeSubscriptionCertificationFailureBoundary::BasisDrift));
    assert!(report.suppressed_failure_boundaries().contains(
        &crate::facade::BridgeSubscriptionCertificationFailureBoundary::DiagnosticsInfluence
    ));
    assert!(report.counters().failure_localization_count() > 0);
}

#[test]
fn certification_comparison_reports_unexpected_rejection_boundary() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let mut left_inputs =
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    left_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::BasisBinding,
        "detail-basis",
        "basis-digest-v1",
    ));
    let left = sealed_certification_bundle(&runtime, left_inputs, false);
    let mut right_inputs =
        active_source_inputs(&active, "declaration-digest-v2", "strategy-digest-v1");
    right_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::BasisBinding,
        "detail-basis",
        "basis-digest-v2",
    ));
    let right = sealed_certification_bundle(&runtime, right_inputs, false);
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(crate::facade::BridgeSubscriptionCertificationFailureBoundary::BasisDrift),
            None,
        )
        .expect("expected rejection plan should admit with boundary");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::RejectedAtUnexpectedBoundary
    );
    assert_eq!(
        report.primary_failure_boundary(),
        Some(crate::facade::BridgeSubscriptionCertificationFailureBoundary::RegistryDrift)
    );
    assert!(report
        .suppressed_failure_boundaries()
        .contains(&crate::facade::BridgeSubscriptionCertificationFailureBoundary::BasisDrift));
}

#[test]
fn certification_comparison_detects_counter_contract_drift() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let inputs = active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1");
    let mut duplicate_scan_inputs = inputs.clone();
    duplicate_scan_inputs.push(crate::facade::BridgeSubscriptionSourceArtifactInput::new(
        crate::facade::BridgeSubscriptionSourceArtifactKind::ActiveDelivery,
        active.active_subscription_identity().as_str(),
        active.digest(),
    ));
    let left = sealed_certification_bundle(&runtime, inputs, false);
    let right = sealed_certification_bundle(&runtime, duplicate_scan_inputs, false);
    let plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::CounterContract,
            None,
            None,
        )
        .expect("counter contract plan should admit");

    let report = runtime.compare_subscription_certification_bundles(plan, &left, &right);

    assert_eq!(
        left.semantic_digests().subscription_digest(),
        right.semantic_digests().subscription_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_registry_digest(),
        right.semantic_digests().subscription_registry_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_basis_digest(),
        right.semantic_digests().subscription_basis_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_lifecycle_digest(),
        right.semantic_digests().subscription_lifecycle_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_delivery_digest(),
        right.semantic_digests().subscription_delivery_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_share_digest(),
        right.semantic_digests().subscription_share_digest()
    );
    assert_eq!(
        left.semantic_digests().subscription_continuation_digest(),
        right.semantic_digests().subscription_continuation_digest()
    );
    assert_eq!(
        left.semantic_digests().consumer_contract_digest(),
        right.semantic_digests().consumer_contract_digest()
    );
    assert_eq!(
        left.semantic_digests().checkpoint_digest(),
        right.semantic_digests().checkpoint_digest()
    );
    assert_eq!(
        left.semantic_digests().routing_digest(),
        right.semantic_digests().routing_digest()
    );
    assert_eq!(
        left.semantic_digests().replay_digest(),
        right.semantic_digests().replay_digest()
    );
    assert_eq!(
        left.semantic_digests().diagnostics_digest(),
        right.semantic_digests().diagnostics_digest()
    );
    assert_eq!(
        left.semantic_digests().failure_digest(),
        right.semantic_digests().failure_digest()
    );
    assert_eq!(
        left.semantic_digests().residue_digest(),
        right.semantic_digests().residue_digest()
    );
    assert_eq!(
        left.semantic_digests().strategy_lowering_digest(),
        right.semantic_digests().strategy_lowering_digest()
    );
    assert_ne!(
        left.semantic_digests().counter_snapshot_digest(),
        right.semantic_digests().counter_snapshot_digest()
    );
    assert_ne!(left.counters().digest(), right.counters().digest());
    assert_eq!(
        report.outcome(),
        crate::facade::BridgeSubscriptionCertificationComparisonOutcome::CounterContractViolation
    );
    assert_eq!(
        report.primary_failure_boundary(),
        Some(
            crate::facade::BridgeSubscriptionCertificationFailureBoundary::CounterContractViolation
        )
    );
    assert_eq!(report.mismatch_count(), 1);
}

#[test]
fn certification_comparison_plan_rejects_expected_rejection_without_boundary() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let rejection = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            None,
            None,
        )
        .expect_err("expected rejection plans must name a boundary");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionCertificationComparisonPlanRejectionKind::ExpectedRejectionRequiresBoundary
    );
}

#[test]
fn certification_comparison_plan_rejects_intentional_divergence_without_axis() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let rejection = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::IntentionalDivergence,
            None,
            None,
        )
        .expect_err("intentional divergence plans must name an axis");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionCertificationComparisonPlanRejectionKind::IntentionalDivergenceRequiresAxis
    );
}
