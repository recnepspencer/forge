use forge_store_io_scheduler::IoSchedulerIsolationAdmission;

use crate::cold_tier_posture::ColdTierIoPosture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TierPlacementIoAdmission {
    scheduler: IoSchedulerIsolationAdmission,
    cold_tier_posture: ColdTierIoPosture,
}

pub fn admit_tier_placement_io(
    scheduler: IoSchedulerIsolationAdmission,
    cold_tier_posture: ColdTierIoPosture,
) -> TierPlacementIoAdmission {
    TierPlacementIoAdmission {
        scheduler,
        cold_tier_posture,
    }
}

impl TierPlacementIoAdmission {
    pub const fn scheduler(&self) -> &IoSchedulerIsolationAdmission {
        &self.scheduler
    }

    pub const fn cold_tier_posture(&self) -> &ColdTierIoPosture {
        &self.cold_tier_posture
    }
}
