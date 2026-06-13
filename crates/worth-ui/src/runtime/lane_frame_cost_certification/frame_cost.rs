use crate::runtime::WorthUiCertifiedFrameExecutionReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFrameCostCertification {
    certified_receipt: WorthUiCertifiedFrameExecutionReceipt,
    counter_backed: bool,
}

impl WorthUiFrameCostCertification {
    pub(crate) fn new(certified_receipt: WorthUiCertifiedFrameExecutionReceipt) -> Self {
        Self {
            certified_receipt,
            counter_backed: true,
        }
    }

    pub fn is_counter_backed(&self) -> bool {
        self.counter_backed
    }

    pub fn certified_receipt(&self) -> &WorthUiCertifiedFrameExecutionReceipt {
        &self.certified_receipt
    }
}
