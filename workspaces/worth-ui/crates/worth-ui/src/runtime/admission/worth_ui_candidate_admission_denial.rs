use crate::runtime::admission::{WorthUiQuerySupportReceipt, WorthUiRuntimeReplacementPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCandidateAdmissionDenial {
    SnapshotMismatch {
        candidate_snapshot_digest: u64,
        active_snapshot_digest: u64,
    },
    DeferredRuntimePosture {
        posture: WorthUiRuntimeReplacementPosture,
    },
    UnsupportedRuntimePosture {
        posture: WorthUiRuntimeReplacementPosture,
    },
    DeferredQuerySupport {
        receipt: WorthUiQuerySupportReceipt,
    },
    UnsupportedQuerySupport {
        receipt: WorthUiQuerySupportReceipt,
    },
    QuerySupportReceiptChanged {
        admitted_receipt_digest: u64,
        current_receipt_digest: u64,
    },
}
