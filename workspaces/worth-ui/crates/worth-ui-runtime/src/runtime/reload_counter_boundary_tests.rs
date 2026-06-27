use super::{
    WorthUiCandidateAdmissionCounters, WorthUiDurableStateReconciliationCounters,
    WorthUiExecutionPlanEquivalenceCounters, WorthUiFrameCostCounter, WorthUiIdentityMatchCounters,
    WorthUiImpactLookupCounters, WorthUiMeasurementBoundary, WorthUiMeasurementCertificationDenial,
    WorthUiMeasurementQueryEvidenceKind, WorthUiPlanLoweringCounters, WorthUiPlanTopologyCounters,
    WorthUiQueryLiveRebindCounters, WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus,
    WorthUiReloadCounterBoundary, WorthUiReloadCounterBoundaryDenialReason,
    WorthUiReloadCounterStopStage, WorthUiReloadLoweringFoundationalBridge,
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeCounterFamily,
    WorthUiRuntimeHandleAllocationCounters,
};

#[test]
fn equivalent_reload_work_produces_equivalent_reload_counters() {
    let left = complete_receipt().expect("complete receipt should seal");
    let right = complete_receipt().expect("complete receipt should seal");

    let left_digests: Vec<_> = left
        .packets()
        .iter()
        .map(|packet| packet.replay_digest())
        .collect();
    let right_digests: Vec<_> = right
        .packets()
        .iter()
        .map(|packet| packet.replay_digest())
        .collect();

    assert_eq!(
        left.stopped_at(),
        WorthUiReloadCounterStopStage::PlanEquivalence
    );
    assert_eq!(left.packets(), right.packets());
    assert_eq!(left_digests, right_digests);
}

#[test]
fn impact_narrowing_counter_detects_full_artifact_scan_regression() {
    let mut impact = impact_counters();
    impact.record_full_artifact_scan_for_test();

    let denial = WorthUiReloadCounterBoundary::reload_completed()
        .record_admission_counters(admission_counters())
        .record_impact_narrowing_counters(impact)
        .seal()
        .expect_err("full artifact scans cannot hide in impact narrowing counters");

    assert_eq!(
        denial.reason(),
        WorthUiReloadCounterBoundaryDenialReason::FullArtifactScanDetected
    );
}

#[test]
fn invalid_candidate_emits_admission_and_preservation_counters() {
    let receipt =
        WorthUiReloadCounterBoundary::stopped_at(WorthUiReloadCounterStopStage::CandidateAdmission)
            .record_admission_counters(admission_counters())
            .seal()
            .expect("invalid reloads still emit reached-boundary counters");

    assert_eq!(
        receipt.stopped_at(),
        WorthUiReloadCounterStopStage::CandidateAdmission
    );
    assert_eq!(receipt.packets().len(), 1);
    assert_eq!(
        receipt.packets()[0].family(),
        WorthUiRuntimeCounterFamily::ReloadCandidateAdmission
    );
    assert!(receipt.packets()[0]
        .counters()
        .iter()
        .any(|counter| counter.value() > 0));
}

#[test]
fn reload_counter_detects_repeated_query_support_rediscovery() {
    let denial = WorthUiReloadCounterBoundary::reload_completed()
        .record_admission_counters(admission_counters())
        .record_carried_query_support_receipt(query_support_receipt())
        .record_query_rebind_counters(query_rebind_counters())
        .record_query_support_rediscovery()
        .seal()
        .expect_err("Query support posture must be carried, not rediscovered");

    assert_eq!(
        denial.reason(),
        WorthUiReloadCounterBoundaryDenialReason::RepeatedQuerySupportRediscovery
    );
}

#[test]
fn query_rebind_counter_receipt_requires_carried_query_evidence() {
    let denial = WorthUiReloadCounterBoundary::reload_completed()
        .record_query_rebind_counters(query_rebind_counters())
        .seal()
        .expect_err("Query-bound counter packets must bind carried Query receipt evidence");

    assert_eq!(
        denial.reason(),
        WorthUiReloadCounterBoundaryDenialReason::MissingCarriedQueryEvidence
    );
}

#[test]
fn query_rebind_counter_receipt_carries_query_evidence_independent_of_builder_order() {
    let receipt = WorthUiReloadCounterBoundary::reload_completed()
        .record_query_rebind_counters(query_rebind_counters())
        .record_carried_query_support_receipt(query_support_receipt())
        .seal()
        .expect("carried Query evidence must bind even when recorded after query counters");

    let query_packet = receipt
        .packets()
        .iter()
        .find(|packet| packet.family() == WorthUiRuntimeCounterFamily::QueryRebindPlanning)
        .expect("query rebind packet should be present");

    assert_eq!(receipt.carried_query_receipt_digests(), &[44]);
    assert_eq!(query_packet.query_evidence().len(), 1);
    assert_eq!(
        query_packet.query_evidence()[0].kind(),
        WorthUiMeasurementQueryEvidenceKind::SubscriptionSelectionDiagnostics
    );
    assert_eq!(query_packet.query_evidence()[0].evidence_digest(), 44);
}

#[test]
fn foundational_counter_receipt_rejects_missing_duplicate_or_unexpected_rows() {
    let missing = WorthUiRuntimeCounterFamily::plan_lowering()
        .at_boundary(WorthUiMeasurementBoundary::plan_lowering())
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.staged_node_inputs",
            1,
        ))
        .seal()
        .expect("packet is valid measurement evidence but incomplete for Phase 32");
    let missing_denial = WorthUiReloadCounterBoundary::reload_completed()
        .record_measurement_packet_for_test(missing)
        .seal()
        .expect_err("missing Phase 32 rows must fail before certification");

    assert_eq!(
        missing_denial.reason(),
        WorthUiReloadCounterBoundaryDenialReason::MissingRequiredCounterRow
    );

    let duplicate_denial = WorthUiRuntimeCounterFamily::plan_lowering()
        .at_boundary(WorthUiMeasurementBoundary::plan_lowering())
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.staged_node_inputs",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.staged_node_inputs",
            1,
        ))
        .seal()
        .expect_err("duplicate rows must not enter Phase 32 receipts");
    assert_eq!(
        duplicate_denial,
        WorthUiMeasurementCertificationDenial::DuplicateCounterName
    );

    let unexpected = WorthUiRuntimeCounterFamily::plan_lowering()
        .at_boundary(WorthUiMeasurementBoundary::plan_lowering())
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.staged_node_inputs",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.query_binding_inputs",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.reconciliation_receipt_inputs",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.component_hook_inputs",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.readiness_verifications",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.epoch_verifications",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.source_parse_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.registry_string_lookup_count",
            0,
        ))
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.unexpected_extra_row",
            1,
        ))
        .seal()
        .expect("unexpected row is syntactically valid but not Phase 32 schema");
    let unexpected_denial = WorthUiReloadCounterBoundary::reload_completed()
        .record_measurement_packet_for_test(unexpected)
        .seal()
        .expect_err("unexpected Phase 32 rows must fail before certification");

    assert_eq!(
        unexpected_denial.reason(),
        WorthUiReloadCounterBoundaryDenialReason::UnexpectedCounterRow
    );

    let wrong_spec_denial = WorthUiRuntimeCounterFamily::PlanAssembly
        .at_boundary(WorthUiMeasurementBoundary::PlanAssembly)
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.staged_node_inputs",
            1,
        ))
        .seal()
        .expect_err("rows attached to the wrong counter spec must fail measurement sealing");

    assert_eq!(
        wrong_spec_denial,
        WorthUiMeasurementCertificationDenial::CounterNameDoesNotMatchFamilyBoundary
    );
}

#[test]
fn reload_counter_receipt_rejects_duplicate_phase_packets() {
    let denial = WorthUiReloadCounterBoundary::reload_completed()
        .record_plan_lowering_counters(plan_lowering_counters())
        .record_plan_lowering_counters(plan_lowering_counters())
        .seal()
        .expect_err("a receipt cannot double-count one reload phase");

    assert_eq!(
        denial.reason(),
        WorthUiReloadCounterBoundaryDenialReason::DuplicateCounterPacket
    );
}

#[test]
fn complete_reload_counter_receipt_lowers_to_foundational_evidence() {
    let certified = complete_receipt()
        .expect("complete receipt should seal")
        .certify()
        .expect("complete receipt should certify");

    let evidence = WorthUiReloadLoweringFoundationalBridge::lower(&certified)
        .expect("certified reload/lowering counters should lower");

    assert_eq!(
        evidence.receipt_count(),
        certified.receipt().packets().len()
    );
    assert_eq!(
        evidence
            .evidence()
            .iter()
            .map(|evidence| evidence.worth_ui_replay_digest())
            .collect::<Vec<_>>(),
        certified
            .receipt()
            .packets()
            .iter()
            .map(|packet| packet.replay_digest())
            .collect::<Vec<_>>()
    );
    assert!(evidence.evidence().iter().all(|evidence| {
        evidence.counter_rows().len() == evidence.counter_specs().len()
            && evidence.canonical_basis_entry_count() > 0
    }));
}

fn complete_receipt(
) -> Result<super::WorthUiReloadLoweringCounterReceipt, super::WorthUiReloadCounterBoundaryDenial> {
    WorthUiReloadCounterBoundary::reload_completed()
        .record_admission_counters(admission_counters())
        .record_artifact_comparison_counters(artifact_comparison_counters())
        .record_impact_narrowing_counters(impact_counters())
        .record_identity_match_counters(identity_counters())
        .record_reconciliation_counters(reconciliation_counters())
        .record_carried_query_support_receipt(query_support_receipt())
        .record_query_rebind_counters(query_rebind_counters())
        .record_plan_lowering_counters(plan_lowering_counters())
        .record_plan_assembly_counters(
            handle_allocation_counters(),
            topology_counters(),
            plan_equivalence_counters(),
        )
        .seal()
}

fn admission_counters() -> WorthUiCandidateAdmissionCounters {
    let mut counters = WorthUiCandidateAdmissionCounters::default();
    counters.record_candidate_proof_check();
    counters.record_snapshot_compatibility_check();
    counters.record_runtime_posture_check();
    counters.record_query_support_check();
    counters
}

fn artifact_comparison_counters() -> WorthUiRuntimeArtifactComparisonCounters {
    let mut counters = WorthUiRuntimeArtifactComparisonCounters::default();
    counters.record_artifact_comparison();
    counters
}

fn impact_counters() -> WorthUiImpactLookupCounters {
    let mut counters = WorthUiImpactLookupCounters::default();
    counters.record_impact_classification_consumed();
    counters.record_dependency_metadata_read();
    counters.record_module_impact_lookup();
    counters.record_subtree_impact_lookup();
    counters.record_runtime_hook_lookup();
    counters.record_subtree_digest_lookup();
    counters
}

fn identity_counters() -> WorthUiIdentityMatchCounters {
    let mut counters = WorthUiIdentityMatchCounters::default();
    counters.record_active_node_indexed();
    counters.record_candidate_node_indexed();
    counters.record_stable_seed_lookup();
    counters.record_match_emitted();
    counters
}

fn reconciliation_counters() -> WorthUiDurableStateReconciliationCounters {
    let mut counters = WorthUiDurableStateReconciliationCounters::default();
    counters.record_family();
    counters.record_node();
    counters.record_query_posture_required();
    counters
}

fn query_rebind_counters() -> WorthUiQueryLiveRebindCounters {
    let mut counters = WorthUiQueryLiveRebindCounters::default();
    counters.record_preserved_binding_for_test();
    counters
}

fn query_support_receipt() -> WorthUiQuerySupportReceipt {
    WorthUiQuerySupportReceipt::for_test(WorthUiQuerySupportStatus::Supported, 44)
}

fn plan_lowering_counters() -> WorthUiPlanLoweringCounters {
    let mut counters = WorthUiPlanLoweringCounters::default();
    counters.record_epoch_verification();
    counters.record_readiness_verification();
    counters.record_staged_node_input();
    counters.record_query_binding_input();
    counters.record_reconciliation_receipts(1);
    counters.record_component_hook_input();
    counters
}

fn handle_allocation_counters() -> WorthUiRuntimeHandleAllocationCounters {
    let mut counters = WorthUiRuntimeHandleAllocationCounters::default();
    counters.record_plan_node_input();
    counters.record_component_handle();
    counters.record_command_handle();
    counters.record_token_handle();
    counters.record_collision_check();
    counters
}

fn topology_counters() -> WorthUiPlanTopologyCounters {
    let mut counters = WorthUiPlanTopologyCounters::default();
    counters.record_plan_node_input();
    counters.record_topology_node();
    counters.record_lookup_entry();
    counters.record_validation();
    counters
}

fn plan_equivalence_counters() -> WorthUiExecutionPlanEquivalenceCounters {
    let mut counters = WorthUiExecutionPlanEquivalenceCounters::default();
    counters.record_plan_digest();
    counters.record_plan_node_digest();
    counters.record_equivalence_comparison();
    counters
}
