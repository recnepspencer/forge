#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoTransactionBoundaryPacketCounters {
    topology_touched_subject_count: usize,
    replay_touched_subject_count: usize,
    undo_touched_subject_count: usize,
    mutation_claim_count: usize,
    replay_raw_row_scan_count: usize,
    replay_broad_receipt_scan_count: usize,
    replay_caller_owned_scan_count: usize,
    replay_retained_replay_binding_count: usize,
    undo_lookup_consumed_workload_handoff_count: usize,
    undo_raw_row_scan_count: usize,
    undo_broad_receipt_scan_count: usize,
    undo_caller_owned_scan_count: usize,
}

impl ReplayUndoTransactionBoundaryPacketCounters {
    pub(crate) const fn empty() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        topology_touched_subject_count: usize,
        replay_touched_subject_count: usize,
        undo_touched_subject_count: usize,
        mutation_claim_count: usize,
        replay_raw_row_scan_count: usize,
        replay_broad_receipt_scan_count: usize,
        replay_caller_owned_scan_count: usize,
        replay_retained_replay_binding_count: usize,
        undo_lookup_consumed_workload_handoff_count: usize,
        undo_raw_row_scan_count: usize,
        undo_broad_receipt_scan_count: usize,
        undo_caller_owned_scan_count: usize,
    ) -> Self {
        Self {
            topology_touched_subject_count,
            replay_touched_subject_count,
            undo_touched_subject_count,
            mutation_claim_count,
            replay_raw_row_scan_count,
            replay_broad_receipt_scan_count,
            replay_caller_owned_scan_count,
            replay_retained_replay_binding_count,
            undo_lookup_consumed_workload_handoff_count,
            undo_raw_row_scan_count,
            undo_broad_receipt_scan_count,
            undo_caller_owned_scan_count,
        }
    }

    pub const fn topology_touched_subject_count(&self) -> usize {
        self.topology_touched_subject_count
    }

    pub const fn replay_touched_subject_count(&self) -> usize {
        self.replay_touched_subject_count
    }

    pub const fn undo_touched_subject_count(&self) -> usize {
        self.undo_touched_subject_count
    }

    pub const fn mutation_claim_count(&self) -> usize {
        self.mutation_claim_count
    }

    pub const fn replay_raw_row_scan_count(&self) -> usize {
        self.replay_raw_row_scan_count
    }

    pub const fn replay_broad_receipt_scan_count(&self) -> usize {
        self.replay_broad_receipt_scan_count
    }

    pub const fn replay_caller_owned_scan_count(&self) -> usize {
        self.replay_caller_owned_scan_count
    }

    pub const fn replay_retained_replay_binding_count(&self) -> usize {
        self.replay_retained_replay_binding_count
    }

    pub const fn undo_lookup_consumed_workload_handoff_count(&self) -> usize {
        self.undo_lookup_consumed_workload_handoff_count
    }

    pub const fn undo_raw_row_scan_count(&self) -> usize {
        self.undo_raw_row_scan_count
    }

    pub const fn undo_broad_receipt_scan_count(&self) -> usize {
        self.undo_broad_receipt_scan_count
    }

    pub const fn undo_caller_owned_scan_count(&self) -> usize {
        self.undo_caller_owned_scan_count
    }
}
