use forge_store_contracts::StableDigest;
use forge_store_operations::{BackupExportCustodyReadiness, S10BackupExportCustodyHandoff};
use forge_store_physical_isolation::{ReadDuringCheckpointVerdict, StablePhysicalReadPlan};

mod proof;
mod release;

use crate::blob_reachability_edges::BlobReachabilityAuthorityKey;
use crate::blob_reachability_reclaim_release::BlobReachabilityReclaimRelease;
use crate::blob_reachability_snapshot::BlobReachabilityCanonicalSnapshot;
use crate::{
    BlobChunkIdentity, BlobChunkSecurityMetadataWitness, BlobLifecycleDeclaration,
    BlobPublicationIntent, BlobReachabilityCounterSnapshot, BlobReachabilityDenial,
    BlobReachabilityEdge, BlobReachabilityEdgeRelease, BlobReachabilityProtectedHold,
    BlobRetentionHold, ScopedBlobChunk, StoredChunkDigest,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BlobChunkReachabilityRegistry {
    authority: Option<BlobReachabilityAuthorityKey>,
    edges: Vec<BlobReachabilityEdge>,
    holds: Vec<BlobReachabilityProtectedHold>,
    released_edges: Vec<BlobReachabilityEdgeRelease>,
    counters: BlobReachabilityCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkReachabilityProofSet {
    authority: BlobReachabilityAuthorityKey,
    reachable_chunks: Vec<BlobChunkIdentity>,
    stored_digest: StoredChunkDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    reference_edges: Vec<StableDigest>,
    protected_holds: Vec<StableDigest>,
    orphan_candidates: Vec<BlobChunkIdentity>,
    counters: BlobReachabilityCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobReachabilityReclaimDecision {
    ReclaimPermitted(BlobReachabilityReclaimRelease),
    ReclaimDenied(BlobReachabilityDenial),
}

impl BlobChunkReachabilityRegistry {
    pub fn new_store_owned() -> Self {
        Self::default()
    }

    pub fn admit_edge(&mut self, edge: BlobReachabilityEdge) -> Result<(), BlobReachabilityDenial> {
        if let Some(authority) = &self.authority {
            if !authority.matches(&edge) {
                self.counters = self.counters.record_wrong_authority_denial();
                return Err(BlobReachabilityDenial::WrongBlobAuthority {
                    counters: self.counters,
                });
            }
        } else {
            self.authority = Some(edge.authority_key());
        }
        if self
            .edges
            .iter()
            .any(|existing| existing.identity() == edge.identity())
        {
            return Ok(());
        }
        self.counters = self.counters.with_edge(edge.is_dedupe());
        self.edges.push(edge);
        self.sort_edges();
        Ok(())
    }

    pub fn admit_lifecycle_primary_reference(
        &mut self,
        declaration: &BlobLifecycleDeclaration,
        scoped_chunk: ScopedBlobChunk,
    ) -> Result<BlobChunkReachabilityProofSet, BlobReachabilityDenial> {
        let edge = BlobReachabilityEdge::primary_lifecycle_reference(declaration, scoped_chunk)?;
        self.admit_edge(edge)?;
        self.prove_reachable_chunks()
    }

    pub fn admit_hold(
        &mut self,
        hold: BlobReachabilityProtectedHold,
    ) -> Result<(), BlobReachabilityDenial> {
        let hold_authority = hold.authority_key();
        if let Some(authority) = &self.authority {
            if authority != &hold_authority {
                self.counters = self.counters.record_wrong_authority_denial();
                return Err(BlobReachabilityDenial::WrongBlobAuthority {
                    counters: self.counters,
                });
            }
        } else if hold.can_seed_registry_authority() {
            self.authority = Some(hold_authority);
        } else {
            self.counters = self.counters.record_wrong_authority_denial();
            return Err(BlobReachabilityDenial::InvalidProtectedHold {
                counters: self.counters,
            });
        }
        if self
            .holds
            .iter()
            .any(|existing| existing.identity() == hold.identity())
        {
            return Ok(());
        }
        self.counters = self.counters.with_hold();
        self.holds.push(hold);
        self.holds
            .sort_by(|left, right| left.identity().as_str().cmp(right.identity().as_str()));
        Ok(())
    }

    pub fn admit_stable_read_plan_hold(
        &mut self,
        plan: &StablePhysicalReadPlan,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = self.registry_bound_hold_authority()?;
        let hold = BlobReachabilityProtectedHold::from_stable_read_plan(plan, authority);
        self.admit_hold(hold)
    }

    pub fn admit_checkpoint_hold(
        &mut self,
        verdict: &ReadDuringCheckpointVerdict,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = self.registry_bound_hold_authority()?;
        let hold = BlobReachabilityProtectedHold::from_checkpoint_verdict(verdict, authority);
        self.admit_hold(hold)
    }

    pub fn admit_export_hold(
        &mut self,
        readiness: &BackupExportCustodyReadiness,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = self.registry_bound_hold_authority()?;
        let hold = BlobReachabilityProtectedHold::from_export_readiness(readiness, authority)?;
        self.admit_hold(hold)
    }

    pub fn admit_s10_backup_hold(
        &mut self,
        handoff: &S10BackupExportCustodyHandoff,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = self.registry_bound_hold_authority()?;
        let hold = BlobReachabilityProtectedHold::from_s10_backup_handoff(handoff, authority)?;
        self.admit_hold(hold)
    }

    pub fn admit_retention_hold(
        &mut self,
        hold: &BlobRetentionHold,
    ) -> Result<(), BlobReachabilityDenial> {
        let authority = self.registry_bound_hold_authority()?;
        let hold = BlobReachabilityProtectedHold::from_retention_hold(hold, authority);
        self.admit_hold(hold)
    }

    fn registry_bound_hold_authority(
        &mut self,
    ) -> Result<BlobReachabilityAuthorityKey, BlobReachabilityDenial> {
        let Some(authority) = self.authority.clone() else {
            self.counters = self.counters.record_wrong_authority_denial();
            return Err(BlobReachabilityDenial::InvalidProtectedHold {
                counters: self.counters,
            });
        };
        Ok(authority)
    }

    fn sort_edges(&mut self) {
        self.edges
            .sort_by(|left, right| left.identity().as_str().cmp(right.identity().as_str()));
    }
}

impl BlobChunkReachabilityProofSet {
    pub(crate) fn matches_lifecycle_declaration(
        &self,
        declaration: &BlobLifecycleDeclaration,
    ) -> bool {
        self.authority.matches_declaration(declaration)
    }

    pub(crate) fn matches_publication_intent(&self, intent: &BlobPublicationIntent) -> bool {
        self.authority.matches_publication_intent(intent)
    }

    pub fn reachable_chunks(&self) -> &[BlobChunkIdentity] {
        &self.reachable_chunks
    }

    pub fn reference_edges(&self) -> &[StableDigest] {
        &self.reference_edges
    }

    pub const fn stored_digest(&self) -> &StoredChunkDigest {
        &self.stored_digest
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub fn protected_holds(&self) -> &[StableDigest] {
        &self.protected_holds
    }

    pub fn orphan_candidates(&self) -> &[BlobChunkIdentity] {
        &self.orphan_candidates
    }

    pub const fn counters(&self) -> BlobReachabilityCounterSnapshot {
        self.counters
    }

    pub(super) fn into_canonical_snapshot(
        self,
        counters: BlobReachabilityCounterSnapshot,
    ) -> BlobReachabilityCanonicalSnapshot {
        let counters = counters
            .with_reachable_chunks(self.reachable_chunks.len() as u64)
            .with_orphan_candidates(self.orphan_candidates.len() as u64);
        BlobReachabilityCanonicalSnapshot::from_parts(
            self.reachable_chunks
                .into_iter()
                .map(|chunk| chunk.chunk_digest().as_str().to_owned())
                .collect(),
            self.reference_edges
                .into_iter()
                .map(|edge| edge.as_str().to_owned())
                .collect(),
            self.protected_holds
                .into_iter()
                .map(|hold| hold.as_str().to_owned())
                .collect(),
            counters,
        )
    }
}
