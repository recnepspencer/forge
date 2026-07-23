use crate::basis_lifecycle::BasisFamily;
use crate::ordinary::read::{
    WorthQueryReadCompletion, WorthQueryReadContextKind, WorthQueryReadContextReceipt,
};
use crate::runtime::{WorthQueryReadReceipt, WorthQueryRuntimeAuthorityIdentity};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryWorkflowPrimaryReadEvidence {
    role: String,
    context_receipt: WorthQueryReadContextReceipt,
    read_receipt: WorthQueryReadReceipt,
    semantic_rows: Vec<crate::memory_workspace::WorthQueryEntity>,
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
}

impl WorthQueryWorkflowPrimaryReadEvidence {
    pub(crate) fn from_completion(
        role: impl Into<String>,
        completion: &WorthQueryReadCompletion,
    ) -> Self {
        Self {
            role: role.into(),
            context_receipt: completion.context_receipt().clone(),
            read_receipt: completion.result().receipt().clone(),
            semantic_rows: completion.result().rows().to_vec(),
            runtime_authority: completion.runtime_authority(),
        }
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn context_receipt(&self) -> &WorthQueryReadContextReceipt {
        &self.context_receipt
    }

    pub fn read_receipt(&self) -> &WorthQueryReadReceipt {
        &self.read_receipt
    }

    pub(crate) fn semantic_replay_eq(&self, candidate: &Self) -> bool {
        self.role == candidate.role
            && self.read_receipt.canonical_query_digest()
                == candidate.read_receipt.canonical_query_digest()
            && self.semantic_rows == candidate.semantic_rows
            && self.read_receipt.graph_family() == candidate.read_receipt.graph_family()
            && self.read_receipt.collection_result_family()
                == candidate.read_receipt.collection_result_family()
            && self.read_receipt.scope_class() == candidate.read_receipt.scope_class()
    }

    pub(crate) fn validates(
        &self,
        canonical: &crate::canonicalization::CanonicalQueryBundle,
        basis: BasisFamily,
        expected_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
        runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    ) -> bool {
        self.runtime_authority == runtime_authority
            && basis == BasisFamily::CurrentHead
            && self.context_receipt.context_kind() == WorthQueryReadContextKind::Current
            && self.context_receipt.canonical_query_digest() == canonical.query().digest().as_str()
            && self.read_receipt.canonical_query_digest() == canonical.query().digest().as_str()
            && self
                .read_receipt
                .snapshot_identity()
                .is_same_current_identity_as(expected_snapshot)
    }
}
