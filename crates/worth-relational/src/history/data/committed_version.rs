use serde::{Deserialize, Serialize};

use super::CommitReference;

/// Ordinary-history evidence for the transaction that created a version.
///
/// This is deliberately smaller than a replay envelope: ordinary consumers
/// can recover commit identity and result cardinality without opening replay
/// authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedVersionSummary {
    commit: CommitReference,
    changed_record_count: usize,
}

impl CommittedVersionSummary {
    pub(crate) const fn new(commit: CommitReference, changed_record_count: usize) -> Self {
        Self {
            commit,
            changed_record_count,
        }
    }

    pub const fn commit(&self) -> &CommitReference {
        &self.commit
    }

    pub const fn changed_record_count(&self) -> usize {
        self.changed_record_count
    }
}
