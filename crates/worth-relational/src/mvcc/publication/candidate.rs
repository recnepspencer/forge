use std::cell::Cell;
use std::marker::PhantomData;

use crate::history::data::BranchId;
use crate::transactions::data::TransactionId;

/// Opaque, single-use owner artifact whose fallible commit preparation is complete.
///
/// A candidate has no publication method. Only the Relational runtime owner can
/// consume it, either through publication or explicit discard.
pub struct PreparedRelationalCommitCandidate {
    runtime_instance_id: u64,
    transaction_id: TransactionId,
    branch_id: BranchId,
    pub(crate) execution: crate::authority::commit::pipeline::PreparedCommitPublicationExecution,
    _not_sync: PhantomData<Cell<()>>,
}

impl PreparedRelationalCommitCandidate {
    pub(crate) fn new(
        runtime_instance_id: u64,
        transaction_id: TransactionId,
        branch_id: BranchId,
        execution: crate::authority::commit::pipeline::PreparedCommitPublicationExecution,
    ) -> Self {
        Self {
            runtime_instance_id,
            transaction_id,
            branch_id,
            execution,
            _not_sync: PhantomData,
        }
    }

    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    pub fn branch(&self) -> &BranchId {
        &self.branch_id
    }

    pub(crate) fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub(crate) fn reservation_count(&self) -> usize {
        self.execution.reservation_count()
    }

    #[cfg(test)]
    pub(crate) fn materialization_counts(&self) -> (u64, u64) {
        self.execution.materialization_counts()
    }
}

impl std::fmt::Debug for PreparedRelationalCommitCandidate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedRelationalCommitCandidate")
            .field("transaction_id", &self.transaction_id)
            .field("branch_id", &self.branch_id)
            .field("reservation_count", &self.reservation_count())
            .finish_non_exhaustive()
    }
}

/// Evidence returned when a prepared candidate is consumed without publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscardedRelationalCommitCandidate {
    transaction_id: TransactionId,
    branch_id: BranchId,
    released_record_reservation_count: usize,
}

impl DiscardedRelationalCommitCandidate {
    pub(crate) fn from_candidate(candidate: &PreparedRelationalCommitCandidate) -> Self {
        Self {
            transaction_id: candidate.transaction_id,
            branch_id: candidate.branch_id.clone(),
            released_record_reservation_count: candidate.reservation_count(),
        }
    }

    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    pub fn branch(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn released_record_reservation_count(&self) -> usize {
        self.released_record_reservation_count
    }
}
