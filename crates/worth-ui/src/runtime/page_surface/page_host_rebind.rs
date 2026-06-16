use crate::runtime::{
    WorthUiRuntimeHost, WorthUiValidationReloadEvidence, WorthUiValidationReloadStatus,
};

use super::{WorthUiPageHostPlan, WorthUiPageHostPlanDenial, WorthUiPageHostRequest};

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
    projection_rebuild_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPageHostRebindDenial {
    RuntimeEvidenceMismatch,
    ReloadNotActivated,
    Plan(WorthUiPageHostPlanDenial),
}

impl WorthUiRuntimeHost {
    pub fn rebind_page_host_after_reload(
        &self,
        current_plan: &WorthUiPageHostPlan,
        request: WorthUiPageHostRequest,
        evidence: &WorthUiValidationReloadEvidence,
    ) -> Result<(WorthUiPageHostPlan, WorthUiPageHostRebindReceipt), WorthUiPageHostRebindDenial>
    {
        if evidence.runtime_instance_witness() != self.instance_id().raw() {
            return Err(WorthUiPageHostRebindDenial::RuntimeEvidenceMismatch);
        }

        match evidence.status() {
            WorthUiValidationReloadStatus::EquivalentNoOp => {
                return Ok(preserved_rebind(
                    current_plan,
                    WorthUiPageHostRebindStatus::PreservedEquivalentReload,
                ));
            }
            WorthUiValidationReloadStatus::Denied(_) => {
                return Ok(preserved_rebind(
                    current_plan,
                    WorthUiPageHostRebindStatus::PreservedDeniedReload,
                ));
            }
            WorthUiValidationReloadStatus::ReadyForFrameBoundary => {
                return Err(WorthUiPageHostRebindDenial::ReloadNotActivated);
            }
            WorthUiValidationReloadStatus::Activated => {}
        }

        if !current_plan
            .dependencies()
            .intersects(evidence.changed_facts())
        {
            return Ok(preserved_rebind(
                current_plan,
                WorthUiPageHostRebindStatus::EquivalentAfterActivation,
            ));
        }

        let next_plan = WorthUiPageHostPlan::from_runtime(self, request)
            .map_err(WorthUiPageHostRebindDenial::Plan)?;
        let status = if next_plan.frame_digest() == current_plan.frame_digest() {
            WorthUiPageHostRebindStatus::EquivalentAfterActivation
        } else {
            WorthUiPageHostRebindStatus::ReboundAfterActivation
        };
        let receipt = WorthUiPageHostRebindReceipt::new(
            status,
            current_plan.frame_digest(),
            next_plan.frame_digest(),
            1,
        );
        Ok((next_plan, receipt))
    }
}

impl WorthUiPageHostRebindReceipt {
    fn new(
        status: WorthUiPageHostRebindStatus,
        previous_frame_digest: u64,
        rebound_frame_digest: u64,
        projection_rebuild_count: usize,
    ) -> Self {
        Self {
            status,
            previous_frame_digest,
            rebound_frame_digest,
            projection_rebuild_count,
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

    pub fn projection_rebuild_count(&self) -> usize {
        self.projection_rebuild_count
    }
}

fn preserved_rebind(
    current_plan: &WorthUiPageHostPlan,
    status: WorthUiPageHostRebindStatus,
) -> (WorthUiPageHostPlan, WorthUiPageHostRebindReceipt) {
    (
        current_plan.clone(),
        WorthUiPageHostRebindReceipt::new(
            status,
            current_plan.frame_digest(),
            current_plan.frame_digest(),
            0,
        ),
    )
}
