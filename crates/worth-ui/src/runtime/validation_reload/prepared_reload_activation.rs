use crate::runtime::validation_reload::activation_guard::reject_stale_prepared_reload_activation;
use crate::runtime::{
    WorthUiRuntimeHost, WorthUiValidationPreparedReload, WorthUiValidationReloadEvidence,
    WorthUiValidationReloadStage,
};

impl WorthUiValidationPreparedReload {
    pub fn evidence(&self) -> &WorthUiValidationReloadEvidence {
        &self.evidence
    }

    pub fn changed_fact_mapping_receipt(
        &self,
    ) -> Option<&crate::runtime::WorthUiValidationChangedFactMappingReceipt> {
        self.changed_fact_mapping_receipt.as_ref()
    }

    pub fn activate(
        self,
        runtime: &mut WorthUiRuntimeHost,
    ) -> Result<WorthUiValidationReloadEvidence, WorthUiValidationReloadStage> {
        if self.runtime_instance_id != runtime.instance_id() {
            return Err(WorthUiValidationReloadStage::RuntimeInstanceMismatch);
        }
        reject_stale_prepared_reload_activation(runtime, &self.evidence)?;
        if self.is_authoring_snapshot_only_activation() {
            return self.activate_authoring_snapshot_only(runtime);
        }
        self.activate_ready_plan(runtime)
    }

    fn is_authoring_snapshot_only_activation(&self) -> bool {
        self.ready.is_none() && self.candidate_plan.is_none()
    }

    fn activate_authoring_snapshot_only(
        self,
        runtime: &mut WorthUiRuntimeHost,
    ) -> Result<WorthUiValidationReloadEvidence, WorthUiValidationReloadStage> {
        if self.candidate_authoring_snapshot.is_none() {
            return Err(WorthUiValidationReloadStage::MissingReadyActivation);
        }
        let _boundary = runtime.safe_frame_boundary();
        runtime.promote_authoring_snapshot_after_activation(self.candidate_authoring_snapshot);
        Ok(mark_reload_activated(runtime, self.evidence))
    }

    fn activate_ready_plan(
        self,
        runtime: &mut WorthUiRuntimeHost,
    ) -> Result<WorthUiValidationReloadEvidence, WorthUiValidationReloadStage> {
        let ready = self
            .ready
            .ok_or(WorthUiValidationReloadStage::MissingReadyActivation)?;
        let candidate_plan = self
            .candidate_plan
            .ok_or(WorthUiValidationReloadStage::MissingReadyActivation)?;
        let boundary = runtime.safe_frame_boundary();
        runtime
            .swap_ready_activation_at_frame_boundary(ready, candidate_plan, boundary)
            .map_err(|_| WorthUiValidationReloadStage::PlanSwap)?;
        runtime.promote_authoring_snapshot_after_activation(self.candidate_authoring_snapshot);
        Ok(mark_reload_activated(runtime, self.evidence))
    }
}

fn mark_reload_activated(
    runtime: &WorthUiRuntimeHost,
    evidence: WorthUiValidationReloadEvidence,
) -> WorthUiValidationReloadEvidence {
    let after = runtime.inspect_active();
    evidence.mark_activated(after.artifact_digest(), after.active_plan_digest())
}
