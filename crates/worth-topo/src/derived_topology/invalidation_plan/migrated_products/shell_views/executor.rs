use std::cell::RefCell;
use std::fmt;

use super::{ShellViewDerivedProductOutput, ShellViewExecutionInput};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionError, DerivedInvalidationProductExecutionReport,
    DerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellViewMigrationError {
    ExecutionReceiptFailed,
    NoShellViewExecutionObserved,
    OutputDigestNotBoundToReceipt,
    OutputSelectedPlanNotBoundToInput,
    ReadStageReceiptNotBoundToSelectedPlan,
    ReadStageReceiptMissingQueryReceipt,
    ReadStageQueryReceiptNotBoundToSource,
    ReadStageReceiptMissingLegalityReceipt,
    ReadStageQueryProofInvalid,
    ReadStageTouchedClosureNotBoundToSelectedPlan,
    ReadStageTouchedClosureSelectedNoShellViewRows,
    ReadStageSelectedRowsExceedTouchedClosure,
    ReadStageCountersNotBoundToRows,
    OldAuthorityResidueMissingRequiredCap,
    ExecutionReceiptMissingShellViewRow,
    SelectedPlanMissingShellViewRow,
    SelectedRowsExceedAvailableRows,
    WholeViewFallbackNotAllowed,
}

impl fmt::Display for ShellViewMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionReceiptFailed => {
                write!(f, "shell-view migration could not execute selected plan")
            }
            Self::NoShellViewExecutionObserved => {
                write!(f, "shell-view migration observed no shell-view execution")
            }
            Self::OutputDigestNotBoundToReceipt => {
                write!(
                    f,
                    "shell-view execution receipt did not bind product output"
                )
            }
            Self::OutputSelectedPlanNotBoundToInput => {
                write!(
                    f,
                    "shell-view product output was not bound to the selected input plan"
                )
            }
            Self::ReadStageReceiptNotBoundToSelectedPlan => {
                write!(
                    f,
                    "shell-view read-stage receipt was not selected-plan bound"
                )
            }
            Self::ReadStageReceiptMissingQueryReceipt => {
                write!(
                    f,
                    "shell-view read-stage receipt lacked native Query read identity"
                )
            }
            Self::ReadStageQueryReceiptNotBoundToSource => {
                write!(
                    f,
                    "shell-view read-stage receipt did not match the read source Query proof"
                )
            }
            Self::ReadStageReceiptMissingLegalityReceipt => {
                write!(
                    f,
                    "shell-view read-stage receipt lacked selected legality identity"
                )
            }
            Self::ReadStageQueryProofInvalid => {
                write!(
                    f,
                    "shell-view read source was not backed by valid Query radial proof"
                )
            }
            Self::ReadStageTouchedClosureNotBoundToSelectedPlan => {
                write!(
                    f,
                    "shell-view read source used a touched closure outside the selected plan"
                )
            }
            Self::ReadStageTouchedClosureSelectedNoShellViewRows => {
                write!(f, "shell-view touched closure selected no shell-view rows")
            }
            Self::ReadStageSelectedRowsExceedTouchedClosure => {
                write!(
                    f,
                    "shell-view read-stage rows exceeded touched-closure shell-view bound"
                )
            }
            Self::ReadStageCountersNotBoundToRows => {
                write!(
                    f,
                    "shell-view read-stage counters were not bound to selected source rows"
                )
            }
            Self::OldAuthorityResidueMissingRequiredCap => {
                write!(
                    f,
                    "shell-view old authority residue did not cap every required old entry point"
                )
            }
            Self::ExecutionReceiptMissingShellViewRow => {
                write!(f, "execution receipt carried no shell-view product row")
            }
            Self::SelectedPlanMissingShellViewRow => {
                write!(f, "selected invalidation plan carried no shell-view row")
            }
            Self::SelectedRowsExceedAvailableRows => {
                write!(f, "selected shell-view rows exceeded available source rows")
            }
            Self::WholeViewFallbackNotAllowed => {
                write!(
                    f,
                    "shell-view migration cannot close over whole-view fallback work"
                )
            }
        }
    }
}

impl std::error::Error for ShellViewMigrationError {}

pub(crate) struct ShellViewDerivedProductExecutor {
    input: ShellViewExecutionInput,
    output: RefCell<Option<ShellViewDerivedProductOutput>>,
}

impl ShellViewDerivedProductExecutor {
    pub(crate) fn new(input: ShellViewExecutionInput) -> Self {
        Self {
            input,
            output: RefCell::new(None),
        }
    }

    pub(crate) fn output(&self) -> Option<ShellViewDerivedProductOutput> {
        self.output.borrow().clone()
    }
}

impl DerivedInvalidationProductExecutor for ShellViewDerivedProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError> {
        if row.family_identity() != DerivedTopologyProductFamilyIdentity::ShellViews {
            return DerivedInvalidationProductExecutionReport::from_selected_row(row, 0, 0, None);
        }

        let output = ShellViewDerivedProductOutput::from_execution_input(&self.input);
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
