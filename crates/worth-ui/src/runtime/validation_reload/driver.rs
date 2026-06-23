use crate::capability::CapabilitySnapshot;
use crate::runtime::validation_reload::driver_support::{
    active_authoring_snapshot_digest, denied_reload,
};
use crate::runtime::{
    WorthUiCandidateRuntimeAuthoringSnapshot, WorthUiExecutionPlan, WorthUiReadyActivation,
    WorthUiRuntimeHost, WorthUiRuntimeInstanceId, WorthUiValidationReloadEvidence,
    WorthUiValidationReloadRequest, WorthUiValidationReloadStage, WorthUiValidationReloadStatus,
};

pub struct WorthUiValidationPreparedReload {
    pub(super) runtime_instance_id: WorthUiRuntimeInstanceId,
    pub(super) evidence: WorthUiValidationReloadEvidence,
    pub(super) changed_fact_mapping_receipt:
        Option<crate::runtime::WorthUiValidationChangedFactMappingReceipt>,
    pub(super) ready: Option<WorthUiReadyActivation>,
    pub(super) candidate_plan: Option<WorthUiExecutionPlan>,
    pub(super) candidate_authoring_snapshot: Option<WorthUiCandidateRuntimeAuthoringSnapshot>,
}

impl WorthUiValidationPreparedReload {
    pub fn is_ready(&self) -> bool {
        self.evidence.status() == WorthUiValidationReloadStatus::ReadyForFrameBoundary
            && (self.ready.is_some() || self.candidate_authoring_snapshot.is_some())
    }
}

impl WorthUiRuntimeHost {
    pub fn prepare_validation_reload(
        &self,
        snapshot: &CapabilitySnapshot,
        request: WorthUiValidationReloadRequest,
    ) -> WorthUiValidationPreparedReload {
        let before = self.inspect_active();
        let evidence = WorthUiValidationReloadEvidence::builder(
            self.instance_id().raw(),
            before.artifact_digest(),
            before.active_plan_digest(),
        )
        .record_active_authoring_snapshot_before(active_authoring_snapshot_digest(self))
        .record_validation_request_adapter();
        if request.is_empty() {
            return denied_reload(evidence, self, WorthUiValidationReloadStage::EmptyRequest);
        }

        let (submission, evidence) =
            match self.lower_validation_reload_request_to_submission(snapshot, request, evidence) {
                Ok(lowered) => lowered,
                Err((stage, evidence)) => return denied_reload(evidence, self, stage),
            };

        self.prepare_validation_reload_from_submission_with_evidence(submission, evidence)
    }
}
