use crate::capability::SurfaceId;

use super::payload::WorthUiInteractionKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMountedInteractionGesture {
    PrimaryClick,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInteractionActivationRequest {
    surface_id: SurfaceId,
    interaction_id: String,
    kind: WorthUiInteractionKind,
    gesture: WorthUiMountedInteractionGesture,
}

impl WorthUiMountedInteractionGesture {
    pub fn primary_click() -> Self {
        Self::PrimaryClick
    }
}

impl WorthUiInteractionActivationRequest {
    pub(crate) fn new(
        surface_id: SurfaceId,
        interaction_id: impl Into<String>,
        kind: WorthUiInteractionKind,
        gesture: WorthUiMountedInteractionGesture,
    ) -> Self {
        Self {
            surface_id,
            interaction_id: interaction_id.into(),
            kind,
            gesture,
        }
    }

    pub fn surface_id(&self) -> &SurfaceId {
        &self.surface_id
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn kind(&self) -> WorthUiInteractionKind {
        self.kind
    }

    pub fn gesture(&self) -> WorthUiMountedInteractionGesture {
        self.gesture
    }
}
