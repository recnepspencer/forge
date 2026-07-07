use super::{
    DerivedInvalidationExecutionError, DerivedInvalidationExecutionErrorKind,
    DerivedInvalidationExecutionOutcome,
};
use crate::derived_topology::materialized_graph::{
    MaterializationFallbackClass, MaterializationReport,
};

pub(crate) fn admit_materialization_report_for_execution_outcome(
    outcome: DerivedInvalidationExecutionOutcome,
    report: &MaterializationReport,
) -> Result<(), DerivedInvalidationExecutionError> {
    if ordinary_whole_view_fallback_is_forbidden(outcome, report) {
        return Err(DerivedInvalidationExecutionError::new(
            DerivedInvalidationExecutionErrorKind::OrdinaryWholeViewFallbackNotAdmitted,
        ));
    }
    Ok(())
}

fn ordinary_whole_view_fallback_is_forbidden(
    outcome: DerivedInvalidationExecutionOutcome,
    report: &MaterializationReport,
) -> bool {
    let is_whole_view_fallback = report.whole_view_materialization
        || report.fallback_class == Some(MaterializationFallbackClass::WholeViewRebuild);
    is_whole_view_fallback
        && !matches!(
            outcome,
            DerivedInvalidationExecutionOutcome::BoundedRebuilt
                | DerivedInvalidationExecutionOutcome::ResidueCapped
        )
}
