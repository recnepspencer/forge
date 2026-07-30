use super::TierLayoutTraversal;
use worth_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use worth_store_reclaim_policy::ReclaimPolicyCounterSnapshot;

use crate::ColdTierIoPosture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierPlacementInterferencePosture {
    ColdTierMovementPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierPlacementAccessBudget {
    reclaim_permits: u32,
    region_bytes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TierPlacementLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: TierLayoutTraversal,
    rebuild_posture: DurableArtifactRebuildPosture,
    interference_posture: TierPlacementInterferencePosture,
    posture: ColdTierIoPosture,
}

impl TierPlacementLayoutReport {
    fn from_posture(posture: &ColdTierIoPosture) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::TierPlacementManifest,
            access_shape: TierLayoutTraversal::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
            interference_posture: TierPlacementInterferencePosture::ColdTierMovementPosture,
            posture: posture.clone(),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> TierLayoutTraversal {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn interference_posture(&self) -> TierPlacementInterferencePosture {
        self.interference_posture
    }

    pub fn declared_budget(&self) -> TierPlacementAccessBudget {
        TierPlacementAccessBudget {
            reclaim_permits: self.posture.reclaim_permit().permits(),
            region_bytes: self.posture.reclaim_region().byte_len(),
        }
    }

    pub fn exact_counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.posture.counters()
    }

    pub fn security_scope(&self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.posture.security_scope()
    }

    pub fn interpretation(&self) -> worth_store_physical_format::ReclaimedByteInterpretation {
        self.posture.interpretation()
    }

    pub fn reclaim_region(&self) -> worth_store_physical_format::PhysicalReclaimRegion {
        self.posture.reclaim_region()
    }
}

impl TierPlacementAccessBudget {
    pub const fn reclaim_permits(&self) -> u32 {
        self.reclaim_permits
    }

    pub const fn region_bytes(&self) -> u32 {
        self.region_bytes
    }
}

impl ColdTierIoPosture {
    pub fn project_tier_placement_layout(&self) -> TierPlacementLayoutReport {
        TierPlacementLayoutReport::from_posture(self)
    }
}
