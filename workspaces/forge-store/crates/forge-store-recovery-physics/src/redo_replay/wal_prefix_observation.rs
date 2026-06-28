use crate::{
    LogSequenceNumber, RecoveryBlockedByIntegrityDamage, RecoveryBlockingIntegritySource,
    WalOnlyTailProof, WalSegmentGeneration, WalTailIntegrityQuarantineHandoff,
};
use forge_store_physical_integrity::WalTailIntegrityPosture;

use super::{RedoPlanningDenial, RedoPlanningDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalPrefixIntegrityObservation {
    inner: WalPrefixFrameObservation,
}

impl WalPrefixIntegrityObservation {
    pub fn from_vetted_wal_tail(
        proof: &WalOnlyTailProof,
        lsn: LogSequenceNumber,
        segment_generation: WalSegmentGeneration,
    ) -> Result<Self, RedoPlanningDenial> {
        if !proof.lsn_range().contains(lsn) {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::FrameOutsideAdmittedSourceRange {
                    frame_lsn: lsn,
                    source_range: proof.lsn_range(),
                },
            ));
        }
        Ok(Self {
            inner: WalPrefixFrameObservation::integrity_vetted(lsn, segment_generation),
        })
    }

    pub fn from_quarantined_wal_tail(
        handoff: &WalTailIntegrityQuarantineHandoff,
        lsn: LogSequenceNumber,
        segment_generation: WalSegmentGeneration,
    ) -> Self {
        Self {
            inner: WalPrefixFrameObservation::from_damaged_tail_posture(
                lsn,
                segment_generation,
                handoff.tail_posture(),
            ),
        }
    }

    pub fn from_recovery_blocking_damage(
        damage: &RecoveryBlockedByIntegrityDamage,
        lsn: LogSequenceNumber,
        segment_generation: WalSegmentGeneration,
    ) -> Result<Self, RedoPlanningDenial> {
        if damage.source() != RecoveryBlockingIntegritySource::WalFrame {
            return Err(recovery_blocked_by(damage));
        }
        let Some(posture) = damage.tail_posture() else {
            return Err(recovery_blocked_by(damage));
        };
        match posture {
            WalTailIntegrityPosture::TornTail | WalTailIntegrityPosture::IntactTail => {
                Err(recovery_blocked_by(damage))
            }
            WalTailIntegrityPosture::UnsupportedTailIntegrity
            | WalTailIntegrityPosture::UnknownTailIntegrity
            | WalTailIntegrityPosture::CheckpointAdjacentDamage
            | WalTailIntegrityPosture::RecoveryPrecedenceRequired => Ok(Self {
                inner: WalPrefixFrameObservation::middle_corruption(lsn, segment_generation),
            }),
        }
    }

    pub(crate) const fn into_frame_observation(self) -> WalPrefixFrameObservation {
        self.inner
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WalPrefixFrameObservation {
    lsn: LogSequenceNumber,
    segment_generation: WalSegmentGeneration,
    posture: WalPrefixFramePosture,
}

impl WalPrefixFrameObservation {
    const fn integrity_vetted(
        lsn: LogSequenceNumber,
        segment_generation: WalSegmentGeneration,
    ) -> Self {
        Self {
            lsn,
            segment_generation,
            posture: WalPrefixFramePosture::IntegrityVetted,
        }
    }

    const fn torn_tail(lsn: LogSequenceNumber, segment_generation: WalSegmentGeneration) -> Self {
        Self {
            lsn,
            segment_generation,
            posture: WalPrefixFramePosture::TornTail,
        }
    }

    const fn middle_corruption(
        lsn: LogSequenceNumber,
        segment_generation: WalSegmentGeneration,
    ) -> Self {
        Self {
            lsn,
            segment_generation,
            posture: WalPrefixFramePosture::MiddleCorruption,
        }
    }

    const fn from_damaged_tail_posture(
        lsn: LogSequenceNumber,
        segment_generation: WalSegmentGeneration,
        posture: WalTailIntegrityPosture,
    ) -> Self {
        match posture {
            WalTailIntegrityPosture::TornTail => Self::torn_tail(lsn, segment_generation),
            WalTailIntegrityPosture::IntactTail => Self::integrity_vetted(lsn, segment_generation),
            WalTailIntegrityPosture::UnsupportedTailIntegrity
            | WalTailIntegrityPosture::UnknownTailIntegrity
            | WalTailIntegrityPosture::CheckpointAdjacentDamage
            | WalTailIntegrityPosture::RecoveryPrecedenceRequired => {
                Self::middle_corruption(lsn, segment_generation)
            }
        }
    }

    pub(crate) const fn lsn(self) -> LogSequenceNumber {
        self.lsn
    }

    pub(crate) const fn segment_generation(self) -> WalSegmentGeneration {
        self.segment_generation
    }

    pub(crate) const fn posture(self) -> WalPrefixFramePosture {
        self.posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WalPrefixFramePosture {
    IntegrityVetted,
    TornTail,
    MiddleCorruption,
}

fn recovery_blocked_by(damage: &RecoveryBlockedByIntegrityDamage) -> RedoPlanningDenial {
    RedoPlanningDenial::new(RedoPlanningDenialKind::RecoveryBlocked {
        damage: damage.clone(),
    })
}
