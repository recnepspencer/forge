use worth_query::facade::runtime::{WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryExecutionReceipt, WorthQueryLowerRuntimeReadmissionReceipt, WorthQueryLowerRuntimeSeamKey};

fn from_readmission_receipt(
    seam: WorthQueryLowerRuntimeSeamKey,
    readmission: &WorthQueryLowerRuntimeReadmissionReceipt,
    receipt: &WorthQueryLowerRuntimeBoundaryExecutionReceipt,
) {
    let _ =
        WorthQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(seam, readmission, receipt);
}

fn main() {}
