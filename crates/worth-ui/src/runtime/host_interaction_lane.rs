use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::{
    WorthUiInteractionActivationRequest, WorthUiInteractionKind, WorthUiInteractionReadiness,
    WorthUiInteractionReceipt, WorthUiMountedInteractionGesture, WorthUiRuntimeHost,
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
    pub fn submit_surface_interaction(
        &mut self,
        request: WorthUiInteractionActivationRequest,
    ) -> Result<WorthUiInteractionReceipt, WorthUiInteractionSubmissionDenial> {
        let surface_id = request.surface_id().clone();
        let component_id = self.active_component_id(&surface_id)?;
        let report = self.admit_interaction_props(&surface_id);
        let admitted = report.status().accepted_receipt().ok_or_else(|| {
            WorthUiInteractionSubmissionDenial::InvalidInteractionValues {
                surface_id: surface_id.as_str().to_owned(),
            }
        })?;
        let props = admitted.prop_set();
        if props.kind() != request.kind() || props.interaction_id() != request.interaction_id() {
            return Err(WorthUiInteractionSubmissionDenial::UnsupportedInteraction {
                surface_id: surface_id.as_str().to_owned(),
                component_id: component_id.as_str().to_owned(),
                kind: request.kind(),
            });
        }
        if props.readiness() == WorthUiInteractionReadiness::Disabled {
            return Err(WorthUiInteractionSubmissionDenial::DisabledInteraction {
                surface_id: surface_id.as_str().to_owned(),
                interaction_id: request.interaction_id().to_owned(),
            });
        }
        if !gesture_admits_kind(request.gesture(), request.kind()) {
            return Err(WorthUiInteractionSubmissionDenial::GestureMismatch {
                surface_id: surface_id.as_str().to_owned(),
                interaction_id: request.interaction_id().to_owned(),
                gesture: request.gesture(),
            });
        }
        Ok(admitted.emit_receipt(&surface_id, &component_id))
    }

    pub fn submit_component_interaction(
        &mut self,
        surface_id: &SurfaceId,
        kind: WorthUiInteractionKind,
    ) -> Result<WorthUiInteractionReceipt, WorthUiInteractionSubmissionDenial> {
        let component_id = self.active_component_id(surface_id)?;
        let report = self.admit_interaction_props(surface_id);
        let admitted = report.status().accepted_receipt().ok_or_else(|| {
            WorthUiInteractionSubmissionDenial::InvalidInteractionValues {
                surface_id: surface_id.as_str().to_owned(),
            }
        })?;
        if admitted.prop_set().kind() != kind {
            return Err(WorthUiInteractionSubmissionDenial::UnsupportedInteraction {
                surface_id: surface_id.as_str().to_owned(),
                component_id: component_id.as_str().to_owned(),
                kind,
            });
        }
        if admitted.prop_set().readiness() == WorthUiInteractionReadiness::Disabled {
            return Err(WorthUiInteractionSubmissionDenial::DisabledInteraction {
                surface_id: surface_id.as_str().to_owned(),
                interaction_id: admitted.prop_set().interaction_id().to_owned(),
            });
        }
        Ok(admitted.emit_receipt(surface_id, &component_id))
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

fn gesture_admits_kind(
    gesture: WorthUiMountedInteractionGesture,
    kind: WorthUiInteractionKind,
) -> bool {
    matches!(gesture, WorthUiMountedInteractionGesture::PrimaryClick)
        && matches!(
            kind,
            WorthUiInteractionKind::Click
                | WorthUiInteractionKind::Submit
                | WorthUiInteractionKind::Command
                | WorthUiInteractionKind::Toggle
                | WorthUiInteractionKind::Open
                | WorthUiInteractionKind::Focus
        )
}
