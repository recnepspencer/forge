use crate::basis_lifecycle::BasisFamily;
use crate::ordinary::read::{
    WorthQueryReadCompletion, WorthQueryReadContextKind, WorthQueryReadContextReceipt,
};
use crate::runtime::{WorthQueryReadReceipt, WorthQueryRuntimeAuthorityIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowPrimaryReadEvidence {
    role: String,
    context_receipt: WorthQueryReadContextReceipt,
    read_receipt: WorthQueryReadReceipt,
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
            && self.read_receipt.snapshot_identity() == expected_snapshot
    }
}
