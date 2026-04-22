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
fn offline_audit_diagnoses_from_canonicalized_sealed_bundles_and_reports() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let left = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1"),
        false,
    );
    let right = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v2", "strategy-digest-v1"),
        false,
    );
    let comparison_plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::ExpectedRejection,
            Some(crate::facade::BridgeSubscriptionCertificationFailureBoundary::RegistryDrift),
            None,
        )
        .expect("expected rejection comparison plan should admit");
    let comparison_report =
        runtime.compare_subscription_certification_bundles(comparison_plan, &left, &right);
    let bundle_index =
        runtime.build_subscription_offline_audit_bundle_index(vec![&right, &left, &left]);

    let audit_plan = runtime
        .plan_subscription_offline_audit(
            &bundle_index,
            vec![&comparison_report, &comparison_report],
            false,
            false,
        )
        .expect("offline audit plan should admit sealed bundle index and comparison reports");
    assert_eq!(
        audit_plan.comparison_report_count(),
        1,
        "audit planning must canonicalize duplicate comparison reports before summarizing outcomes"
    );
    assert_eq!(audit_plan.outcome_summary().expected_rejection_count(), 1);
    let audit_report = runtime.audit_subscription_certification_bundle_offline(audit_plan);
    let inspection = runtime.inspect_subscription_certification(&audit_report);

    assert_eq!(bundle_index.bundle_count(), 2);
    assert_eq!(
        audit_report.outcome(),
        crate::facade::BridgeSubscriptionOfflineAuditOutcome::DiagnosedOffline
    );
    assert_eq!(audit_report.comparison_report_count(), 1);
    assert_eq!(audit_report.outcome_summary().expected_rejection_count(), 1);
    assert_eq!(audit_report.outcome_summary().equivalent_count(), 0);
    assert_eq!(
        audit_report.counters().offline_audit_bundle_index_count(),
        1
    );
    assert_eq!(audit_report.counters().offline_audit_plan_count(), 1);
    assert_eq!(audit_report.counters().offline_audit_report_count(), 1);
    assert_eq!(audit_report.counters().offline_audit_bundle_count(), 2);
    assert_eq!(
        audit_report
            .counters()
            .offline_audit_comparison_report_count(),
        1
    );
    assert_eq!(inspection.audit_report_digest(), audit_report.digest());
    assert_eq!(inspection.outcome(), audit_report.outcome());
    assert_eq!(
        inspection.counter_digest(),
        audit_report.counters().digest().as_ref()
    );
    assert_eq!(
        inspection.outcome_summary_digest(),
        audit_report.outcome_summary().digest()
    );
    assert_eq!(inspection.host_log_dependency_count(), 0);
    assert_eq!(inspection.live_state_dependency_count(), 0);
}

#[test]
fn offline_audit_rejects_host_log_and_live_state_dependencies() {
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
    let comparison_plan = runtime
        .plan_subscription_certification_comparison(
            crate::facade::BridgeSubscriptionCertificationComparisonRelationship::SemanticEquivalence,
            None,
            None,
        )
        .expect("semantic equivalence comparison plan should admit");
    let comparison_report =
        runtime.compare_subscription_certification_bundles(comparison_plan, &left, &right);
    let bundle_index = runtime.build_subscription_offline_audit_bundle_index(vec![&left, &right]);

    let host_log_rejection = runtime
        .plan_subscription_offline_audit(&bundle_index, vec![&comparison_report], true, false)
        .expect_err("offline audit must reject host log dependencies");
    assert_eq!(
        host_log_rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionOfflineAuditPlanRejectionKind::HostLogDependencyForbidden
    );
    assert_eq!(host_log_rejection.counters().host_log_dependency_count(), 1);

    let live_state_rejection = runtime
        .plan_subscription_offline_audit(&bundle_index, vec![&comparison_report], false, true)
        .expect_err("offline audit must reject live runtime dependencies");
    assert_eq!(
        live_state_rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionOfflineAuditPlanRejectionKind::LiveStateDependencyForbidden
    );
    assert_eq!(
        live_state_rejection
            .counters()
            .live_state_dependency_count(),
        1
    );
}

#[test]
fn offline_audit_requires_bundles_and_comparison_reports() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let left = sealed_certification_bundle(
        &runtime,
        active_source_inputs(&active, "declaration-digest-v1", "strategy-digest-v1"),
        false,
    );
    let empty_index = runtime.build_subscription_offline_audit_bundle_index(Vec::new());
    let populated_index = runtime.build_subscription_offline_audit_bundle_index(vec![&left]);

    let empty_index_rejection = runtime
        .plan_subscription_offline_audit(&empty_index, Vec::new(), false, false)
        .expect_err("offline audit requires emitted sealed bundles");
    assert_eq!(
        empty_index_rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionOfflineAuditPlanRejectionKind::EmptyBundleIndex
    );

    let missing_reports_rejection = runtime
        .plan_subscription_offline_audit(&populated_index, Vec::new(), false, false)
        .expect_err("offline audit requires comparison reports");
    assert_eq!(
        missing_reports_rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionOfflineAuditPlanRejectionKind::MissingComparisonReports
    );
}
