use crate::accounting::BankInvariantWitness;
use crate::model::BankSnapshotVersion;

use super::snapshot::BankSnapshotBasis;
use super::{BankIdempotencyIntent, BankProposedEffect, BankSnapshot};

pub struct BankInvariantApprovedProposal {
    basis: BankSnapshotBasis,
    idempotency_intent: BankIdempotencyIntent,
    effects: Vec<BankProposedEffect>,
    proposed: BankSnapshot,
    _invariant_witness: BankInvariantWitness,
}

impl BankInvariantApprovedProposal {
    pub(crate) fn new(
        basis: BankSnapshotBasis,
        idempotency_intent: BankIdempotencyIntent,
        effects: Vec<BankProposedEffect>,
        proposed: BankSnapshot,
        invariant_witness: BankInvariantWitness,
    ) -> Self {
        Self {
            basis,
            idempotency_intent,
            effects,
            proposed,
            _invariant_witness: invariant_witness,
        }
    }

    pub const fn basis(&self) -> BankSnapshotVersion {
        self.basis.version()
    }

    /// Proves whether this proposal's invariant witness was minted from this
    /// exact causal in-memory snapshot, not merely the same numeric version.
    pub fn matches_basis(&self, snapshot: &BankSnapshot) -> bool {
        self.basis.matches(snapshot)
    }

    pub const fn idempotency_intent(&self) -> BankIdempotencyIntent {
        self.idempotency_intent
    }

    pub fn effects(&self) -> &[BankProposedEffect] {
        &self.effects
    }

    pub fn proposed_snapshot(&self) -> &BankSnapshot {
        &self.proposed
    }
}
