use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionReceipt,
    WorthUiMountedInteractionActivationEligibleReceipt, WorthUiMountedInteractionGesture,
    WorthUiMountedInteractionPlan, WorthUiMountedInteractionPlanRequest,
    WorthUiMountedInteractionTargetBinding, WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiInteractionSubmissionDenial {
    MissingSurface {
        surface_id: String,
    },
    MissingAuthoredSurface {
        surface_id: String,
    },
    UnsupportedInteraction {
        surface_id: String,
        component_id: String,
        kind: WorthUiInteractionKind,
    },
    DisabledInteraction {
        surface_id: String,
        interaction_id: String,
    },
    StaleActivationReceipt {
        surface_id: String,
        interaction_id: String,
        expected_digest: u64,
        received_digest: u64,
    },
    GestureMismatch {
        surface_id: String,
        interaction_id: String,
        gesture: WorthUiMountedInteractionGesture,
    },
    InvalidInteractionValues {
        surface_id: String,
    },
}

impl WorthUiRuntimeHost {
    pub(crate) fn resolve_mounted_interaction_plan(
        &self,
        request: WorthUiMountedInteractionPlanRequest,
    ) -> Result<WorthUiMountedInteractionPlan, WorthUiInteractionSubmissionDenial> {
        let surface_id = request.surface_id().clone();
        let component_id = self.active_component_id(&surface_id)?;
        let report = self.admit_interaction_props(&surface_id);
        let admitted = report.status().accepted_receipt().ok_or_else(|| {
            WorthUiInteractionSubmissionDenial::InvalidInteractionValues {
                surface_id: surface_id.as_str().to_owned(),
            }
        })?;
        let primitive_report = self.admit_primitive_props(&surface_id);
        let primitive_receipt = primitive_report
            .status()
            .accepted_receipt()
            .ok_or_else(
                || WorthUiInteractionSubmissionDenial::InvalidInteractionValues {
                    surface_id: surface_id.as_str().to_owned(),
                },
            )?;
        let dependency_facts = self.graph_authority().plan_mounted_interaction_operability(
            &surface_id,
            admitted.prop_set().interaction_id(),
            interaction_target_dependency_facts(admitted.prop_set().target()),
        );
        Ok(WorthUiMountedInteractionPlan::from_admitted_interaction(
            self.graph_authority(),
            request,
            component_id,
            admitted,
            primitive_receipt.prop_set().disabled(),
            primitive_receipt.prop_set().focus(),
            dependency_facts,
        ))
    }

    pub fn resolve_mounted_interaction_plan_for_target(
        &self,
        target: WorthUiMountedInteractionTargetBinding,
    ) -> Result<WorthUiMountedInteractionPlan, WorthUiInteractionSubmissionDenial> {
        self.resolve_mounted_interaction_plan(
            WorthUiMountedInteractionPlanRequest::primary_click_for_target(target),
        )
    }

    pub fn submit_mounted_interaction(
        &mut self,
        eligible: WorthUiMountedInteractionActivationEligibleReceipt,
    ) -> Result<WorthUiInteractionReceipt, WorthUiInteractionSubmissionDenial> {
        let surface_id = eligible.surface_id().clone();
        let current_plan = self.resolve_mounted_interaction_plan(
            WorthUiMountedInteractionPlanRequest::primary_click(surface_id.clone()),
        )?;
        let Some(current_eligible) = current_plan.activation().eligible() else {
            return Err(WorthUiInteractionSubmissionDenial::DisabledInteraction {
                surface_id: surface_id.as_str().to_owned(),
                interaction_id: eligible.interaction_id().to_owned(),
            });
        };
        if current_eligible.component_id() != eligible.component_id() {
            return Err(WorthUiInteractionSubmissionDenial::UnsupportedInteraction {
                surface_id: surface_id.as_str().to_owned(),
                component_id: current_eligible.component_id().as_str().to_owned(),
                kind: eligible.kind(),
            });
        }
        if current_eligible.receipt_digest() != eligible.receipt_digest() {
            return Err(WorthUiInteractionSubmissionDenial::StaleActivationReceipt {
                surface_id: surface_id.as_str().to_owned(),
                interaction_id: eligible.interaction_id().to_owned(),
                expected_digest: current_eligible.receipt_digest(),
                received_digest: eligible.receipt_digest(),
            });
        }
        Ok(eligible.emit_interaction_receipt())
    }

    pub fn submit_component_interaction_for_target(
        &mut self,
        target: WorthUiMountedInteractionTargetBinding,
        kind: WorthUiInteractionKind,
    ) -> Result<WorthUiInteractionReceipt, WorthUiInteractionSubmissionDenial> {
        let plan = self.resolve_mounted_interaction_plan_for_target(target)?;
        let Some(eligible) = plan.activation().eligible().cloned() else {
            return Err(WorthUiInteractionSubmissionDenial::DisabledInteraction {
                surface_id: plan.surface_id().to_owned(),
                interaction_id: plan
                    .activation()
                    .denied()
                    .map(|denial| denial.interaction_id().to_owned())
                    .unwrap_or_else(|| "unknown".to_owned()),
            });
        };
        if eligible.kind() != kind {
            return Err(WorthUiInteractionSubmissionDenial::UnsupportedInteraction {
                surface_id: plan.surface_id().to_owned(),
                component_id: eligible.component_id().as_str().to_owned(),
                kind,
            });
        }
        self.submit_mounted_interaction(eligible)
    }

    fn active_component_id(
        &self,
        surface_id: &SurfaceId,
    ) -> Result<ComponentId, WorthUiInteractionSubmissionDenial> {
        let Some(surface) = self.inspect_active_surface_descriptor(surface_id) else {
            return Err(WorthUiInteractionSubmissionDenial::MissingSurface {
                surface_id: surface_id.as_str().to_owned(),
            });
        };
        let authored_component_id = self
            .inspect_active_authored_surface_component_id(surface_id)
            .unwrap_or_else(|| surface.component_id().as_str());
        ComponentId::new(authored_component_id).map_err(|_| {
            WorthUiInteractionSubmissionDenial::MissingAuthoredSurface {
                surface_id: surface_id.as_str().to_owned(),
            }
        })
    }
}

fn interaction_target_dependency_facts(
    target: &crate::runtime::WorthUiInteractionTarget,
) -> Vec<WorthUiRuntimeFactId> {
    match target {
        crate::runtime::WorthUiInteractionTarget::Command(command_id) => {
            crate::capability::CommandId::new(command_id)
                .ok()
                .map(|command_id| WorthUiRuntimeFactId::command(&command_id))
                .into_iter()
                .collect()
        }
        _ => Vec::new(),
    }
}
