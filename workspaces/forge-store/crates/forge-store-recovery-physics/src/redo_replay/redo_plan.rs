use crate::{
    AdmittedRecoverySource, RecoveryBlockedByIntegrityDamage, RecoverySourceDecisionTrace,
    WalLsnRange,
};
use forge_store_physical_format::PhysicalPageId;

use super::{AdmittedRedoFrame, RedoApplicationCursor, RedoExecutionReceipt, WalValidPrefix};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryRedoPlan {
    valid_prefix: WalValidPrefix,
    frames: Vec<AdmittedRedoFrame>,
    source_trace: RecoverySourceDecisionTrace,
    expected: RedoPlanCounterExpectation,
}

impl RecoveryRedoPlan {
    pub fn from_valid_prefix(
        source: &AdmittedRecoverySource,
        valid_prefix: WalValidPrefix,
        mut frames: Vec<AdmittedRedoFrame>,
    ) -> Result<Self, RedoPlanningDenial> {
        let source_trace = source.trace().clone();
        if let AdmittedRecoverySource::RecoveryBlocked { damage, .. } = source {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::RecoveryBlocked {
                    damage: damage.clone(),
                },
            ));
        }
        let Some(selected_wal_tail) = source.selected_wal_tail() else {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::NoAdmittedWalTail,
            ));
        };
        if valid_prefix.source_range() != selected_wal_tail.lsn_range() {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::WalPrefixSourceMismatch {
                    prefix_source_range: valid_prefix.source_range(),
                    selected_source_range: selected_wal_tail.lsn_range(),
                },
            ));
        }
        frames.sort_by_key(|frame| frame.redo_lsn());
        if frames.len() != valid_prefix.admitted_frame_count() {
            return Err(RedoPlanningDenial::new(
                RedoPlanningDenialKind::MissingAdmittedRedoFrames {
                    expected: valid_prefix.admitted_frame_count(),
                    planned: frames.len(),
                },
            ));
        }
        for (offset, frame) in frames.iter().enumerate() {
            let expected_lsn = crate::LogSequenceNumber::new(
                valid_prefix.prefix_range().start().get() + offset as u64,
            );
            if frame.redo_lsn() != expected_lsn {
                return Err(RedoPlanningDenial::new(
                    RedoPlanningDenialKind::NonContiguousPlannedRedoFrame {
                        expected_lsn,
                        observed_lsn: frame.redo_lsn(),
                    },
                ));
            }
        }
        let expected = RedoPlanCounterExpectation::new(valid_prefix.admitted_frame_count())
            .with_planned_frames(frames.len());
        Ok(Self {
            valid_prefix,
            frames,
            source_trace,
            expected,
        })
    }

    pub fn execute(
        &self,
        cursor: &RedoApplicationCursor,
    ) -> Result<RedoExecutionReceipt, RedoPlanningDenial> {
        RedoExecutionReceipt::from_plan(self, cursor)
    }

    pub fn valid_prefix(&self) -> &WalValidPrefix {
        &self.valid_prefix
    }

    pub fn frames(&self) -> &[AdmittedRedoFrame] {
        &self.frames
    }

    pub const fn source_trace(&self) -> &RecoverySourceDecisionTrace {
        &self.source_trace
    }

    pub const fn expected(&self) -> RedoPlanCounterExpectation {
        self.expected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RedoPlanCounterExpectation {
    admitted_prefix_frames: usize,
    planned_frames: usize,
}

impl RedoPlanCounterExpectation {
    pub(crate) const fn new(admitted_prefix_frames: usize) -> Self {
        Self {
            admitted_prefix_frames,
            planned_frames: 0,
        }
    }

    pub(crate) const fn with_planned_frames(self, planned_frames: usize) -> Self {
        Self {
            planned_frames,
            ..self
        }
    }

    pub const fn admitted_prefix_frames(self) -> usize {
        self.admitted_prefix_frames
    }

    pub const fn planned_frames(self) -> usize {
        self.planned_frames
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedoPlanningDenial {
    kind: RedoPlanningDenialKind,
}

impl RedoPlanningDenial {
    pub(crate) const fn new(kind: RedoPlanningDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> &RedoPlanningDenialKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedoPlanningDenialKind {
    RecoveryBlocked {
        damage: RecoveryBlockedByIntegrityDamage,
    },
    NoAdmittedWalTail,
    FrameOutsideAdmittedSourceRange {
        frame_lsn: crate::LogSequenceNumber,
        source_range: WalLsnRange,
    },
    WrongPageLsnBasis {
        frame_lsn: crate::LogSequenceNumber,
        page_lsn_basis: crate::PageLsn,
    },
    RedoTargetPageGenerationMismatch {
        target_page: PhysicalPageId,
        generation_page: PhysicalPageId,
    },
    CursorPageGenerationMismatch {
        cursor_page: PhysicalPageId,
        eligibility_page: PhysicalPageId,
        digest_page: PhysicalPageId,
    },
    WalPrefixSourceMismatch {
        prefix_source_range: WalLsnRange,
        selected_source_range: WalLsnRange,
    },
    MissingPageEligibility {
        frame_lsn: crate::LogSequenceNumber,
    },
    MissingAdmittedRedoFrames {
        expected: usize,
        planned: usize,
    },
    NonContiguousPlannedRedoFrame {
        expected_lsn: crate::LogSequenceNumber,
        observed_lsn: crate::LogSequenceNumber,
    },
    PageRedoDenied {
        frame_lsn: crate::LogSequenceNumber,
        denial: crate::UnadmittedDirtyPagePublicationDenial,
    },
    MiddleWalCorruption(super::MiddleWalCorruptionDenial),
    MissingAcknowledgedWalRange(super::MissingAcknowledgedWalRangeDenial),
    StaleWalGeneration(super::StaleWalGenerationDenial),
}
