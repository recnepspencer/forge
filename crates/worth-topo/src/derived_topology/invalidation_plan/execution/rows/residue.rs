use serde::Serialize;

use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionOutcome;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationResidueRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationResidueExecutionRow {
    residue_label: String,
    capped_count: usize,
    source_residue_row_digest: String,
    outcome: DerivedInvalidationExecutionOutcome,
    execution_work_count: usize,
    row_digest: String,
}

impl DerivedInvalidationResidueExecutionRow {
    pub(in crate::derived_topology::invalidation_plan::execution) fn from_residue_row(
        row: &DerivedInvalidationResidueRow,
    ) -> Self {
        let outcome = DerivedInvalidationExecutionOutcome::ResidueCapped;
        let execution_work_count = 0;
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-residue-execution-row:v1".to_string(),
            format!("label:{}", row.residue_label()),
            format!("capped-count:{}", row.capped_count()),
            format!("source-residue-row:{}", row.row_digest()),
            format!("outcome:{}", outcome.as_str()),
            format!("execution-work:{execution_work_count}"),
        ]);
        Self {
            residue_label: row.residue_label().to_string(),
            capped_count: row.capped_count(),
            source_residue_row_digest: row.row_digest().to_string(),
            outcome,
            execution_work_count,
            row_digest,
        }
    }

    pub fn residue_label(&self) -> &str {
        &self.residue_label
    }

    pub const fn capped_count(&self) -> usize {
        self.capped_count
    }

    pub fn source_residue_row_digest(&self) -> &str {
        &self.source_residue_row_digest
    }

    pub const fn outcome(&self) -> DerivedInvalidationExecutionOutcome {
        self.outcome
    }

    pub const fn execution_work_count(&self) -> usize {
        self.execution_work_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
