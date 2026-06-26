use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyQueryReceiptPosture,
};
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionOutcome;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDenialKind, DerivedInvalidationDenialRow,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDeniedProductExecutionRow {
    family_identity: DerivedTopologyProductFamilyIdentity,
    family_digest: String,
    source_denial_digest: String,
    denial_kind: DerivedInvalidationDenialKind,
    required_query_posture: Option<DerivedTopologyQueryReceiptPosture>,
    required_legality_posture: Option<DerivedTopologyLegalityReceiptPosture>,
    outcome: DerivedInvalidationExecutionOutcome,
    execution_work_count: usize,
    row_digest: String,
}

impl DerivedInvalidationDeniedProductExecutionRow {
    pub(in crate::derived_topology::invalidation_plan::execution) fn from_denial_row(
        row: &DerivedInvalidationDenialRow,
    ) -> Self {
        let outcome = DerivedInvalidationExecutionOutcome::Denied;
        let execution_work_count = 0;
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-denied-product-execution-row:v1".to_string(),
            format!("family:{}", row.family_identity().as_str()),
            format!("family-digest:{}", row.family_digest()),
            format!("source-denial:{}", row.denial_digest()),
            format!("denial-kind:{}", row.kind().as_str()),
            format!(
                "query:{}",
                row.required_query_posture()
                    .map(DerivedTopologyQueryReceiptPosture::as_str)
                    .unwrap_or("not-applicable")
            ),
            format!(
                "legality:{}",
                row.required_legality_posture()
                    .map(DerivedTopologyLegalityReceiptPosture::as_str)
                    .unwrap_or("not-applicable")
            ),
            format!("outcome:{}", outcome.as_str()),
            format!("execution-work:{execution_work_count}"),
        ]);
        Self {
            family_identity: row.family_identity(),
            family_digest: row.family_digest().to_string(),
            source_denial_digest: row.denial_digest().to_string(),
            denial_kind: row.kind(),
            required_query_posture: row.required_query_posture(),
            required_legality_posture: row.required_legality_posture(),
            outcome,
            execution_work_count,
            row_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub fn source_denial_digest(&self) -> &str {
        &self.source_denial_digest
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub const fn denial_kind(&self) -> DerivedInvalidationDenialKind {
        self.denial_kind
    }

    pub const fn required_query_posture(&self) -> Option<DerivedTopologyQueryReceiptPosture> {
        self.required_query_posture
    }

    pub const fn required_legality_posture(&self) -> Option<DerivedTopologyLegalityReceiptPosture> {
        self.required_legality_posture
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
