use super::{ForgeQueryGraphReadAccessAuthorityCounters, ForgeQueryGraphReadAccessBasisScope};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessAuthorityReceipt {
    basis_scope: ForgeQueryGraphReadAccessBasisScope,
    policy_tenant_digest: Option<String>,
    relationship_proof_digest: Option<String>,
    counters: ForgeQueryGraphReadAccessAuthorityCounters,
    digest: String,
}

impl ForgeQueryGraphReadAccessAuthorityReceipt {
    pub fn basis_scope(&self) -> &ForgeQueryGraphReadAccessBasisScope {
        &self.basis_scope
    }

    pub fn policy_tenant_digest(&self) -> Option<&str> {
        self.policy_tenant_digest.as_deref()
    }

    pub fn relationship_proof_digest(&self) -> Option<&str> {
        self.relationship_proof_digest.as_deref()
    }

    pub fn counters(&self) -> &ForgeQueryGraphReadAccessAuthorityCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn runtime_current_compatibility() -> Self {
        Self::new(
            ForgeQueryGraphReadAccessBasisScope::runtime_current(),
            None,
            None,
            ForgeQueryGraphReadAccessAuthorityCounters::admitted(false, false),
        )
    }

    pub(crate) fn new(
        basis_scope: ForgeQueryGraphReadAccessBasisScope,
        policy_tenant_digest: Option<String>,
        relationship_proof_digest: Option<String>,
        counters: ForgeQueryGraphReadAccessAuthorityCounters,
    ) -> Self {
        let digest = hash_parts(&[
            "forge_query_graph_read_access_authority_receipt_v1".to_string(),
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
