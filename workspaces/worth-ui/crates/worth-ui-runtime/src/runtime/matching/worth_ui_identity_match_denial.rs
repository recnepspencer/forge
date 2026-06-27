use crate::runtime::WorthUiIdentityMatchCounters;
use crate::runtime::WorthUiIdentityMatchNodeKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiIdentityMatchDenial {
    NarrowingActiveBasisMismatch {
        narrowing_active_artifact_digest: u64,
        active_artifact_digest: u64,
        counters: WorthUiIdentityMatchCounters,
    },
    NarrowingCandidateMismatch {
        narrowing_candidate_artifact_digest: u64,
        admitted_candidate_artifact_digest: u64,
        counters: WorthUiIdentityMatchCounters,
    },
    AdmissionReceiptChanged {
        counters: WorthUiIdentityMatchCounters,
    },
    DuplicateActiveIdentity {
        identity_basis: String,
        first_node_summary: String,
        second_node_summary: String,
        counters: WorthUiIdentityMatchCounters,
    },
    DuplicateCandidateIdentity {
        identity_basis: String,
        first_node_summary: String,
        second_node_summary: String,
        counters: WorthUiIdentityMatchCounters,
    },
    ActiveIdentityKindMismatch {
        identity_basis: String,
        first_kind: WorthUiIdentityMatchNodeKind,
        second_kind: WorthUiIdentityMatchNodeKind,
        first_node_summary: String,
        second_node_summary: String,
        counters: WorthUiIdentityMatchCounters,
    },
    CandidateIdentityKindMismatch {
        identity_basis: String,
        first_kind: WorthUiIdentityMatchNodeKind,
        second_kind: WorthUiIdentityMatchNodeKind,
        first_node_summary: String,
        second_node_summary: String,
        counters: WorthUiIdentityMatchCounters,
    },
    IdentityKindMismatch {
        identity_basis: String,
        active_kind: WorthUiIdentityMatchNodeKind,
        candidate_kind: WorthUiIdentityMatchNodeKind,
        active_node_summary: String,
        candidate_node_summary: String,
        counters: WorthUiIdentityMatchCounters,
    },
    PositionOnlyRepeatedTemplateIdentity {
        identity_basis: String,
        node_summary: String,
        counters: WorthUiIdentityMatchCounters,
    },
}
