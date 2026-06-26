use super::lane_frame_cost_certification_test_support::{
    complete_frame_receipt, complete_lane_frame_cost_scenario, complete_synthetic_frame_receipt,
    full_collection_frame_denial, partial_lane_receipt, realtime_ordinary_traversal_denial,
    scenario_with_mismatched_cross_lane_parity, scenario_without_cross_lane_parity,
    scenario_without_foundational_readiness, scenario_without_scale_variation,
    source_parse_frame_denial,
};
use super::{
    WorthUiLaneFrameCostCertificationDenialReason, WorthUiLaneFrameCostCertificationScenario,
    WorthUiSteadyFrameCounterDenialReason,
};

#[test]
fn lane_and_frame_cost_certification_closes_all_platform_lanes() {
    let certification = super::WorthUiLaneAndFrameCostCertification::certify(
        complete_lane_frame_cost_scenario(77),
        77,
    )
    .expect("complete lane/frame-cost evidence certifies");

    assert!(certification
        .lane_certification()
        .covers_all_platform_lanes());
    assert!(certification.frame_cost_certification().is_counter_backed());
    assert_eq!(
        certification.no_source_frame_proof().forbidden_work_count(),
        0
    );
    assert_eq!(
        certification
            .broad_scan_regression_denial()
            .broad_scan_count(),
        0
    );
    assert_eq!(
        certification
            .scale_variation_proof()
            .virtualized_data_sample_count(),
        2
    );
    assert_eq!(
        certification
            .scale_variation_proof()
            .realtime_sample_count(),
        2
    );
    assert!(certification
        .foundational_readiness()
        .is_required_and_satisfied());
    assert_eq!(
        certification
            .foundational_readiness()
            .certified_foundational_receipt_count(),
        certification
            .foundational_readiness()
            .foundational_evidence()
            .receipt_count()
    );
    assert_eq!(certification.counters().certified_frame_receipt_count(), 1);
    assert!(certification.counters().foundational_receipt_count() >= 5);
}

#[test]
fn certification_rejects_frame_receipts_from_another_active_plan() {
    let denial = super::WorthUiLaneAndFrameCostCertification::certify(
        complete_lane_frame_cost_scenario(77),
        78,
    )
    .expect_err("active plan digest mismatch cannot certify frame cost");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::ActivePlanDigestMismatch {
            active_plan_digest: 78,
            receipt_plan_digest: 77
        }
    );
}

#[test]
fn certification_rejects_missing_platform_lane_evidence() {
    let scenario = WorthUiLaneFrameCostCertificationScenario::named("missing-lanes")
        .with_steady_frame_receipt(partial_lane_receipt(5));

    let denial = super::WorthUiLaneAndFrameCostCertification::certify(scenario, 5)
        .expect_err("ordinary-only receipt cannot certify lane coverage");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::MissingLaneEvidence
    );
}

#[test]
fn counter_only_lane_receipts_cannot_fake_lane_certification_evidence() {
    let scenario = WorthUiLaneFrameCostCertificationScenario::named("synthetic-counter-only")
        .with_steady_frame_receipt(complete_synthetic_frame_receipt(21));

    let denial = super::WorthUiLaneAndFrameCostCertification::certify(scenario, 21)
        .expect_err("counter-only lane receipts cannot stand in for lane certification");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::MissingLaneCertificationEvidence
    );
}

#[test]
fn scale_samples_must_belong_to_the_same_active_plan() {
    let scenario = complete_lane_frame_cost_scenario(31)
        .with_virtualized_data_scale_sample(complete_frame_receipt(32, 240, 24, 1));

    let denial = super::WorthUiLaneAndFrameCostCertification::certify(scenario, 31)
        .expect_err("cross-plan scale samples cannot certify frame-cost pressure");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::ActivePlanDigestMismatch {
            active_plan_digest: 31,
            receipt_plan_digest: 32
        }
    );
}

#[test]
fn counter_only_data_scale_samples_cannot_fake_scale_certification() {
    let scenario = complete_lane_frame_cost_scenario(51)
        .with_virtualized_data_scale_sample(complete_frame_receipt(51, 240, 24, 1));

    let denial = super::WorthUiLaneAndFrameCostCertification::certify(scenario, 51)
        .expect_err("counter-only data scale samples cannot prove real lane pressure");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::MissingLaneCertificationEvidence
    );
}

#[test]
fn counter_only_realtime_scale_samples_cannot_fake_scale_certification() {
    let scenario = complete_lane_frame_cost_scenario(52)
        .with_realtime_scale_sample(complete_frame_receipt(52, 120, 24, 3));

    let denial = super::WorthUiLaneAndFrameCostCertification::certify(scenario, 52)
        .expect_err("counter-only realtime scale samples cannot prove real lane pressure");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::MissingLaneCertificationEvidence
    );
}

#[test]
fn data_and_realtime_certification_requires_scale_variation() {
    let denial = super::WorthUiLaneAndFrameCostCertification::certify(
        scenario_without_scale_variation(9),
        9,
    )
    .expect_err("constant-size samples cannot prove scale-sensitive behavior");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::MissingScaleVariation
    );
}

#[test]
fn cross_lane_parity_is_required_before_final_frame_cost_closure() {
    let denial = super::WorthUiLaneAndFrameCostCertification::certify(
        scenario_without_cross_lane_parity(12),
        12,
    )
    .expect_err("mechanics-only evidence cannot close without parity proof");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::MissingCrossLaneParity
    );
}

#[test]
fn cross_lane_parity_must_bind_the_active_plan_under_certification() {
    let denial = super::WorthUiLaneAndFrameCostCertification::certify(
        scenario_with_mismatched_cross_lane_parity(41, 42),
        41,
    )
    .expect_err("cross-lane parity from another active plan cannot certify");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::CrossLaneParityPlanDigestMismatch {
            active_plan_digest: 41,
            parity_active_plan_digest: 42
        }
    );
}

#[test]
fn foundational_readiness_is_mandatory_for_phase_closure() {
    let denial = super::WorthUiLaneAndFrameCostCertification::certify(
        scenario_without_foundational_readiness(44),
        44,
    )
    .expect_err("Phase 37 closure cannot skip Foundational readiness");

    assert_eq!(
        denial.reason(),
        WorthUiLaneFrameCostCertificationDenialReason::FoundationalReadinessNotRequested
    );
}

#[test]
fn source_registry_and_broad_scan_work_cannot_enter_certification_evidence() {
    assert_eq!(
        source_parse_frame_denial(14),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );
    assert_eq!(
        full_collection_frame_denial(14),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );
}

#[test]
fn realtime_lane_cannot_certify_through_ordinary_widget_traversal() {
    assert_eq!(
        realtime_ordinary_traversal_denial(15),
        WorthUiSteadyFrameCounterDenialReason::ForbiddenFramePathWork
    );
}
