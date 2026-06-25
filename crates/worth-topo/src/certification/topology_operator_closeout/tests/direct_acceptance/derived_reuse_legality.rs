use crate::facade::{
    MilestoneThreeHostileScenario, MilestoneThreeMutationFalloutClass, ReplayParityStatus,
};

#[test]
fn milestone_three_reuse_legality_rows_are_suppression_honest() {
    let report = crate::certification::test_support::cached_milestone_three_hostile_suite_report();

    assert_eq!(report.derived_reuse_legality_rows.len(), 5);
    assert!(report.derived_reuse_legality_rows.iter().all(|row| {
        !row.recompute_suppression_claimed()
            && !row.equivalence_contract_required()
            && row.replay_materialized_topology_equivalent()
            && row.row_digest().contains("suppression_claimed=false")
    }));
    assert!(report.derived_reuse_legality_rows.iter().any(|row| {
        row.scenario() == MilestoneThreeHostileScenario::SplitCollapseChurn
            && row.fallout_class() == MilestoneThreeMutationFalloutClass::WholeViewFallback
            && row.fallback_count() == 1
            && row.derived_validation_digest().is_some()
    }));
    assert!(report.derived_reuse_legality_rows.iter().any(|row| {
        row.scenario() == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.fallout_class() == MilestoneThreeMutationFalloutClass::RejectedBeforeDerivedWork
            && row.fallback_count() == 0
            && row.derived_validation_digest().is_none()
    }));
    assert!(report
        .mutation_replay_parity_rows
        .iter()
        .all(|row| row.parity_status() == ReplayParityStatus::Match));
}
