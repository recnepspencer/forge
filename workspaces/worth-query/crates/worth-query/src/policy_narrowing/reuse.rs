#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedPolicyNarrowingReuseDisposition {
    LegalNoSemanticChange,
    LegalRequiresFreshNarrowing,
    IllegalSemanticDrift,
}

impl SavedPolicyNarrowingReuseDisposition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LegalNoSemanticChange => "legal_no_semantic_change",
            Self::LegalRequiresFreshNarrowing => "legal_requires_fresh_narrowing",
            Self::IllegalSemanticDrift => "illegal_semantic_drift",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedPolicyNarrowingReuseDescriptor {
    saved_query_digest: String,
    prior_narrowed_artifact_digest: String,
    prior_policy_digest: String,
    prior_tenant_truth_basis_digest: String,
    prior_tenant_schema_basis_digest: String,
    prior_authorized_projection_digest: String,
    prior_relationship_proof_digest: String,
    new_policy_digest: String,
    new_tenant_truth_basis_digest: String,
    new_tenant_schema_basis_digest: String,
    new_authorized_projection_digest: String,
    new_relationship_proof_digest: String,
}

impl SavedPolicyNarrowingReuseDescriptor {
    #[cfg(test)]
    pub(crate) fn new(
        saved_query_digest: impl Into<String>,
        prior_narrowed_artifact_digest: impl Into<String>,
        prior_policy_digest: impl Into<String>,
        prior_tenant_truth_basis_digest: impl Into<String>,
        prior_tenant_schema_basis_digest: impl Into<String>,
        prior_authorized_projection_digest: impl Into<String>,
        prior_relationship_proof_digest: impl Into<String>,
        new_policy_digest: impl Into<String>,
        new_tenant_truth_basis_digest: impl Into<String>,
        new_tenant_schema_basis_digest: impl Into<String>,
        new_authorized_projection_digest: impl Into<String>,
        new_relationship_proof_digest: impl Into<String>,
    ) -> Self {
        Self {
            saved_query_digest: saved_query_digest.into(),
            prior_narrowed_artifact_digest: prior_narrowed_artifact_digest.into(),
            prior_policy_digest: prior_policy_digest.into(),
            prior_tenant_truth_basis_digest: prior_tenant_truth_basis_digest.into(),
            prior_tenant_schema_basis_digest: prior_tenant_schema_basis_digest.into(),
            prior_authorized_projection_digest: prior_authorized_projection_digest.into(),
            prior_relationship_proof_digest: prior_relationship_proof_digest.into(),
            new_policy_digest: new_policy_digest.into(),
            new_tenant_truth_basis_digest: new_tenant_truth_basis_digest.into(),
            new_tenant_schema_basis_digest: new_tenant_schema_basis_digest.into(),
            new_authorized_projection_digest: new_authorized_projection_digest.into(),
            new_relationship_proof_digest: new_relationship_proof_digest.into(),
        }
    }

    pub fn saved_query_digest(&self) -> &str {
        &self.saved_query_digest
    }

    pub fn prior_narrowed_artifact_digest(&self) -> &str {
        &self.prior_narrowed_artifact_digest
    }

    pub(crate) fn exact_narrowing_match(&self) -> bool {
        self.same_policy_tenant_basis()
            && self.prior_authorized_projection_digest == self.new_authorized_projection_digest
            && self.prior_relationship_proof_digest == self.new_relationship_proof_digest
    }

    pub(crate) fn same_policy_tenant_basis(&self) -> bool {
        self.prior_policy_digest == self.new_policy_digest
            && self.prior_tenant_truth_basis_digest == self.new_tenant_truth_basis_digest
            && self.prior_tenant_schema_basis_digest == self.new_tenant_schema_basis_digest
    }
}
