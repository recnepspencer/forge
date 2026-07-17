/// Frozen 3.8 -> 3.9 handoff. Lowering receives only committed allocation
/// truth and its companion report/transaction lineage.
#[derive(Clone, Debug, PartialEq)]
pub struct UiCommittedAllocationLoweringInput {
    receipt: super::UiAllocationReceipt,
    report: super::UiAllocationReceiptReport,
    transaction: super::UiAllocationReplanTransaction,
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
}

impl UiCommittedAllocationLoweringInput {
    pub(super) fn from_receipt(
        receipt: &super::UiAllocationReceipt,
    ) -> Result<Self, super::UiAllocationFreshnessConsumptionDenial> {
        super::admit_execution_lowering(receipt.report())?;
        Ok(Self {
            receipt: receipt.clone(),
            report: receipt.report().clone(),
            transaction: receipt.transaction().clone(),
            frame_epoch: receipt.transaction().frame_epoch(),
        })
    }

    pub fn receipt(&self) -> &super::UiAllocationReceipt {
        &self.receipt
    }
    pub fn report(&self) -> &super::UiAllocationReceiptReport {
        &self.report
    }
    pub fn transaction(&self) -> &super::UiAllocationReplanTransaction {
        &self.transaction
    }
    pub fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.frame_epoch
    }
}
