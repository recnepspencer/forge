use crate::identity::hash_parts;
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;

use super::{PolicyAwareExecutionMode, PolicyAwareSeamCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareExecutionSeamIdentity(String);

impl PolicyAwareExecutionSeamIdentity {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareExecutionSeam {
    identity: PolicyAwareExecutionSeamIdentity,
    source_narrowed_artifact_digest: String,
    authorized_projection_digest: String,
    relationship_proof_digest: String,
    policy_digest: String,
    tenant_truth_basis_digest: String,
    tenant_schema_basis_digest: String,
    branch_access_digest: String,
    narrowed_result_shape_digest: String,
    mode: PolicyAwareExecutionMode,
    counters: PolicyAwareSeamCounters,
}

impl PolicyAwareExecutionSeam {
    pub(crate) fn from_narrowed(
        artifact: &NarrowedPolicyQueryArtifact,
        mode: PolicyAwareExecutionMode,
        counters: PolicyAwareSeamCounters,
    ) -> Self {
        let parts = vec![
            format!("narrowed:{}", artifact.digest()),
            format!(
                "authorized_projection:{}",
                artifact.authorized_projection().identity().as_str()
            ),
            format!(
                "relationship_proof:{}",
                artifact.relationship_proof().identity().as_str()
            ),
            format!("policy:{}", artifact.policy_digest()),
            format!("tenant_truth:{}", artifact.tenant_truth_basis_digest()),
            format!("tenant_schema:{}", artifact.tenant_schema_basis_digest()),
            format!("branch:{}", artifact.branch_access_digest()),
            format!("shape:{}", artifact.narrowed_result_shape_digest()),
            format!("mode:{}", mode.as_str()),
            format!("counters:{}", hash_parts(&counters.digest_parts())),
        ];
        Self {
            identity: PolicyAwareExecutionSeamIdentity::new(hash_parts(&parts)),
            source_narrowed_artifact_digest: artifact.digest().to_string(),
            authorized_projection_digest: artifact
                .authorized_projection()
                .identity()
                .as_str()
                .to_string(),
            relationship_proof_digest: artifact
                .relationship_proof()
                .identity()
                .as_str()
                .to_string(),
            policy_digest: artifact.policy_digest().to_string(),
            tenant_truth_basis_digest: artifact.tenant_truth_basis_digest().to_string(),
            tenant_schema_basis_digest: artifact.tenant_schema_basis_digest().to_string(),
            branch_access_digest: artifact.branch_access_digest().to_string(),
            narrowed_result_shape_digest: artifact.narrowed_result_shape_digest().to_string(),
            mode,
            counters,
        }
    }

    pub fn identity(&self) -> &PolicyAwareExecutionSeamIdentity {
        &self.identity
    }

    pub fn source_narrowed_artifact_digest(&self) -> &str {
        &self.source_narrowed_artifact_digest
    }

    pub fn authorized_projection_digest(&self) -> &str {
        &self.authorized_projection_digest
    }

    pub fn relationship_proof_digest(&self) -> &str {
        &self.relationship_proof_digest
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_truth_basis_digest(&self) -> &str {
        &self.tenant_truth_basis_digest
    }

    pub fn tenant_schema_basis_digest(&self) -> &str {
        &self.tenant_schema_basis_digest
    }

    pub fn branch_access_digest(&self) -> &str {
        &self.branch_access_digest
    }

    pub fn narrowed_result_shape_digest(&self) -> &str {
        &self.narrowed_result_shape_digest
    }

    pub fn mode(&self) -> PolicyAwareExecutionMode {
        self.mode
    }

    pub fn counters(&self) -> &PolicyAwareSeamCounters {
        &self.counters
    }
}
