use crate::runtime::WorthUiRuntimeHandleAllocationReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeCertification {
    hud_plan_digest: u64,
    support_digest: u64,
    policy_digest: u64,
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
}

impl WorthUiRealtimeCertification {
    pub(crate) fn new(
        hud_plan_digest: u64,
        support_digest: u64,
        policy_digest: u64,
        handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    ) -> Self {
        Self {
            hud_plan_digest,
            support_digest,
            policy_digest,
            handle_receipt,
        }
    }

    pub fn hud_plan_digest(self) -> u64 {
        self.hud_plan_digest
    }

    pub fn support_digest(self) -> u64 {
        self.support_digest
    }

    pub fn policy_digest(self) -> u64 {
        self.policy_digest
    }

    pub fn handle_receipt(self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }
}
