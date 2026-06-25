use serde::Serialize;

use super::error::{
    DerivedInvalidationOperatorCutoverError, DerivedInvalidationOperatorCutoverErrorKind,
};
use super::operator_receipt::DerivedInvalidationOperatorCutoverReceipt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ProjectionReadStageConsumptionScope {
    CommittedRead,
}

impl ProjectionReadStageConsumptionScope {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommittedRead => "committed_read",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationProjectionReadStageReceipt {
    operator_cutover_receipt_digest: String,
    execution_receipt_digest: String,
    selected_plan_digest: String,
    touched_closure_digest: String,
    consumption_scope: ProjectionReadStageConsumptionScope,
    projection_dirty_expansion_count: usize,
    receipt_digest: String,
}

impl DerivedInvalidationProjectionReadStageReceipt {
    pub fn consume_operator_cutover(
        operator_cutover: &DerivedInvalidationOperatorCutoverReceipt,
        consumption_scope: ProjectionReadStageConsumptionScope,
        projection_dirty_expansion_count: usize,
    ) -> Result<Self, DerivedInvalidationOperatorCutoverError> {
        if projection_dirty_expansion_count != 0 {
            return Err(DerivedInvalidationOperatorCutoverError::new(
                DerivedInvalidationOperatorCutoverErrorKind::ProjectionReadStageScopeExpandedDirtyProducts,
                "projection read-stage consumption must not expand dirty product scope after invalidation planning",
            ));
        }
        let receipt_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-projection-read-stage-receipt:v1".to_string(),
            format!("operator-cutover:{}", operator_cutover.receipt_digest()),
            format!("execution:{}", operator_cutover.execution_receipt_digest()),
            format!("selected-plan:{}", operator_cutover.selected_plan_digest()),
            format!(
                "touched-closure:{}",
                operator_cutover.touched_closure_digest()
            ),
            format!("scope:{}", consumption_scope.as_str()),
            format!("projection-dirty-expansion:{projection_dirty_expansion_count}"),
        ]);
        Ok(Self {
            operator_cutover_receipt_digest: operator_cutover.receipt_digest().to_string(),
            execution_receipt_digest: operator_cutover.execution_receipt_digest().to_string(),
            selected_plan_digest: operator_cutover.selected_plan_digest().to_string(),
            touched_closure_digest: operator_cutover.touched_closure_digest().to_string(),
            consumption_scope,
            projection_dirty_expansion_count,
            receipt_digest,
        })
    }

    pub fn operator_cutover_receipt_digest(&self) -> &str {
        &self.operator_cutover_receipt_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub const fn consumption_scope(&self) -> ProjectionReadStageConsumptionScope {
        self.consumption_scope
    }

    pub const fn projection_dirty_expansion_count(&self) -> usize {
        self.projection_dirty_expansion_count
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
