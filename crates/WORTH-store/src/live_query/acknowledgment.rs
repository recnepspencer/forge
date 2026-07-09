use crate::authority::{DurableCursorAcknowledgeRequest, DurableCursorResumePlan};
use crate::failure::{StoreError, StoreErrorKind};
use crate::live_query::continuation::AdmittedNarrowBatchReceipt;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContinuationAdvanceReceipt {
    batch: AdmittedNarrowBatchReceipt,
}

impl ContinuationAdvanceReceipt {
    pub(crate) fn new(batch: AdmittedNarrowBatchReceipt) -> Self {
        Self { batch }
    }

    pub fn batch(&self) -> &AdmittedNarrowBatchReceipt {
        &self.batch
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AcknowledgedContinuationAdvance {
    receipt: ContinuationAdvanceReceipt,
}

impl AcknowledgedContinuationAdvance {
    pub(crate) fn new(receipt: ContinuationAdvanceReceipt) -> Self {
        Self { receipt }
    }

    pub fn receipt(&self) -> &ContinuationAdvanceReceipt {
        &self.receipt
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContinuationAcknowledgmentEffect {
    Parity,
    IllegalAcknowledgment,
    BatchDuplicate,
    BatchGap,
}

#[derive(Debug)]
pub(crate) struct PlannedContinuationAcknowledgment {
    acknowledge_request: DurableCursorAcknowledgeRequest,
    effects: Vec<ContinuationAcknowledgmentEffect>,
}

impl PlannedContinuationAcknowledgment {
    pub(crate) fn into_parts(
        self,
    ) -> (
        DurableCursorAcknowledgeRequest,
        Vec<ContinuationAcknowledgmentEffect>,
    ) {
        (self.acknowledge_request, self.effects)
    }
}

#[derive(Debug)]
pub(crate) struct ContinuationAcknowledgmentFailure {
    error: StoreError,
    effects: Vec<ContinuationAcknowledgmentEffect>,
}

impl ContinuationAcknowledgmentFailure {
    fn new(error: StoreError, effects: Vec<ContinuationAcknowledgmentEffect>) -> Self {
        Self { error, effects }
    }

    pub(crate) fn into_parts(self) -> (StoreError, Vec<ContinuationAcknowledgmentEffect>) {
        (self.error, self.effects)
    }
}

pub(crate) fn admit_continuation_advance(
    receipt: ContinuationAdvanceReceipt,
) -> Result<AcknowledgedContinuationAdvance, ContinuationAcknowledgmentFailure> {
    if receipt.batch().covered_commit_range().0 .0 > receipt.batch().covered_commit_range().1 .0 {
        return Err(ContinuationAcknowledgmentFailure::new(
            StoreError::new(
                StoreErrorKind::ContinuationBatchOrderingViolation,
                "continuation advance receipts must reference an ordered covered commit range",
            ),
            vec![ContinuationAcknowledgmentEffect::IllegalAcknowledgment],
        ));
    }
    Ok(AcknowledgedContinuationAdvance::new(receipt))
}

pub(crate) fn plan_continuation_acknowledgment(
    receipt: &ContinuationAdvanceReceipt,
    resume_plan: &DurableCursorResumePlan,
    schema_support_present: bool,
) -> Result<PlannedContinuationAcknowledgment, ContinuationAcknowledgmentFailure> {
    let batch = receipt.batch();
    if batch.covered_commit_range().0 .0 > batch.covered_commit_range().1 .0
        || batch.from_frontier_commit_id().0 > batch.to_frontier_commit_id().0
    {
        return Err(ContinuationAcknowledgmentFailure::new(
            StoreError::new(
                StoreErrorKind::ContinuationBatchOrderingViolation,
                "continuation advance receipts must reference ordered continuation frontiers",
            ),
            vec![ContinuationAcknowledgmentEffect::IllegalAcknowledgment],
        ));
    }

    let latest_checkpoint = resume_plan.latest_checkpoint();
    if latest_checkpoint.basis_commit_id == batch.to_frontier_commit_id() {
        return Err(ContinuationAcknowledgmentFailure::new(
            StoreError::new(
                StoreErrorKind::ContinuationBatchDuplicate,
                format!(
                    "continuation batch `{}` was already acknowledged at frontier {}",
                    batch.batch_id().as_str(),
                    batch.to_frontier_commit_id().0
                ),
            ),
            vec![
                ContinuationAcknowledgmentEffect::BatchDuplicate,
                ContinuationAcknowledgmentEffect::IllegalAcknowledgment,
            ],
        ));
    }
    if latest_checkpoint.basis_commit_id != batch.from_frontier_commit_id() {
        let is_gap = latest_checkpoint.basis_commit_id.0 < batch.from_frontier_commit_id().0;
        return Err(ContinuationAcknowledgmentFailure::new(
            StoreError::new(
                if is_gap {
                    StoreErrorKind::ContinuationBatchGap
                } else {
                    StoreErrorKind::ContinuationIllegalAdvance
                },
                format!(
                    "continuation batch `{}` expected latest durable frontier {} but found {}",
                    batch.batch_id().as_str(),
                    batch.from_frontier_commit_id().0,
                    latest_checkpoint.basis_commit_id.0
                ),
            ),
            if is_gap {
                vec![
                    ContinuationAcknowledgmentEffect::BatchGap,
                    ContinuationAcknowledgmentEffect::IllegalAcknowledgment,
                ]
            } else {
                vec![ContinuationAcknowledgmentEffect::IllegalAcknowledgment]
            },
        ));
    }

    let mut acknowledge_request = DurableCursorAcknowledgeRequest::new(
        batch.cursor_id(),
        batch.subscriber_id(),
        batch.branch_id().clone(),
        batch.feed_shape_id(),
        batch.schema_interpretation_id(),
        batch.cursor_semantics_version(),
        batch.to_frontier_commit_id(),
    );
    if schema_support_present {
        acknowledge_request = acknowledge_request
            .with_schema_support_artifact_id(batch.schema_boundary_artifact_id().to_string());
    }
    Ok(PlannedContinuationAcknowledgment {
        acknowledge_request,
        effects: vec![ContinuationAcknowledgmentEffect::Parity],
    })
}
