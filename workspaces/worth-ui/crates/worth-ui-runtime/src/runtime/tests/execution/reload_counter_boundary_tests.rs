use super::reload_counter_test_support::{
    admission_counters, complete_receipt, impact_counters, plan_lowering_counters,
    query_rebind_counters,
};
use super::{
    WorthUiDurableStateReconciliationCounters, WorthUiFrameCostCounter, WorthUiMeasurementBoundary,
    WorthUiMeasurementCertificationDenial, WorthUiReloadCounterBoundary,
    WorthUiReloadCounterBoundaryDenialReason, WorthUiReloadCounterStopStage,
    WorthUiReloadLoweringFoundationalBridge, WorthUiRuntimeCounterFamily,
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
fn zero_work_phase_does_not_fabricate_a_counter_packet() {
    let receipt = WorthUiReloadCounterBoundary::reload_completed()
        .record_admission_counters(admission_counters())
        .record_reconciliation_counters(WorthUiDurableStateReconciliationCounters::default())
        .seal()
        .expect("real admission work remains a valid receipt");

    assert_eq!(receipt.packets().len(), 1);
    assert_eq!(
        receipt.packets()[0].family(),
        WorthUiRuntimeCounterFamily::ReloadCandidateAdmission
    );
    let certified = receipt
        .certify()
        .expect("the compact receipt passes Worth UI certification");
    WorthUiReloadLoweringFoundationalBridge::lower(&certified)
        .expect("no synthetic zero packet reaches Foundational");
}

#[test]
fn query_rebind_counter_receipt_records_real_work_without_a_fake_support_receipt() {
    let receipt = WorthUiReloadCounterBoundary::reload_completed()
        .record_query_rebind_counters(query_rebind_counters())
        .seal()
        .expect("Query rebind work is measured without reconstructing Query authority");

    assert_eq!(receipt.packets().len(), 1);
    assert!(receipt.packets()[0].query_evidence().is_empty());
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
