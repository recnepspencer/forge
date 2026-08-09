use super::{
    UiHostProtocolAgreement, UiHostSurfaceIdentity, UiHostSurfacePresentationMode,
    UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};
use crate::WorthUiHostCapabilityObservationGeneration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSurfaceRegistrationRequest {
    host_session_identity: u64,
    semantic_surface_identity: UiSemanticSurfaceIdentity,
    host_surface_identity: UiHostSurfaceIdentity,
    binding_generation: UiSurfaceBindingGeneration,
    protocol: UiHostProtocolAgreement,
    capability_generation: WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    presentation_mode: UiHostSurfacePresentationMode,
}

#[doc(hidden)]
pub struct UiHostSurfaceRegistrationInput {
    pub host_session_identity: u64,
    pub semantic_surface_identity: UiSemanticSurfaceIdentity,
    pub host_surface_identity: UiHostSurfaceIdentity,
    pub binding_generation: UiSurfaceBindingGeneration,
    pub protocol: UiHostProtocolAgreement,
    pub capability_generation: WorthUiHostCapabilityObservationGeneration,
    pub capability_profile_digest: u64,
    pub presentation_mode: UiHostSurfacePresentationMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSurfaceBaselineIdentity {
    semantic_surface_identity: UiSemanticSurfaceIdentity,
    host_surface_identity: UiHostSurfaceIdentity,
    binding_generation: UiSurfaceBindingGeneration,
    capability_generation: WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    presentation_mode: UiHostSurfacePresentationMode,
    transparent_rgba8: [u8; 4],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSurfaceDeregistrationReceipt {
    host_session_identity: u64,
    host_surface_identity: UiHostSurfaceIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSurfaceRegistrationIndeterminate {
    request: UiHostSurfaceRegistrationRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostSurfaceDeregistrationIndeterminate {
    request: UiHostSurfaceRegistrationRequest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSurfaceRegistrationDenial {
    Unsupported,
    KnownEmptyBaselineUnavailable,
    ForeignRegistration,
    CapacityExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSurfaceRegistrationOutcome {
    RejectedBeforeEffects(UiHostSurfaceRegistrationDenial),
    RegisteredKnownEmpty,
    RegistrationIndeterminate(UiHostSurfaceRegistrationIndeterminate),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSurfaceDeregistrationOutcome {
    RejectedBeforeEffects(UiHostSurfaceRegistrationDenial),
    Deregistered(UiHostSurfaceDeregistrationReceipt),
    DeregistrationIndeterminate(UiHostSurfaceDeregistrationIndeterminate),
}

impl UiHostSurfaceRegistrationRequest {
    #[doc(hidden)]
    pub const fn from_runtime(input: UiHostSurfaceRegistrationInput) -> Self {
        Self {
            host_session_identity: input.host_session_identity,
            semantic_surface_identity: input.semantic_surface_identity,
            host_surface_identity: input.host_surface_identity,
            binding_generation: input.binding_generation,
            protocol: input.protocol,
            capability_generation: input.capability_generation,
            capability_profile_digest: input.capability_profile_digest,
            presentation_mode: input.presentation_mode,
        }
    }

    pub const fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }

    pub const fn semantic_surface_identity(self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface_identity
    }

    pub const fn host_surface_identity(self) -> UiHostSurfaceIdentity {
        self.host_surface_identity
    }

    pub const fn binding_generation(self) -> UiSurfaceBindingGeneration {
        self.binding_generation
    }

    pub const fn protocol(self) -> UiHostProtocolAgreement {
        self.protocol
    }

    pub const fn capability_generation(self) -> WorthUiHostCapabilityObservationGeneration {
        self.capability_generation
    }

    pub const fn capability_profile_digest(self) -> u64 {
        self.capability_profile_digest
    }

    pub const fn presentation_mode(self) -> UiHostSurfacePresentationMode {
        self.presentation_mode
    }

    #[doc(hidden)]
    pub const fn baseline_identity(self) -> UiHostSurfaceBaselineIdentity {
        UiHostSurfaceBaselineIdentity::from_surface_binding(
            self.semantic_surface_identity,
            self.host_surface_identity,
            self.binding_generation,
            self.capability_generation,
            self.capability_profile_digest,
            self.presentation_mode,
        )
    }
}

impl UiHostSurfaceBaselineIdentity {
    pub(crate) const fn from_surface_binding(
        semantic_surface_identity: UiSemanticSurfaceIdentity,
        host_surface_identity: UiHostSurfaceIdentity,
        binding_generation: UiSurfaceBindingGeneration,
        capability_generation: WorthUiHostCapabilityObservationGeneration,
        capability_profile_digest: u64,
        presentation_mode: UiHostSurfacePresentationMode,
    ) -> Self {
        Self {
            semantic_surface_identity,
            host_surface_identity,
            binding_generation,
            capability_generation,
            capability_profile_digest,
            presentation_mode,
            transparent_rgba8: [0, 0, 0, 0],
        }
    }

    pub const fn semantic_surface_identity(self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface_identity
    }

    pub const fn host_surface_identity(self) -> UiHostSurfaceIdentity {
        self.host_surface_identity
    }

    pub const fn binding_generation(self) -> UiSurfaceBindingGeneration {
        self.binding_generation
    }

    pub const fn capability_generation(self) -> WorthUiHostCapabilityObservationGeneration {
        self.capability_generation
    }

    pub const fn capability_profile_digest(self) -> u64 {
        self.capability_profile_digest
    }

    pub const fn presentation_mode(self) -> UiHostSurfacePresentationMode {
        self.presentation_mode
    }

    pub const fn transparent_rgba8(self) -> [u8; 4] {
        self.transparent_rgba8
    }
}

impl UiHostSurfaceDeregistrationReceipt {
    #[doc(hidden)]
    pub const fn from_runtime(
        host_session_identity: u64,
        host_surface_identity: UiHostSurfaceIdentity,
    ) -> Self {
        Self {
            host_session_identity,
            host_surface_identity,
        }
    }

    pub const fn host_session_identity(self) -> u64 {
        self.host_session_identity
    }

    pub const fn host_surface_identity(self) -> UiHostSurfaceIdentity {
        self.host_surface_identity
    }
}

impl UiHostSurfaceRegistrationIndeterminate {
    pub const fn after_effects_may_have_begun(request: UiHostSurfaceRegistrationRequest) -> Self {
        Self { request }
    }

    pub const fn request(self) -> UiHostSurfaceRegistrationRequest {
        self.request
    }
}

impl UiHostSurfaceDeregistrationIndeterminate {
    pub const fn after_effects_may_have_begun(request: UiHostSurfaceRegistrationRequest) -> Self {
        Self { request }
    }

    pub const fn request(self) -> UiHostSurfaceRegistrationRequest {
        self.request
    }
}
