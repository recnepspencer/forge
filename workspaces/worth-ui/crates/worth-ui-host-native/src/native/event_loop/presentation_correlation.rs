#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiNativePhysicalPresentationCorrelation {
    attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    physical_sequence: u64,
}

impl UiNativePhysicalPresentationCorrelation {
    pub(super) const fn issued(
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        physical_sequence: u64,
    ) -> Self {
        Self {
            attempt,
            surface,
            binding,
            physical_sequence,
        }
    }

    pub const fn attempt(self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub const fn surface(self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.surface
    }

    pub const fn binding(self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.binding
    }

    #[cfg(feature = "certification-support")]
    #[doc(hidden)]
    pub fn from_certification(
        attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        physical_sequence: u64,
    ) -> Option<Self> {
        (physical_sequence != 0).then_some(Self::issued(
            attempt,
            surface,
            binding,
            physical_sequence,
        ))
    }
}
