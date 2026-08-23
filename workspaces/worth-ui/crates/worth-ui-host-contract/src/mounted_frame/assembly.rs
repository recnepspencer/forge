use crate::{
    UiHostSurfaceBaselineIdentity, UiHostSurfaceIdentity, UiHostSurfacePresentationMode,
    UiMountedFrameIdentity, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
    WorthUiHostCapabilityObservationGeneration,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiMountedLaneParticipation {
    Ordinary,
    Virtualized,
    CanvasSpatial,
    Realtime,
    Preview,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRequiredLaneContributionStatus {
    Admitted,
    ExplicitEmpty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRequiredLaneContribution {
    surface: UiSemanticSurfaceIdentity,
    lane: UiMountedLaneParticipation,
    status: UiRequiredLaneContributionStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedSurfaceBindingRequirement {
    semantic_surface: UiSemanticSurfaceIdentity,
    host_surface: UiHostSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    capability_generation: WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    presentation_mode: UiHostSurfacePresentationMode,
    baseline: UiHostSurfaceBaselineIdentity,
    device_scale_milli: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedFrameManifest {
    surfaces: Box<[UiMountedSurfaceBindingRequirement]>,
    lane_contributions: Box<[UiRequiredLaneContribution]>,
}

impl UiRequiredLaneContribution {
    pub fn new(
        surface: UiSemanticSurfaceIdentity,
        lane: UiMountedLaneParticipation,
        status: UiRequiredLaneContributionStatus,
    ) -> Self {
        Self {
            surface,
            lane,
            status,
        }
    }

    pub fn surface(self) -> UiSemanticSurfaceIdentity {
        self.surface
    }

    pub fn lane(self) -> UiMountedLaneParticipation {
        self.lane
    }

    pub fn status(self) -> UiRequiredLaneContributionStatus {
        self.status
    }
}

impl UiMountedSurfaceBindingRequirement {
    pub fn new(
        semantic_surface: UiSemanticSurfaceIdentity,
        host_surface: UiHostSurfaceIdentity,
        binding: UiSurfaceBindingGeneration,
        capability_generation: WorthUiHostCapabilityObservationGeneration,
        capability_profile_digest: u64,
        presentation_mode: UiHostSurfacePresentationMode,
    ) -> Self {
        let baseline = UiHostSurfaceBaselineIdentity::from_surface_binding(
            semantic_surface,
            host_surface,
            binding,
            capability_generation,
            capability_profile_digest,
            presentation_mode,
        );
        Self::with_baseline(
            semantic_surface,
            host_surface,
            binding,
            capability_generation,
            capability_profile_digest,
            presentation_mode,
            baseline,
        )
    }

    #[doc(hidden)]
    pub fn with_baseline(
        semantic_surface: UiSemanticSurfaceIdentity,
        host_surface: UiHostSurfaceIdentity,
        binding: UiSurfaceBindingGeneration,
        capability_generation: WorthUiHostCapabilityObservationGeneration,
        capability_profile_digest: u64,
        presentation_mode: UiHostSurfacePresentationMode,
        baseline: UiHostSurfaceBaselineIdentity,
    ) -> Self {
        Self::with_baseline_and_device_scale(
            semantic_surface,
            host_surface,
            binding,
            capability_generation,
            capability_profile_digest,
            presentation_mode,
            baseline,
            1_000,
        )
    }

    #[doc(hidden)]
    pub fn with_baseline_and_device_scale(
        semantic_surface: UiSemanticSurfaceIdentity,
        host_surface: UiHostSurfaceIdentity,
        binding: UiSurfaceBindingGeneration,
        capability_generation: WorthUiHostCapabilityObservationGeneration,
        capability_profile_digest: u64,
        presentation_mode: UiHostSurfacePresentationMode,
        baseline: UiHostSurfaceBaselineIdentity,
        device_scale_milli: u32,
    ) -> Self {
        assert!(device_scale_milli != 0, "mounted device scale is nonzero");
        Self {
            semantic_surface,
            host_surface,
            binding,
            capability_generation,
            capability_profile_digest,
            presentation_mode,
            baseline,
            device_scale_milli,
        }
    }

    pub fn semantic_surface(self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface
    }

    pub fn host_surface(self) -> UiHostSurfaceIdentity {
        self.host_surface
    }

    pub fn binding(self) -> UiSurfaceBindingGeneration {
        self.binding
    }

    pub fn capability_generation(self) -> WorthUiHostCapabilityObservationGeneration {
        self.capability_generation
    }

    pub fn capability_profile_digest(self) -> u64 {
        self.capability_profile_digest
    }

    pub fn presentation_mode(self) -> UiHostSurfacePresentationMode {
        self.presentation_mode
    }

    pub fn baseline(self) -> UiHostSurfaceBaselineIdentity {
        self.baseline
    }

    pub fn device_scale_milli(self) -> u32 {
        self.device_scale_milli
    }
}

impl UiMountedFrameManifest {
    pub fn new(
        mut surfaces: Vec<UiMountedSurfaceBindingRequirement>,
        mut lane_contributions: Vec<UiRequiredLaneContribution>,
    ) -> Self {
        surfaces.sort_by_key(|requirement| requirement.semantic_surface);
        lane_contributions.sort_by_key(|cell| (cell.surface, cell.lane));
        Self {
            surfaces: surfaces.into_boxed_slice(),
            lane_contributions: lane_contributions.into_boxed_slice(),
        }
    }

    pub fn surfaces(&self) -> &[UiMountedSurfaceBindingRequirement] {
        &self.surfaces
    }

    pub fn lane_contributions(&self) -> &[UiRequiredLaneContribution] {
        &self.lane_contributions
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedFrameCanonicalCore {
    frame: UiMountedFrameIdentity,
    plan_digest: u64,
    graph_world: u64,
    allocation_truth_revision: u64,
    table_range_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedFrameIntegrity {
    value: u64,
}

impl UiMountedFrameCanonicalCore {
    pub fn new(
        frame: UiMountedFrameIdentity,
        plan_digest: u64,
        graph_world: u64,
        allocation_truth_revision: u64,
        table_range_digest: u64,
    ) -> Self {
        Self {
            frame,
            plan_digest,
            graph_world,
            allocation_truth_revision,
            table_range_digest,
        }
    }

    pub fn frame(self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub fn plan_digest(self) -> u64 {
        self.plan_digest
    }

    pub fn graph_world(self) -> u64 {
        self.graph_world
    }

    pub fn allocation_truth_revision(self) -> u64 {
        self.allocation_truth_revision
    }

    pub fn table_range_digest(self) -> u64 {
        self.table_range_digest
    }
}

impl UiMountedFrameIntegrity {
    pub fn derive(core: UiMountedFrameCanonicalCore, manifest: &UiMountedFrameManifest) -> Self {
        let mut value = core.frame.diagnostic_value()
            ^ core.plan_digest.rotate_left(7)
            ^ core.graph_world.rotate_left(19)
            ^ core.allocation_truth_revision.rotate_left(31)
            ^ core.table_range_digest.rotate_left(43);
        for surface in manifest.surfaces() {
            value = value.rotate_left(5)
                ^ surface.semantic_surface.diagnostic_value()
                ^ surface.host_surface.diagnostic_value().rotate_left(11)
                ^ surface.binding.diagnostic_value().rotate_left(23)
                ^ surface.capability_generation.as_u64().rotate_left(37)
                ^ surface.capability_profile_digest
                ^ presentation_mode_tag(surface.presentation_mode).rotate_left(47)
                ^ u64::from(surface.device_scale_milli).rotate_left(59);
        }
        for cell in manifest.lane_contributions() {
            value = value.rotate_left(3)
                ^ cell.surface.diagnostic_value()
                ^ lane_tag(cell.lane)
                ^ status_tag(cell.status);
        }
        Self { value }
    }

    pub fn diagnostic_value(self) -> u64 {
        self.value
    }

    pub fn verifies(
        self,
        core: UiMountedFrameCanonicalCore,
        manifest: &UiMountedFrameManifest,
    ) -> bool {
        self == Self::derive(core, manifest)
    }
}

fn lane_tag(lane: UiMountedLaneParticipation) -> u64 {
    match lane {
        UiMountedLaneParticipation::Ordinary => 1,
        UiMountedLaneParticipation::Virtualized => 2,
        UiMountedLaneParticipation::CanvasSpatial => 3,
        UiMountedLaneParticipation::Realtime => 4,
        UiMountedLaneParticipation::Preview => 5,
    }
}

fn status_tag(status: UiRequiredLaneContributionStatus) -> u64 {
    match status {
        UiRequiredLaneContributionStatus::Admitted => 11,
        UiRequiredLaneContributionStatus::ExplicitEmpty => 17,
    }
}

fn presentation_mode_tag(mode: UiHostSurfacePresentationMode) -> u64 {
    match mode {
        UiHostSurfacePresentationMode::NativeDisplay => 23,
        UiHostSurfacePresentationMode::RecordOnly => 29,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrity_changes_with_capability_generation_and_presentation_mode() {
        let core = UiMountedFrameCanonicalCore::new(
            UiMountedFrameIdentity::mint_unbound().unwrap(),
            11,
            13,
            17,
            19,
        );
        let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
        let host = UiHostSurfaceIdentity::mint_unbound().unwrap();
        let binding = UiSurfaceBindingGeneration::mint_unbound().unwrap();
        let baseline = manifest(
            surface,
            host,
            binding,
            WorthUiHostCapabilityObservationGeneration::new(23),
            UiHostSurfacePresentationMode::RecordOnly,
        );
        let changed_generation = manifest(
            surface,
            host,
            binding,
            WorthUiHostCapabilityObservationGeneration::new(29),
            UiHostSurfacePresentationMode::RecordOnly,
        );
        let changed_mode = manifest(
            surface,
            host,
            binding,
            WorthUiHostCapabilityObservationGeneration::new(23),
            UiHostSurfacePresentationMode::NativeDisplay,
        );

        let integrity = UiMountedFrameIntegrity::derive(core, &baseline);
        assert_ne!(
            integrity,
            UiMountedFrameIntegrity::derive(core, &changed_generation)
        );
        assert_ne!(
            integrity,
            UiMountedFrameIntegrity::derive(core, &changed_mode)
        );
    }

    fn manifest(
        surface: UiSemanticSurfaceIdentity,
        host: UiHostSurfaceIdentity,
        binding: UiSurfaceBindingGeneration,
        capability_generation: WorthUiHostCapabilityObservationGeneration,
        presentation_mode: UiHostSurfacePresentationMode,
    ) -> UiMountedFrameManifest {
        UiMountedFrameManifest::new(
            vec![UiMountedSurfaceBindingRequirement::new(
                surface,
                host,
                binding,
                capability_generation,
                31,
                presentation_mode,
            )],
            Vec::new(),
        )
    }
}
