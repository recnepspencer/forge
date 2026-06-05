use forge_query::facade::{
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    ForgeQueryLowerRuntimeReadmissionReceipt, ForgeQueryLowerRuntimeSeamKey,
};

fn from_readmission_receipt(
    seam: ForgeQueryLowerRuntimeSeamKey,
    readmission: &ForgeQueryLowerRuntimeReadmissionReceipt,
    receipt: &ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
) {
    let _ =
        ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(seam, readmission, receipt);
}

fn main() {}
