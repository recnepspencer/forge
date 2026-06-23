use crate::runtime::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiCapabilityReloadEvidence,
    WorthUiProjectionPlanAdmissionDenial, WorthUiProjectionRebindBatchReceipt,
    WorthUiProjectionRebindPlan, WorthUiProjectionRebindPlanDenial,
    WorthUiRuntimeChangeAdmissionDenial, WorthUiRuntimeHost,
};

use super::{
    WorthUiDropdownProjectionPlan, WorthUiDropdownProjectionPlanDenial,
    WorthUiDropdownProjectionRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownProjectionRebindDenial {
    CapabilityReloadNotActivated,
    RuntimeEvidenceMismatch,
    RuntimeChange(WorthUiRuntimeChangeAdmissionDenial),
    ProjectionAdmission(WorthUiProjectionPlanAdmissionDenial),
    Plan(WorthUiDropdownProjectionPlanDenial),
}

impl WorthUiRuntimeHost {
    pub fn rebind_dropdown_projection_after_capability_reload(
        &mut self,
        current_plan: &WorthUiDropdownProjectionPlan,
        request: WorthUiDropdownProjectionRequest,
        evidence: &WorthUiCapabilityReloadEvidence,
    ) -> Result<
        (
            WorthUiDropdownProjectionPlan,
            WorthUiProjectionRebindBatchReceipt,
        ),
        WorthUiDropdownProjectionRebindDenial,
    > {
        let admitted_change = self
            .admit_capability_runtime_change(evidence)
            .map_err(map_runtime_change_denial)?;
        self.rebind_dropdown_projection_after_admitted_change(
            current_plan,
            request,
            &admitted_change,
        )
    }

    fn rebind_dropdown_projection_after_admitted_change(
        &mut self,
        current_plan: &WorthUiDropdownProjectionPlan,
        request: WorthUiDropdownProjectionRequest,
        evidence: &WorthUiAdmittedRuntimeChangeEvidence,
    ) -> Result<
        (
            WorthUiDropdownProjectionPlan,
            WorthUiProjectionRebindBatchReceipt,
        ),
        WorthUiDropdownProjectionRebindDenial,
    > {
        let admitted_current = self
            .admit_projection_plan(current_plan.clone())
            .map_err(WorthUiDropdownProjectionRebindDenial::ProjectionAdmission)?;
        let rebind = self
            .prepare_projection_rebind(evidence, admitted_current)
            .map_err(map_rebind_denial)?;
        match rebind {
            WorthUiProjectionRebindPlan::Preserve(plan) => {
                let (_, receipt) = plan.complete_preserved();
                self.active_state_for_swap_mut()
                    .record_dropdown_selection_state(
                        request.projection_id(),
                        current_plan.execute_frame().selection_state(),
                    );
                Ok((current_plan.clone(), receipt))
            }
            WorthUiProjectionRebindPlan::Rebuild(plan) => {
                let rebound = WorthUiDropdownProjectionPlan::rebuild_from_snapshot(
                    self.active_state_for_read().capability_snapshot(),
                    request.clone(),
                    self.active_state_for_read()
                        .dropdown_selection_state(request.projection_id()),
                )
                .map_err(WorthUiDropdownProjectionRebindDenial::Plan)?;
                let admitted_rebound = self
                    .admit_projection_plan(rebound.clone())
                    .map_err(WorthUiDropdownProjectionRebindDenial::ProjectionAdmission)?;
                let (_, receipt) = plan.complete_rebuild(admitted_rebound);
                self.active_state_for_swap_mut()
                    .record_dropdown_selection_state(
                        request.projection_id(),
                        rebound.execute_frame().selection_state(),
                    );
                Ok((rebound, receipt))
            }
        }
    }
}

fn map_runtime_change_denial(
    denial: WorthUiRuntimeChangeAdmissionDenial,
) -> WorthUiDropdownProjectionRebindDenial {
    match denial {
        WorthUiRuntimeChangeAdmissionDenial::RuntimeInstanceMismatch => {
            WorthUiDropdownProjectionRebindDenial::RuntimeEvidenceMismatch
        }
        WorthUiRuntimeChangeAdmissionDenial::ActivatedFamilyWithoutChangedFacts => {
            WorthUiDropdownProjectionRebindDenial::RuntimeChange(denial)
        }
    }
}

fn map_rebind_denial(
    denial: WorthUiProjectionRebindPlanDenial,
) -> WorthUiDropdownProjectionRebindDenial {
    match denial {
        WorthUiProjectionRebindPlanDenial::RuntimeEvidenceMismatch => {
            WorthUiDropdownProjectionRebindDenial::RuntimeEvidenceMismatch
        }
        WorthUiProjectionRebindPlanDenial::ReloadNotActivated => {
            WorthUiDropdownProjectionRebindDenial::CapabilityReloadNotActivated
        }
    }
}
