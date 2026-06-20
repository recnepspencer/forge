use super::ForgeQueryGraphReadAccessAuthorityCounters;
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAccessAuthorityDenialKind {
    PolicyTenantDenied,
    PolicyTenantBasisScopeMismatch,
    RelationshipProofRequiresPolicyTenantContext,
    RelationshipProofPolicyTenantMismatch,
}

impl ForgeQueryGraphReadAccessAuthorityDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PolicyTenantDenied => "policy_tenant_denied",
            Self::PolicyTenantBasisScopeMismatch => "policy_tenant_basis_scope_mismatch",
            Self::RelationshipProofRequiresPolicyTenantContext => {
                "relationship_proof_requires_policy_tenant_context"
            }
            Self::RelationshipProofPolicyTenantMismatch => {
                "relationship_proof_policy_tenant_mismatch"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessAuthorityDenial {
    kind: ForgeQueryGraphReadAccessAuthorityDenialKind,
    detail: String,
    counters: ForgeQueryGraphReadAccessAuthorityCounters,
    digest: String,
}

impl ForgeQueryGraphReadAccessAuthorityDenial {
    pub fn kind(&self) -> ForgeQueryGraphReadAccessAuthorityDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn counters(&self) -> &ForgeQueryGraphReadAccessAuthorityCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub(crate) fn new(
        kind: ForgeQueryGraphReadAccessAuthorityDenialKind,
        detail: impl Into<String>,
        counters: ForgeQueryGraphReadAccessAuthorityCounters,
    ) -> Self {
        let detail = detail.into();
        let digest = hash_parts(&[
            "forge_query_graph_read_access_authority_denial_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("detail:{detail}"),
            counters.digest_part(),
        ]);
        Self {
            kind,
            detail,
            counters,
            digest,
        }
    }
}
