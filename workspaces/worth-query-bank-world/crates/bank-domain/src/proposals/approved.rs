use crate::accounting::BankInvariantWitness;
use crate::model::BankSnapshotVersion;

use super::snapshot::BankSnapshotBasis;
use super::{
    BankIdempotencyClaim, BankIdempotencyIntent, BankIdempotencyKeyIdentity, BankProposedEffect,
    BankSnapshot,
};

pub struct BankInvariantApprovedProposal {
    basis: BankSnapshotBasis,
    idempotency: BankIdempotencyClaim,
    effects: Vec<BankProposedEffect>,
    proposed: BankSnapshot,
    _invariant_witness: BankInvariantWitness,
}

impl BankInvariantApprovedProposal {
    pub(crate) fn new(
        basis: BankSnapshotBasis,
        idempotency: BankIdempotencyClaim,
        effects: Vec<BankProposedEffect>,
        proposed: BankSnapshot,
        invariant_witness: BankInvariantWitness,
    ) -> Self {
        Self {
            basis,
            idempotency,
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
        self.idempotency.intent()
    }

    pub const fn idempotency_key_identity(&self) -> BankIdempotencyKeyIdentity {
        self.idempotency.key()
    }

    pub fn effects(&self) -> &[BankProposedEffect] {
        &self.effects
    }

    pub fn proposed_snapshot(&self) -> &BankSnapshot {
        &self.proposed
    }
}
