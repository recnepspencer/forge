#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PartialPublicationCounterSnapshot {
    observed_crash_edges: usize,
    rejected_residue_promotions: usize,
    rejected_live_ack_promotions: usize,
    rejected_log_only_promotions: usize,
    replayable_unacknowledged_wal: usize,
    torn_publication_denials: usize,
    no_undo_denials: usize,
    no_undo_postures: usize,
    ambiguous_outcomes: usize,
}

impl PartialPublicationCounterSnapshot {
    pub const fn with_observed_crash_edge(mut self) -> Self {
        self.observed_crash_edges += 1;
        self
    }

    pub const fn with_rejected_residue_promotion(mut self) -> Self {
        self.rejected_residue_promotions += 1;
        self
    }

    pub const fn with_rejected_live_ack_promotion(mut self) -> Self {
        self.rejected_live_ack_promotions += 1;
        self
    }

    pub const fn with_rejected_log_only_promotion(mut self) -> Self {
        self.rejected_log_only_promotions += 1;
        self
    }

    pub const fn with_replayable_unacknowledged_wal(mut self) -> Self {
        self.replayable_unacknowledged_wal += 1;
        self
    }

    pub const fn with_torn_publication_denial(mut self) -> Self {
        self.torn_publication_denials += 1;
        self
    }

    pub const fn with_no_undo_denial(mut self) -> Self {
        self.no_undo_denials += 1;
        self
    }

    pub const fn with_no_undo_posture(mut self) -> Self {
        self.no_undo_postures += 1;
        self
    }

    pub const fn with_ambiguous_outcome(mut self) -> Self {
        self.ambiguous_outcomes += 1;
        self
    }

    pub const fn observed_crash_edges(self) -> usize {
        self.observed_crash_edges
    }

    pub const fn rejected_residue_promotions(self) -> usize {
        self.rejected_residue_promotions
    }

    pub const fn rejected_live_ack_promotions(self) -> usize {
        self.rejected_live_ack_promotions
    }

    pub const fn rejected_log_only_promotions(self) -> usize {
        self.rejected_log_only_promotions
    }

    pub const fn replayable_unacknowledged_wal(self) -> usize {
        self.replayable_unacknowledged_wal
    }

    pub const fn torn_publication_denials(self) -> usize {
        self.torn_publication_denials
    }

    pub const fn no_undo_denials(self) -> usize {
        self.no_undo_denials
    }

    pub const fn no_undo_postures(self) -> usize {
        self.no_undo_postures
    }

    pub const fn ambiguous_outcomes(self) -> usize {
        self.ambiguous_outcomes
    }
}
