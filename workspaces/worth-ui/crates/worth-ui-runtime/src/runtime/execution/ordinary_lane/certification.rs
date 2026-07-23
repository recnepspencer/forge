use crate::runtime::{WorthUiOrdinaryExecutionLane, WorthUiRuntimeHandleAllocationReceipt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLaneCertification {
    lane: WorthUiOrdinaryExecutionLane,
    ordinary_plan_digest: u64,
    support_digest: u64,
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
}

impl WorthUiOrdinaryLaneCertification {
    pub(crate) fn new(
        lane: WorthUiOrdinaryExecutionLane,
        ordinary_plan_digest: u64,
        support_digest: u64,
        handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    ) -> Self {
        Self {
            lane,
            ordinary_plan_digest,
            support_digest,
            handle_receipt,
        }
    }

    pub fn lane(self) -> WorthUiOrdinaryExecutionLane {
        self.lane
    }

    pub fn ordinary_plan_digest(self) -> u64 {
        self.ordinary_plan_digest
    }

    pub fn support_digest(self) -> u64 {
        self.support_digest
    }

    pub fn handle_receipt(self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }
}
