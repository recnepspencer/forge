use std::sync::{Arc, Mutex};

use super::ForgeQueryJournalReplayDenialKind;

#[derive(Clone, Debug, Default)]
pub(in crate::runtime) struct ForgeQueryJournalReplayCounters {
    state: Arc<Mutex<ForgeQueryJournalReplayCounterState>>,
}

impl ForgeQueryJournalReplayCounters {
    pub fn record_admitted_replay(
        &self,
        scanned_entry_count: usize,
        resolved_entry_count: usize,
        journal_gap_count: usize,
        outcome_digest: &str,
    ) {
        let mut state = self.state.lock().expect("journal replay counters lock");
        state.replay_admission_count += 1;
        state.replay_scanned_entry_count += scanned_entry_count;
        state.replay_resolved_entry_count += resolved_entry_count;
        state.replay_gap_count += journal_gap_count;
        state.last_replay_outcome_digest = Some(outcome_digest.to_string());
    }

    pub fn record_denial(&self, kind: ForgeQueryJournalReplayDenialKind) {
        let mut state = self.state.lock().expect("journal replay counters lock");
        *state.denial_count_mut(kind) += 1;
    }

    pub fn snapshot(&self, retained_entry_count: usize) -> ForgeQueryJournalReplayCounterSnapshot {
        let state = self.state.lock().expect("journal replay counters lock");
        ForgeQueryJournalReplayCounterSnapshot {
            retained_entry_count,
            replay_admission_count: state.replay_admission_count,
            invalid_segment_bounds_denial_count: state.invalid_segment_bounds_denial_count,
            unknown_segment_identity_denial_count: state.unknown_segment_identity_denial_count,
            stale_basis_replay_denial_count: state.stale_basis_replay_denial_count,
            cross_scheme_replay_denial_count: state.cross_scheme_replay_denial_count,
            journal_gap_denial_count: state.journal_gap_denial_count,
            replay_scanned_entry_count: state.replay_scanned_entry_count,
            replay_resolved_entry_count: state.replay_resolved_entry_count,
            replay_gap_count: state.replay_gap_count,
            replay_residue_count: state.replay_residue_count,
            last_replay_outcome_digest: state.last_replay_outcome_digest.clone(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ForgeQueryJournalReplayCounterState {
    replay_admission_count: usize,
    invalid_segment_bounds_denial_count: usize,
    unknown_segment_identity_denial_count: usize,
    stale_basis_replay_denial_count: usize,
    cross_scheme_replay_denial_count: usize,
    journal_gap_denial_count: usize,
    replay_scanned_entry_count: usize,
    replay_resolved_entry_count: usize,
    replay_gap_count: usize,
    replay_residue_count: usize,
    last_replay_outcome_digest: Option<String>,
}

impl ForgeQueryJournalReplayCounterState {
    fn denial_count_mut(&mut self, kind: ForgeQueryJournalReplayDenialKind) -> &mut usize {
        match kind {
            ForgeQueryJournalReplayDenialKind::InvalidSegmentBounds => {
                &mut self.invalid_segment_bounds_denial_count
            }
            ForgeQueryJournalReplayDenialKind::UnknownSegmentIdentity => {
                &mut self.unknown_segment_identity_denial_count
            }
            ForgeQueryJournalReplayDenialKind::StaleBasisReplay => {
                &mut self.stale_basis_replay_denial_count
            }
            ForgeQueryJournalReplayDenialKind::CrossSchemeReplay => {
                &mut self.cross_scheme_replay_denial_count
            }
            ForgeQueryJournalReplayDenialKind::JournalGap => &mut self.journal_gap_denial_count,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalReplayCounterSnapshot {
    retained_entry_count: usize,
    replay_admission_count: usize,
    invalid_segment_bounds_denial_count: usize,
    unknown_segment_identity_denial_count: usize,
    stale_basis_replay_denial_count: usize,
    cross_scheme_replay_denial_count: usize,
    journal_gap_denial_count: usize,
    replay_scanned_entry_count: usize,
    replay_resolved_entry_count: usize,
    replay_gap_count: usize,
    replay_residue_count: usize,
    last_replay_outcome_digest: Option<String>,
}

impl ForgeQueryJournalReplayCounterSnapshot {
    pub fn retained_entry_count(&self) -> usize {
        self.retained_entry_count
    }

    pub fn replay_admission_count(&self) -> usize {
        self.replay_admission_count
    }

    pub fn denial_count(&self, kind: ForgeQueryJournalReplayDenialKind) -> usize {
        match kind {
            ForgeQueryJournalReplayDenialKind::InvalidSegmentBounds => {
                self.invalid_segment_bounds_denial_count
            }
            ForgeQueryJournalReplayDenialKind::UnknownSegmentIdentity => {
                self.unknown_segment_identity_denial_count
            }
            ForgeQueryJournalReplayDenialKind::StaleBasisReplay => {
                self.stale_basis_replay_denial_count
            }
            ForgeQueryJournalReplayDenialKind::CrossSchemeReplay => {
                self.cross_scheme_replay_denial_count
            }
            ForgeQueryJournalReplayDenialKind::JournalGap => self.journal_gap_denial_count,
        }
    }

    pub fn replay_scanned_entry_count(&self) -> usize {
        self.replay_scanned_entry_count
    }

    pub fn replay_resolved_entry_count(&self) -> usize {
        self.replay_resolved_entry_count
    }

    pub fn replay_gap_count(&self) -> usize {
        self.replay_gap_count
    }

    pub fn replay_residue_count(&self) -> usize {
        self.replay_residue_count
    }

    pub fn last_replay_outcome_digest(&self) -> Option<&str> {
        self.last_replay_outcome_digest.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalReplayDiagnostics {
    counter_snapshot: ForgeQueryJournalReplayCounterSnapshot,
}

impl ForgeQueryJournalReplayDiagnostics {
    pub(in crate::runtime) fn new(
        counter_snapshot: ForgeQueryJournalReplayCounterSnapshot,
    ) -> Self {
        Self { counter_snapshot }
    }

    pub fn counter_snapshot(&self) -> &ForgeQueryJournalReplayCounterSnapshot {
        &self.counter_snapshot
    }
}
