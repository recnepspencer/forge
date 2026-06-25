use crate::{RoadmapScope, StoreContractError, StoreContractResult, ROADMAP_2_S1_SCOPE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalAuthorityScope {
    PhysicalFoundationVocabulary,
    PhysicalEvidenceExport,
    PhysicalSubstrateReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorePhysicalAuthorityWitness {
    roadmap_scope: RoadmapScope,
    authority_scope: PhysicalAuthorityScope,
}

impl StorePhysicalAuthorityWitness {
    pub fn for_s1_vocabulary(roadmap_scope: RoadmapScope) -> StoreContractResult<Self> {
        Self::admit(
            roadmap_scope,
            PhysicalAuthorityScope::PhysicalFoundationVocabulary,
        )
    }

    pub const fn roadmap_scope(&self) -> RoadmapScope {
        self.roadmap_scope
    }

    pub const fn authority_scope(&self) -> PhysicalAuthorityScope {
        self.authority_scope
    }

    fn admit(
        roadmap_scope: RoadmapScope,
        authority_scope: PhysicalAuthorityScope,
    ) -> StoreContractResult<Self> {
        if roadmap_scope != ROADMAP_2_S1_SCOPE {
            return Err(StoreContractError::PhysicalAuthorityScopeMismatch);
        }
        Ok(Self {
            roadmap_scope,
            authority_scope,
        })
    }
}
