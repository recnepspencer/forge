use crate::authority::{
    DurableCursorAcknowledgeRequest, DurableCursorResumePlan, DurableCursorResumeRequest,
    FetchedDurableCursorIdentity, PersistedSubscriberCheckpoint,
};
use crate::failure::StoreError;
use crate::live_query::{
    AcknowledgedContinuationAdvance, ContinuationAdvanceReceipt, ContinuationBatchResult,
    CursorContinuationPlan, StableBasisHandle, StableBasisReadRequest,
};

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub(crate) fn record_stable_basis_lookup(&self) {
        dispatch_ref!(self, |backend| backend.record_stable_basis_lookup())
    }
    pub(crate) fn record_stable_basis_read(
        &self,
        support_rows_read: u64,
        scope_lookup_count: u64,
        used_fallback: bool,
    ) {
        dispatch_ref!(self, |backend| backend.record_stable_basis_read(
            support_rows_read,
            scope_lookup_count,
            used_fallback,
        ))
    }
    pub(crate) fn record_stable_basis_broadening(&self) {
        dispatch_ref!(self, |backend| backend.record_stable_basis_broadening())
    }
    pub(crate) fn record_continuation_plan(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_plan())
    }
    pub(crate) fn record_continuation_identity_lookup(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_identity_lookup())
    }
    pub(crate) fn record_continuation_checkpoint_lookup(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_checkpoint_lookup())
    }
    pub(crate) fn record_continuation_batch(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_batch())
    }
    pub(crate) fn record_continuation_broadening(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_broadening())
    }
    pub(crate) fn record_continuation_parity(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_parity())
    }
    pub(crate) fn record_continuation_illegal_acknowledgment(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_illegal_acknowledgment())
    }
    pub(crate) fn record_continuation_batch_gap(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_batch_gap())
    }
    pub(crate) fn record_continuation_batch_duplicate(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_batch_duplicate())
    }
    pub(crate) fn record_continuation_schema_mismatch(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_schema_mismatch())
    }
    pub(crate) fn record_continuation_scope_mismatch(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_scope_mismatch())
    }
    pub(crate) fn record_continuation_degraded_basis(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_degraded_basis())
    }
    pub(crate) fn record_continuation_rejected_basis(&self) {
        dispatch_ref!(self, |backend| backend.record_continuation_rejected_basis())
    }

    pub fn publish_stable_basis(
        &mut self,
        request: StableBasisReadRequest,
    ) -> Result<StableBasisHandle, StoreError> {
        dispatch_mut!(self, |backend| backend.publish_stable_basis(request))
    }
    pub fn fetch_stable_basis(
        &self,
        stable_basis_id: &str,
    ) -> Result<StableBasisHandle, StoreError> {
        dispatch_ref!(self, |backend| backend.fetch_stable_basis(stable_basis_id))
    }
    pub fn execute_cursor_continuation(
        &self,
        plan: CursorContinuationPlan,
    ) -> Result<ContinuationBatchResult, StoreError> {
        dispatch_ref!(self, |backend| backend.execute_cursor_continuation(plan))
    }
    pub fn verify_cursor_continuation_budget(
        &self,
        plan: &CursorContinuationPlan,
    ) -> Result<(), StoreError> {
        dispatch_ref!(self, |backend| backend.verify_cursor_continuation_budget(plan))
    }
    pub fn acknowledge_cursor_continuation(
        &mut self,
        receipt: ContinuationAdvanceReceipt,
    ) -> Result<AcknowledgedContinuationAdvance, StoreError> {
        dispatch_mut!(self, |backend| backend.acknowledge_cursor_continuation(receipt))
    }
    pub fn acknowledge_cursor(
        &mut self,
        request: DurableCursorAcknowledgeRequest,
    ) -> Result<PersistedSubscriberCheckpoint, StoreError> {
        dispatch_mut!(self, |backend| backend.acknowledge_cursor(request))
    }
    pub fn fetch_durable_cursor_identity(
        &self,
        cursor_id: &str,
    ) -> Result<FetchedDurableCursorIdentity, StoreError> {
        dispatch_ref!(self, |backend| backend.fetch_durable_cursor_identity(cursor_id))
    }
    pub fn plan_cursor_resume(
        &self,
        request: DurableCursorResumeRequest,
    ) -> Result<DurableCursorResumePlan, StoreError> {
        dispatch_ref!(self, |backend| backend.plan_cursor_resume(request))
    }
}
