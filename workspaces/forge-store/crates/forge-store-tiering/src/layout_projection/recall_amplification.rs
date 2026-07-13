use super::TierLayoutTraversal;
use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_reclaim_policy::ReclaimPolicyCounterSnapshot;

use crate::ColdTierIoPosture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecallAmplificationInterferencePosture {
    ColdTierReadAmplification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecallAmplificationAccessBudget {
    reclaim_permits: u32,
    region_bytes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecallAmplificationLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: TierLayoutTraversal,
    rebuild_posture: DurableArtifactRebuildPosture,
    interference_posture: RecallAmplificationInterferencePosture,
    posture: ColdTierIoPosture,
}

impl RecallAmplificationLayoutReport {
    fn from_posture(posture: &ColdTierIoPosture) -> Self {
        RecallAmplificationLayoutReport {
            family_id: DurableArtifactFamilyId::RecallAmplificationIndex,
            access_shape: TierLayoutTraversal::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::PartialRebuildOnly,
            interference_posture: RecallAmplificationInterferencePosture::ColdTierReadAmplification,
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

    pub const fn interference_posture(&self) -> RecallAmplificationInterferencePosture {
        self.interference_posture
    }

    pub fn declared_budget(&self) -> RecallAmplificationAccessBudget {
        RecallAmplificationAccessBudget {
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

impl RecallAmplificationAccessBudget {
    pub const fn reclaim_permits(&self) -> u32 {
        self.reclaim_permits
    }

    pub const fn region_bytes(&self) -> u32 {
        self.region_bytes
    }
}

impl ColdTierIoPosture {
    pub fn project_recall_amplification_layout(&self) -> RecallAmplificationLayoutReport {
        RecallAmplificationLayoutReport::from_posture(self)
    }
}
