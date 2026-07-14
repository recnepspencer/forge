use crate::identity::CanonicalQueryDigest;
use crate::query_context::{QueryContextFamily, ScopedQueryBasisContext};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisScopeEvidence {
    admitted_query_digest: String,
    expected_canonical_query_digest: String,
    basis_digest: String,
    basis_family: QueryContextFamily,
}

impl BasisScopeEvidence {
    pub fn from_admitted_context_for_canonical_query(
        context: &ScopedQueryBasisContext,
        canonical_query_digest: &CanonicalQueryDigest,
    ) -> Self {
        Self {
            admitted_query_digest: context.query_digest().to_string(),
            expected_canonical_query_digest: canonical_query_digest.as_str().to_string(),
            basis_digest: context.basis_digest().to_string(),
            basis_family: context.family().clone(),
        }
    }

    pub fn admitted_query_digest(&self) -> &str {
        &self.admitted_query_digest
    }

    pub fn expected_canonical_query_digest(&self) -> &str {
        &self.expected_canonical_query_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn basis_family(&self) -> &QueryContextFamily {
        &self.basis_family
    }
}
