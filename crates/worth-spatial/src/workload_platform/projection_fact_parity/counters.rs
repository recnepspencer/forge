#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProjectionFactParityCounters {
    lanes_compared: usize,
    receipt_backed_lanes: usize,
    denied_lanes: usize,
    policy_required_lanes: usize,
}

impl ProjectionFactParityCounters {
    pub(crate) fn new(
        lanes_compared: usize,
        receipt_backed_lanes: usize,
        denied_lanes: usize,
        policy_required_lanes: usize,
    ) -> Self {
        Self {
            lanes_compared,
            receipt_backed_lanes,
            denied_lanes,
            policy_required_lanes,
        }
    }

    pub fn lanes_compared(self) -> usize {
        self.lanes_compared
    }

    pub fn receipt_backed_lanes(self) -> usize {
        self.receipt_backed_lanes
    }

    pub fn denied_lanes(self) -> usize {
        self.denied_lanes
    }

    pub fn policy_required_lanes(self) -> usize {
        self.policy_required_lanes
    }
}
