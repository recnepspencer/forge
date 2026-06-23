use crate::runtime::active::WorthUiActiveExecutionPlan;
use crate::runtime::{
    WorthUiAdmittedCapabilityReloadBatch, WorthUiCapabilityReloadEvidence,
    WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus, WorthUiRuntimeHost,
};

#[derive(Debug)]
pub struct WorthUiCapabilityPreparedReload {
    runtime_instance_witness: u64,
    evidence: WorthUiCapabilityReloadEvidence,
    admitted_batch: Option<WorthUiAdmittedCapabilityReloadBatch>,
}

impl WorthUiCapabilityPreparedReload {
    pub(crate) fn new(
        runtime_instance_witness: u64,
        evidence: WorthUiCapabilityReloadEvidence,
        admitted_batch: Option<WorthUiAdmittedCapabilityReloadBatch>,
    ) -> Self {
        Self {
            runtime_instance_witness,
            evidence,
            admitted_batch,
        }
    }

    pub fn evidence(&self) -> &WorthUiCapabilityReloadEvidence {
        &self.evidence
    }

    pub fn is_ready(&self) -> bool {
        matches!(
            self.evidence.status(),
            WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
        )
    }

    pub fn activate(
        self,
        runtime: &mut WorthUiRuntimeHost,
    ) -> Result<WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStage> {
        if runtime.instance_id().raw() != self.runtime_instance_witness {
            return Err(WorthUiCapabilityReloadStage::RuntimeInstanceMismatch);
        }
        if !self.is_ready() {
            return Err(WorthUiCapabilityReloadStage::MissingReadyActivation);
        }
        if runtime.inspect_active().snapshot_digest()
            != self.evidence.active_snapshot_digest_before()
        {
            return Err(WorthUiCapabilityReloadStage::ActiveSnapshotDrift);
        }
        let admitted_batch = self
            .admitted_batch
            .ok_or(WorthUiCapabilityReloadStage::MissingReadyActivation)?;
        let candidate_snapshot = admitted_batch.into_candidate_snapshot();

        let active_state = runtime.active_state_for_swap_mut();
        let artifact_digest = active_state.active_artifact().digest();
        let active_plan = WorthUiActiveExecutionPlan::from_launch_authority(
            artifact_digest,
            candidate_snapshot.digest(),
        );
        active_state.replace_capability_snapshot(candidate_snapshot, active_plan);
        runtime.record_last_valid_from_active_for_swap();
        Ok(self
            .evidence
            .mark_activated(runtime.inspect_active().snapshot_digest()))
    }
}
