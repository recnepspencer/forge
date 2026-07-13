use crate::{
    RoadmapScope, StoreContractError, StoreContractResult, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    ROADMAP_2_S1_SCOPE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAuthorityBoundaryInstance {
    label: &'static str,
}

impl PhysicalAuthorityBoundaryInstance {
    const fn new(label: &'static str) -> Self {
        Self { label }
    }

    pub const fn label(&self) -> &'static str {
        self.label
    }
}

pub const ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY: PhysicalAuthorityBoundaryInstance =
    PhysicalAuthorityBoundaryInstance::new("roadmap-2.physical.primary");
pub const ROADMAP_2_REPLAY_PHYSICAL_BOUNDARY: PhysicalAuthorityBoundaryInstance =
    PhysicalAuthorityBoundaryInstance::new("roadmap-2.physical.replay");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalAuthorityScope {
    AspectNativeBoundaryVocabulary,
    PhysicalFoundationVocabulary,
    PhysicalEvidenceExport,
    PhysicalSubstrateReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorePhysicalAuthorityWitness {
    roadmap_scope: RoadmapScope,
    authority_scope: PhysicalAuthorityScope,
    boundary_instance: PhysicalAuthorityBoundaryInstance,
}

impl StorePhysicalAuthorityWitness {
    pub fn for_aspect_native_boundary(roadmap_scope: RoadmapScope) -> StoreContractResult<Self> {
        Self::for_aspect_native_boundary_instance(
            roadmap_scope,
            ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY,
        )
    }

    pub fn for_aspect_native_boundary_instance(
        roadmap_scope: RoadmapScope,
        boundary_instance: PhysicalAuthorityBoundaryInstance,
    ) -> StoreContractResult<Self> {
        Self::admit(
            roadmap_scope,
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
            PhysicalAuthorityScope::AspectNativeBoundaryVocabulary,
            boundary_instance,
        )
    }

    pub fn for_physical_format_vocabulary(
        roadmap_scope: RoadmapScope,
    ) -> StoreContractResult<Self> {
        Self::admit(
            roadmap_scope,
            ROADMAP_2_S1_SCOPE,
            PhysicalAuthorityScope::PhysicalFoundationVocabulary,
            ROADMAP_2_PRIMARY_PHYSICAL_BOUNDARY,
        )
    }

    pub const fn roadmap_scope(&self) -> RoadmapScope {
        self.roadmap_scope
    }

    pub const fn authority_scope(&self) -> PhysicalAuthorityScope {
        self.authority_scope
    }

    pub const fn boundary_instance(&self) -> PhysicalAuthorityBoundaryInstance {
        self.boundary_instance
    }

    fn admit(
        roadmap_scope: RoadmapScope,
        expected_scope: RoadmapScope,
        authority_scope: PhysicalAuthorityScope,
        boundary_instance: PhysicalAuthorityBoundaryInstance,
    ) -> StoreContractResult<Self> {
        if roadmap_scope != expected_scope {
            return Err(StoreContractError::PhysicalAuthorityScopeMismatch);
        }
        Ok(Self {
            roadmap_scope,
            authority_scope,
            boundary_instance,
        })
    }
}
