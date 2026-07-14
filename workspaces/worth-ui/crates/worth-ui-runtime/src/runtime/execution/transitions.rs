use crate::runtime::UiAllocationReceipt;

/// Handle allocation entry proof: requires committed allocation truth.
#[derive(Debug, Clone)]
pub struct WorthUiExecutionLaneInput<'a>(pub(crate) &'a UiAllocationReceipt);

impl<'a> WorthUiExecutionLaneInput<'a> {
    pub fn new(allocation_receipt: &'a UiAllocationReceipt) -> Self {
        Self(allocation_receipt)
    }

    pub fn allocation_receipt(&self) -> &UiAllocationReceipt {
        self.0
    }
}
