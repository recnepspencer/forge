use crate::authorized_projection::{AuthorizedProjectionArtifact, AuthorizedProjectionFieldPath};
use crate::identity::hash_parts;
use crate::policy_basis::{AdmittedPolicyTenantContext, PolicyCostPosture, PolicyWorkBudget};
use crate::relationship_proof::RelationshipProofAdmission;

use super::PolicyNarrowingCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyNarrowingCostPosture {
    ConstantProof,
    BoundedRelationshipProof,
    NonDisclosingFieldUse,
}

impl PolicyNarrowingCostPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConstantProof => "constant_proof",
            Self::BoundedRelationshipProof => "bounded_relationship_proof",
            Self::NonDisclosingFieldUse => "non_disclosing_field_use",
        }
    }

    pub fn from_policy(value: PolicyCostPosture) -> Option<Self> {
        match value {
            PolicyCostPosture::ConstantProof => Some(Self::ConstantProof),
            PolicyCostPosture::BoundedRelationshipProof => Some(Self::BoundedRelationshipProof),
            PolicyCostPosture::NonDisclosingFieldUse => Some(Self::NonDisclosingFieldUse),
            PolicyCostPosture::UnknownCost | PolicyCostPosture::CrossTenantFanout => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PolicyNarrowingWorkBudget {
    max_field_references: usize,
    max_projected_fields: usize,
    max_masked_fields: usize,
    max_relationship_descriptors: usize,
    max_relationship_topology_width: usize,
    max_validation_denials_retained: usize,
    max_digest_part_count: usize,
}

impl PolicyNarrowingWorkBudget {
    #[allow(clippy::too_many_arguments)]
    pub fn bounded(
        max_field_references: usize,
        max_projected_fields: usize,
        max_masked_fields: usize,
        max_relationship_descriptors: usize,
        max_relationship_topology_width: usize,
        max_validation_denials_retained: usize,
        max_digest_part_count: usize,
    ) -> Self {
        Self {
            max_field_references,
            max_projected_fields,
            max_masked_fields,
            max_relationship_descriptors,
            max_relationship_topology_width,
            max_validation_denials_retained,
            max_digest_part_count,
        }
    }

    pub fn from_policy_budget(policy_budget: PolicyWorkBudget) -> Self {
        Self::bounded(
            usize::try_from(policy_budget.max_policy_predicates()).unwrap_or(usize::MAX) + 16,
            usize::try_from(policy_budget.max_policy_predicates()).unwrap_or(usize::MAX) + 16,
            usize::try_from(policy_budget.max_policy_predicates()).unwrap_or(usize::MAX) + 16,
            usize::try_from(policy_budget.max_relationship_checks()).unwrap_or(usize::MAX),
            usize::try_from(policy_budget.max_relationship_checks()).unwrap_or(usize::MAX),
            8,
            64,
        )
    }

    pub fn max_field_references(&self) -> usize {
        self.max_field_references
    }

    pub fn max_projected_fields(&self) -> usize {
        self.max_projected_fields
    }

    pub fn max_masked_fields(&self) -> usize {
        self.max_masked_fields
    }

    pub fn max_relationship_descriptors(&self) -> usize {
        self.max_relationship_descriptors
    }

    pub fn max_relationship_topology_width(&self) -> usize {
        self.max_relationship_topology_width
    }

    pub fn max_validation_denials_retained(&self) -> usize {
        self.max_validation_denials_retained
    }

    pub fn max_digest_part_count(&self) -> usize {
        self.max_digest_part_count
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "narrowing_budget:{}:{}:{}:{}:{}:{}:{}",
            self.max_field_references,
            self.max_projected_fields,
            self.max_masked_fields,
            self.max_relationship_descriptors,
            self.max_relationship_topology_width,
            self.max_validation_denials_retained,
            self.max_digest_part_count
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareValidationReport {
    digest: String,
    failure_digests: Vec<String>,
    counter_snapshot_digest: String,
}

impl PolicyAwareValidationReport {
    pub(crate) fn success(
        authorized_projection: &AuthorizedProjectionArtifact,
        relationship_proof: &RelationshipProofAdmission,
        counters: &PolicyNarrowingCounters,
    ) -> Self {
        let counter_snapshot_digest = hash_parts(&counters.digest_parts());
        let parts = vec![
            format!(
                "authorized_projection:{}",
                authorized_projection.identity().as_str()
            ),
            format!(
                "authorized_influence:{}",
                authorized_projection.influence_set().digest()
            ),
            format!(
                "narrowed_shape:{}",
                authorized_projection.narrowed_result_shape_digest()
            ),
            format!(
                "relationship_proof:{}",
                relationship_proof.identity().as_str()
            ),
            format!("counter_snapshot:{counter_snapshot_digest}"),
        ];
        Self {
            digest: hash_parts(&parts),
            failure_digests: Vec::new(),
            counter_snapshot_digest,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn failure_digests(&self) -> &[String] {
        &self.failure_digests
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }
}

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
    #[allow(clippy::too_many_arguments)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareOptimizerInput {
    source_narrowed_artifact_digest: String,
    authorized_projection_digest: String,
    visible_fields: Vec<AuthorizedProjectionFieldPath>,
    relationship_proof_digest: String,
    validation_report_digest: String,
    optimizer_input_digest: String,
}

impl PolicyAwareOptimizerInput {
    pub(crate) fn from_narrowed(artifact: &NarrowedPolicyQueryArtifact) -> Self {
        let source_narrowed_artifact_digest = artifact.digest().to_string();
        let authorized_projection_digest = artifact
            .authorized_projection()
            .identity()
            .as_str()
            .to_string();
        let visible_fields = artifact
            .authorized_projection()
            .visible_field_paths()
            .to_vec();
        let relationship_proof_digest = artifact
            .relationship_proof()
            .identity()
            .as_str()
            .to_string();
        let validation_report_digest = artifact.validation_report().digest().to_string();
        let mut parts = vec![
            format!("narrowed:{source_narrowed_artifact_digest}"),
            format!("authorized_projection:{authorized_projection_digest}"),
            format!("relationship_proof:{relationship_proof_digest}"),
            format!("validation:{validation_report_digest}"),
        ];
        parts.extend(
            visible_fields
                .iter()
                .map(|field| format!("visible:{}", field.terminal_projection_for_boundary())),
        );
        Self {
            source_narrowed_artifact_digest,
            authorized_projection_digest,
            visible_fields,
            relationship_proof_digest,
            validation_report_digest,
            optimizer_input_digest: hash_parts(&parts),
        }
    }

    pub fn source_narrowed_artifact_digest(&self) -> &str {
        &self.source_narrowed_artifact_digest
    }

    pub fn authorized_projection_digest(&self) -> &str {
        &self.authorized_projection_digest
    }

    pub fn visible_field_paths(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.visible_fields
    }

    pub fn relationship_proof_digest(&self) -> &str {
        &self.relationship_proof_digest
    }

    pub fn validation_report_digest(&self) -> &str {
        &self.validation_report_digest
    }

    pub fn optimizer_input_digest(&self) -> &str {
        &self.optimizer_input_digest
    }
}

impl NarrowedPolicyQueryArtifact {
    #[allow(clippy::too_many_arguments)]
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
