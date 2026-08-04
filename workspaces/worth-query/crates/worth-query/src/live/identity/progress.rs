use super::subscription::{LiveChangeSequenceId, LiveSubscriptionDigest};
use crate::basis::ResolvedSnapshotBasis;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiveChangeOrdinal(pub(in crate::live) u64);

impl LiveChangeOrdinal {
    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn from_value(value: u64) -> Self {
        Self(value)
    }

    pub(in crate::live) fn zero() -> Self {
        Self(0)
    }

    pub(in crate::live) fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiveReplayDigest(String);

impl LiveReplayDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(in crate::live) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveStartBasis {
    basis: ResolvedSnapshotBasis,
}

impl LiveStartBasis {
    pub fn basis(&self) -> &ResolvedSnapshotBasis {
        &self.basis
    }

    pub(in crate::live) fn new(basis: ResolvedSnapshotBasis) -> Self {
        Self { basis }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveProgressBasis {
    current_basis: ResolvedSnapshotBasis,
    change_sequence_id: LiveChangeSequenceId,
    last_ordinal: LiveChangeOrdinal,
    replay_digest: LiveReplayDigest,
}

impl LiveProgressBasis {
    pub fn current_basis(&self) -> &ResolvedSnapshotBasis {
        &self.current_basis
    }

    pub fn change_sequence_id(&self) -> &LiveChangeSequenceId {
        &self.change_sequence_id
    }

    pub fn last_ordinal(&self) -> &LiveChangeOrdinal {
        &self.last_ordinal
    }

    pub fn replay_digest(&self) -> &LiveReplayDigest {
        &self.replay_digest
    }

    pub(in crate::live) fn initial(
        subscription_digest: &LiveSubscriptionDigest,
        start_basis: &LiveStartBasis,
    ) -> Self {
        let change_sequence_id =
            LiveChangeSequenceId::from_subscription_digest(subscription_digest);
        let last_ordinal = LiveChangeOrdinal::zero();
        let replay_digest = LiveReplayDigest::from_parts(&[
            format!("subscription:{}", subscription_digest.as_str()),
            format!("basis:{}", start_basis.basis().proof().digest().as_str()),
            format!("change_sequence:{}", change_sequence_id.as_str()),
            format!("ordinal:{}", last_ordinal.value()),
        ]);
        Self {
            current_basis: start_basis.basis().clone(),
            change_sequence_id,
            last_ordinal,
            replay_digest,
        }
    }

    pub fn advance(
        &self,
        change_sequence_id: &LiveChangeSequenceId,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<Self, LiveProgressError> {
        if self.change_sequence_id != *change_sequence_id {
            return Err(LiveProgressError::ChangeSequenceMismatch);
        }

        let expected = self.last_ordinal.next();
        if next_ordinal.value() > expected.value() {
            return Err(LiveProgressError::ChangeSequenceGap {
                expected: expected.value(),
                received: next_ordinal.value(),
            });
        }
        if next_ordinal != expected {
            return Err(LiveProgressError::NonMonotonicOrdinal {
                expected: expected.value(),
                received: next_ordinal.value(),
            });
        }

        let replay_digest = LiveReplayDigest::from_parts(&[
            format!("basis:{}", next_basis.proof().digest().as_str()),
            format!("change_sequence:{}", change_sequence_id.as_str()),
            format!("ordinal:{}", next_ordinal.value()),
        ]);

        Ok(Self {
            current_basis: next_basis,
            change_sequence_id: change_sequence_id.clone(),
            last_ordinal: next_ordinal,
            replay_digest,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveProgressError {
    ChangeSequenceMismatch,
    ChangeSequenceGap { expected: u64, received: u64 },
    NonMonotonicOrdinal { expected: u64, received: u64 },
}
