use serde::Serialize;

use super::counters::DerivedInvalidationOperatorCutoverCounters;
use super::error::DerivedInvalidationOperatorCutoverError;
use super::operator_receipt::DerivedInvalidationOperatorCutoverReceipt;
use super::phase_eight_seed::DerivedInvalidationPhaseEightSeed;
use super::projection_read_stage_receipt::{
    DerivedInvalidationProjectionReadStageReceipt, ProjectionReadStageConsumptionScope,
};

pub fn close_derived_invalidation_operator_cutover(
    operator_cutover: DerivedInvalidationOperatorCutoverReceipt,
) -> Result<DerivedInvalidationOperatorCutoverCloseout, DerivedInvalidationOperatorCutoverError> {
    DerivedInvalidationOperatorCutoverCloseout::close(operator_cutover)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationOperatorCutoverCloseout {
    operator_cutover: DerivedInvalidationOperatorCutoverReceipt,
    projection_read_stage: DerivedInvalidationProjectionReadStageReceipt,
    counters: DerivedInvalidationOperatorCutoverCounters,
    phase_eight_seed: DerivedInvalidationPhaseEightSeed,
    closeout_digest: String,
}

impl DerivedInvalidationOperatorCutoverCloseout {
    fn close(
        operator_cutover: DerivedInvalidationOperatorCutoverReceipt,
    ) -> Result<Self, DerivedInvalidationOperatorCutoverError> {
        let projection_read_stage =
            DerivedInvalidationProjectionReadStageReceipt::consume_operator_cutover(
                &operator_cutover,
                ProjectionReadStageConsumptionScope::CommittedRead,
                0,
            )?;
        let counters = operator_cutover.counters().clone();
        let phase_eight_seed = DerivedInvalidationPhaseEightSeed::from_cutover_receipts(
            &operator_cutover,
            &projection_read_stage,
            &counters,
        );
        let closeout_digest = closeout_digest(&operator_cutover, &projection_read_stage, &counters);
        Ok(Self {
            operator_cutover,
            projection_read_stage,
            counters,
            phase_eight_seed,
            closeout_digest,
        })
    }

    pub const fn operator_cutover(&self) -> &DerivedInvalidationOperatorCutoverReceipt {
        &self.operator_cutover
    }

    pub const fn projection_read_stage(&self) -> &DerivedInvalidationProjectionReadStageReceipt {
        &self.projection_read_stage
    }

    pub const fn counters(&self) -> &DerivedInvalidationOperatorCutoverCounters {
        &self.counters
    }

    pub const fn phase_eight_seed(&self) -> &DerivedInvalidationPhaseEightSeed {
        &self.phase_eight_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn closeout_digest(
    operator_cutover: &DerivedInvalidationOperatorCutoverReceipt,
    projection_read_stage: &DerivedInvalidationProjectionReadStageReceipt,
    counters: &DerivedInvalidationOperatorCutoverCounters,
) -> String {
    super::super::catalog::catalog_digest([
        "worth-topo:derived-invalidation-operator-cutover-closeout:v1".to_string(),
        format!("operator-cutover:{}", operator_cutover.receipt_digest()),
        format!(
            "projection-read-stage:{}",
            projection_read_stage.receipt_digest()
        ),
        format!("counters:{}", counters.counters_digest()),
    ])
}
