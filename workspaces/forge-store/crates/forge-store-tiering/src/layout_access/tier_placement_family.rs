use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_io_scheduler::IoSchedulerS6CounterSnapshot;
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase26_tier_placement_rule, AdmittedTierPlacementLayoutRule,
};

use crate::S7PlacementIoReadinessSeed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TierPlacementLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TierPlacementLayoutAdmission {
    _rule: AdmittedTierPlacementLayoutRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedTierPlacementLayoutFamily {
    _admission: TierPlacementLayoutAdmission,
}

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
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    interference_posture: TierPlacementInterferencePosture,
    seed: S7PlacementIoReadinessSeed,
}

impl TierPlacementLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(self, rule: AdmittedTierPlacementLayoutRule) -> TierPlacementLayoutAdmission {
        let _ = self;
        TierPlacementLayoutAdmission { _rule: rule }
    }
}

fn tier_placement_layout() -> AdmittedTierPlacementLayoutFamily {
    AdmittedTierPlacementLayoutFamily {
        _admission: TierPlacementLayoutFamilyHome::s8().admit(
            phase26_tier_placement_rule().expect("phase 26 tier placement rule must stay admitted"),
        ),
    }
}

impl AdmittedTierPlacementLayoutFamily {
    fn admit_tier_placement(&self, seed: &S7PlacementIoReadinessSeed) -> TierPlacementLayoutReport {
        let _ = self;
        TierPlacementLayoutReport {
            family_id: DurableArtifactFamilyId::TierPlacementManifest,
            access_shape: S8AccessShape::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
            interference_posture: TierPlacementInterferencePosture::PublishedSchedulerCounters,
            seed: seed.clone(),
        }
    }
}

impl TierPlacementLayoutReport {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> S8AccessShape {
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
            reclaim_permits: self.seed.cold_tier_posture().reclaim_permit().permits(),
            blocked_maintenance_count: self
                .seed
                .handoff()
                .background_maintenance()
                .blocked_maintenance_count(),
            protected_byte_footprint: self.seed.handoff().counters().protected_byte_footprint(),
        }
    }

    pub const fn exact_counters(&self) -> IoSchedulerS6CounterSnapshot {
        self.seed.handoff().counters()
    }

    pub fn security_scope(&self) -> forge_store_security::StoreSecurityScopeIdentity {
        self.seed.cold_tier_posture().security_scope()
    }

    pub fn interpretation(&self) -> forge_store_physical_format::ReclaimedByteInterpretation {
        self.seed.cold_tier_posture().interpretation()
    }

    pub fn reclaim_region(&self) -> forge_store_physical_format::PhysicalReclaimRegion {
        self.seed.cold_tier_posture().reclaim_region()
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

impl S7PlacementIoReadinessSeed {
    pub fn admit_tier_placement_layout(&self) -> TierPlacementLayoutReport {
        tier_placement_layout().admit_tier_placement(self)
    }
}
