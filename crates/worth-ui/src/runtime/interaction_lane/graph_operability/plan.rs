use crate::capability::ComponentId;
use crate::runtime::{
    WorthUiInteractionAdmissionReceipt, WorthUiMountedInteractionActivation,
    WorthUiMountedInteractionActivationDeniedReceipt,
    WorthUiMountedInteractionActivationEligibleReceipt, WorthUiPrimitiveFocusPosture,
    WorthUiRuntimeFactId, WorthUiRuntimeGraphAuthority,
};

use super::digest::mounted_plan_digest;
use super::{WorthUiInteractionOperabilityReceipt, WorthUiMountedInteractionPlanRequest};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedInteractionPlan {
    surface_id: String,
    target_binding_digest: Option<u64>,
    activation: WorthUiMountedInteractionActivation,
    operability: WorthUiInteractionOperabilityReceipt,
    plan_digest: u64,
}

impl WorthUiMountedInteractionPlan {
    pub(crate) fn from_admitted_interaction(
        graph_authority: &WorthUiRuntimeGraphAuthority,
        request: WorthUiMountedInteractionPlanRequest,
        component_id: ComponentId,
        admission: &WorthUiInteractionAdmissionReceipt,
        primitive_disabled: bool,
        primitive_focus: WorthUiPrimitiveFocusPosture,
        dependency_facts: Vec<WorthUiRuntimeFactId>,
    ) -> Self {
        let prop_set = admission.prop_set();
        let target_binding_digest = request
            .target_binding()
            .map(|target_binding| target_binding.binding_digest());
        let receipt = admission.emit_receipt(request.surface_id(), &component_id);
        let operability = WorthUiInteractionOperabilityReceipt::resolve(
            graph_authority,
            request.surface_id(),
            prop_set.interaction_id(),
            primitive_disabled,
            prop_set.readiness(),
            prop_set.kind(),
            prop_set.target(),
            request.gesture(),
            primitive_focus,
            dependency_facts,
        );
        let activation = if operability.is_eligible() {
            WorthUiMountedInteractionActivation::Eligible(
                WorthUiMountedInteractionActivationEligibleReceipt::new(
                    request.surface_id().clone(),
                    component_id,
                    prop_set.interaction_id().to_owned(),
                    prop_set.kind(),
                    request.gesture(),
                    receipt,
                    operability.clone(),
                ),
            )
        } else {
            WorthUiMountedInteractionActivation::Denied(
                WorthUiMountedInteractionActivationDeniedReceipt::new(
                    request.surface_id().as_str(),
                    prop_set.interaction_id(),
                    prop_set.kind(),
                    request.gesture(),
                    prop_set.target().clone(),
                    operability.clone(),
                ),
            )
        };
        let plan_digest = mounted_plan_digest(request.surface_id().as_str(), &activation);
        Self {
            surface_id: request.surface_id().as_str().to_owned(),
            target_binding_digest,
            activation,
            operability,
            plan_digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn activation(&self) -> &WorthUiMountedInteractionActivation {
        &self.activation
    }

    pub fn target_binding_digest(&self) -> Option<u64> {
        self.target_binding_digest
    }

    pub fn operability(&self) -> &WorthUiInteractionOperabilityReceipt {
        &self.operability
    }

    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
    }
}
