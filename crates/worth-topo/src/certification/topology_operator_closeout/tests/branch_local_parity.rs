use crate::facade::{
    certify_milestone_three_hostile_suite, MilestoneThreeHostileOutcomeClass,
    MilestoneThreeHostileScenario, TopologyEditFamily, TopologyEditNamingOutcome,
    TopologyEditRejectionClass,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn milestone_three_hostile_suite_exposes_branch_local_edit_parity_row() {
    let report = certify_milestone_three_hostile_suite(
        || build_milestone_one_runtime().expect("milestone one runtime builder"),
        "m3.branch_local_parity",
    )
    .expect("milestone three hostile suite should certify");

    assert_eq!(report.edit_branch_local_parity_rows.len(), 3);
    let row = report
        .edit_branch_local_parity_rows
        .iter()
        .find(|row| row.outcome_class() == MilestoneThreeHostileOutcomeClass::Accepted)
        .expect("accepted branch-local parity row should be present");
    assert_eq!(row.scenario(), None);
    assert_eq!(row.mutation_origin(), "branch_local_application");
    assert!(row.branch_label().contains("m3.branch_local_parity"));
    assert_eq!(row.branch_id(), row.branch_label());
    assert_eq!(
        row.outcome_class(),
        MilestoneThreeHostileOutcomeClass::Accepted
    );
    assert_eq!(row.rejection_class(), None);
    assert_eq!(
        row.edit_families(),
        &[TopologyEditFamily::CreateTopologyEntity]
    );
    assert_eq!(row.topology_edit_digest().contract_count, 1);
    assert_eq!(
        row.naming_edit_continuity_matrix().outcome_class(),
        TopologyEditNamingOutcome::Preserved
    );
    assert!(row.branch_head_diverged_from_main());
    assert!(!row.branch_head_unchanged_after_rejection());
    assert_eq!(row.branch_truth_digest().unwrap().algorithm, "fnv1a64");
    assert!(row.row_digest().starts_with(&format!(
        "branch={};origin=branch_local_application;outcome=accepted;",
        row.branch_label()
    )));
    for scenario in [
        MilestoneThreeHostileScenario::BowtieAdjacentRewire,
        MilestoneThreeHostileScenario::BrokenRadialLocalization,
    ] {
        let rejected = report
            .edit_branch_local_parity_rows
            .iter()
            .find(|row| row.scenario() == Some(scenario))
            .expect("rejected branch-local parity row should be present");
        assert_eq!(
            rejected.outcome_class(),
            MilestoneThreeHostileOutcomeClass::Rejected
        );
        assert_eq!(
            rejected.rejection_class(),
            Some(TopologyEditRejectionClass::InvariantBlocked)
        );
        assert!(rejected.branch_head_unchanged_after_rejection());
        assert!(!rejected.branch_head_diverged_from_main());
        assert!(rejected.branch_truth_digest().is_none());
        assert!(rejected.row_digest().contains("outcome=rejected"));
    }
}
