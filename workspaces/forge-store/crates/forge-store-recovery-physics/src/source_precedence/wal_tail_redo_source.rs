use super::{RecoveryCandidateDiscoveryTrace, WalTailIntegrityQuarantineHandoff};
use crate::{
    CheckpointCutoverReceipt, CheckpointId, ContiguousWalTailProof, IntegrityVettedWalFrame,
    ReplayCursor, WalLsnRange,
};
use forge_store_physical_integrity::{WalFrameIntegrityInputIdentity, WalTailIntegrityPosture};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalOnlyTailProof {
    lsn_range: WalLsnRange,
    input_identity: WalFrameIntegrityInputIdentity,
    tail_posture: WalTailIntegrityPosture,
    ordered_range_count: usize,
}

impl WalOnlyTailProof {
    pub fn from_vetted_wal_frame(
        record: &IntegrityVettedWalFrame,
        replay_cursor: &ReplayCursor,
    ) -> Result<Self, WalOnlyTailProofDenial> {
        Self::admit_intact_tail(
            record.input_identity(),
            record.tail_posture(),
            replay_cursor,
        )
    }

    pub fn from_quarantined_wal_tail(
        handoff: &WalTailIntegrityQuarantineHandoff,
        replay_cursor: &ReplayCursor,
    ) -> Result<Self, WalOnlyTailProofDenial> {
        Self::admit_intact_tail(
            handoff.input_identity(),
            handoff.tail_posture(),
            replay_cursor,
        )
    }

    pub const fn lsn_range(&self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn input_identity(&self) -> WalFrameIntegrityInputIdentity {
        self.input_identity
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }

    pub const fn ordered_range_count(&self) -> usize {
        self.ordered_range_count
    }

    fn admit_intact_tail(
        input_identity: WalFrameIntegrityInputIdentity,
        tail_posture: WalTailIntegrityPosture,
        replay_cursor: &ReplayCursor,
    ) -> Result<Self, WalOnlyTailProofDenial> {
        if tail_posture != WalTailIntegrityPosture::IntactTail {
            return Err(WalOnlyTailProofDenial::BlockedByWalTailIntegrity {
                posture: tail_posture,
            });
        }
        let lsn_range = WalLsnRange::new(replay_cursor.first_lsn(), replay_cursor.end_lsn())
            .map_err(|_| WalOnlyTailProofDenial::UnorderedWalTail)?;
        Ok(Self {
            lsn_range,
            input_identity,
            tail_posture,
            ordered_range_count: replay_cursor.ordering_proof().ordered_range_count(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalOnlyTailProofDenial {
    BlockedByWalTailIntegrity { posture: WalTailIntegrityPosture },
    UnorderedWalTail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalTailRedoSource {
    checkpoint_id: Option<CheckpointId>,
    lsn_range: WalLsnRange,
    trace: RecoveryCandidateDiscoveryTrace,
}

impl WalTailRedoSource {
    pub fn from_contiguous_tail(
        checkpoint: &CheckpointCutoverReceipt,
        tail: ContiguousWalTailProof,
        trace: RecoveryCandidateDiscoveryTrace,
    ) -> Option<Self> {
        if tail.checkpoint_id() != checkpoint.checkpoint_id() {
            return None;
        }
        Some(Self {
            checkpoint_id: Some(checkpoint.checkpoint_id().clone()),
            lsn_range: tail.tail_range(),
            trace,
        })
    }

    pub fn wal_only(proof: WalOnlyTailProof, trace: RecoveryCandidateDiscoveryTrace) -> Self {
        Self {
            checkpoint_id: None,
            lsn_range: proof.lsn_range(),
            trace,
        }
    }

    pub const fn checkpoint_id(&self) -> Option<&CheckpointId> {
        self.checkpoint_id.as_ref()
    }

    pub const fn lsn_range(&self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn trace(&self) -> &RecoveryCandidateDiscoveryTrace {
        &self.trace
    }
}
