use worth_ui_host_contract::{UiMountedFrameIdentity, UiSurfaceBindingGeneration};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedSurfaceReconciliationBinding {
    affected: UiSurfaceBindingGeneration,
    replacement: UiSurfaceBindingGeneration,
}

pub enum UiHostPresentationReconciliation {
    KnownEmptyBaseline {
        affected_binding: UiSurfaceBindingGeneration,
        replacement: super::super::UiSurfaceBindingIdentityView,
    },
}

impl UiMountedSurfaceReconciliationBinding {
    pub fn new(
        affected: UiSurfaceBindingGeneration,
        replacement: UiSurfaceBindingGeneration,
    ) -> Self {
        Self {
            affected,
            replacement,
        }
    }

    pub fn affected(self) -> UiSurfaceBindingGeneration {
        self.affected
    }

    pub fn replacement(self) -> UiSurfaceBindingGeneration {
        self.replacement
    }
}

impl UiHostPresentationReconciliation {
    pub fn affected_binding(&self) -> UiSurfaceBindingGeneration {
        match self {
            Self::KnownEmptyBaseline {
                affected_binding, ..
            } => *affected_binding,
        }
    }

    pub(crate) fn proves(
        &self,
        requirement: worth_ui_host_contract::UiMountedSurfaceBindingRequirement,
        current_frame: Option<UiMountedFrameIdentity>,
    ) -> bool {
        match self {
            Self::KnownEmptyBaseline {
                affected_binding,
                replacement,
            } => {
                let baseline = replacement.baseline();
                current_frame.is_none()
                    && replacement.binding_generation() != *affected_binding
                    && replacement.semantic_surface_identity() == requirement.semantic_surface()
                    && baseline.semantic_surface_identity()
                        == replacement.semantic_surface_identity()
                    && baseline.host_surface_identity() == replacement.host_surface_identity()
                    && baseline.presentation_mode() == replacement.presentation_mode()
            }
        }
    }
}
