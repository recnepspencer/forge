use super::{WorthQueryGraphReadAccessAuthorityCounters, WorthQueryGraphReadAccessBasisScope};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadAccessAuthorityReceipt {
    basis_scope: WorthQueryGraphReadAccessBasisScope,
    policy_tenant_digest: Option<String>,
    relationship_proof_digest: Option<String>,
    counters: WorthQueryGraphReadAccessAuthorityCounters,
    digest: String,
}

impl WorthQueryGraphReadAccessAuthorityReceipt {
    pub fn basis_scope(&self) -> &WorthQueryGraphReadAccessBasisScope {
        &self.basis_scope
    }

    pub fn policy_tenant_digest(&self) -> Option<&str> {
        self.policy_tenant_digest.as_deref()
    }

    pub fn relationship_proof_digest(&self) -> Option<&str> {
        self.relationship_proof_digest.as_deref()
    }

    pub fn counters(&self) -> &WorthQueryGraphReadAccessAuthorityCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn runtime_current_compatibility() -> Self {
        Self::new(
            WorthQueryGraphReadAccessBasisScope::runtime_current(),
            None,
            None,
            WorthQueryGraphReadAccessAuthorityCounters::admitted(false, false),
        )
    }

    pub(crate) fn new(
        basis_scope: WorthQueryGraphReadAccessBasisScope,
        policy_tenant_digest: Option<String>,
        relationship_proof_digest: Option<String>,
        counters: WorthQueryGraphReadAccessAuthorityCounters,
    ) -> Self {
        let digest = hash_parts(&[
            "worth_query_graph_read_access_authority_receipt_v1".to_string(),
            basis_scope.digest_part(),
            format!(
                "policy_tenant:{}",
                policy_tenant_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "relationship_proof:{}",
                relationship_proof_digest.as_deref().unwrap_or("none")
            ),
            counters.digest_part(),
        ]);
        Self {
            basis_scope,
            policy_tenant_digest,
            relationship_proof_digest,
            counters,
            digest,
        }
    }
}
