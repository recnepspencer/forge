use worth_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use worth_store_retention::RetentionDisposition;
use worth_store_security::StoreSecurityScopeIdentity;

use super::behavior::{
    corruption_behavior_for, declared_rebuild_posture, BlobLayoutCorruptionBehavior,
    BlobLayoutScopeSafeAbsenceBehavior,
};
use super::{
    BlobLayoutAccessDenial, BlobLayoutAccessDenialKind, BlobLayoutAccessPathEvidence,
    BlobLayoutAccessShape,
};
use crate::{BlobChunkReachabilityProofSet, BlobRetentionReclaimPermit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetentionLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: BlobLayoutAccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    absence_behavior: BlobLayoutScopeSafeAbsenceBehavior,
    corruption_behavior: BlobLayoutCorruptionBehavior,
    security_scope: StoreSecurityScopeIdentity,
    protected_holds: u64,
    retained_chunks: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl RetentionLayoutReport {
    fn project_retention(
        proof: &BlobChunkReachabilityProofSet,
    ) -> Result<RetentionLayoutReport, BlobLayoutAccessDenial> {
        if proof.protected_holds().is_empty() {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::RetentionLayoutRequiresProtectedHoldEvidence,
            ));
        }
        Ok(RetentionLayoutReport::from_proof(proof))
    }
}

impl RetentionLayoutReport {
    fn from_proof(proof: &BlobChunkReachabilityProofSet) -> Self {
        let family_id = DurableArtifactFamilyId::RetentionHold;
        let rebuild_posture = declared_rebuild_posture(family_id);
        Self {
            family_id,
            access_shape: BlobLayoutAccessShape::BoundedScan,
            rebuild_posture,
            absence_behavior: BlobLayoutScopeSafeAbsenceBehavior::ScopedMaintenanceScan,
            corruption_behavior: corruption_behavior_for(rebuild_posture),
            security_scope: proof.security_metadata().identity(),
            protected_holds: proof.protected_holds().len() as u64,
            retained_chunks: proof.reachable_chunks().len() as u64,
            counter_evidence: BlobLayoutAccessPathEvidence::from_reachability(
                family_id,
                proof.counters(),
            ),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> BlobLayoutAccessShape {
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

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub const fn protected_holds(&self) -> u64 {
        self.protected_holds
    }

    pub const fn retained_chunks(&self) -> u64 {
        self.retained_chunks
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }

    pub const fn requires_rebuild_parity(&self) -> bool {
        !matches!(
            self.rebuild_posture,
            DurableArtifactRebuildPosture::NoRebuild
        )
    }
}

impl BlobChunkReachabilityProofSet {
    pub fn project_retention_layout(
        &self,
    ) -> Result<RetentionLayoutReport, BlobLayoutAccessDenial> {
        RetentionLayoutReport::project_retention(self)
    }
}

impl BlobRetentionReclaimPermit {
    pub fn project_retention_layout(
        &self,
        _disposition: RetentionDisposition,
    ) -> Result<RetentionLayoutReport, BlobLayoutAccessDenial> {
        Err(BlobLayoutAccessDenial::new(
            BlobLayoutAccessDenialKind::ReclaimReceiptCannotStandInForRetentionLayoutAuthority,
        ))
    }
}
