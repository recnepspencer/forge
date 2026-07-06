//! Named boundary: streaming-window allocation receipts carry counter evidence for blob-chunks residency proofs.
use forge_store_budgets::AllocationScope;

use crate::{AllocationAdmission, AllocationReceipt, AllocationRequest, AllocationRequestKind};

/// Streaming ingest/read residency proofs require a recorded `StreamingWindow` allocation receipt.
pub fn streaming_window_allocation_receipt(
    admission: &mut AllocationAdmission,
    window_bytes: u64,
) -> Result<AllocationReceipt, crate::AllocationDenial> {
    let grant = admission.admit(AllocationRequest::streaming_window(
        AllocationScope::Streaming,
        window_bytes,
    )?)?;
    admission.record_allocation(grant)
}

pub const fn streaming_allocation_kind() -> AllocationRequestKind {
    AllocationRequestKind::StreamingWindow
}