#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkDedupeCounterSnapshot {
    digest_comparisons: u64,
    foundational_equivalence_comparisons: u64,
    same_scope_admissions: u64,
    cross_scope_denials: u64,
    digest_only_denials: u64,
    collision_probes: u64,
    byte_verify_probes: u64,
    collision_denials: u64,
}

impl BlobChunkDedupeCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            digest_comparisons: 1,
            foundational_equivalence_comparisons: 0,
            same_scope_admissions: 0,
            cross_scope_denials: 0,
            digest_only_denials: 0,
            collision_probes: 0,
            byte_verify_probes: 0,
            collision_denials: 0,
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
            ..self
        }
    }

    pub(crate) const fn record_cross_scope_denial(self) -> Self {
        Self {
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

    pub const fn cross_scope_denials(self) -> u64 {
        self.cross_scope_denials
    }

    pub const fn digest_only_denials(self) -> u64 {
        self.digest_only_denials
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
}
