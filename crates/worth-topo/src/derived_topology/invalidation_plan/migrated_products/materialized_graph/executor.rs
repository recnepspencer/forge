use std::cell::RefCell;
use std::fmt;

use super::{MaterializedGraphDerivedProductOutput, MaterializedGraphExecutionInput};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionError, DerivedInvalidationProductExecutionReport,
    DerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterializedGraphMigrationError {
    ExecutionReceiptFailed,
    NoMaterializedGraphExecutionObserved,
    OutputDigestNotBoundToReceipt,
    OutputSelectedPlanNotBoundToInput,
    ReadStageReceiptNotBoundToSelectedPlan,
    ReadStageReceiptMissingQueryReceipt,
    ReadStageReceiptMissingLegalityReceipt,
    OldAuthorityResidueNotCapped,
    OldAuthorityResidueMissingRequiredCap,
    ExecutionReceiptMissingMaterializedGraphRow,
    SelectedPlanMissingMaterializedGraphRow,
    ReadStageSelectedRowsExceedAvailableRows,
}

impl fmt::Display for MaterializedGraphMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutionReceiptFailed => {
                write!(
                    f,
                    "materialized-graph migration could not execute selected plan"
                )
            }
            Self::NoMaterializedGraphExecutionObserved => {
                write!(f, "materialized-graph migration observed no selected work")
            }
            Self::OutputDigestNotBoundToReceipt => {
                write!(f, "materialized-graph receipt did not bind product output")
            }
            Self::OutputSelectedPlanNotBoundToInput => {
                write!(
                    f,
                    "materialized-graph output was not bound to selected input"
                )
            }
            Self::ReadStageReceiptNotBoundToSelectedPlan => {
                write!(
                    f,
                    "materialized-graph read-stage receipt was not selected-plan bound"
                )
            }
            Self::ReadStageReceiptMissingQueryReceipt => {
                write!(
                    f,
                    "materialized-graph read-stage receipt lacked native Query read identity"
                )
            }
            Self::ReadStageReceiptMissingLegalityReceipt => {
                write!(
                    f,
                    "materialized-graph read-stage receipt lacked selected legality identity"
                )
            }
            Self::OldAuthorityResidueNotCapped => {
                write!(f, "materialized-graph old authority residue was not capped")
            }
            Self::OldAuthorityResidueMissingRequiredCap => {
                write!(
                    f,
                    "materialized-graph old authority residue missed a required cap"
                )
            }
            Self::ExecutionReceiptMissingMaterializedGraphRow => {
                write!(
                    f,
                    "execution receipt carried no materialized-graph product row"
                )
            }
            Self::SelectedPlanMissingMaterializedGraphRow => {
                write!(f, "selected plan carried no materialized-graph row")
            }
            Self::ReadStageSelectedRowsExceedAvailableRows => {
                write!(
                    f,
                    "materialized-graph selected read rows exceeded available rows"
                )
            }
        }
    }
}

impl std::error::Error for MaterializedGraphMigrationError {}

pub(crate) struct MaterializedGraphDerivedProductExecutor {
    input: MaterializedGraphExecutionInput,
    output: RefCell<Option<MaterializedGraphDerivedProductOutput>>,
}

impl MaterializedGraphDerivedProductExecutor {
    pub(crate) fn new(input: MaterializedGraphExecutionInput) -> Self {
        Self {
            input,
            output: RefCell::new(None),
        }
    }

    pub(crate) fn output(&self) -> Option<MaterializedGraphDerivedProductOutput> {
        self.output.borrow().clone()
    }
}

impl DerivedInvalidationProductExecutor for MaterializedGraphDerivedProductExecutor {
    fn execute_selected_row(
        &self,
        row: &DerivedInvalidationSelectedRow,
    ) -> Result<DerivedInvalidationProductExecutionReport, DerivedInvalidationExecutionError> {
        if row.family_identity() != DerivedTopologyProductFamilyIdentity::MaterializedGraph {
            return DerivedInvalidationProductExecutionReport::from_selected_row(row, 0, 0, None);
        }

        let output = MaterializedGraphDerivedProductOutput::from_execution_input(&self.input);
        let output_digest = output.output_digest().to_string();
        let execution_work_count =
            self.input.selected_entity_count() + self.input.selected_relation_count();
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
