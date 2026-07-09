use crate::{WalLsnRange, WalSegmentGeneration, WalSegmentId};

use super::{
    CheckpointCutoverReceipt, CheckpointId, CheckpointRecoveryCounterSnapshot,
    CheckpointValidationDenial, CheckpointValidationDenialKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContiguousWalTailProof {
    checkpoint_id: CheckpointId,
    tail_range: WalLsnRange,
}

impl ContiguousWalTailProof {
    pub fn prove(
        checkpoint: &CheckpointCutoverReceipt,
        tail_range: WalLsnRange,
    ) -> Result<Self, CheckpointValidationDenial> {
        if checkpoint.covered_lsn_range().range().end_exclusive() != tail_range.start() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::WalRetentionWithoutContiguousTail,
                checkpoint.counters().with_retention_decision(),
            )
            .with_lsn_pair(
                checkpoint.covered_lsn_range().range().end_exclusive(),
                tail_range.start(),
            ));
        }
        Ok(Self {
            checkpoint_id: checkpoint.checkpoint_id().clone(),
            tail_range,
        })
    }

    pub fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    pub const fn tail_range(&self) -> WalLsnRange {
        self.tail_range
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalRetentionCandidateSegment {
    segment_id: WalSegmentId,
    generation: WalSegmentGeneration,
    lsn_range: WalLsnRange,
}

impl WalRetentionCandidateSegment {
    pub const fn new(
        segment_id: WalSegmentId,
        generation: WalSegmentGeneration,
        lsn_range: WalLsnRange,
    ) -> Self {
        Self {
            segment_id,
            generation,
            lsn_range,
        }
    }

    pub const fn segment_id(self) -> WalSegmentId {
        self.segment_id
    }

    pub const fn generation(self) -> WalSegmentGeneration {
        self.generation
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalRetentionAction {
    Delete,
    Recycle,
    Exclude,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalRetentionRequest {
    action: WalRetentionAction,
    segment: WalRetentionCandidateSegment,
}

impl WalRetentionRequest {
    pub const fn new(action: WalRetentionAction, segment: WalRetentionCandidateSegment) -> Self {
        Self { action, segment }
    }

    pub const fn action(self) -> WalRetentionAction {
        self.action
    }

    pub const fn segment(self) -> WalRetentionCandidateSegment {
        self.segment
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalRetentionEligibility {
    segment: WalRetentionCandidateSegment,
    counters: CheckpointRecoveryCounterSnapshot,
}

impl WalRetentionEligibility {
    pub(crate) fn admit(
        checkpoint: &CheckpointCutoverReceipt,
        tail: ContiguousWalTailProof,
        segment: WalRetentionCandidateSegment,
    ) -> Result<Self, CheckpointValidationDenial> {
        let counters = checkpoint.counters().with_retention_decision();
        if tail.checkpoint_id() != checkpoint.checkpoint_id() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::WalRetentionCheckpointMismatch,
                counters,
            ));
        }
        if tail.tail_range().start() != checkpoint.covered_lsn_range().range().end_exclusive() {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::WalRetentionWithoutContiguousTail,
                counters,
            )
            .with_lsn_pair(
                checkpoint.covered_lsn_range().range().end_exclusive(),
                tail.tail_range().start(),
            ));
        }
        if segment.lsn_range().end_exclusive()
            > checkpoint.covered_lsn_range().range().end_exclusive()
        {
            return Err(CheckpointValidationDenial::new(
                CheckpointValidationDenialKind::WalRetentionWithoutCoveringCheckpoint,
                counters,
            )
            .with_lsn_pair(
                checkpoint.covered_lsn_range().range().end_exclusive(),
                segment.lsn_range().end_exclusive(),
            ));
        }
        Ok(Self { segment, counters })
    }

    pub const fn segment(self) -> WalRetentionCandidateSegment {
        self.segment
    }

    pub const fn counters(self) -> CheckpointRecoveryCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalRetentionAdmittedAction {
    request: WalRetentionRequest,
    eligibility: WalRetentionEligibility,
}

impl WalRetentionAdmittedAction {
    pub fn admit(
        checkpoint: &CheckpointCutoverReceipt,
        tail: ContiguousWalTailProof,
        request: WalRetentionRequest,
    ) -> Result<Self, CheckpointValidationDenial> {
        let eligibility = WalRetentionEligibility::admit(checkpoint, tail, request.segment())?;
        Ok(Self {
            request,
            eligibility,
        })
    }

    pub const fn request(self) -> WalRetentionRequest {
        self.request
    }

    pub const fn eligibility(self) -> WalRetentionEligibility {
        self.eligibility
    }
}
