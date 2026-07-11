use crate::{PageLsn, RecoverySourceDecisionTrace, RedoExecutionReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryCompletion {
    recovered_root: String,
    admitted_page_lsn_frontier: Option<PageLsn>,
    replayed_frames: usize,
    source_candidate_count: usize,
    source_decision_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryCompletionDenial {
    SourceDecisionDoesNotMatchReplay,
}

pub fn complete_recovery(
    replay_receipt: RedoExecutionReceipt,
    source_precedence_trace: RecoverySourceDecisionTrace,
) -> Result<RecoveryCompletion, RecoveryCompletionDenial> {
    let recovered_state = replay_receipt.recovered_state();
    if recovered_state.source_replay_basis() != source_precedence_trace.replay_basis() {
        return Err(RecoveryCompletionDenial::SourceDecisionDoesNotMatchReplay);
    }

    Ok(RecoveryCompletion {
        recovered_root: recovered_state.recovered_physical_root().to_owned(),
        admitted_page_lsn_frontier: recovered_state.page_lsn_frontier(),
        replayed_frames: replay_receipt.applied_frame_count(),
        source_candidate_count: source_precedence_trace.candidate_count(),
        source_decision_digest: source_precedence_trace.canonical_replay_digest(),
    })
}

impl RecoveryCompletion {
    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn admitted_page_lsn_frontier(&self) -> Option<PageLsn> {
        self.admitted_page_lsn_frontier
    }

    pub const fn replayed_frames(&self) -> usize {
        self.replayed_frames
    }

    pub const fn source_candidate_count(&self) -> usize {
        self.source_candidate_count
    }

    pub fn source_decision_digest(&self) -> &str {
        &self.source_decision_digest
    }
}
