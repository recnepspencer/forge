use worth_store_budgets::CounterEvidenceStrength;
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_reclaim_policy::ReclaimPolicyCounterSnapshot;

use crate::{
    BlobChunkDedupeCounterSnapshot, BlobCompactionCounterSnapshot, BlobCorruptionCounterSnapshot,
    BlobPublicationCounterSnapshot, BlobReachabilityCounterSnapshot,
    BlobRetentionReclaimCounterSnapshot, BlobStreamingReadCounterSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobLayoutAccessShape {
    PointLookup,
    BoundedScan,
    CompactionRead,
    QuarantineRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobLayoutAccessPathEvidence {
    family_id: DurableArtifactFamilyId,
    strength: CounterEvidenceStrength,
    publication_steps: u64,
    verified_chunks: u64,
    bytes_read: u64,
    windows_observed: u64,
    peak_resident_bytes: u64,
}

impl BlobLayoutAccessPathEvidence {
    pub const fn from_publication(
        family_id: DurableArtifactFamilyId,
        counters: BlobPublicationCounterSnapshot,
    ) -> Self {
        Self {
            family_id,
            strength: CounterEvidenceStrength::Derived,
            publication_steps: counters.root_candidates()
                + counters.reachability_staged()
                + counters.wal_records()
                + counters.session_closeouts()
                + counters.committed_publications()
                + counters.visible_observations(),
            verified_chunks: 0,
            bytes_read: 0,
            windows_observed: 0,
            peak_resident_bytes: 0,
        }
    }

    pub const fn from_streaming(
        family_id: DurableArtifactFamilyId,
        counters: BlobStreamingReadCounterSnapshot,
    ) -> Self {
        Self {
            family_id,
            strength: counters.counter_strength(),
            publication_steps: 0,
            verified_chunks: counters.chunks_verified(),
            bytes_read: counters.bytes_read(),
            windows_observed: counters.windows_observed(),
            peak_resident_bytes: counters.peak_resident_bytes(),
        }
    }

    pub const fn from_dedupe(
        family_id: DurableArtifactFamilyId,
        counters: BlobChunkDedupeCounterSnapshot,
    ) -> Self {
        Self {
            family_id,
            strength: CounterEvidenceStrength::Derived,
            publication_steps: counters.digest_comparisons()
                + counters.foundational_equivalence_comparisons()
                + counters.same_scope_admissions()
                + counters.dedupe_hits()
                + counters.dedupe_misses(),
            verified_chunks: counters.byte_verify_probes(),
            bytes_read: 0,
            windows_observed: counters.collision_probes(),
            peak_resident_bytes: counters.digest_rewrites(),
        }
    }

    pub const fn from_reachability(
        family_id: DurableArtifactFamilyId,
        counters: BlobReachabilityCounterSnapshot,
    ) -> Self {
        Self {
            family_id,
            strength: counters.strength(),
            publication_steps: counters.reachable_chunks(),
            verified_chunks: counters.reference_edges(),
            bytes_read: 0,
            windows_observed: counters.protected_holds(),
            peak_resident_bytes: counters.orphan_candidates(),
        }
    }

    pub const fn from_retention_holds(family_id: DurableArtifactFamilyId, hold_count: u64) -> Self {
        Self {
            family_id,
            strength: CounterEvidenceStrength::Derived,
            publication_steps: hold_count,
            verified_chunks: 0,
            bytes_read: 0,
            windows_observed: 0,
            peak_resident_bytes: 0,
        }
    }

    pub const fn from_reclaim(
        family_id: DurableArtifactFamilyId,
        counters: BlobRetentionReclaimCounterSnapshot,
    ) -> Self {
        Self {
            family_id,
            strength: counters.strength(),
            publication_steps: counters.orphan_candidates(),
            verified_chunks: counters.reclaim_permits(),
            bytes_read: 0,
            windows_observed: counters.reclaimed_chunks(),
            peak_resident_bytes: counters.residue_localizations(),
        }
    }

    pub const fn from_reclaim_policy(
        family_id: DurableArtifactFamilyId,
        counters: ReclaimPolicyCounterSnapshot,
    ) -> Self {
        Self {
            family_id,
            strength: CounterEvidenceStrength::Exact,
            publication_steps: counters.admission_requests() + counters.admitted(),
            verified_chunks: counters.executed(),
            bytes_read: 0,
            windows_observed: counters.protected_reachability_checks()
                + counters.security_scope_checks(),
            peak_resident_bytes: counters.byte_interpretation_observations()
                + counters.non_claim_handoffs(),
        }
    }

    pub const fn from_compaction(
        family_id: DurableArtifactFamilyId,
        counters: BlobCompactionCounterSnapshot,
    ) -> Self {
        Self {
            family_id,
            strength: CounterEvidenceStrength::Derived,
            publication_steps: counters.chunks_scanned(),
            verified_chunks: counters.chunks_rewritten(),
            bytes_read: counters.bytes_moved(),
            windows_observed: counters.dedupe_edges_preserved(),
            peak_resident_bytes: counters.foreground_yields(),
        }
    }

    pub const fn from_corruption(
        family_id: DurableArtifactFamilyId,
        counters: BlobCorruptionCounterSnapshot,
    ) -> Self {
        Self {
            family_id,
            strength: counters.strength(),
            publication_steps: counters.damage_case_classifications() + counters.localizations(),
            verified_chunks: counters.affected_reference_edges(),
            bytes_read: 0,
            windows_observed: counters.quarantine_holds(),
            peak_resident_bytes: counters.derived_rebuild_admissions()
                + counters.authoritative_repair_postures()
                + counters.authoritative_restore_postures()
                + counters.authoritative_degraded_truth_postures(),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn strength(&self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn publication_steps(&self) -> u64 {
        self.publication_steps
    }

    pub const fn verified_chunks(&self) -> u64 {
        self.verified_chunks
    }

    pub const fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    pub const fn windows_observed(&self) -> u64 {
        self.windows_observed
    }

    pub const fn peak_resident_bytes(&self) -> u64 {
        self.peak_resident_bytes
    }
}
