use std::sync::Arc;

use crate::durability::data::DurabilityError;
use crate::history::data::{PositionedCanonicalCommit, RelationalCommitReceipt};
use crate::publication::patch::data::PatchStreamPosition;

/// Owner-held capability for retrying the durable settlement of one exact
/// publication that has already crossed the canonical branch cutover.
///
/// The capability is deliberately non-serializable. Clones retain the same
/// sealed route evidence; only the runtime that performed that route can mint
/// one or accept it for repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredPublicationSettlement {
    runtime_instance_id: u64,
    positioned: Arc<PositionedCanonicalCommit>,
    performed_result: Arc<crate::transactions::data::CommitResult>,
    snapshot_closeout: crate::runtime::PublishedSnapshotCloseout,
}

impl DeferredPublicationSettlement {
    pub(crate) fn new(
        runtime_instance_id: u64,
        positioned: Arc<PositionedCanonicalCommit>,
        performed_result: crate::transactions::data::CommitResult,
        snapshot_closeout: crate::runtime::PublishedSnapshotCloseout,
    ) -> Self {
        Self {
            runtime_instance_id,
            positioned,
            performed_result: Arc::new(performed_result),
            snapshot_closeout,
        }
    }

    pub fn commit(&self) -> &RelationalCommitReceipt {
        &self.positioned.envelope().commit
    }

    pub fn patch_position(&self) -> PatchStreamPosition {
        self.positioned.position()
    }

    /// Returns the exact result of the in-memory publication that already
    /// crossed the canonical branch cutover. Durability repair changes only
    /// settlement posture; it does not reconstruct or reinterpret this result.
    pub fn performed_result(&self) -> &crate::transactions::data::CommitResult {
        self.performed_result.as_ref()
    }

    pub(crate) const fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub(crate) fn positioned(&self) -> &Arc<PositionedCanonicalCommit> {
        &self.positioned
    }

    pub(crate) fn close_published_snapshot(&self) {
        self.snapshot_closeout.close();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeferredPublicationSettlementError {
    RecoveryUnavailable {
        commit_id: crate::history::data::CommitId,
    },
    ForeignRuntime {
        expected_runtime_instance_id: u64,
        actual_runtime_instance_id: u64,
    },
    PerformedRouteMissing,
    PerformedRouteMismatch,
    DurableAppend(DurabilityError),
}
