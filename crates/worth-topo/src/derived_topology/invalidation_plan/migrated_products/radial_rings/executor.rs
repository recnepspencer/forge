use std::cell::RefCell;
use std::fmt;

use super::{RadialRingDerivedProductOutput, RadialRingExecutionInput};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionError, DerivedInvalidationProductExecutionReport,
    DerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadialRingMigrationError {
    ExecutionReceiptFailed,
    NoRadialRingExecutionObserved,
    OutputDigestNotBoundToReceipt,
    OutputSelectedPlanNotBoundToInput,
    ReadStageReceiptNotBoundToSelectedPlan,
    ReadStageReceiptMissingQueryReceipt,
    ReadStageReceiptMissingLegalityReceipt,
    ReadStageQueryProofInvalid,
    ReadStageTouchedClosureNotBoundToSelectedPlan,
    ReadStageTouchedClosureSelectedNoRadialRingRows,
    ReadStageSelectedRowsExceedTouchedClosure,
    ReadStageCountersNotBoundToRows,
    OldAuthorityResidueMissingRequiredCap,
    ExecutionReceiptMissingRadialRingRow,
    SelectedPlanMissingRadialRingRow,
    SelectedRowsExceedAvailableRows,
    WholeViewFallbackNotAllowed,
}

impl fmt::Display for RadialRingMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionReceiptFailed => {
                write!(f, "radial-ring migration could not execute selected plan")
            }
            Self::NoRadialRingExecutionObserved => {
                write!(f, "radial-ring migration observed no radial-ring execution")
            }
            Self::OutputDigestNotBoundToReceipt => {
                write!(
                    f,
                    "radial-ring execution receipt did not bind product output"
                )
            }
            Self::OutputSelectedPlanNotBoundToInput => {
                write!(
                    f,
                    "radial-ring product output was not bound to the selected input plan"
                )
            }
            Self::ReadStageReceiptNotBoundToSelectedPlan => {
                write!(
                    f,
                    "radial-ring read-stage receipt was not selected-plan bound"
                )
            }
            Self::ReadStageReceiptMissingQueryReceipt => {
                write!(
                    f,
                    "radial-ring read-stage receipt lacked native Query read identity"
                )
            }
            Self::ReadStageReceiptMissingLegalityReceipt => {
                write!(
                    f,
                    "radial-ring read-stage receipt lacked selected legality identity"
                )
            }
            Self::ReadStageQueryProofInvalid => {
                write!(
                    f,
                    "radial-ring read source was not backed by valid Query radial proof"
                )
            }
            Self::ReadStageTouchedClosureNotBoundToSelectedPlan => {
                write!(
                    f,
                    "radial-ring read source used a touched closure outside the selected plan"
                )
            }
            Self::ReadStageTouchedClosureSelectedNoRadialRingRows => {
                write!(
                    f,
                    "radial-ring touched closure selected no radial-ring rows"
                )
            }
            Self::ReadStageSelectedRowsExceedTouchedClosure => {
                write!(
                    f,
                    "radial-ring read-stage rows exceeded touched-closure radial-ring bound"
                )
            }
            Self::ReadStageCountersNotBoundToRows => {
                write!(
                    f,
                    "radial-ring read-stage counters were not bound to selected source rows"
                )
            }
            Self::OldAuthorityResidueMissingRequiredCap => {
                write!(
                    f,
                    "radial-ring old authority residue did not cap every required old entry point"
                )
            }
            Self::ExecutionReceiptMissingRadialRingRow => {
                write!(f, "execution receipt carried no radial-ring product row")
            }
            Self::SelectedPlanMissingRadialRingRow => {
                write!(f, "selected invalidation plan carried no radial-ring row")
            }
            Self::SelectedRowsExceedAvailableRows => {
                write!(
                    f,
                    "selected radial-ring rows exceeded available source rows"
                )
            }
            Self::WholeViewFallbackNotAllowed => {
                write!(
                    f,
                    "radial-ring migration cannot close over whole-view fallback work"
                )
            }
        }
    }
}

impl std::error::Error for RadialRingMigrationError {}

pub(crate) struct RadialRingDerivedProductExecutor {
    input: RadialRingExecutionInput,
    output: RefCell<Option<RadialRingDerivedProductOutput>>,
}

impl RadialRingDerivedProductExecutor {
    pub(crate) fn new(input: RadialRingExecutionInput) -> Self {
        Self {
            input,
            output: RefCell::new(None),
        }
    }

    pub(crate) fn output(&self) -> Option<RadialRingDerivedProductOutput> {
        self.output.borrow().clone()
    }
}

impl DerivedInvalidationProductExecutor for RadialRingDerivedProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError> {
        if row.family_identity() != DerivedTopologyProductFamilyIdentity::RadialRings {
            return DerivedInvalidationProductExecutionReport::from_selected_row(row, 0, 0, None);
        }

        let output = RadialRingDerivedProductOutput::from_execution_input(&self.input);
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
