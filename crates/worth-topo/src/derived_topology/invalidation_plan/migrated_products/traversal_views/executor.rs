use std::cell::RefCell;
use std::fmt;

use super::{TraversalViewsDerivedProductOutput, TraversalViewsExecutionInput};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionError, DerivedInvalidationProductExecutionReport,
    DerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalViewsMigrationError {
    ExecutionReceiptFailed,
    NoTraversalViewsExecutionObserved,
    OutputDigestNotBoundToReceipt,
    OutputSelectedPlanNotBoundToInput,
    ReadStageReceiptNotBoundToSelectedPlan,
    ReadStageReceiptMissingQueryReceipt,
    ReadStageReceiptMissingLegalityReceipt,
    ReadStageTouchedClosureNotBoundToSelectedPlan,
    ReadStageTouchedClosureSelectedNoTraversalRows,
    ReadStageSelectedRowsExceedTouchedClosure,
    OldAuthorityResidueNotCapped,
    OldAuthorityResidueMissingRequiredCap,
    ExecutionReceiptMissingTraversalViewsRow,
    SelectedPlanMissingTraversalViewsRow,
    ReadStageSelectedRowsExceedAvailableRows,
}

impl fmt::Display for TraversalViewsMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionReceiptFailed => {
                write!(
                    f,
                    "traversal-views migration could not execute selected plan"
                )
            }
            Self::NoTraversalViewsExecutionObserved => {
                write!(f, "traversal-views migration observed no selected work")
            }
            Self::OutputDigestNotBoundToReceipt => {
                write!(f, "traversal-views receipt did not bind product output")
            }
            Self::OutputSelectedPlanNotBoundToInput => {
                write!(f, "traversal-views output was not bound to selected input")
            }
            Self::ReadStageReceiptNotBoundToSelectedPlan => {
                write!(
                    f,
                    "traversal-views read-stage receipt was not selected-plan bound"
                )
            }
            Self::ReadStageReceiptMissingQueryReceipt => {
                write!(
                    f,
                    "traversal-views read-stage receipt lacked native Query read identity"
                )
            }
            Self::ReadStageReceiptMissingLegalityReceipt => {
                write!(
                    f,
                    "traversal-views read-stage receipt lacked selected legality identity"
                )
            }
            Self::ReadStageTouchedClosureNotBoundToSelectedPlan => {
                write!(
                    f,
                    "traversal-views read source used a touched closure outside the selected plan"
                )
            }
            Self::ReadStageTouchedClosureSelectedNoTraversalRows => {
                write!(
                    f,
                    "traversal-views touched closure selected no traversal rows"
                )
            }
            Self::ReadStageSelectedRowsExceedTouchedClosure => {
                write!(
                    f,
                    "traversal-views read-stage rows exceeded touched-closure traversal bound"
                )
            }
            Self::OldAuthorityResidueNotCapped => {
                write!(f, "traversal-views old authority residue was not capped")
            }
            Self::OldAuthorityResidueMissingRequiredCap => {
                write!(
                    f,
                    "traversal-views old authority residue missed a required cap"
                )
            }
            Self::ExecutionReceiptMissingTraversalViewsRow => {
                write!(
                    f,
                    "execution receipt carried no traversal-views product row"
                )
            }
            Self::SelectedPlanMissingTraversalViewsRow => {
                write!(f, "selected plan carried no traversal-views row")
            }
            Self::ReadStageSelectedRowsExceedAvailableRows => {
                write!(
                    f,
                    "traversal-views selected read rows exceeded available rows"
                )
            }
        }
    }
}

impl std::error::Error for TraversalViewsMigrationError {}

pub(crate) struct TraversalViewsDerivedProductExecutor {
    input: TraversalViewsExecutionInput,
    output: RefCell<Option<TraversalViewsDerivedProductOutput>>,
}

impl TraversalViewsDerivedProductExecutor {
    pub(crate) fn new(input: TraversalViewsExecutionInput) -> Self {
        Self {
            input,
            output: RefCell::new(None),
        }
    }

    pub(crate) fn output(&self) -> Option<TraversalViewsDerivedProductOutput> {
        self.output.borrow().clone()
    }
}

impl DerivedInvalidationProductExecutor for TraversalViewsDerivedProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError> {
        if row.family_identity() != DerivedTopologyProductFamilyIdentity::TraversalViews {
            return DerivedInvalidationProductExecutionReport::from_selected_row(row, 0, 0, None);
        }

        let output = TraversalViewsDerivedProductOutput::from_execution_input(&self.input);
        let output_digest = output.output_digest().to_string();
        let execution_work_count = self.input.selected_traversal_count();
        *self.output.borrow_mut() = Some(output);
        DerivedInvalidationProductExecutionReport::from_selected_row_with_product_output(
            row,
            execution_work_count,
            0,
            None,
            Some(&output_digest),
        )
    }
}
