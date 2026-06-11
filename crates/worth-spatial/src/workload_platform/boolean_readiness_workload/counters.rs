#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanReadinessWorkloadCounters {
    required_evidence_stages_consumed: usize,
    ledger_rows_consumed: usize,
    parity_lanes_consumed: usize,
    closeout_rows_consumed: usize,
    query_boundary_rows: usize,
    blocked_branch_count: usize,
}

impl PlanarBooleanReadinessWorkloadCounters {
    pub(crate) fn certified(
        required_evidence_stages_consumed: usize,
        ledger_rows_consumed: usize,
        parity_lanes_consumed: usize,
        closeout_rows_consumed: usize,
        query_boundary_rows: usize,
    ) -> Self {
        Self {
            required_evidence_stages_consumed,
            ledger_rows_consumed,
            parity_lanes_consumed,
            closeout_rows_consumed,
            query_boundary_rows,
            blocked_branch_count: 0,
        }
    }

    pub(crate) fn blocked(required_evidence_stages_consumed: usize) -> Self {
        Self {
            required_evidence_stages_consumed,
            ledger_rows_consumed: 0,
            parity_lanes_consumed: 0,
            closeout_rows_consumed: 0,
            query_boundary_rows: 0,
            blocked_branch_count: 1,
        }
    }

    pub fn required_evidence_stages_consumed(self) -> usize {
        self.required_evidence_stages_consumed
    }

    pub fn ledger_rows_consumed(self) -> usize {
        self.ledger_rows_consumed
    }

    pub fn parity_lanes_consumed(self) -> usize {
        self.parity_lanes_consumed
    }

    pub fn closeout_rows_consumed(self) -> usize {
        self.closeout_rows_consumed
    }

    pub fn query_boundary_rows(self) -> usize {
        self.query_boundary_rows
    }

    pub fn blocked_branch_count(self) -> usize {
        self.blocked_branch_count
    }
}
