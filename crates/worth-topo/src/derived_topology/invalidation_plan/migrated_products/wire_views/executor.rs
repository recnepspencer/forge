use std::cell::RefCell;
use std::fmt;

use super::{WireViewDerivedProductOutput, WireViewExecutionInput};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionError, DerivedInvalidationProductExecutionReport,
    DerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireViewMigrationError {
    ExecutionReceiptFailed,
    NoWireViewExecutionObserved,
    OutputDigestNotBoundToReceipt,
    OutputSelectedPlanNotBoundToInput,
    ReadStageReceiptNotBoundToSelectedPlan,
    ReadStageReceiptMissingQueryReceipt,
    ReadStageQueryReceiptNotBoundToSource,
    ReadStageReceiptMissingLegalityReceipt,
    ReadStageQueryProofInvalid,
    ReadStageTouchedClosureNotBoundToSelectedPlan,
    ReadStageTouchedClosureSelectedNoWireViewRows,
    ReadStageSelectedRowsExceedTouchedClosure,
    ReadStageCountersNotBoundToRows,
    OldAuthorityResidueNotCapped,
    OldAuthorityResidueMissingRequiredCap,
    ExecutionReceiptMissingWireViewRow,
    SelectedPlanMissingWireViewRow,
    SelectedRowsExceedAvailableRows,
    WholeViewFallbackNotAllowed,
}

impl fmt::Display for WireViewMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionReceiptFailed => {
                write!(f, "wire-view migration could not execute selected plan")
            }
            Self::NoWireViewExecutionObserved => {
                write!(f, "wire-view migration observed no wire-view execution")
            }
            Self::OutputDigestNotBoundToReceipt => {
                write!(f, "wire-view execution receipt did not bind product output")
            }
            Self::OutputSelectedPlanNotBoundToInput => {
                write!(
                    f,
                    "wire-view product output was not bound to the selected input plan"
                )
            }
            Self::ReadStageReceiptNotBoundToSelectedPlan => {
                write!(
                    f,
                    "wire-view read-stage receipt was not selected-plan bound"
                )
            }
            Self::ReadStageReceiptMissingQueryReceipt => {
                write!(
                    f,
                    "wire-view read-stage receipt lacked native Query read identity"
                )
            }
            Self::ReadStageQueryReceiptNotBoundToSource => {
                write!(
                    f,
                    "wire-view read-stage receipt did not match the read source Query proof"
                )
            }
            Self::ReadStageReceiptMissingLegalityReceipt => {
                write!(
                    f,
                    "wire-view read-stage receipt lacked selected legality identity"
                )
            }
            Self::ReadStageQueryProofInvalid => {
                write!(
                    f,
                    "wire-view read source was not backed by valid Query proof"
                )
            }
            Self::ReadStageTouchedClosureNotBoundToSelectedPlan => {
                write!(
                    f,
                    "wire-view read source used a touched closure outside the selected plan"
                )
            }
            Self::ReadStageTouchedClosureSelectedNoWireViewRows => {
                write!(f, "wire-view touched closure selected no wire-view rows")
            }
            Self::ReadStageSelectedRowsExceedTouchedClosure => {
                write!(
                    f,
                    "wire-view read-stage rows exceeded touched-closure wire-view bound"
                )
            }
            Self::ReadStageCountersNotBoundToRows => {
                write!(
                    f,
                    "wire-view read-stage counters were not bound to selected source rows"
                )
            }
            Self::OldAuthorityResidueNotCapped => {
                write!(f, "wire-view old authority residue was not capped")
            }
            Self::OldAuthorityResidueMissingRequiredCap => {
                write!(
                    f,
                    "wire-view old authority residue did not cap every required old entry point"
                )
            }
            Self::ExecutionReceiptMissingWireViewRow => {
                write!(f, "execution receipt carried no wire-view product row")
            }
            Self::SelectedPlanMissingWireViewRow => {
                write!(f, "selected invalidation plan carried no wire-view row")
            }
            Self::SelectedRowsExceedAvailableRows => {
                write!(f, "selected wire-view rows exceeded available source rows")
            }
            Self::WholeViewFallbackNotAllowed => {
                write!(
                    f,
                    "wire-view migration cannot close over whole-view fallback work"
                )
            }
        }
    }
}

impl std::error::Error for WireViewMigrationError {}

pub(crate) struct WireViewDerivedProductExecutor {
    input: WireViewExecutionInput,
    output: RefCell<Option<WireViewDerivedProductOutput>>,
}

impl WireViewDerivedProductExecutor {
    pub(crate) fn new(input: WireViewExecutionInput) -> Self {
        Self {
            input,
            output: RefCell::new(None),
        }
    }

    pub(crate) fn output(&self) -> Option<WireViewDerivedProductOutput> {
        self.output.borrow().clone()
    }
}

impl DerivedInvalidationProductExecutor for WireViewDerivedProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError> {
        if row.family_identity() != DerivedTopologyProductFamilyIdentity::WireViews {
            return DerivedInvalidationProductExecutionReport::from_selected_row(row, 0, 0, None);
        }

        let output = WireViewDerivedProductOutput::from_execution_input(&self.input);
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
