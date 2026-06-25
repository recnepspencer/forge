use super::DerivedInvalidationProductExecutionReport;
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

pub(crate) trait DerivedInvalidationProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlannedDerivedInvalidationProductExecutor;

impl DerivedInvalidationProductExecutor for PlannedDerivedInvalidationProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError> {
        DerivedInvalidationProductExecutionReport::from_selected_row(row, 1, 0, None)
    }
}
