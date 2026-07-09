use worth_store_budgets::CounterEvidenceStrength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobReachabilityCounterSnapshot {
    strength: CounterEvidenceStrength,
    reachable_chunks: u64,
    reference_edges: u64,
    dedupe_reference_edges: u64,
    protected_holds: u64,
    orphan_candidates: u64,
    stale_reference_denials: u64,
    copied_row_denials: u64,
    empty_proof_denials: u64,
    wrong_authority_denials: u64,
    reclaim_denials: u64,
    held_reclaim_checks: u64,
    reclaimable_chunks: u64,
    replay_convergence_checks: u64,
}

impl BlobReachabilityCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            strength: CounterEvidenceStrength::Exact,
            reachable_chunks: 0,
            reference_edges: 0,
            dedupe_reference_edges: 0,
            protected_holds: 0,
            orphan_candidates: 0,
            stale_reference_denials: 0,
            copied_row_denials: 0,
            empty_proof_denials: 0,
            wrong_authority_denials: 0,
            reclaim_denials: 0,
            held_reclaim_checks: 0,
            reclaimable_chunks: 0,
            replay_convergence_checks: 0,
        }
    }

    pub(crate) const fn with_classified_reclaim_outcome(
        self,
        case: &crate::reachability::classification::ReachabilityReclaimCase,
    ) -> Self {
        use crate::reachability::classification::ReachabilityReclaimCase;
        match case {
            ReachabilityReclaimCase::Reachable => Self {
                reclaim_denials: self.reclaim_denials + 1,
                ..self
            },
            ReachabilityReclaimCase::Held => Self {
                held_reclaim_checks: self.held_reclaim_checks + 1,
                reclaim_denials: self.reclaim_denials + 1,
                ..self
            },
            ReachabilityReclaimCase::DeniedMissingRelease => Self {
                reclaim_denials: self.reclaim_denials + 1,
                ..self
            },
            ReachabilityReclaimCase::Reclaimable { .. } => Self {
                reclaimable_chunks: self.reclaimable_chunks + 1,
                ..self
            },
        }
    }

    pub(crate) const fn with_edge(self, is_dedupe: bool) -> Self {
        Self {
            reference_edges: self.reference_edges + 1,
            dedupe_reference_edges: self.dedupe_reference_edges + is_dedupe as u64,
            ..self
        }
    }

    pub(crate) const fn with_hold(self) -> Self {
        Self {
            protected_holds: self.protected_holds + 1,
            ..self
        }
    }

    pub(crate) const fn with_current_reference_edges(
        self,
        reference_edges: u64,
        dedupe_reference_edges: u64,
    ) -> Self {
        Self {
            reference_edges,
            dedupe_reference_edges,
            ..self
        }
    }

    pub(crate) const fn with_current_protected_holds(self, protected_holds: u64) -> Self {
        Self {
            protected_holds,
            ..self
        }
    }

    pub(crate) const fn with_reachable_chunks(self, count: u64) -> Self {
        Self {
            reachable_chunks: count,
            ..self
        }
    }

    pub(crate) const fn with_orphan_candidates(self, count: u64) -> Self {
        Self {
            orphan_candidates: count,
            ..self
        }
    }

    pub(crate) const fn record_stale_reference_denial(self) -> Self {
        Self {
            stale_reference_denials: self.stale_reference_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_copied_row_denial(self) -> Self {
        Self {
            copied_row_denials: self.copied_row_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_empty_proof_denial(self) -> Self {
        Self {
            empty_proof_denials: self.empty_proof_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_wrong_authority_denial(self) -> Self {
        Self {
            wrong_authority_denials: self.wrong_authority_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_replay_convergence_check(self) -> Self {
        Self {
            replay_convergence_checks: self.replay_convergence_checks + 1,
            ..self
        }
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn reachable_chunks(self) -> u64 {
        self.reachable_chunks
    }

    pub const fn reference_edges(self) -> u64 {
        self.reference_edges
    }

    pub const fn dedupe_reference_edges(self) -> u64 {
        self.dedupe_reference_edges
    }

    pub const fn protected_holds(self) -> u64 {
        self.protected_holds
    }

    pub const fn orphan_candidates(self) -> u64 {
        self.orphan_candidates
    }

    pub const fn stale_reference_denials(self) -> u64 {
        self.stale_reference_denials
    }

    pub const fn copied_row_denials(self) -> u64 {
        self.copied_row_denials
    }

    pub const fn empty_proof_denials(self) -> u64 {
        self.empty_proof_denials
    }

    pub const fn wrong_authority_denials(self) -> u64 {
        self.wrong_authority_denials
    }

    pub const fn reclaim_denials(self) -> u64 {
        self.reclaim_denials
    }

    pub const fn held_reclaim_checks(self) -> u64 {
        self.held_reclaim_checks
    }

    pub const fn reclaimable_chunks(self) -> u64 {
        self.reclaimable_chunks
    }

    pub const fn replay_convergence_checks(self) -> u64 {
        self.replay_convergence_checks
    }
}

impl Default for BlobReachabilityCounterSnapshot {
    fn default() -> Self {
        Self::start()
    }
}
