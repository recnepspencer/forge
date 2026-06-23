use worth_ui::facade::{
    SurfaceId, WorthUiInteractionSubmissionDenial, WorthUiMountedInteractionGesture,
    WorthUiPrimitiveProofDenial, WorthUiPrimitiveProofReceipt,
};

use crate::runtime_workbench::ValidationComponentInteractionApplicationDenial;

use super::ValidationWorkbenchApp;

impl ValidationWorkbenchApp {
    pub fn centered_primitive_proof(
        &self,
    ) -> Result<WorthUiPrimitiveProofReceipt, WorthUiPrimitiveProofDenial> {
        let primitive_surface_id =
            SurfaceId::new("worth.surface.preview.primitive.proof").expect("valid surface id");
        self.workbench
            .runtime()
            .resolve_primitive_proof(&primitive_surface_id)
    }

    pub fn submit_mounted_primitive_primary_click(
        &mut self,
        surface_id: &SurfaceId,
    ) -> Result<
        worth_ui::facade::WorthUiComponentInteractionReceipt,
        ValidationMountedPrimitiveInteractionDenial,
    > {
        let proof = self
            .workbench
            .runtime()
            .resolve_primitive_proof(surface_id)
            .map_err(ValidationMountedPrimitiveInteractionDenial::PrimitiveProof)?;
        let request = proof.interaction().activation_request(
            surface_id,
            WorthUiMountedInteractionGesture::primary_click(),
        );
        self.workbench
            .submit_surface_interaction(request)
            .map_err(ValidationMountedPrimitiveInteractionDenial::InteractionApplication)
    }

    pub(super) fn apply_mounted_primitive_primary_click(&mut self, surface_id: &SurfaceId) {
        match self.submit_mounted_primitive_primary_click(surface_id) {
            Ok(receipt) => {
                self.last_primitive_interaction = Some(receipt);
                self.last_primitive_interaction_denial = None;
            }
            Err(denial) => {
                self.last_primitive_interaction = None;
                self.last_primitive_interaction_denial = Some(format!("{denial:?}"));
            }
        }
    }
}

#[derive(Debug)]
pub enum ValidationMountedPrimitiveInteractionDenial {
    PrimitiveProof(WorthUiPrimitiveProofDenial),
    InteractionApplication(ValidationComponentInteractionApplicationDenial),
}

impl ValidationMountedPrimitiveInteractionDenial {
    pub fn interaction_submission_denial(&self) -> Option<&WorthUiInteractionSubmissionDenial> {
        match self {
            Self::PrimitiveProof(_) => None,
            Self::InteractionApplication(
                ValidationComponentInteractionApplicationDenial::Interaction(denial),
            ) => Some(denial),
            Self::InteractionApplication(
                ValidationComponentInteractionApplicationDenial::RuntimeChange(_),
            ) => None,
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
        let primitive_surface_id =
            SurfaceId::new("worth.surface.preview.primitive.proof").expect("valid surface id");
        self.submit_mounted_primitive_primary_click(&primitive_surface_id)
            .inspect(|receipt| {
                self.last_primitive_interaction = Some(receipt.clone());
                self.last_primitive_interaction_denial = None;
            })
            .inspect_err(|denial| {
                self.last_primitive_interaction = None;
                self.last_primitive_interaction_denial = Some(format!("{denial:?}"));
            })
    }

    pub fn last_primitive_interaction(
        &self,
    ) -> Option<&worth_ui::facade::WorthUiComponentInteractionReceipt> {
        self.last_primitive_interaction.as_ref()
    }
}
