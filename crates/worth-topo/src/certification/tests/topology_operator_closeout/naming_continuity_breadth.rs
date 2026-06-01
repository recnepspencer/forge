use super::super::*;
use crate::facade::{MilestoneThreeHostileScenario, TopologyMutationNamingOutcome};

#[test]
fn milestone_three_closeout_requires_naming_continuity_breadth_rows() {
    let report = certify_milestone_three_closeout(
        || {
            crate::validation::reference_integrity::milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "milestone-three-naming-continuity-breadth",
    )
    .expect("milestone three closeout should certify");

    assert_eq!(report.naming_continuity_breadth_rows.len(), 5);
    assert!(report.naming_continuity_breadth_rows.iter().all(|row| {
        row.continuity_row_count() > 0
            && row.naming_scope_count() > 0
            && row.replay_step_count() > 0
            && row.replay_checked()
            && row.continuity_row_count()
                == row.preserved_count() + row.ambiguous_count() + row.rejected_count()
            && row.row_digest().contains("continuity_rows=")
            && row.row_digest().contains("naming_scopes=")
    }));
    assert!(report.naming_continuity_breadth_rows.iter().any(|row| {
        row.scenario() == MilestoneThreeHostileScenario::AmbiguousLocalRewireContinuity
            && row.outcome_class() == TopologyMutationNamingOutcome::Ambiguous
            && row.ambiguous_count() > 0
            && row.preserved_count() == 0
    }));
    assert!(report.naming_continuity_breadth_rows.iter().any(|row| {
        row.scenario() == MilestoneThreeHostileScenario::SplitCollapseChurn
            && row.outcome_class() == TopologyMutationNamingOutcome::Rejected
            && row.rejected_count() > 0
    }));
}
