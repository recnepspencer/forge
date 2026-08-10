use crate::authorized_projection::AuthorizedProjectionArtifact;
use crate::identity::hash_parts;
use crate::policy_basis::AdmittedPolicyTenantContext;
use crate::relationship_proof::RelationshipProofAdmission;

use super::{
    PolicyAwareValidationReport, PolicyNarrowingCostPosture, PolicyNarrowingCounters,
    PolicyNarrowingWorkBudget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NarrowedPolicyQueryArtifact {
    digest: String,
    canonical_query_digest: String,
    canonical_result_shape_digest: String,
    narrowed_result_shape_digest: String,
    policy_tenant_admission_digest: String,
    policy_digest: String,
    tenant_truth_basis_digest: String,
    tenant_schema_basis_digest: String,
    branch_access_digest: String,
    authorized_projection: AuthorizedProjectionArtifact,
    relationship_proof: RelationshipProofAdmission,
    validation_report: PolicyAwareValidationReport,
    cost_posture: PolicyNarrowingCostPosture,
    work_budget: PolicyNarrowingWorkBudget,
    counters: PolicyNarrowingCounters,
}

impl NarrowedPolicyQueryArtifact {
    pub(crate) fn new(
        admitted: &AdmittedPolicyTenantContext,
        canonical_result_shape_digest: String,
        authorized_projection: AuthorizedProjectionArtifact,
        relationship_proof: RelationshipProofAdmission,
        validation_report: PolicyAwareValidationReport,
        cost_posture: PolicyNarrowingCostPosture,
        work_budget: PolicyNarrowingWorkBudget,
        counters: PolicyNarrowingCounters,
    ) -> Self {
        let canonical_query_digest = admitted.bundle().canonical_query_digest().to_string();
        let policy_tenant_admission_digest = admitted.bundle().digest().as_str().to_string();
        let policy_digest = admitted.bundle().policy_digest().to_string();
        let tenant_truth_basis_digest = admitted.bundle().tenant_truth_basis_digest().to_string();
        let tenant_schema_basis_digest = admitted.bundle().tenant_schema_basis_digest().to_string();
        let branch_access_digest = admitted.bundle().branch_access_digest().to_string();
        let narrowed_result_shape_digest = authorized_projection
            .narrowed_result_shape_digest()
            .to_string();
        let digest = hash_parts(&[
            format!("query:{canonical_query_digest}"),
            format!("result_shape:{canonical_result_shape_digest}"),
            format!("narrowed_shape:{narrowed_result_shape_digest}"),
            format!(
                "authorized_projection:{}",
                authorized_projection.identity().as_str()
            ),
            format!("admission:{}", admitted.bundle().digest().as_str()),
            format!("policy:{}", admitted.bundle().policy_digest()),
            format!(
                "tenant_truth:{}",
                admitted.bundle().tenant_truth_basis_digest()
            ),
            format!(
                "tenant_schema:{}",
                admitted.bundle().tenant_schema_basis_digest()
            ),
            format!("branch:{}", admitted.bundle().branch_access_digest()),
            format!("proof:{}", relationship_proof.identity().as_str()),
            format!("validation:{}", validation_report.digest()),
            format!("cost_posture:{}", cost_posture.as_str()),
            work_budget.digest_part(),
            format!("counters:{}", validation_report.counter_snapshot_digest()),
        ]);
        Self {
            digest,
            canonical_query_digest,
            canonical_result_shape_digest,
            narrowed_result_shape_digest,
            policy_tenant_admission_digest,
            policy_digest,
            tenant_truth_basis_digest,
            tenant_schema_basis_digest,
            branch_access_digest,
            authorized_projection,
            relationship_proof,
            validation_report,
            cost_posture,
            work_budget,
            counters,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn canonical_query_digest(&self) -> &str {
        &self.canonical_query_digest
    }

    pub fn canonical_result_shape_digest(&self) -> &str {
        &self.canonical_result_shape_digest
    }

    pub fn narrowed_result_shape_digest(&self) -> &str {
        &self.narrowed_result_shape_digest
    }

    pub fn policy_tenant_admission_digest(&self) -> &str {
        &self.policy_tenant_admission_digest
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

    pub fn authorized_projection(&self) -> &AuthorizedProjectionArtifact {
        &self.authorized_projection
    }

    pub fn relationship_proof(&self) -> &RelationshipProofAdmission {
        &self.relationship_proof
    }

    pub fn validation_report(&self) -> &PolicyAwareValidationReport {
        &self.validation_report
    }

    pub fn cost_posture(&self) -> PolicyNarrowingCostPosture {
        self.cost_posture
    }

    pub fn work_budget(&self) -> PolicyNarrowingWorkBudget {
        self.work_budget
    }

    pub fn counters(&self) -> &PolicyNarrowingCounters {
        &self.counters
    }
}
