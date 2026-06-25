use worth_ui::facade::{
    CommandId, CommandProjectionId, SurfaceId, WorthUiComponentInteractionDenial,
    WorthUiComponentInteractionKind, WorthUiComponentInteractionReceipt,
    WorthUiDropdownSelectionInteractionDenial, WorthUiDropdownSelectionInteractionReceipt,
    WorthUiHeaderFrameRebindDenial, WorthUiMountedInteractionActivationEligibleReceipt,
    WorthUiRebindPhaseExecutionReceipt, WorthUiRuntimeChangeAdmissionDenial,
};

use super::ValidationRuntimeWorkbench;

#[derive(Debug)]
pub enum ValidationDropdownSelectionApplicationDenial {
    Interaction(WorthUiDropdownSelectionInteractionDenial),
    RuntimeChange(WorthUiRuntimeChangeAdmissionDenial),
    HeaderRebind(WorthUiHeaderFrameRebindDenial),
}

#[derive(Debug)]
pub enum ValidationComponentInteractionApplicationDenial {
    Interaction(WorthUiComponentInteractionDenial),
    RuntimeChange(WorthUiRuntimeChangeAdmissionDenial),
}

impl ValidationRuntimeWorkbench {
    pub fn select_dropdown_command(
        &mut self,
        projection_id: &CommandProjectionId,
        command_id: &CommandId,
    ) -> Result<
        WorthUiDropdownSelectionInteractionReceipt,
        ValidationDropdownSelectionApplicationDenial,
    > {
        let receipt = self
            .runtime
            .select_dropdown_command(projection_id, command_id)
            .map_err(ValidationDropdownSelectionApplicationDenial::Interaction)?;
        let admitted_change = self
            .runtime
            .admit_dropdown_selection_runtime_change(&receipt)
            .map_err(ValidationDropdownSelectionApplicationDenial::RuntimeChange)?;
        let header_receipt = self
            .runtime_change_rebind_receipts(&admitted_change)
            .map(|receipt: WorthUiRebindPhaseExecutionReceipt| receipt.header_rebind().clone())
            .ok_or(WorthUiHeaderFrameRebindDenial::RuntimeEvidenceMismatch)
            .map_err(ValidationDropdownSelectionApplicationDenial::HeaderRebind)?;
        let _ = header_receipt;
        Ok(receipt)
    }

    pub fn submit_component_interaction(
        &mut self,
        surface_id: &SurfaceId,
        kind: WorthUiComponentInteractionKind,
    ) -> Result<WorthUiComponentInteractionReceipt, ValidationComponentInteractionApplicationDenial>
    {
        let receipt = self
            .runtime
            .bind_authored_mounted_interaction_target(surface_id)
            .map_err(|_| {
                ValidationComponentInteractionApplicationDenial::Interaction(
                    WorthUiComponentInteractionDenial::MissingSurface {
                        surface_id: surface_id.as_str().to_owned(),
                    },
                )
            })
            .and_then(|target| {
                self.runtime
                    .submit_component_interaction_for_target(target, kind)
                    .map_err(ValidationComponentInteractionApplicationDenial::Interaction)
            })?;
        self.runtime
            .admit_component_interaction_runtime_change(&receipt)
            .map_err(ValidationComponentInteractionApplicationDenial::RuntimeChange)?;
        Ok(receipt)
    }

    pub fn submit_mounted_interaction(
        &mut self,
        eligible: WorthUiMountedInteractionActivationEligibleReceipt,
    ) -> Result<WorthUiComponentInteractionReceipt, ValidationComponentInteractionApplicationDenial>
    {
        let receipt = self
            .runtime
            .submit_mounted_interaction(eligible)
            .map_err(ValidationComponentInteractionApplicationDenial::Interaction)?;
        self.runtime
            .admit_component_interaction_runtime_change(&receipt)
            .map_err(ValidationComponentInteractionApplicationDenial::RuntimeChange)?;
        Ok(receipt)
    }
}
