use worth_ui::facade::{
    SurfaceId, WorthUiInteractionSubmissionDenial, WorthUiMountedInteractionActivation,
    WorthUiMountedInteractionActivationDeniedReceipt, WorthUiPrimitiveProofDenial,
    WorthUiPrimitiveProofReceipt, WorthUiUserIntentTargetDenial,
};

use crate::runtime_workbench::ValidationComponentInteractionApplicationDenial;

use super::ValidationWorkbenchApp;

impl ValidationWorkbenchApp {
    pub fn centered_primitive_proof(
        &self,
    ) -> Result<WorthUiPrimitiveProofReceipt, WorthUiPrimitiveProofDenial> {
        let primitive_surface_id = self
            .workbench
            .runtime()
            .bind_visible_primitive_proof_target(self.workbench.page_host_plan(), "button_proof");
        let primitive_surface_id =
            primitive_surface_id.map_err(primitive_target_denial_to_proof_denial)?;
        self.workbench
            .runtime()
            .resolve_primitive_proof_for_target(&primitive_surface_id)
    }

    pub fn authored_primitive_proof(
        &self,
        surface_id: &SurfaceId,
    ) -> Result<WorthUiPrimitiveProofReceipt, WorthUiPrimitiveProofDenial> {
        let target = self
            .workbench
            .runtime()
            .bind_authored_primitive_proof_target(surface_id)
            .map_err(primitive_target_denial_to_proof_denial)?;
        self.workbench
            .runtime()
            .resolve_primitive_proof_for_target(&target)
    }

    pub fn submit_mounted_primitive_primary_click(
        &mut self,
        surface_id: &SurfaceId,
    ) -> Result<
        worth_ui::facade::WorthUiComponentInteractionReceipt,
        ValidationMountedPrimitiveInteractionDenial,
    > {
        let proof = self
            .authored_primitive_proof(surface_id)
            .map_err(ValidationMountedPrimitiveInteractionDenial::PrimitiveProof)?;
        let plan = self
            .workbench
            .runtime()
            .resolve_mounted_interaction_plan_for_target(
                proof
                    .target_binding()
                    .for_mounted_interaction(self.workbench.runtime().graph_authority()),
            )
            .map_err(|denial| {
                ValidationMountedPrimitiveInteractionDenial::InteractionApplication(
                    ValidationComponentInteractionApplicationDenial::Interaction(denial),
                )
            })?;
        match plan.activation().clone() {
            WorthUiMountedInteractionActivation::Eligible(eligible) => self
                .workbench
                .submit_mounted_interaction(eligible)
                .map_err(ValidationMountedPrimitiveInteractionDenial::InteractionApplication),
            WorthUiMountedInteractionActivation::Denied(denial) => Err(
                ValidationMountedPrimitiveInteractionDenial::Activation(denial),
            ),
        }
    }
}

#[derive(Debug)]
pub enum ValidationMountedPrimitiveInteractionDenial {
    PrimitiveProof(WorthUiPrimitiveProofDenial),
    Activation(WorthUiMountedInteractionActivationDeniedReceipt),
    InteractionApplication(ValidationComponentInteractionApplicationDenial),
}

impl ValidationMountedPrimitiveInteractionDenial {
    pub fn interaction_submission_denial(&self) -> Option<&WorthUiInteractionSubmissionDenial> {
        match self {
            Self::PrimitiveProof(_) => None,
            Self::Activation(_) => None,
            Self::InteractionApplication(
                ValidationComponentInteractionApplicationDenial::Interaction(denial),
            ) => Some(denial),
            Self::InteractionApplication(
                ValidationComponentInteractionApplicationDenial::RuntimeChange(_),
            ) => None,
        }
    }

    pub fn presentation_line(&self) -> String {
        match self {
            Self::PrimitiveProof(denial) => format!("primitive proof denied: {denial}"),
            Self::Activation(receipt) => format!(
                "activation denied: interaction={} operability={:?}/{:?}",
                receipt.interaction_id(),
                receipt.operability().posture(),
                receipt.operability().basis()
            ),
            Self::InteractionApplication(
                ValidationComponentInteractionApplicationDenial::Interaction(denial),
            ) => interaction_submission_denial_line(denial),
            Self::InteractionApplication(
                ValidationComponentInteractionApplicationDenial::RuntimeChange(_),
            ) => "runtime change admission denied during interaction submission".to_owned(),
        }
    }
}

fn interaction_submission_denial_line(denial: &WorthUiInteractionSubmissionDenial) -> String {
    match denial {
        WorthUiInteractionSubmissionDenial::MissingSurface { surface_id } => {
            format!("interaction denied: missing surface {surface_id}")
        }
        WorthUiInteractionSubmissionDenial::MissingAuthoredSurface { surface_id } => {
            format!("interaction denied: missing authored surface {surface_id}")
        }
        WorthUiInteractionSubmissionDenial::UnsupportedInteraction {
            surface_id,
            component_id,
            kind,
        } => format!(
            "interaction denied: unsupported {} on {} for {}",
            kind.token(),
            surface_id,
            component_id
        ),
        WorthUiInteractionSubmissionDenial::DisabledInteraction {
            surface_id,
            interaction_id,
        } => format!("interaction denied: disabled {interaction_id} on {surface_id}"),
        WorthUiInteractionSubmissionDenial::StaleActivationReceipt {
            surface_id,
            interaction_id,
            expected_digest,
            received_digest,
        } => format!(
            "interaction denied: stale activation {interaction_id} on {surface_id} expected {expected_digest} received {received_digest}"
        ),
        WorthUiInteractionSubmissionDenial::GestureMismatch {
            surface_id,
            interaction_id,
            gesture,
        } => format!(
            "interaction denied: gesture {:?} mismatched {interaction_id} on {surface_id}",
            gesture
        ),
        WorthUiInteractionSubmissionDenial::InvalidInteractionValues { surface_id } => {
            format!("interaction denied: invalid interaction values on {surface_id}")
        }
    }
}

impl ValidationWorkbenchApp {
    pub fn click_centered_primitive_for_proof(
        &mut self,
    ) -> Result<
        worth_ui::facade::WorthUiComponentInteractionReceipt,
        ValidationMountedPrimitiveInteractionDenial,
    > {
        let target = self
            .workbench
            .runtime()
            .bind_visible_primitive_proof_target(self.workbench.page_host_plan(), "button_proof")
            .map_err(|denial| {
                ValidationMountedPrimitiveInteractionDenial::PrimitiveProof(
                    primitive_target_denial_to_proof_denial(denial),
                )
            })?;
        self.submit_mounted_primitive_primary_click(target.surface_id())
            .inspect(|receipt| {
                self.last_primitive_interaction = Some(receipt.clone());
                self.last_primitive_interaction_denial = None;
            })
            .inspect_err(|denial| {
                self.last_primitive_interaction = None;
                self.last_primitive_interaction_denial = Some(denial.presentation_line());
            })
    }

    pub fn last_primitive_interaction(
        &self,
    ) -> Option<&worth_ui::facade::WorthUiComponentInteractionReceipt> {
        self.last_primitive_interaction.as_ref()
    }
}

fn primitive_target_denial_to_proof_denial(
    denial: WorthUiUserIntentTargetDenial,
) -> WorthUiPrimitiveProofDenial {
    match denial {
        WorthUiUserIntentTargetDenial::MissingSlot {
            page_name,
            slot_name,
            ..
        } => WorthUiPrimitiveProofDenial::MissingSurface {
            surface_id: format!("{page_name}.{slot_name}"),
        },
        WorthUiUserIntentTargetDenial::MissingSurface { surface_id, .. }
        | WorthUiUserIntentTargetDenial::InvalidSurfaceId { surface_id, .. } => {
            WorthUiPrimitiveProofDenial::MissingSurface { surface_id }
        }
        WorthUiUserIntentTargetDenial::InvalidComponentId {
            surface_id,
            component_id,
            ..
        } => WorthUiPrimitiveProofDenial::ComponentMismatch {
            surface_id,
            expected_component_id: "worth.component.primitive_proof".to_owned(),
            actual_component_id: component_id,
        },
    }
}
