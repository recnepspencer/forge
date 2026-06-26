use crate::runtime::WorthUiRuntimeHandleAllocationReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataCertification {
    data_plan_digest: u64,
    support_digest: u64,
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
}

impl WorthUiVirtualizedDataCertification {
    pub(crate) fn new(
        data_plan_digest: u64,
        support_digest: u64,
        handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    ) -> Self {
        Self {
            data_plan_digest,
            support_digest,
            handle_receipt,
        }
    }

    pub fn data_plan_digest(self) -> u64 {
        self.data_plan_digest
    }

    pub fn support_digest(self) -> u64 {
        self.support_digest
    }

    pub fn handle_receipt(self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }
}
