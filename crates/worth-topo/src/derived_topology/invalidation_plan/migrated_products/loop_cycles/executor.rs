use std::cell::RefCell;
use std::fmt;

use super::{LoopCycleDerivedProductOutput, LoopCycleExecutionInput};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionError, DerivedInvalidationProductExecutionReport,
    DerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopCycleMigrationError {
    ExecutionReceiptFailed,
    NoLoopCycleExecutionObserved,
    OutputDigestNotBoundToReceipt,
    OutputSelectedPlanNotBoundToInput,
    ReadStageReceiptNotBoundToSelectedPlan,
    ReadStageReceiptMissingQueryReceipt,
    ReadStageReceiptMissingLegalityReceipt,
    ReadStageTouchedClosureNotBoundToSelectedPlan,
    ReadStageTouchedClosureSelectedNoLoopCycleRows,
    ReadStageSelectedRowsExceedTouchedClosure,
    ReadStageCountersNotBoundToRows,
    OldAuthorityResidueMissingRequiredCap,
    ExecutionReceiptMissingLoopCycleRow,
    SelectedPlanMissingLoopCycleRow,
    SelectedRowsExceedAvailableRows,
    WholeViewFallbackNotAllowed,
}

impl fmt::Display for LoopCycleMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionReceiptFailed => {
                write!(f, "loop-cycle migration could not execute selected plan")
            }
            Self::NoLoopCycleExecutionObserved => {
                write!(f, "loop-cycle migration observed no loop-cycle execution")
            }
            Self::OutputDigestNotBoundToReceipt => {
                write!(
                    f,
                    "loop-cycle execution receipt did not bind product output"
                )
            }
            Self::OutputSelectedPlanNotBoundToInput => {
                write!(
                    f,
                    "loop-cycle product output was not bound to the selected input plan"
                )
            }
            Self::ReadStageReceiptNotBoundToSelectedPlan => {
                write!(
                    f,
                    "loop-cycle read-stage receipt was not selected-plan bound"
                )
            }
            Self::ReadStageReceiptMissingQueryReceipt => {
                write!(
                    f,
                    "loop-cycle read-stage receipt lacked native Query read identity"
                )
            }
            Self::ReadStageReceiptMissingLegalityReceipt => {
                write!(
                    f,
                    "loop-cycle read-stage receipt lacked selected legality identity"
                )
            }
            Self::ReadStageTouchedClosureNotBoundToSelectedPlan => {
                write!(
                    f,
                    "loop-cycle read source used a touched closure outside the selected plan"
                )
            }
            Self::ReadStageTouchedClosureSelectedNoLoopCycleRows => {
                write!(f, "loop-cycle touched closure selected no loop-cycle rows")
            }
            Self::ReadStageSelectedRowsExceedTouchedClosure => {
                write!(
                    f,
                    "loop-cycle read-stage rows exceeded touched-closure loop-cycle bound"
                )
            }
            Self::ReadStageCountersNotBoundToRows => {
                write!(
                    f,
                    "loop-cycle read-stage counters were not bound to selected source rows"
                )
            }
            Self::OldAuthorityResidueMissingRequiredCap => {
                write!(
                    f,
                    "loop-cycle old authority residue did not cap every required old entry point"
                )
            }
            Self::ExecutionReceiptMissingLoopCycleRow => {
                write!(f, "execution receipt carried no loop-cycle product row")
            }
            Self::SelectedPlanMissingLoopCycleRow => {
                write!(f, "selected invalidation plan carried no loop-cycle row")
            }
            Self::SelectedRowsExceedAvailableRows => {
                write!(f, "selected loop-cycle rows exceeded available source rows")
            }
            Self::WholeViewFallbackNotAllowed => {
                write!(
                    f,
                    "loop-cycle migration cannot close over whole-view fallback work"
                )
            }
        }
    }
}

impl std::error::Error for LoopCycleMigrationError {}

pub(crate) struct LoopCycleDerivedProductExecutor {
    input: LoopCycleExecutionInput,
    output: RefCell<Option<LoopCycleDerivedProductOutput>>,
}

impl LoopCycleDerivedProductExecutor {
    pub(crate) fn new(input: LoopCycleExecutionInput) -> Self {
        Self {
            input,
            output: RefCell::new(None),
        }
    }

    pub(crate) fn output(&self) -> Option<LoopCycleDerivedProductOutput> {
        self.output.borrow().clone()
    }
}

impl DerivedInvalidationProductExecutor for LoopCycleDerivedProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError> {
        if row.family_identity() != DerivedTopologyProductFamilyIdentity::LoopCycles {
            return DerivedInvalidationProductExecutionReport::from_selected_row(row, 0, 0, None);
        }

        let output = LoopCycleDerivedProductOutput::from_execution_input(&self.input);
        let output_digest = output.output_digest().to_string();
        *self.output.borrow_mut() = Some(output);
        DerivedInvalidationProductExecutionReport::from_selected_row_with_product_output(
            row,
            self.input.selected_row_count(),
            0,
            None,
            Some(&output_digest),
        )
    }
}
