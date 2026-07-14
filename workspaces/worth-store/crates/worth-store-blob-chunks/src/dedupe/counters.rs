#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkDedupeCounterSnapshot {
    digest_comparisons: u64,
    foundational_equivalence_comparisons: u64,
    dedupe_hits: u64,
    dedupe_misses: u64,
    same_scope_admissions: u64,
    cross_scope_denials: u64,
    stale_key_version_denials: u64,
    authenticity_mismatch_denials: u64,
    custody_mismatch_denials: u64,
    digest_only_denials: u64,
    quarantine_denials: u64,
    index_partition_denials: u64,
    digest_rewrites: u64,
    collision_probes: u64,
    byte_verify_probes: u64,
    collision_denials: u64,
    reference_edges_admitted: u64,
    reference_edges_denied: u64,
    reclaim_blocked_by_reference_edges: u64,
}

impl BlobChunkDedupeCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            digest_comparisons: 1,
            foundational_equivalence_comparisons: 0,
            dedupe_hits: 0,
            dedupe_misses: 0,
            same_scope_admissions: 0,
            cross_scope_denials: 0,
            stale_key_version_denials: 0,
            authenticity_mismatch_denials: 0,
            custody_mismatch_denials: 0,
            digest_only_denials: 0,
            quarantine_denials: 0,
            index_partition_denials: 0,
            digest_rewrites: 0,
            collision_probes: 0,
            byte_verify_probes: 0,
            collision_denials: 0,
            reference_edges_admitted: 0,
            reference_edges_denied: 0,
            reclaim_blocked_by_reference_edges: 0,
        }
    }

    pub(crate) const fn record_equivalence_comparison(self) -> Self {
        Self {
            foundational_equivalence_comparisons: self.foundational_equivalence_comparisons + 1,
            ..self
        }
    }

    pub(crate) const fn record_same_scope_admission(self) -> Self {
        Self {
            same_scope_admissions: self.same_scope_admissions + 1,
            dedupe_hits: self.dedupe_hits + 1,
            reference_edges_admitted: self.reference_edges_admitted + 1,
            ..self
        }
    }

    pub(crate) const fn record_dedupe_miss(self) -> Self {
        Self {
            dedupe_misses: self.dedupe_misses + 1,
            ..self
        }
    }

    pub(crate) const fn record_cross_scope_denial(self) -> Self {
        Self {
            cross_scope_denials: self.cross_scope_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_stale_key_version_denial(self) -> Self {
        Self {
            stale_key_version_denials: self.stale_key_version_denials + 1,
            cross_scope_denials: self.cross_scope_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_authenticity_mismatch_denial(self) -> Self {
        Self {
            authenticity_mismatch_denials: self.authenticity_mismatch_denials + 1,
            cross_scope_denials: self.cross_scope_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_custody_mismatch_denial(self) -> Self {
        Self {
            custody_mismatch_denials: self.custody_mismatch_denials + 1,
            cross_scope_denials: self.cross_scope_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_digest_only_denial(self) -> Self {
        Self {
            digest_only_denials: self.digest_only_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_quarantine_denial(self) -> Self {
        Self {
            quarantine_denials: self.quarantine_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_index_partition_denial(self) -> Self {
        Self {
            index_partition_denials: self.index_partition_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_digest_rewrite(self) -> Self {
        Self {
            digest_rewrites: self.digest_rewrites + 1,
            ..self
        }
    }

    pub(crate) const fn record_collision_probe(self) -> Self {
        Self {
            collision_probes: self.collision_probes + 1,
            ..self
        }
    }

    pub(crate) const fn record_byte_verify_probe(self) -> Self {
        Self {
            byte_verify_probes: self.byte_verify_probes + 1,
            ..self
        }
    }

    pub(crate) const fn record_collision_denial(self) -> Self {
        Self {
            collision_denials: self.collision_denials + 1,
            dedupe_misses: self.dedupe_misses + 1,
            ..self
        }
    }

    pub(crate) const fn record_reference_edges_denied(self, edges: u64) -> Self {
        Self {
            reference_edges_denied: self.reference_edges_denied + edges,
            ..self
        }
    }

    pub(crate) const fn record_reference_edge_admitted(self) -> Self {
        Self {
            reference_edges_admitted: self.reference_edges_admitted + 1,
            ..self
        }
    }

    pub(crate) const fn record_reclaim_blocked_by_reference_edge(self) -> Self {
        Self {
            reclaim_blocked_by_reference_edges: self.reclaim_blocked_by_reference_edges + 1,
            ..self
        }
    }

    pub const fn digest_comparisons(self) -> u64 {
        self.digest_comparisons
    }

    pub const fn foundational_equivalence_comparisons(self) -> u64 {
        self.foundational_equivalence_comparisons
    }

    pub const fn same_scope_admissions(self) -> u64 {
        self.same_scope_admissions
    }

    pub const fn dedupe_hits(self) -> u64 {
        self.dedupe_hits
    }

    pub const fn dedupe_misses(self) -> u64 {
        self.dedupe_misses
    }

    pub const fn cross_scope_denials(self) -> u64 {
        self.cross_scope_denials
    }

    pub const fn digest_only_denials(self) -> u64 {
        self.digest_only_denials
    }

    pub const fn quarantine_denials(self) -> u64 {
        self.quarantine_denials
    }

    pub const fn index_partition_denials(self) -> u64 {
        self.index_partition_denials
    }

    pub const fn digest_rewrites(self) -> u64 {
        self.digest_rewrites
    }

    pub const fn collision_probes(self) -> u64 {
        self.collision_probes
    }

    pub const fn byte_verify_probes(self) -> u64 {
        self.byte_verify_probes
    }

    pub const fn collision_denials(self) -> u64 {
        self.collision_denials
    }

    pub const fn stale_key_version_denials(self) -> u64 {
        self.stale_key_version_denials
    }

    pub const fn authenticity_mismatch_denials(self) -> u64 {
        self.authenticity_mismatch_denials
    }

    pub const fn custody_mismatch_denials(self) -> u64 {
        self.custody_mismatch_denials
    }

    pub const fn reference_edges_admitted(self) -> u64 {
        self.reference_edges_admitted
    }

    pub const fn reference_edges_denied(self) -> u64 {
        self.reference_edges_denied
    }

    pub const fn reclaim_blocked_by_reference_edges(self) -> u64 {
        self.reclaim_blocked_by_reference_edges
    }
}
