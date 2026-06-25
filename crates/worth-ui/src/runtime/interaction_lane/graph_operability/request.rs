use crate::capability::SurfaceId;
use crate::runtime::{WorthUiMountedInteractionGesture, WorthUiMountedInteractionTargetBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiMountedInteractionPlanRequest {
    surface_id: SurfaceId,
    gesture: WorthUiMountedInteractionGesture,
    target_binding: Option<WorthUiMountedInteractionTargetBinding>,
}

impl WorthUiMountedInteractionPlanRequest {
    pub fn primary_click(surface_id: SurfaceId) -> Self {
        Self {
            surface_id,
            gesture: WorthUiMountedInteractionGesture::primary_click(),
            target_binding: None,
        }
    }

    pub fn primary_click_for_target(
        target_binding: WorthUiMountedInteractionTargetBinding,
    ) -> Self {
        Self {
            surface_id: target_binding.surface_id().clone(),
            gesture: WorthUiMountedInteractionGesture::primary_click(),
            target_binding: Some(target_binding),
        }
    }

    pub fn surface_id(&self) -> &SurfaceId {
        &self.surface_id
    }

    pub fn gesture(&self) -> WorthUiMountedInteractionGesture {
        self.gesture
    }

    pub fn target_binding(&self) -> Option<&WorthUiMountedInteractionTargetBinding> {
        self.target_binding.as_ref()
    }
}
