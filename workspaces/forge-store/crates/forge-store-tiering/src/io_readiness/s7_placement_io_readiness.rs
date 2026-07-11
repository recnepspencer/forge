use forge_store_contracts::S7PlacementReadinessNonClaim;
use forge_store_io_scheduler::S7PlacementIoReadinessHandoff;

use crate::cold_tier_posture::S6ColdTierIoPosture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7PlacementIoReadinessSeed {
    handoff: S7PlacementIoReadinessHandoff,
    cold_tier_posture: S6ColdTierIoPosture,
}

pub fn admit_s7_placement_io_readiness_seed(
    handoff: S7PlacementIoReadinessHandoff,
    cold_tier_posture: S6ColdTierIoPosture,
) -> S7PlacementIoReadinessSeed {
    S7PlacementIoReadinessSeed {
        handoff,
        cold_tier_posture,
    }
}

impl S7PlacementIoReadinessSeed {
    pub const fn handoff(&self) -> &S7PlacementIoReadinessHandoff {
        &self.handoff
    }

    pub const fn cold_tier_posture(&self) -> &S6ColdTierIoPosture {
        &self.cold_tier_posture
    }

    pub const fn non_claims(&self) -> &[S7PlacementReadinessNonClaim; 3] {
        self.handoff.non_claims()
    }

    pub const fn carries_blob_lifecycle_claim(&self) -> bool {
        false
    }

    pub const fn carries_placement_policy_claim(&self) -> bool {
        false
    }
}
