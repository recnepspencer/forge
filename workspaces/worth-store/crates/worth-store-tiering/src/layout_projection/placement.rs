use super::TierLayoutTraversal;
use worth_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use worth_store_io_scheduler::IoSchedulerIsolationCounterSnapshot;

use crate::TierPlacementIoAdmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierPlacementInterferencePosture {
    PublishedSchedulerCounters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierPlacementAccessBudget {
    reclaim_permits: u32,
    blocked_maintenance_count: u64,
    protected_byte_footprint: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TierPlacementLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: TierLayoutTraversal,
    rebuild_posture: DurableArtifactRebuildPosture,
    interference_posture: TierPlacementInterferencePosture,
    admission: TierPlacementIoAdmission,
}

impl TierPlacementLayoutReport {
    fn from_admission(admission: &TierPlacementIoAdmission) -> Self {
        TierPlacementLayoutReport {
            family_id: DurableArtifactFamilyId::TierPlacementManifest,
            access_shape: TierLayoutTraversal::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
            interference_posture: TierPlacementInterferencePosture::PublishedSchedulerCounters,
            admission: admission.clone(),
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
            reclaim_permits: self
                .admission
                .cold_tier_posture()
                .reclaim_permit()
                .permits(),
            blocked_maintenance_count: self
                .admission
                .scheduler()
                .background_maintenance()
                .blocked_maintenance_count(),
            protected_byte_footprint: self
                .admission
                .scheduler()
                .counters()
                .protected_byte_footprint(),
        }
    }

    pub const fn exact_counters(&self) -> IoSchedulerIsolationCounterSnapshot {
        self.admission.scheduler().counters()
    }

    pub fn security_scope(&self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.admission.cold_tier_posture().security_scope()
    }

    pub fn interpretation(&self) -> worth_store_physical_format::ReclaimedByteInterpretation {
        self.admission.cold_tier_posture().interpretation()
    }

    pub fn reclaim_region(&self) -> worth_store_physical_format::PhysicalReclaimRegion {
        self.admission.cold_tier_posture().reclaim_region()
    }
}

impl TierPlacementAccessBudget {
    pub const fn reclaim_permits(&self) -> u32 {
        self.reclaim_permits
    }

    pub const fn blocked_maintenance_count(&self) -> u64 {
        self.blocked_maintenance_count
    }

    pub const fn protected_byte_footprint(&self) -> u64 {
        self.protected_byte_footprint
    }
}

impl TierPlacementIoAdmission {
    pub fn project_tier_placement_layout(&self) -> TierPlacementLayoutReport {
        TierPlacementLayoutReport::from_admission(self)
    }
}
