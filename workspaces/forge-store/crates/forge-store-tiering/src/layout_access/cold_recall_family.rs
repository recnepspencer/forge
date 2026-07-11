use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase26_cold_recall_rule, AdmittedColdRecallLayoutRule,
};
use forge_store_reclaim_policy::ReclaimPolicyCounterSnapshot;

use crate::ColdTierIoPosture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColdRecallLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColdRecallLayoutAdmission {
    _rule: AdmittedColdRecallLayoutRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedColdRecallLayoutFamily {
    _admission: ColdRecallLayoutAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColdRecallInterferencePosture {
    ColdTierMovementPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColdRecallAccessBudget {
    reclaim_permits: u32,
    region_bytes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColdRecallLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    interference_posture: ColdRecallInterferencePosture,
    posture: ColdTierIoPosture,
}

impl ColdRecallLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(self, rule: AdmittedColdRecallLayoutRule) -> ColdRecallLayoutAdmission {
        let _ = self;
        ColdRecallLayoutAdmission { _rule: rule }
    }
}

fn cold_recall_layout() -> AdmittedColdRecallLayoutFamily {
    AdmittedColdRecallLayoutFamily {
        _admission: ColdRecallLayoutFamilyHome::s8().admit(
            phase26_cold_recall_rule().expect("phase 26 cold recall rule must stay admitted"),
        ),
    }
}

impl AdmittedColdRecallLayoutFamily {
    fn admit_cold_recall(&self, posture: &ColdTierIoPosture) -> ColdRecallLayoutReport {
        let _ = self;
        ColdRecallLayoutReport {
            family_id: DurableArtifactFamilyId::ColdRecallQueue,
            access_shape: S8AccessShape::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::PartialRebuildOnly,
            interference_posture: ColdRecallInterferencePosture::ColdTierMovementPosture,
            posture: posture.clone(),
        }
    }
}

impl ColdRecallLayoutReport {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> S8AccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn interference_posture(&self) -> ColdRecallInterferencePosture {
        self.interference_posture
    }

    pub fn declared_budget(&self) -> ColdRecallAccessBudget {
        ColdRecallAccessBudget {
            reclaim_permits: self.posture.reclaim_permit().permits(),
            region_bytes: self.posture.reclaim_region().byte_len(),
        }
    }

    pub fn security_scope(&self) -> forge_store_security::StoreSecurityScopeIdentity {
        self.posture.security_scope()
    }

    pub fn interpretation(&self) -> forge_store_physical_format::ReclaimedByteInterpretation {
        self.posture.interpretation()
    }

    pub fn exact_counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.posture.counters()
    }
}

impl ColdRecallAccessBudget {
    pub const fn reclaim_permits(&self) -> u32 {
        self.reclaim_permits
    }

    pub const fn region_bytes(&self) -> u32 {
        self.region_bytes
    }
}

impl ColdTierIoPosture {
    pub fn admit_cold_recall_layout(&self) -> ColdRecallLayoutReport {
        cold_recall_layout().admit_cold_recall(self)
    }
}
