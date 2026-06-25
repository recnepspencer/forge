use std::cell::RefCell;
use std::fmt;

use super::{VertexDiskDerivedProductOutput, VertexDiskExecutionInput};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionError, DerivedInvalidationProductExecutionReport,
    DerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexDiskMigrationError {
    ExecutionReceiptFailed,
    NoVertexDiskExecutionObserved,
    OutputDigestNotBoundToReceipt,
    OutputSelectedPlanNotBoundToInput,
    ReadStageReceiptNotBoundToSelectedPlan,
    ReadStageReceiptMissingQueryReceipt,
    ReadStageQueryReceiptNotBoundToSource,
    ReadStageReceiptMissingLegalityReceipt,
    ReadStageQueryProofInvalid,
    ReadStageTouchedClosureNotBoundToSelectedPlan,
    ReadStageTouchedClosureSelectedNoVertexDiskRows,
    ReadStageSelectedRowsExceedTouchedClosure,
    ReadStageCountersNotBoundToRows,
    OldAuthorityResidueMissingRequiredCap,
    ExecutionReceiptMissingVertexDiskRow,
    SelectedPlanMissingVertexDiskRow,
    SelectedRowsExceedAvailableRows,
    WholeViewFallbackNotAllowed,
}

impl fmt::Display for VertexDiskMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionReceiptFailed => {
                write!(f, "vertex-disk migration could not execute selected plan")
            }
            Self::NoVertexDiskExecutionObserved => {
                write!(f, "vertex-disk migration observed no vertex-disk execution")
            }
            Self::OutputDigestNotBoundToReceipt => {
                write!(
                    f,
                    "vertex-disk execution receipt did not bind product output"
                )
            }
            Self::OutputSelectedPlanNotBoundToInput => {
                write!(
                    f,
                    "vertex-disk product output was not bound to the selected input plan"
                )
            }
            Self::ReadStageReceiptNotBoundToSelectedPlan => {
                write!(
                    f,
                    "vertex-disk read-stage receipt was not selected-plan bound"
                )
            }
            Self::ReadStageReceiptMissingQueryReceipt => {
                write!(
                    f,
                    "vertex-disk read-stage receipt lacked native Query read identity"
                )
            }
            Self::ReadStageQueryReceiptNotBoundToSource => {
                write!(
                    f,
                    "vertex-disk read-stage receipt did not match the read source Query proof"
                )
            }
            Self::ReadStageReceiptMissingLegalityReceipt => {
                write!(
                    f,
                    "vertex-disk read-stage receipt lacked selected legality identity"
                )
            }
            Self::ReadStageQueryProofInvalid => {
                write!(
                    f,
                    "vertex-disk read source was not backed by valid Query shared-vertex proof"
                )
            }
            Self::ReadStageTouchedClosureNotBoundToSelectedPlan => {
                write!(
                    f,
                    "vertex-disk read source used a touched closure outside the selected plan"
                )
            }
            Self::ReadStageTouchedClosureSelectedNoVertexDiskRows => {
                write!(
                    f,
                    "vertex-disk touched closure selected no vertex-disk rows"
                )
            }
            Self::ReadStageSelectedRowsExceedTouchedClosure => {
                write!(
                    f,
                    "vertex-disk read-stage rows exceeded touched-closure vertex-disk bound"
                )
            }
            Self::ReadStageCountersNotBoundToRows => {
                write!(
                    f,
                    "vertex-disk read-stage counters were not bound to selected source rows"
                )
            }
            Self::OldAuthorityResidueMissingRequiredCap => {
                write!(
                    f,
                    "vertex-disk old authority residue did not cap every required old entry point"
                )
            }
            Self::ExecutionReceiptMissingVertexDiskRow => {
                write!(f, "execution receipt carried no vertex-disk product row")
            }
            Self::SelectedPlanMissingVertexDiskRow => {
                write!(f, "selected invalidation plan carried no vertex-disk row")
            }
            Self::SelectedRowsExceedAvailableRows => {
                write!(
                    f,
                    "selected vertex-disk rows exceeded available source rows"
                )
            }
            Self::WholeViewFallbackNotAllowed => {
                write!(
                    f,
                    "vertex-disk migration cannot close over whole-view fallback work"
                )
            }
        }
    }
}

impl std::error::Error for VertexDiskMigrationError {}

pub(crate) struct VertexDiskDerivedProductExecutor {
    input: VertexDiskExecutionInput,
    output: RefCell<Option<VertexDiskDerivedProductOutput>>,
}

impl VertexDiskDerivedProductExecutor {
    pub(crate) fn new(input: VertexDiskExecutionInput) -> Self {
        Self {
            input,
            output: RefCell::new(None),
        }
    }

    pub(crate) fn output(&self) -> Option<VertexDiskDerivedProductOutput> {
        self.output.borrow().clone()
    }
}

impl DerivedInvalidationProductExecutor for VertexDiskDerivedProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError> {
        if row.family_identity() != DerivedTopologyProductFamilyIdentity::VertexDisks {
            return DerivedInvalidationProductExecutionReport::from_selected_row(row, 0, 0, None);
        }

        let output = VertexDiskDerivedProductOutput::from_execution_input(&self.input);
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
