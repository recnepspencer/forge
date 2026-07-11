use crate::{
    AdmittedRecoverySource, AdmittedReplayTailCursor, CheckpointId, RecoverySourceDecisionRow,
    WalLsnRange,
};

use super::{
    CheckpointCutoverLayoutReport, RecoveryLayoutAccessDenial, RecoveryLayoutAccessDenialKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedReplayIndexLayoutRule {
    _private: (),
}

impl AdmittedReplayIndexLayoutRule {
    pub(crate) const fn internal_phase22() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase22-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase22() -> Self {
        Self::internal_phase22()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayIndexLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayIndexLayoutAdmission {
    _private: (),
}

impl ReplayIndexLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        self,
        _rule: &AdmittedReplayIndexLayoutRule,
    ) -> Result<ReplayIndexLayoutAdmission, RecoveryLayoutAccessDenial> {
        Ok(ReplayIndexLayoutAdmission { _private: () })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedReplayIndexLayoutFamily {
    _admission: ReplayIndexLayoutAdmission,
}

impl AdmittedReplayIndexLayoutFamily {
    pub(crate) const fn new(admission: ReplayIndexLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn admit_checkpoint_replay_index(
        &self,
        checkpoint: &CheckpointCutoverLayoutReport,
        cursor: &AdmittedReplayTailCursor,
    ) -> Result<ReplayIndexLayoutReport, RecoveryLayoutAccessDenial> {
        if checkpoint.covered_lsn_range().range().end_exclusive() != cursor.first_lsn() {
            return Err(RecoveryLayoutAccessDenial::new(
                RecoveryLayoutAccessDenialKind::ReplayTailCheckpointGap,
            ));
        }
        Ok(ReplayIndexLayoutReport::from_cursor(
            Some(checkpoint.checkpoint_id().clone()),
            cursor,
        ))
    }

    pub fn admit_wal_only_replay_index(
        &self,
        cursor: &AdmittedReplayTailCursor,
    ) -> ReplayIndexLayoutReport {
        ReplayIndexLayoutReport::from_cursor(None, cursor)
    }

    pub fn admit_recovery_source_replay_index(
        &self,
        source: &AdmittedRecoverySource,
    ) -> Result<ReplayIndexLayoutReport, RecoveryLayoutAccessDenial> {
        let replay_frontier = source
            .selected_wal_tail()
            .map(|tail| tail.lsn_range())
            .ok_or_else(|| {
                RecoveryLayoutAccessDenial::new(
                    RecoveryLayoutAccessDenialKind::ReplayProjectionCannotStandInForWalAuthority,
                )
            })?;
        Ok(ReplayIndexLayoutReport {
            checkpoint_id: source
                .selected_checkpoint()
                .map(|checkpoint| checkpoint.checkpoint_id().clone()),
            replay_frontier,
            segment_count: 1,
            ordered_range_count: usize::from(
                replay_frontier.start() != replay_frontier.end_exclusive(),
            ),
            counters: ReplayIndexLayoutCounters {
                checkpoint_cutover_inputs: u64::from(source.selected_checkpoint().is_some()),
                replay_tail_cursor_inputs: 1,
            },
        })
    }

    pub fn reject_row_projection(
        &self,
        _row: &RecoverySourceDecisionRow,
    ) -> Result<(), RecoveryLayoutAccessDenial> {
        Err(RecoveryLayoutAccessDenial::new(
            RecoveryLayoutAccessDenialKind::ReplayProjectionCannotStandInForWalAuthority,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayIndexLayoutCounters {
    checkpoint_cutover_inputs: u64,
    replay_tail_cursor_inputs: u64,
}

impl ReplayIndexLayoutCounters {
    pub const fn checkpoint_cutover_inputs(&self) -> u64 {
        self.checkpoint_cutover_inputs
    }

    pub const fn replay_tail_cursor_inputs(&self) -> u64 {
        self.replay_tail_cursor_inputs
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayIndexLayoutReport {
    checkpoint_id: Option<CheckpointId>,
    replay_frontier: WalLsnRange,
    segment_count: usize,
    ordered_range_count: usize,
    counters: ReplayIndexLayoutCounters,
}

impl ReplayIndexLayoutReport {
    fn from_cursor(checkpoint_id: Option<CheckpointId>, cursor: &AdmittedReplayTailCursor) -> Self {
        let checkpoint_cutover_inputs = u64::from(checkpoint_id.is_some());
        Self {
            checkpoint_id,
            replay_frontier: WalLsnRange::new(cursor.first_lsn(), cursor.end_lsn())
                .expect("admitted replay tail cursor stays ordered"),
            segment_count: cursor.segments().len(),
            ordered_range_count: cursor.ordering_proof().ordered_range_count(),
            counters: ReplayIndexLayoutCounters {
                checkpoint_cutover_inputs,
                replay_tail_cursor_inputs: 1,
            },
        }
    }

    pub fn checkpoint_id(&self) -> Option<&CheckpointId> {
        self.checkpoint_id.as_ref()
    }

    pub const fn replay_frontier(&self) -> WalLsnRange {
        self.replay_frontier
    }

    pub const fn segment_count(&self) -> usize {
        self.segment_count
    }

    pub const fn ordered_range_count(&self) -> usize {
        self.ordered_range_count
    }

    pub const fn counters(&self) -> ReplayIndexLayoutCounters {
        self.counters
    }
}
