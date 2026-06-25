use serde::Serialize;

use super::counters::DerivedInvalidationOperatorCutoverCounters;
use super::operator_receipt::DerivedInvalidationOperatorCutoverReceipt;
use super::projection_read_stage_receipt::DerivedInvalidationProjectionReadStageReceipt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationPhaseEightSeed {
    operator_cutover_receipt_digest: String,
    projection_read_stage_receipt_digest: String,
    selected_plan_digest: String,
    execution_receipt_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    counters_digest: String,
    seed_digest: String,
}

impl DerivedInvalidationPhaseEightSeed {
    pub(crate) fn from_cutover_receipts(
        operator_cutover: &DerivedInvalidationOperatorCutoverReceipt,
        projection_read_stage: &DerivedInvalidationProjectionReadStageReceipt,
        counters: &DerivedInvalidationOperatorCutoverCounters,
    ) -> Self {
        let seed_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-phase-eight-seed:v1".to_string(),
            format!("operator-cutover:{}", operator_cutover.receipt_digest()),
            format!(
                "projection-read-stage:{}",
                projection_read_stage.receipt_digest()
            ),
            format!("selected-plan:{}", operator_cutover.selected_plan_digest()),
            format!(
                "execution-receipt:{}",
                operator_cutover.execution_receipt_digest()
            ),
            format!(
                "touched-closure:{}",
                operator_cutover.touched_closure_digest()
            ),
            format!("query-support:{}", operator_cutover.query_support_digest()),
            format!(
                "legality-support:{}",
                operator_cutover.legality_support_digest()
            ),
            format!("counters:{}", counters.counters_digest()),
        ]);
        Self {
            operator_cutover_receipt_digest: operator_cutover.receipt_digest().to_string(),
            projection_read_stage_receipt_digest: projection_read_stage
                .receipt_digest()
                .to_string(),
            selected_plan_digest: operator_cutover.selected_plan_digest().to_string(),
            execution_receipt_digest: operator_cutover.execution_receipt_digest().to_string(),
            touched_closure_digest: operator_cutover.touched_closure_digest().to_string(),
            query_support_digest: operator_cutover.query_support_digest().to_string(),
            legality_support_digest: operator_cutover.legality_support_digest().to_string(),
            counters_digest: counters.counters_digest().to_string(),
            seed_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_deletion_closeout_test_parts(selected_plan_digest: &str) -> Self {
        let seed_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-phase-eight-seed:test:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
        ]);
        Self {
            operator_cutover_receipt_digest: "operator-cutover.test".to_string(),
            projection_read_stage_receipt_digest: "projection-read-stage.test".to_string(),
            selected_plan_digest: selected_plan_digest.to_string(),
            execution_receipt_digest: "execution-receipt.test".to_string(),
            touched_closure_digest: "touched-closure.test".to_string(),
            query_support_digest: "query-support.test".to_string(),
            legality_support_digest: "legality-support.test".to_string(),
            counters_digest: "counters.test".to_string(),
            seed_digest,
        }
    }

    pub fn operator_cutover_receipt_digest(&self) -> &str {
        &self.operator_cutover_receipt_digest
    }

    pub fn projection_read_stage_receipt_digest(&self) -> &str {
        &self.projection_read_stage_receipt_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn legality_support_digest(&self) -> &str {
        &self.legality_support_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
