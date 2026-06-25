use super::super::{
    admit_materialization_report_for_execution_outcome, DerivedInvalidationExecutionErrorKind,
    DerivedInvalidationExecutionOutcome,
};
use super::support::{bounded_materialization_report, whole_view_materialization_report};

#[test]
fn ordinary_whole_view_fallback_is_rejected_for_incremental_execution() {
    let error = admit_materialization_report_for_execution_outcome(
        DerivedInvalidationExecutionOutcome::IncrementalUpdated,
        &whole_view_materialization_report(),
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        DerivedInvalidationExecutionErrorKind::OrdinaryWholeViewFallbackNotAdmitted
    );
}

#[test]
fn whole_view_fallback_is_visible_only_for_bounded_or_residue_outcomes() {
    admit_materialization_report_for_execution_outcome(
        DerivedInvalidationExecutionOutcome::BoundedRebuilt,
        &whole_view_materialization_report(),
    )
    .unwrap();
    admit_materialization_report_for_execution_outcome(
        DerivedInvalidationExecutionOutcome::ResidueCapped,
        &whole_view_materialization_report(),
    )
    .unwrap();
    admit_materialization_report_for_execution_outcome(
        DerivedInvalidationExecutionOutcome::IncrementalUpdated,
        &bounded_materialization_report(),
    )
    .unwrap();
}
