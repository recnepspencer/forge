use crate::authority::DurableCursorResumeRequest;
use crate::failure::{StoreError, StoreErrorKind};
use crate::live_query::{
    acknowledgment::{plan_continuation_acknowledgment, ContinuationAcknowledgmentEffect},
    basis::stable_basis_handle_from_record_with_survival,
    continuation::{execute_cursor_continuation as execute_live_query_continuation, ContinuationExecutionEffect},
    AcknowledgedContinuationAdvance, ContinuationAdvanceReceipt, ContinuationBatchResult,
    CursorContinuationPlan, StableBasisHandle, StableBasisReadRequest,
};

use super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn publish_stable_basis(
        &mut self,
        request: StableBasisReadRequest,
    ) -> Result<StableBasisHandle, StoreError> {
        let publication = self.state.admit_stable_basis_publication(request)?;
        let record = publication.to_record();
        if let Some(existing) = self.state.stable_basis_records.get(&record.artifact_id) {
            let survival = self.state.classify_stable_basis_survival(existing)?;
            self.counters.record_stable_basis_lookup();
            self.counters.record_stable_basis_read(
                1,
                1,
                !matches!(
                    survival,
                    crate::live_query::restart::StableBasisSurvival::Retained
                ),
            );
            return Ok(stable_basis_handle_from_record_with_survival(existing, survival));
        }

        let mut next = self.state.clone();
        next.stable_basis_records
            .insert(record.artifact_id.clone(), record.clone());
        next.authoritative_artifact_digests.insert(
            super::super::integrity::digest_artifact_key(
                &crate::backend::records::AuthoritativeArtifactFamily::StableBasisRecord,
                &record.artifact_id,
                self.state.canonicalization_version,
            ),
            crate::backend::records::AuthoritativeArtifactDigestRecord {
                artifact_family:
                    crate::backend::records::AuthoritativeArtifactFamily::StableBasisRecord,
                artifact_id: record.artifact_id.clone(),
                canonicalization_version: self.state.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: super::super::integrity::stable_structural_digest(&record)?,
            },
        );
        self.commit_replacement_state(next)?;
        self.counters.record_stable_basis_lookup();
        self.counters.record_stable_basis_read(
            1,
            1,
            !matches!(
                crate::live_query::restart::StableBasisSurvival::from_request(&record.request),
                crate::live_query::restart::StableBasisSurvival::Retained
            ),
        );
        Ok(publication.into_handle())
    }

    pub fn fetch_stable_basis(
        &self,
        stable_basis_id: &str,
    ) -> Result<StableBasisHandle, StoreError> {
        self.counters.record_stable_basis_lookup();
        let artifact_id = super::super::integrity::stable_basis_artifact_id(stable_basis_id);
        let record = self
            .state
            .stable_basis_records
            .get(&artifact_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::StableBasisArtifactMissing,
                    format!("stable basis `{stable_basis_id}` not found"),
                )
            })?;
        let survival = self.state.classify_stable_basis_survival(record)?;
        self.counters.record_stable_basis_read(
            1,
            1,
            !matches!(
                survival,
                crate::live_query::restart::StableBasisSurvival::Retained
            ),
        );
        Ok(stable_basis_handle_from_record_with_survival(record, survival))
    }

    pub fn execute_cursor_continuation(
        &self,
        plan: CursorContinuationPlan,
    ) -> Result<ContinuationBatchResult, StoreError> {
        let executed = execute_live_query_continuation(&self.state, &plan)?;
        let (result, metrics, effects) = executed.into_parts();
        for effect in effects {
            match effect {
                ContinuationExecutionEffect::Batch => self.counters.record_continuation_batch(),
                ContinuationExecutionEffect::Broadening => {
                    self.counters.record_continuation_broadening()
                }
                ContinuationExecutionEffect::ControlLaneFallback => {
                    self.counters.record_continuation_control_lane_fallback()
                }
            }
        }
        self.counters.record_continuation_batch_metrics(
            metrics.support_rows_read,
            metrics.narrowed_item_count,
            metrics.broadened_item_count,
            metrics.step_count,
        );
        Ok(result)
    }

    pub fn verify_cursor_continuation_budget(
        &self,
        plan: &CursorContinuationPlan,
    ) -> Result<(), StoreError> {
        crate::live_query::continuation::verify_cursor_continuation_budget(&self.state, plan)
    }

    pub fn acknowledge_cursor_continuation(
        &mut self,
        receipt: ContinuationAdvanceReceipt,
    ) -> Result<AcknowledgedContinuationAdvance, StoreError> {
        let batch = receipt.batch();
        let resume_plan = self.plan_cursor_resume(DurableCursorResumeRequest::new(
            batch.cursor_id(),
            batch.subscriber_id(),
            batch.branch_id().clone(),
            batch.feed_shape_id(),
            batch.schema_interpretation_id(),
            batch.cursor_semantics_version(),
        ))?;
        let schema_support_present = self
            .state
            .schema_support_records
            .contains_key(batch.schema_boundary_artifact_id());
        match plan_continuation_acknowledgment(&receipt, &resume_plan, schema_support_present) {
            Ok(planned) => {
                let (acknowledge_request, effects) = planned.into_parts();
                self.acknowledge_cursor(acknowledge_request)?;
                record_continuation_ack_effects(&self.counters, effects);
                Ok(AcknowledgedContinuationAdvance::new(receipt))
            }
            Err(failure) => {
                let (error, effects) = failure.into_parts();
                record_continuation_ack_effects(&self.counters, effects);
                Err(error)
            }
        }
    }
}

fn record_continuation_ack_effects(
    counters: &crate::evidence::StoreCounters,
    effects: Vec<ContinuationAcknowledgmentEffect>,
) {
    for effect in effects {
        match effect {
            ContinuationAcknowledgmentEffect::Parity => counters.record_continuation_parity(),
            ContinuationAcknowledgmentEffect::IllegalAcknowledgment => {
                counters.record_continuation_illegal_acknowledgment()
            }
            ContinuationAcknowledgmentEffect::BatchDuplicate => {
                counters.record_continuation_batch_duplicate()
            }
            ContinuationAcknowledgmentEffect::BatchGap => counters.record_continuation_batch_gap(),
        }
    }
}
