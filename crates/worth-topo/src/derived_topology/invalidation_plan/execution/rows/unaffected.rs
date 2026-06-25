use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionOutcome;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationUnaffectedRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationUnaffectedProductExecutionRow {
    family_identity: DerivedTopologyProductFamilyIdentity,
    family_digest: String,
    source_unaffected_row_digest: String,
    outcome: DerivedInvalidationExecutionOutcome,
    execution_work_count: usize,
    row_digest: String,
}

impl DerivedInvalidationUnaffectedProductExecutionRow {
    pub(in crate::derived_topology::invalidation_plan::execution) fn from_unaffected_row(
        row: &DerivedInvalidationUnaffectedRow,
    ) -> Self {
        let outcome = DerivedInvalidationExecutionOutcome::Unaffected;
        let execution_work_count = 0;
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-unaffected-product-execution-row:v1".to_string(),
            format!("family:{}", row.family_identity().as_str()),
            format!("family-digest:{}", row.family_digest()),
            format!("source-unaffected-row:{}", row.row_digest()),
            format!("outcome:{}", outcome.as_str()),
            format!("execution-work:{execution_work_count}"),
        ]);
        Self {
            family_identity: row.family_identity(),
            family_digest: row.family_digest().to_string(),
            source_unaffected_row_digest: row.row_digest().to_string(),
            outcome,
            execution_work_count,
            row_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub fn source_unaffected_row_digest(&self) -> &str {
        &self.source_unaffected_row_digest
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
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
