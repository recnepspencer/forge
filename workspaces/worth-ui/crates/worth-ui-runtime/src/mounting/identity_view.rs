use crate::graph::UiGraphNodeIdentity;
use worth_ui_host_contract::{
    UiHostSurfaceBaselineIdentity, UiHostSurfaceIdentity, UiHostSurfacePresentationMode,
    UiHostSurfaceRegistrationRequest, UiMountIncarnation, UiMountedFrameIdentity,
    UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration, WorthUiHostCapabilityObservationGeneration,
};

use super::{UiMountedIdentityBasis, UiSurfaceBindingProfile};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedInstanceIdentityView {
    identity: UiMountedInstanceIdentity,
    basis: UiMountedIdentityBasis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiSurfaceBindingIdentityView {
    semantic_surface_identity: UiSemanticSurfaceIdentity,
    host_surface_identity: UiHostSurfaceIdentity,
    binding_generation: UiSurfaceBindingGeneration,
    capability_observation_generation: WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    presentation_mode: UiHostSurfacePresentationMode,
    profile: UiSurfaceBindingProfile,
    baseline: UiHostSurfaceBaselineIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedFrameIdentityView {
    frame_identity: UiMountedFrameIdentity,
    mounted_instance_identity: UiMountedInstanceIdentity,
    node_receipt_identity: UiMountedNodeReceiptIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedIdentityView {
    mounted_instances: Box<[UiMountedInstanceIdentityView]>,
    surface_bindings: Box<[UiSurfaceBindingIdentityView]>,
    current_frame: Option<UiMountedFrameIdentity>,
    frame_receipts: Box<[UiMountedFrameIdentityView]>,
}

pub(super) struct UiSurfaceBindingConstruction {
    pub(super) request: UiHostSurfaceRegistrationRequest,
    pub(super) binding_generation: UiSurfaceBindingGeneration,
    pub(super) profile: UiSurfaceBindingProfile,
    pub(super) baseline: UiHostSurfaceBaselineIdentity,
}

impl UiMountedInstanceIdentityView {
    pub(crate) fn new(identity: UiMountedInstanceIdentity, basis: UiMountedIdentityBasis) -> Self {
        Self { identity, basis }
    }

    pub fn identity(&self) -> UiMountedInstanceIdentity {
        self.identity
    }

    pub fn basis(&self) -> &UiMountedIdentityBasis {
        &self.basis
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.basis.graph_node_identity()
    }

    pub fn mount_incarnation(&self) -> UiMountIncarnation {
        self.basis.mount_incarnation()
    }
}

impl UiSurfaceBindingIdentityView {
    pub(super) fn new(construction: UiSurfaceBindingConstruction) -> Self {
        Self {
            semantic_surface_identity: construction.request.semantic_surface_identity(),
            host_surface_identity: construction.request.host_surface_identity(),
            binding_generation: construction.binding_generation,
            capability_observation_generation: construction.request.capability_generation(),
            capability_profile_digest: construction.request.capability_profile_digest(),
            presentation_mode: construction.request.presentation_mode(),
            profile: construction.profile,
            baseline: construction.baseline,
        }
    }

    pub fn semantic_surface_identity(self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface_identity
    }

    pub fn host_surface_identity(self) -> UiHostSurfaceIdentity {
        self.host_surface_identity
    }

    pub fn binding_generation(self) -> UiSurfaceBindingGeneration {
        self.binding_generation
    }

    pub fn capability_observation_generation(self) -> WorthUiHostCapabilityObservationGeneration {
        self.capability_observation_generation
    }

    pub fn capability_profile_digest(self) -> u64 {
        self.capability_profile_digest
    }

    pub fn presentation_mode(self) -> UiHostSurfacePresentationMode {
        self.presentation_mode
    }

    pub fn profile(self) -> UiSurfaceBindingProfile {
        self.profile
    }

    pub fn baseline(self) -> UiHostSurfaceBaselineIdentity {
        self.baseline
    }
}

impl UiMountedFrameIdentityView {
    pub(crate) fn new(
        frame_identity: UiMountedFrameIdentity,
        mounted_instance_identity: UiMountedInstanceIdentity,
        node_receipt_identity: UiMountedNodeReceiptIdentity,
    ) -> Self {
        Self {
            frame_identity,
            mounted_instance_identity,
            node_receipt_identity,
        }
    }

    pub fn frame_identity(self) -> UiMountedFrameIdentity {
        self.frame_identity
    }

    pub fn mounted_instance_identity(self) -> UiMountedInstanceIdentity {
        self.mounted_instance_identity
    }

    pub fn node_receipt_identity(self) -> UiMountedNodeReceiptIdentity {
        self.node_receipt_identity
    }
}

impl UiMountedIdentityView {
    pub(crate) fn new(
        mounted_instances: Vec<UiMountedInstanceIdentityView>,
        surface_bindings: Vec<UiSurfaceBindingIdentityView>,
        current_frame: Option<UiMountedFrameIdentity>,
        frame_receipts: Vec<UiMountedFrameIdentityView>,
    ) -> Self {
        Self {
            mounted_instances: mounted_instances.into_boxed_slice(),
            surface_bindings: surface_bindings.into_boxed_slice(),
            current_frame,
            frame_receipts: frame_receipts.into_boxed_slice(),
        }
    }

    pub fn mounted_instances(&self) -> &[UiMountedInstanceIdentityView] {
        &self.mounted_instances
    }

    pub fn surface_bindings(&self) -> &[UiSurfaceBindingIdentityView] {
        &self.surface_bindings
    }

    pub fn current_frame(&self) -> Option<UiMountedFrameIdentity> {
        self.current_frame
    }

    pub fn frame_receipts(&self) -> &[UiMountedFrameIdentityView] {
        &self.frame_receipts
    }
}
