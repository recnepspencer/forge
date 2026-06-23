use crate::runtime::{
    WorthUiCapabilityReloadEvidence, WorthUiPageHostPlan, WorthUiPageHostPlanDenial,
    WorthUiPageHostRequest, WorthUiProjectionPlanAdmissionDenial,
    WorthUiProjectionRebindBatchReceipt, WorthUiProjectionRebindPlan,
    WorthUiProjectionRebindPlanDenial, WorthUiProjectionRebindStatus,
    WorthUiRuntimeChangeAdmissionDenial, WorthUiRuntimeHost, WorthUiValidationReloadEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPageHostRebindStatus {
    PreservedEquivalentReload,
    PreservedDeniedReload,
    EquivalentAfterActivation,
    ReboundAfterActivation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPageHostRebindReceipt {
    status: WorthUiPageHostRebindStatus,
    previous_frame_digest: u64,
    rebound_frame_digest: u64,
    projection_batch: WorthUiProjectionRebindBatchReceipt,
    projection_rebuild_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPageHostRebindDenial {
    RuntimeEvidenceMismatch,
    ReloadNotActivated,
    RuntimeChange(WorthUiRuntimeChangeAdmissionDenial),
    ProjectionAdmission(WorthUiProjectionPlanAdmissionDenial),
    Plan(WorthUiPageHostPlanDenial),
}

impl WorthUiRuntimeHost {
    pub fn rebind_page_host_after_capability_reload(
        &self,
        current_plan: &WorthUiPageHostPlan,
        request: WorthUiPageHostRequest,
        evidence: &WorthUiCapabilityReloadEvidence,
    ) -> Result<(WorthUiPageHostPlan, WorthUiPageHostRebindReceipt), WorthUiPageHostRebindDenial>
    {
        let admitted_change = self
            .admit_capability_runtime_change(evidence)
            .map_err(map_runtime_change_denial)?;
        self.rebind_page_host_after_admitted_change(current_plan, request, &admitted_change)
    }

    pub fn rebind_page_host_after_reload(
        &self,
        current_plan: &WorthUiPageHostPlan,
        request: WorthUiPageHostRequest,
        evidence: &WorthUiValidationReloadEvidence,
    ) -> Result<(WorthUiPageHostPlan, WorthUiPageHostRebindReceipt), WorthUiPageHostRebindDenial>
    {
        self.verify_page_host_validation_evidence_active_digests(evidence)?;
        let admitted_change = self
            .admit_validation_runtime_change(evidence)
            .map_err(map_runtime_change_denial)?;
        self.rebind_page_host_after_admitted_change(current_plan, request, &admitted_change)
    }

    fn rebind_page_host_after_admitted_change(
        &self,
        current_plan: &WorthUiPageHostPlan,
        request: WorthUiPageHostRequest,
        admitted_change: &crate::runtime::WorthUiAdmittedRuntimeChangeEvidence,
    ) -> Result<(WorthUiPageHostPlan, WorthUiPageHostRebindReceipt), WorthUiPageHostRebindDenial>
    {
        let admitted_current = self
            .admit_projection_plan(current_plan.clone())
            .map_err(WorthUiPageHostRebindDenial::ProjectionAdmission)?;
        let rebind_plan = self
            .prepare_projection_rebind(admitted_change, admitted_current)
            .map_err(map_rebind_denial)?;
        match rebind_plan {
            WorthUiProjectionRebindPlan::Preserve(preserved_plan) => {
                let (admitted, batch) = preserved_plan.complete_preserved();
                Ok((
                    admitted.plan().clone(),
                    WorthUiPageHostRebindReceipt::from_projection_batch(&batch),
                ))
            }
            WorthUiProjectionRebindPlan::Rebuild(activated_plan) => {
                let rebound = WorthUiPageHostPlan::from_runtime(self, request)
                    .map_err(WorthUiPageHostRebindDenial::Plan)?;
                let admitted_rebound = self
                    .admit_projection_plan(rebound)
                    .map_err(WorthUiPageHostRebindDenial::ProjectionAdmission)?;
                let (admitted, batch) = activated_plan.complete_rebuild(admitted_rebound);
                Ok((
                    admitted.plan().clone(),
                    WorthUiPageHostRebindReceipt::from_projection_batch(&batch),
                ))
            }
        }
    }

    pub fn rebind_page_host_from_phase_plan(
        &self,
        request: WorthUiPageHostRequest,
        phase_plan: WorthUiProjectionRebindPlan<WorthUiPageHostPlan>,
    ) -> Result<(WorthUiPageHostPlan, WorthUiPageHostRebindReceipt), WorthUiPageHostRebindDenial>
    {
        match phase_plan {
            WorthUiProjectionRebindPlan::Preserve(preserved_plan) => {
                let (admitted, batch) = preserved_plan.complete_preserved();
                Ok((
                    admitted.plan().clone(),
                    WorthUiPageHostRebindReceipt::from_projection_batch(&batch),
                ))
            }
            WorthUiProjectionRebindPlan::Rebuild(activated_plan) => {
                let rebound = WorthUiPageHostPlan::from_runtime(self, request)
                    .map_err(WorthUiPageHostRebindDenial::Plan)?;
                let admitted_rebound = self
                    .admit_projection_plan(rebound)
                    .map_err(WorthUiPageHostRebindDenial::ProjectionAdmission)?;
                let (admitted, batch) = activated_plan.complete_rebuild(admitted_rebound);
                Ok((
                    admitted.plan().clone(),
                    WorthUiPageHostRebindReceipt::from_projection_batch(&batch),
                ))
            }
        }
    }

    fn verify_page_host_validation_evidence_active_digests(
        &self,
        evidence: &WorthUiValidationReloadEvidence,
    ) -> Result<(), WorthUiPageHostRebindDenial> {
        let active = self.inspect_active();
        if evidence.active_artifact_digest_after() != active.artifact_digest()
            || evidence.active_plan_digest_after() != active.active_plan_digest()
        {
            return Err(WorthUiPageHostRebindDenial::RuntimeEvidenceMismatch);
        }
        Ok(())
    }
}

impl WorthUiPageHostRebindReceipt {
    fn from_projection_batch(batch: &WorthUiProjectionRebindBatchReceipt) -> Self {
        let row = batch
            .rows()
            .first()
            .expect("single projection rebind batch carries one row");
        Self {
            status: page_host_status(row.status()),
            previous_frame_digest: row.previous_frame_digest(),
            rebound_frame_digest: row.rebound_frame_digest(),
            projection_batch: batch.clone(),
            projection_rebuild_count: batch.counters().rebuild_attempt_count(),
        }
    }

    pub fn status(&self) -> WorthUiPageHostRebindStatus {
        self.status
    }

    pub fn previous_frame_digest(&self) -> u64 {
        self.previous_frame_digest
    }

    pub fn rebound_frame_digest(&self) -> u64 {
        self.rebound_frame_digest
    }

    pub fn projection_rebind_batch(&self) -> &WorthUiProjectionRebindBatchReceipt {
        &self.projection_batch
    }

    pub fn projection_rebuild_count(&self) -> usize {
        self.projection_rebuild_count
    }
}

fn map_runtime_change_denial(
    denial: WorthUiRuntimeChangeAdmissionDenial,
) -> WorthUiPageHostRebindDenial {
    match denial {
        WorthUiRuntimeChangeAdmissionDenial::RuntimeInstanceMismatch => {
            WorthUiPageHostRebindDenial::RuntimeEvidenceMismatch
        }
        WorthUiRuntimeChangeAdmissionDenial::ActivatedFamilyWithoutChangedFacts => {
            WorthUiPageHostRebindDenial::RuntimeChange(denial)
        }
    }
}

fn map_rebind_denial(denial: WorthUiProjectionRebindPlanDenial) -> WorthUiPageHostRebindDenial {
    match denial {
        WorthUiProjectionRebindPlanDenial::RuntimeEvidenceMismatch => {
            WorthUiPageHostRebindDenial::RuntimeEvidenceMismatch
        }
        WorthUiProjectionRebindPlanDenial::ReloadNotActivated => {
            WorthUiPageHostRebindDenial::ReloadNotActivated
        }
    }
}

fn page_host_status(status: WorthUiProjectionRebindStatus) -> WorthUiPageHostRebindStatus {
    match status {
        WorthUiProjectionRebindStatus::PreservedEquivalentReload => {
            WorthUiPageHostRebindStatus::PreservedEquivalentReload
        }
        WorthUiProjectionRebindStatus::PreservedDeniedReload
        | WorthUiProjectionRebindStatus::DeniedReloadNotActivated => {
            WorthUiPageHostRebindStatus::PreservedDeniedReload
        }
        WorthUiProjectionRebindStatus::EquivalentAfterActivation => {
            WorthUiPageHostRebindStatus::EquivalentAfterActivation
        }
        WorthUiProjectionRebindStatus::ReboundAfterActivation => {
            WorthUiPageHostRebindStatus::ReboundAfterActivation
        }
    }
}
