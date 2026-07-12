use crate::runtime::{
    UiAllocationReceipt, UiAllocationReceiptDenialReport, UiAllocationReceiptReport,
    UiAllocationReplanTransaction,
};

/// Read-only, receipt-owned explanation for ordinary allocation inspection.
///
/// It is projected from the commit result; it never reconstructs locality or
/// denial from host state.
#[derive(Clone, Debug, PartialEq)]
pub struct UiAllocationReceiptInspectionReceipt {
    report: UiAllocationReceiptReport,
    transaction: UiAllocationReplanTransaction,
    geometry: crate::runtime::UiCommittedAllocationGeometryEvidence,
}

impl UiAllocationReceiptInspectionReceipt {
    pub fn report(&self) -> &UiAllocationReceiptReport {
        &self.report
    }
    pub fn transaction(&self) -> &UiAllocationReplanTransaction {
        &self.transaction
    }
    pub fn geometry(&self) -> &crate::runtime::UiCommittedAllocationGeometryEvidence {
        &self.geometry
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationReceiptDenialInspectionReceipt {
    denial: UiAllocationReceiptDenialReport,
}

impl UiAllocationReceiptDenialInspectionReceipt {
    pub fn denial(&self) -> &UiAllocationReceiptDenialReport {
        &self.denial
    }
}

pub(crate) fn project_allocation_receipt_inspection(
    receipt: &UiAllocationReceipt,
) -> UiAllocationReceiptInspectionReceipt {
    UiAllocationReceiptInspectionReceipt {
        report: receipt.report().clone(),
        transaction: receipt.transaction().clone(),
        geometry: receipt.geometry_evidence().clone(),
    }
}

pub(crate) fn project_allocation_receipt_denial_inspection(
    denial: &UiAllocationReceiptDenialReport,
) -> UiAllocationReceiptDenialInspectionReceipt {
    UiAllocationReceiptDenialInspectionReceipt {
        denial: denial.clone(),
    }
}
