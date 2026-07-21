use worth_foundational::{
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceEvidenceStrength,
};

use super::{
    WorthUiComplexityContract, WorthUiCounterCaptureRichness, WorthUiFoundationalCounterBridge,
    WorthUiFrameCostCounter, WorthUiMeasurementBoundary, WorthUiMeasurementCertificationDenial,
    WorthUiMeasurementQueryEvidence, WorthUiRuntimeCounterFamily,
};

#[test]
fn counter_taxonomy_replay_is_deterministic() {
    let left = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            2,
        ))
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.query_receipts_checked",
            4,
        ))
        .with_query_evidence(
            WorthUiMeasurementQueryEvidence::subscription_selection_diagnostics(11),
        )
        .seal()
        .expect("packet should seal");

    let right = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .with_query_evidence(
            WorthUiMeasurementQueryEvidence::subscription_selection_diagnostics(11),
        )
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.query_receipts_checked",
            4,
        ))
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            2,
        ))
        .seal()
        .expect("packet should seal");

    assert_eq!(left.replay_digest(), right.replay_digest());
    assert_eq!(left.counters(), right.counters());
}

#[test]
fn hot_path_without_counter_boundary_rejected_by_certification() {
    let packet = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            1,
        ))
        .seal()
        .expect("packet should seal");

    let denial = packet
        .certify_against(WorthUiComplexityContract::hot_path(
            "reload.candidate_admission",
        ))
        .expect_err("hot path needs a named measurement boundary");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::HotPathWithoutMeasurementBoundary
    );
}

#[test]
fn counter_richness_does_not_change_active_plan_digest() {
    let minimal = packet_with_richness(WorthUiCounterCaptureRichness::Minimal);
    let full = packet_with_richness(WorthUiCounterCaptureRichness::Full);

    assert_eq!(
        minimal.capture_richness(),
        WorthUiCounterCaptureRichness::Minimal
    );
    assert_eq!(full.capture_richness(), WorthUiCounterCaptureRichness::Full);
    assert_ne!(minimal.replay_digest(), full.replay_digest());
    assert_eq!(minimal.active_plan_digest(), full.active_plan_digest());
}

#[test]
fn counter_taxonomy_rejects_unattributed_work_bucket() {
    let denial = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::unattributed_work_bucket(10))
        .seal()
        .expect_err("unattributed work must fail closed");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::UnattributedWorkBucket
    );
}

#[test]
fn foundational_performance_claim_without_worth_ui_counter_denied_before_bridge() {
    let packet = WorthUiRuntimeCounterFamily::steady_frame_rendering()
        .at_boundary(WorthUiMeasurementBoundary::steady_frame_rendering())
        .record(WorthUiFrameCostCounter::elapsed_time_auxiliary(
            "frame.steady_rendering.elapsed_micros",
            120,
        ))
        .seal()
        .expect("auxiliary packet can be inspected");

    let denial = packet
        .certify_against(
            WorthUiComplexityContract::hot_path("frame.steady_rendering")
                .requires_boundary(WorthUiMeasurementBoundary::steady_frame_rendering())
                .requires_counter_family(WorthUiRuntimeCounterFamily::steady_frame_rendering()),
        )
        .expect_err("Foundational lowering needs certified Worth UI execution counters");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::ElapsedTimeOnlyFrameCost
    );
}

#[test]
fn duplicate_counter_names_are_rejected_before_foundational_receipt_construction() {
    let denial = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            1,
        ))
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            2,
        ))
        .seal()
        .expect_err("duplicate counter rows make replay evidence ambiguous");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::DuplicateCounterName
    );
}

#[test]
fn elapsed_time_only_frame_cost_counter_is_auxiliary_not_certifying() {
    let packet = WorthUiRuntimeCounterFamily::steady_frame_rendering()
        .at_boundary(WorthUiMeasurementBoundary::steady_frame_rendering())
        .record(WorthUiFrameCostCounter::elapsed_time_auxiliary(
            "frame.steady_rendering.elapsed_micros",
            120,
        ))
        .seal()
        .expect("auxiliary packet can be inspected");

    let denial = packet
        .certify_against(
            WorthUiComplexityContract::hot_path("frame.steady_rendering")
                .requires_boundary(WorthUiMeasurementBoundary::steady_frame_rendering())
                .requires_counter_family(WorthUiRuntimeCounterFamily::steady_frame_rendering()),
        )
        .expect_err("elapsed time alone must not certify frame cost");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::ElapsedTimeOnlyFrameCost
    );
}

#[test]
fn certified_counter_packet_lowers_to_foundational_counter_evidence() {
    let packet = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            3,
        ))
        .seal()
        .expect("packet should seal");

    let certified = packet
        .certify_against(
            WorthUiComplexityContract::hot_path("reload.candidate_admission")
                .requires_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
                .requires_counter_family(WorthUiRuntimeCounterFamily::reload_candidate_admission())
                .foundational_boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
                .access_pattern(FoundationalPerformanceAccessPatternPosture::TraversalLocal)
                .evidence_strength(
                    FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
                ),
        )
        .expect("Worth UI counters should certify");

    let evidence = WorthUiFoundationalCounterBridge::lower_certified_packet(&certified)
        .expect("certified evidence should lower");

    assert_eq!(evidence.counter_specs().len(), 1);
    assert_eq!(evidence.counter_rows().len(), 1);
    assert_eq!(evidence.counter_backed_receipt().counter_rows().len(), 1);
    assert_eq!(
        evidence
            .counter_backed_receipt()
            .bundle()
            .claim()
            .access_pattern(),
        FoundationalPerformanceAccessPatternPosture::TraversalLocal
    );
    assert!(evidence.canonical_basis_entry_count() > 1);
    assert_eq!(evidence.worth_ui_replay_digest(), certified.replay_digest());
}

#[test]
fn plan_lowering_counter_backed_work_uses_foundational_maintenance_execution() {
    let packet = WorthUiRuntimeCounterFamily::plan_lowering()
        .at_boundary(WorthUiMeasurementBoundary::plan_lowering())
        .record(WorthUiFrameCostCounter::count(
            "plan.lowering.nodes_lowered",
            4,
        ))
        .seal()
        .expect("packet should seal");

    let certified = packet
        .certify_against(
            WorthUiComplexityContract::hot_path("plan.lowering")
                .requires_boundary(WorthUiMeasurementBoundary::plan_lowering())
                .requires_counter_family(WorthUiRuntimeCounterFamily::plan_lowering())
                .foundational_boundary(FoundationalPerformanceBoundary::MaintenanceExecution),
        )
        .expect("counted plan lowering should certify as maintenance execution");

    let evidence = WorthUiFoundationalCounterBridge::lower_certified_packet(&certified)
        .expect("maintenance execution evidence should lower");
    assert_eq!(evidence.counter_backed_receipt().counter_rows().len(), 1);
}

#[test]
fn counter_name_must_preserve_family_boundary() {
    let denial = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count("frame.fake.rows", 1))
        .seal()
        .expect_err("counter name must stay under the family token");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::CounterNameDoesNotMatchFamilyBoundary
    );
}

#[test]
fn zero_counted_work_cannot_certify_hot_path_claim() {
    let packet = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            0,
        ))
        .seal()
        .expect("zero counter packets remain inspectable");

    let denial = packet
        .certify_against(
            WorthUiComplexityContract::hot_path("reload.candidate_admission")
                .requires_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
                .requires_counter_family(WorthUiRuntimeCounterFamily::reload_candidate_admission()),
        )
        .expect_err("zero counted work is not counter-backed execution proof");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::MissingNonzeroWorthUiCounterEvidence
    );
}

#[test]
fn invalid_foundational_counter_label_rejected_before_certification() {
    let denial = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.BadCounter",
            1,
        ))
        .seal()
        .expect_err("invalid Foundational label shape must fail at packet seal");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::InvalidFoundationalCounterName
    );
}

#[test]
fn invalid_foundational_contract_label_rejected_before_bridge_lowering() {
    let packet = WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            1,
        ))
        .seal()
        .expect("packet should seal");

    let denial = packet
        .certify_against(
            WorthUiComplexityContract::hot_path("Reload.CandidateAdmission")
                .requires_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
                .requires_counter_family(WorthUiRuntimeCounterFamily::reload_candidate_admission()),
        )
        .expect_err("invalid contract label must not certify");

    assert_eq!(
        denial,
        WorthUiMeasurementCertificationDenial::InvalidFoundationalContractName
    );
}

#[test]
fn ui_query_rebind_planning_counters_do_not_forge_query_evidence() {
    let packet = WorthUiRuntimeCounterFamily::QueryRebindPlanning
        .at_boundary(WorthUiMeasurementBoundary::QueryRebindPlanning)
        .record(WorthUiFrameCostCounter::count(
            "reload.query_rebind_planning.bindings_rebound",
            1,
        ))
        .seal()
        .expect("packet should seal");

    let certified = packet
        .certify_against(
            WorthUiComplexityContract::hot_path("reload.query_rebind_planning")
                .requires_boundary(WorthUiMeasurementBoundary::QueryRebindPlanning)
                .requires_counter_family(WorthUiRuntimeCounterFamily::QueryRebindPlanning),
        )
        .expect("UI planning work is certified by UI counters");

    assert!(certified.packet().query_evidence().is_empty());
}

fn packet_with_richness(
    capture_richness: WorthUiCounterCaptureRichness,
) -> super::WorthUiMeasurementCounterPacket {
    WorthUiRuntimeCounterFamily::reload_candidate_admission()
        .at_boundary(WorthUiMeasurementBoundary::reload_candidate_admission())
        .with_active_plan_digest(99)
        .with_capture_richness(capture_richness)
        .record(WorthUiFrameCostCounter::count(
            "reload.candidate_admission.candidates_admitted",
            1,
        ))
        .seal()
        .expect("packet should seal")
}
