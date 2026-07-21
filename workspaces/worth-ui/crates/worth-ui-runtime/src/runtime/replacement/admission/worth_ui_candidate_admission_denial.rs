use crate::runtime::replacement::admission::{
    WorthUiQuerySupportReceipt, WorthUiRuntimeReplacementPosture,
};

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
    QuerySupportContractChanged {
        admitted_contract_identity: worth_ui_query_binding::WorthUiQueryBindingContractIdentity,
        current_contract_identity: worth_ui_query_binding::WorthUiQueryBindingContractIdentity,
    },
}
