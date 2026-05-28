use crate::facade::{
    certify_milestone_three_hostile_suite, MilestoneThreeDerivedWorkBreadthClass,
    MilestoneThreeHostileScenario,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn milestone_three_derived_work_breadth_exposes_actual_rebuild_scope() {
    let report = certify_milestone_three_hostile_suite(
        || build_milestone_one_runtime().expect(" milestone one runtime builder"),
        "m3.derived_work_breadth",
    )
    .expect("milestone three hostile suite should certify");

    assert_eq!(report.derived_work_breadth_rows.len(), 5);
    assert!(report.derived_work_breadth_rows.iter().all(|row| {
        row.declared_changed_scope_count() > 0
            && row.declared_derived_region_count() > 0
            && row
                .row_digest()
                .starts_with(&format!("scenario={};", row.scenario().as_str()))
    }));
    assert!(report.derived_work_breadth_rows.iter().any(|row| {
        row.scenario() == MilestoneThreeHostileScenario::SplitCollapseChurn
            && row.invalidation_breadth_class()
                == MilestoneThreeDerivedWorkBreadthClass::DeclaredRegions
            && row.rebuild_breadth_class()
                == MilestoneThreeDerivedWorkBreadthClass::WholeViewFallback
            && row.actual_derived_validation_row_count() > 0
            && row.locality_claim_mismatch()
    }));
    assert!(report.derived_work_breadth_rows.iter().any(|row| {
        row.scenario() == MilestoneThreeHostileScenario::BrokenRadialLocalization
            && row.invalidation_breadth_class()
                == MilestoneThreeDerivedWorkBreadthClass::RejectedBeforeDerivedWork
            && row.rebuild_breadth_class()
                == MilestoneThreeDerivedWorkBreadthClass::RejectedBeforeDerivedWork
            && row.actual_derived_validation_row_count() == 0
            && !row.locality_claimed()
    }));
}




