use super::lane_meaning_parity_test_support::{
    plan_with_command_semantics_changed, query_preserving_lane_change_fixture,
};
use crate::runtime::{WorthUiCrossLaneSemanticFamily, WorthUiLaneParityDenialReason};

#[test]
fn same_artifact_meaning_preserved_across_admitted_lane_transition() {
    let fixture = query_preserving_lane_change_fixture();

    let report = fixture
        .runtime
        .certify_lane_meaning_parity(
            &fixture.node_plan,
            &fixture.narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &fixture.query_comparison,
            Some(&fixture.query_rebind_plan),
        )
        .expect("lane meaning parity certifies");

    assert!(report.certifies_activation());
    assert_eq!(report.transitions().len(), 1);
    assert!(report.transitions()[0].mechanics_changed());
    assert_eq!(report.transitions()[0].active_lane(), None);
    assert_eq!(report.transitions()[0].candidate_lane(), None);
    assert!(report.transitions()[0]
        .meaning_parity()
        .iter()
        .all(|parity| parity.reference().preserves_meaning()));
    assert!(report.transitions()[0]
        .meaning_parity()
        .iter()
        .any(|parity| parity.reference().family()
            == WorthUiCrossLaneSemanticFamily::LaneChangeIdentity));
    assert_eq!(report.counters().semantic_mismatch_count(), 0);
    assert_eq!(report.counters().source_parse_count(), 0);
    assert_eq!(report.counters().registry_lookup_count(), 0);
    assert_eq!(report.counters().frame_execution_count(), 0);
}

#[test]
fn lane_report_does_not_guess_transition_lane_from_plan_partition_shape() {
    let fixture = query_preserving_lane_change_fixture();

    let report = fixture
        .runtime
        .certify_lane_meaning_parity(
            &fixture.node_plan,
            &fixture.narrowing,
            &fixture.active_plan,
            &fixture.candidate_plan,
            &fixture.query_comparison,
            Some(&fixture.query_rebind_plan),
        )
        .expect("lane meaning parity certifies");
    let transition = &report.transitions()[0];

    assert!(transition.mechanics_changed());
    assert_eq!(transition.active_lane(), None);
    assert_eq!(transition.candidate_lane(), None);
}

#[test]
fn visual_similarity_without_semantic_parity_does_not_certify_lane_transition() {
    let fixture = query_preserving_lane_change_fixture();
    let candidate_plan = plan_with_command_semantics_changed(&fixture.active_plan);

    let denial = fixture
        .runtime
        .certify_lane_meaning_parity(
            &fixture.node_plan,
            &fixture.narrowing,
            &fixture.active_plan,
            &candidate_plan,
            &fixture.query_comparison,
            Some(&fixture.query_rebind_plan),
        )
        .expect_err("visual similarity cannot replace semantic parity");

    assert_eq!(
        denial.reason(),
        WorthUiLaneParityDenialReason::VisualSimilarityWithoutSemanticParity
    );
    assert_eq!(denial.counters().visual_only_evidence_rejected_count(), 1);
}
