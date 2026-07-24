use worth_ui_host_contract::{
    UiHostSurfaceRegistrationRequest, UiMountedSurfaceBindingRequirement,
    UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiHostTruthNativeLifecycleKind {
    Registration,
    Deregistration,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiHostTruthNativeLifecycleObligation {
    kind: UiHostTruthNativeLifecycleKind,
    request: UiHostSurfaceRegistrationRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiHostTruthIndeterminateObligations {
    semantic_surface: UiSemanticSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    native_lifecycle: Option<UiHostTruthNativeLifecycleObligation>,
    presentation: Option<UiMountedSurfaceBindingRequirement>,
}

impl UiHostTruthNativeLifecycleObligation {
    pub(super) fn kind(self) -> UiHostTruthNativeLifecycleKind {
        self.kind
    }

    pub(super) fn request(self) -> UiHostSurfaceRegistrationRequest {
        self.request
    }
}

impl UiHostTruthIndeterminateObligations {
    pub(super) fn native_lifecycle(
        kind: UiHostTruthNativeLifecycleKind,
        request: UiHostSurfaceRegistrationRequest,
    ) -> Self {
        Self {
            semantic_surface: request.semantic_surface_identity(),
            binding: request.binding_generation(),
            native_lifecycle: Some(UiHostTruthNativeLifecycleObligation { kind, request }),
            presentation: None,
        }
    }

    pub(super) fn presentation(requirement: UiMountedSurfaceBindingRequirement) -> Self {
        Self {
            semantic_surface: requirement.semantic_surface(),
            binding: requirement.binding(),
            native_lifecycle: None,
            presentation: Some(requirement),
        }
    }

    pub(super) fn record_native_lifecycle(
        &mut self,
        kind: UiHostTruthNativeLifecycleKind,
        request: UiHostSurfaceRegistrationRequest,
    ) {
        debug_assert_eq!(self.semantic_surface, request.semantic_surface_identity());
        debug_assert_eq!(self.binding, request.binding_generation());
        self.native_lifecycle = Some(UiHostTruthNativeLifecycleObligation { kind, request });
    }

    pub(super) fn record_presentation(&mut self, requirement: UiMountedSurfaceBindingRequirement) {
        debug_assert_eq!(self.semantic_surface, requirement.semantic_surface());
        debug_assert_eq!(self.binding, requirement.binding());
        self.presentation = Some(requirement);
    }

    pub(super) fn semantic_surface(self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface
    }

    pub(super) fn native_lifecycle_obligation(
        self,
    ) -> Option<UiHostTruthNativeLifecycleObligation> {
        self.native_lifecycle
    }

    pub(super) fn presentation_requirement(self) -> Option<UiMountedSurfaceBindingRequirement> {
        self.presentation
    }

    pub(super) fn clear_native_lifecycle(&mut self) {
        self.native_lifecycle = None;
    }

    pub(super) fn clear_presentation(&mut self) {
        self.presentation = None;
    }

    pub(super) fn is_empty(self) -> bool {
        self.native_lifecycle.is_none() && self.presentation.is_none()
    }
}
