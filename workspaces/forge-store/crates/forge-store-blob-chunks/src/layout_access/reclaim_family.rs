use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture, StableDigest};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_physical_format::{PhysicalReclaimRegion, ReclaimedByteInterpretation};
use forge_store_security::StoreSecurityScopeIdentity;

use super::behavior::{
    corruption_behavior_for, declared_rebuild_posture, BlobLayoutCorruptionBehavior,
    BlobLayoutScopeSafeAbsenceBehavior,
};
use super::{BlobLayoutAccessDenial, BlobLayoutAccessDenialKind, BlobLayoutAccessPathEvidence};
use crate::{BlobChunkIdentity, BlobRetentionReclaimPermit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    absence_behavior: BlobLayoutScopeSafeAbsenceBehavior,
    corruption_behavior: BlobLayoutCorruptionBehavior,
    permit_identity: StableDigest,
    chunk_identity: BlobChunkIdentity,
    security_scope: StoreSecurityScopeIdentity,
    reclaim_region: PhysicalReclaimRegion,
    observed_interpretation: ReclaimedByteInterpretation,
    released_edges: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl ReclaimLayoutReport {
    fn admit_reclaim(
        permit: &BlobRetentionReclaimPermit,
    ) -> Result<ReclaimLayoutReport, BlobLayoutAccessDenial> {
        if permit.reclaim_release().released_edges().is_empty() {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::ReclaimLayoutRequiresReachabilityBoundPolicyExecution,
            ));
        }
        Ok(ReclaimLayoutReport::from_permit(permit))
    }
}

impl ReclaimLayoutReport {
    fn from_permit(permit: &BlobRetentionReclaimPermit) -> Self {
        let family_id = DurableArtifactFamilyId::ReclaimReceipt;
        let rebuild_posture = declared_rebuild_posture(family_id);
        let receipt = permit.s6_posture().receipt();
        Self {
            family_id,
            access_shape: S8AccessShape::BoundedScan,
            rebuild_posture,
            absence_behavior: BlobLayoutScopeSafeAbsenceBehavior::ScopedMaintenanceScan,
            corruption_behavior: corruption_behavior_for(rebuild_posture),
            permit_identity: permit.identity().clone(),
            chunk_identity: permit.chunk_identity().clone(),
            security_scope: permit.s6_posture().security_scope(),
            reclaim_region: receipt.policy().region(),
            observed_interpretation: receipt.observed_interpretation(),
            released_edges: permit.reclaim_release().released_edges().len() as u64,
            counter_evidence: BlobLayoutAccessPathEvidence::from_reclaim_policy(
                family_id,
                receipt.counters(),
            ),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> S8AccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn absence_behavior(&self) -> BlobLayoutScopeSafeAbsenceBehavior {
        self.absence_behavior
    }

    pub const fn corruption_behavior(&self) -> BlobLayoutCorruptionBehavior {
        self.corruption_behavior
    }

    pub const fn permit_identity(&self) -> &StableDigest {
        &self.permit_identity
    }

    pub const fn chunk_identity(&self) -> &BlobChunkIdentity {
        &self.chunk_identity
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub const fn reclaim_region(&self) -> PhysicalReclaimRegion {
        self.reclaim_region
    }

    pub const fn observed_interpretation(&self) -> ReclaimedByteInterpretation {
        self.observed_interpretation
    }

    pub const fn released_edges(&self) -> u64 {
        self.released_edges
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }

    pub fn proves_scope_safe_absence_for_chunk(
        &self,
        chunk_identity: &BlobChunkIdentity,
        scope: StoreSecurityScopeIdentity,
    ) -> Result<(), BlobLayoutAccessDenial> {
        if &self.chunk_identity != chunk_identity
            || self.security_scope != scope
            || self.released_edges == 0
        {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::ScopeSafeAbsenceRequiresReclaimReleaseMatch,
            ));
        }
        Ok(())
    }
}

impl BlobRetentionReclaimPermit {
    pub fn admit_reclaim_layout(&self) -> Result<ReclaimLayoutReport, BlobLayoutAccessDenial> {
        ReclaimLayoutReport::admit_reclaim(self)
    }
}
