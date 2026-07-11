use forge_store_budgets::CounterEvidenceStrength;
use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase25_reachability_rule, AdmittedReachabilityLayoutRule,
};
use forge_store_security::StoreSecurityScopeIdentity;

use super::behavior::{
    corruption_behavior_for, declared_rebuild_posture, BlobLayoutCorruptionBehavior,
    BlobLayoutScopeSafeAbsenceBehavior,
};
use super::{BlobLayoutAccessDenial, BlobLayoutAccessDenialKind, BlobLayoutAccessPathEvidence};
use crate::{BlobChunkReachabilityProofSet, StoredChunkDigest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReachabilityLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReachabilityLayoutAdmission {
    _rule: AdmittedReachabilityLayoutRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedReachabilityLayoutFamily {
    _admission: ReachabilityLayoutAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReachabilityLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    absence_behavior: BlobLayoutScopeSafeAbsenceBehavior,
    corruption_behavior: BlobLayoutCorruptionBehavior,
    stored_digest: StoredChunkDigest,
    security_scope: StoreSecurityScopeIdentity,
    reachable_chunks: u64,
    reference_edges: u64,
    protected_holds: u64,
    orphan_candidates: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl ReachabilityLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(self, rule: AdmittedReachabilityLayoutRule) -> ReachabilityLayoutAdmission {
        let _ = self;
        ReachabilityLayoutAdmission { _rule: rule }
    }
}

fn reachability_layout() -> AdmittedReachabilityLayoutFamily {
    AdmittedReachabilityLayoutFamily {
        _admission: ReachabilityLayoutFamilyHome::s8().admit(
            phase25_reachability_rule().expect("phase 25 reachability rule must stay admitted"),
        ),
    }
}

impl AdmittedReachabilityLayoutFamily {
    fn admit_reachability(
        &self,
        proof: &BlobChunkReachabilityProofSet,
    ) -> Result<ReachabilityLayoutReport, BlobLayoutAccessDenial> {
        let _ = self;
        if proof.reachable_chunks().is_empty() {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::EmptyReachabilityProofCannotStandInForReachabilityLayoutAuthority,
            ));
        }
        Ok(ReachabilityLayoutReport::from_proof(proof))
    }
}

impl ReachabilityLayoutReport {
    fn from_proof(proof: &BlobChunkReachabilityProofSet) -> Self {
        let family_id = DurableArtifactFamilyId::ReachabilityEdge;
        let rebuild_posture = declared_rebuild_posture(family_id);
        Self {
            family_id,
            access_shape: S8AccessShape::BoundedScan,
            rebuild_posture,
            absence_behavior: BlobLayoutScopeSafeAbsenceBehavior::ScopedMaintenanceScan,
            corruption_behavior: corruption_behavior_for(rebuild_posture),
            stored_digest: proof.stored_digest().clone(),
            security_scope: proof.security_metadata().identity(),
            reachable_chunks: proof.reachable_chunks().len() as u64,
            reference_edges: proof.reference_edges().len() as u64,
            protected_holds: proof.protected_holds().len() as u64,
            orphan_candidates: proof.orphan_candidates().len() as u64,
            counter_evidence: BlobLayoutAccessPathEvidence::from_reachability(
                family_id,
                proof.counters(),
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

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub const fn reachable_chunks(&self) -> u64 {
        self.reachable_chunks
    }

    pub const fn reference_edges(&self) -> u64 {
        self.reference_edges
    }

    pub const fn protected_holds(&self) -> u64 {
        self.protected_holds
    }

    pub const fn orphan_candidates(&self) -> u64 {
        self.orphan_candidates
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }

    pub fn requires_exact_counter_evidence(&self) -> bool {
        self.counter_evidence.strength() == CounterEvidenceStrength::Exact
    }
}

impl BlobChunkReachabilityProofSet {
    pub fn admit_reachability_layout(
        &self,
    ) -> Result<ReachabilityLayoutReport, BlobLayoutAccessDenial> {
        reachability_layout().admit_reachability(self)
    }
}
