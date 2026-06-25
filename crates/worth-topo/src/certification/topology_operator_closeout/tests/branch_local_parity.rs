use crate::facade::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    TopologyBranchAuthoringBoundary, TopologyMutationDerivedFallbackPolicy,
    TopologyMutationRejectionClass,
};

const SCHEMA_BRANCH_AUTHORITY_PROJECTION_MARKER: &str = "schema_branch_authority_projection";

#[test]
fn milestone_three_hostile_suite_exposes_branch_local_mutation_parity_row() {
    let report = crate::certification::test_support::cached_milestone_three_hostile_suite_report();

    assert_eq!(report.mutation_branch_local_parity_rows.len(), 5);
    for scenario in [
        MilestoneThreeHostileScenario::CancellationChainParity,
        MilestoneThreeHostileScenario::SplitCollapseChurn,
        MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity,
    ] {
        let row = report
            .mutation_branch_local_parity_rows
            .iter()
            .find(|row| row.scenario() == Some(scenario))
            .expect("accepted branch-local parity row should be scenario-specific");
        let scenario_report = report
            .scenario_reports
            .iter()
            .find(|report| report.scenario == scenario)
            .expect("scenario report should exist");
        assert_eq!(row.mutation_origin(), "branch_local_application");
        assert_eq!(
            row.branch_authoring_boundary(),
            TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring
        );
        assert!(row.branch_label().contains("m3.cached_closeout"));
        assert_eq!(row.branch_id(), row.branch_label());
        assert_eq!(
            row.outcome_class(),
            MilestoneThreeHostileOutcomeClass::Accepted
        );
        assert_eq!(row.rejection_class(), None);
        assert_eq!(row.mutation_families(), scenario_report.mutation_families());
        assert_eq!(
            row.topology_mutation_digest().mutation_record_count,
            scenario_report
                .topology_mutation_digest()
                .mutation_record_count
        );
        assert_eq!(
            row.naming_mutation_continuity_matrix().outcome_class(),
            scenario_report.continuity_outcome_class()
        );
        assert_eq!(
            row.derived_fallback_policy(),
            Some(TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback)
        );
        assert_eq!(
            row.derived_fallback_policy(),
            scenario_report.derived_fallback_policy()
        );
        assert!(row.branch_head_diverged_from_main());
        assert!(!row.branch_head_unchanged_after_rejection());
        assert_eq!(row.branch_truth_digest().unwrap().algorithm, "fnv1a64");
        assert!(row.row_digest().contains("outcome=accepted"));
        assert!(row
            .row_digest()
            .contains(SCHEMA_BRANCH_AUTHORITY_PROJECTION_MARKER));
        assert!(row.row_digest().contains(scenario.as_str()));
    }
    for scenario in [
        MilestoneThreeHostileScenario::BowtieAdjacentRewire,
        MilestoneThreeHostileScenario::BrokenRadialLocalization,
    ] {
        let rejected = report
            .mutation_branch_local_parity_rows
            .iter()
            .find(|row| row.scenario() == Some(scenario))
            .expect("rejected branch-local parity row should be present");
        let scenario_report = report
            .scenario_reports
            .iter()
            .find(|report| report.scenario == scenario)
            .expect("scenario report should exist");
        assert_eq!(
            rejected.outcome_class(),
            MilestoneThreeHostileOutcomeClass::Rejected
        );
        assert_eq!(
            rejected.rejection_class(),
            Some(TopologyMutationRejectionClass::InvariantBlocked)
        );
        assert_eq!(
            rejected.branch_authoring_boundary(),
            TopologyBranchAuthoringBoundary::SchemaTopologyAuthoring
        );
        assert!(rejected.branch_head_unchanged_after_rejection());
        assert!(!rejected.branch_head_diverged_from_main());
        assert_eq!(rejected.derived_fallback_policy(), None);
        assert!(rejected.branch_truth_digest().is_none());
        assert_eq!(
            rejected.mutation_families(),
            scenario_report.mutation_families()
        );
        assert_eq!(
            rejected.topology_mutation_digest().mutation_record_count,
            scenario_report
                .topology_mutation_digest()
                .mutation_record_count
        );
        assert_eq!(
            rejected.naming_mutation_continuity_matrix().outcome_class(),
            scenario_report.continuity_outcome_class()
        );
        assert!(rejected.row_digest().contains("outcome=rejected"));
        assert!(rejected
            .row_digest()
            .contains("attempted_rejected_execution=true"));
    }
}
