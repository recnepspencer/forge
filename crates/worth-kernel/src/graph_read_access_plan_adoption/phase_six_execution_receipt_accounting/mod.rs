mod batch_accounting;
mod closeout;
mod closeout_digest;
mod counter_accounting;
mod errors;
mod phase_seven_seed;
mod receipt_accounting;

#[cfg(test)]
mod tests;

pub use batch_accounting::{
    WorthGraphReadAccessBatchAccountingReport, WorthGraphReadAccessBatchAccountingRow,
};
pub use closeout::{
    current_worth_graph_read_access_execution_receipt_accounting_closeout,
    WorthGraphReadAccessExecutionReceiptAccountingCloseout,
};
pub use counter_accounting::{
    WorthGraphReadAccessCallerOwnedWorkBreakdown, WorthGraphReadAccessCounterAccountingReport,
    WorthGraphReadAccessCounterAccountingRow, WorthGraphReadAccessCounterAccountingStatus,
    WorthGraphReadAccessSourceCounterProof, WorthGraphReadAccessSourceCounterProofKind,
};
pub use errors::{
    WorthGraphReadAccessExecutionReceiptAccountingError,
    WorthGraphReadAccessExecutionReceiptAccountingErrorKind,
};
pub use phase_seven_seed::WorthGraphReadAccessExecutionReceiptAccountingPhaseSevenSeed;
pub use receipt_accounting::{
    WorthGraphReadAccessReceiptAccountingReport, WorthGraphReadAccessReceiptAccountingRow,
    WorthGraphReadAccessReceiptIdentity, WorthGraphReadAccessReceiptStatus,
};

pub(crate) fn stable_digest(parts: &[String]) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update(b"\0");
    }
    format!("{:x}", hasher.finalize())
}
