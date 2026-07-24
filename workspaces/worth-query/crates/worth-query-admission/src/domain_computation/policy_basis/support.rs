use crate::admission_digest::hash_parts;

use super::PolicyExecutionModeRequest;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyTenantPhaseOneSurface {
    PolicyRuleSnapshot,
    TenantBindingSnapshot,
    BranchAccessGrant,
    SchemaVariantSnapshot,
    PolicyWorkBudget,
    SavedQueryPolicyReuseClassification,
    RelationshipProofLowering,
    AuthorizedProjection,
    ExecutionSeamParity,
    DeliveryMetadata,
    DurableStoreBackedArtifacts,
}

impl PolicyTenantPhaseOneSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PolicyRuleSnapshot => "policy_rule_snapshot",
            Self::TenantBindingSnapshot => "tenant_binding_snapshot",
            Self::BranchAccessGrant => "branch_access_grant",
            Self::SchemaVariantSnapshot => "schema_variant_snapshot",
            Self::PolicyWorkBudget => "policy_work_budget",
            Self::SavedQueryPolicyReuseClassification => "saved_query_policy_reuse_classification",
            Self::RelationshipProofLowering => "relationship_proof_lowering",
            Self::AuthorizedProjection => "authorized_projection",
            Self::ExecutionSeamParity => "execution_seam_parity",
            Self::DeliveryMetadata => "delivery_metadata",
            Self::DurableStoreBackedArtifacts => "durable_store_backed_artifacts",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyTenantSupportStatus {
    Verified,
    Deferred,
}

impl PolicyTenantSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyTenantAdmissionSupportProfile {
    admitted_execution_modes: Vec<PolicyExecutionModeRequest>,
    deferred_execution_modes: Vec<PolicyExecutionModeRequest>,
    surfaces: Vec<(PolicyTenantPhaseOneSurface, PolicyTenantSupportStatus)>,
    profile_digest: String,
}

impl PolicyTenantAdmissionSupportProfile {
    pub fn admitted_execution_modes(&self) -> &[PolicyExecutionModeRequest] {
        &self.admitted_execution_modes
    }

    pub fn deferred_execution_modes(&self) -> &[PolicyExecutionModeRequest] {
        &self.deferred_execution_modes
    }

    pub fn surfaces(&self) -> &[(PolicyTenantPhaseOneSurface, PolicyTenantSupportStatus)] {
        &self.surfaces
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }
}

pub fn runtime_backed_policy_tenant_admission_support_profile(
) -> PolicyTenantAdmissionSupportProfile {
    let admitted_execution_modes = vec![
        PolicyExecutionModeRequest::CurrentRead,
        PolicyExecutionModeRequest::BranchRead,
        PolicyExecutionModeRequest::HistoricalRead,
        PolicyExecutionModeRequest::GraphMutation,
    ];
    let deferred_execution_modes = vec![
        PolicyExecutionModeRequest::HistoricalDiff,
        PolicyExecutionModeRequest::LiveSubscription,
        PolicyExecutionModeRequest::DeliveryOnly,
    ];
    let surfaces = vec![
        (
            PolicyTenantPhaseOneSurface::PolicyRuleSnapshot,
            PolicyTenantSupportStatus::Verified,
        ),
        (
            PolicyTenantPhaseOneSurface::TenantBindingSnapshot,
            PolicyTenantSupportStatus::Verified,
        ),
        (
            PolicyTenantPhaseOneSurface::BranchAccessGrant,
            PolicyTenantSupportStatus::Verified,
        ),
        (
            PolicyTenantPhaseOneSurface::SchemaVariantSnapshot,
            PolicyTenantSupportStatus::Verified,
        ),
        (
            PolicyTenantPhaseOneSurface::PolicyWorkBudget,
            PolicyTenantSupportStatus::Verified,
        ),
        (
            PolicyTenantPhaseOneSurface::SavedQueryPolicyReuseClassification,
            PolicyTenantSupportStatus::Verified,
        ),
        (
            PolicyTenantPhaseOneSurface::RelationshipProofLowering,
            PolicyTenantSupportStatus::Deferred,
        ),
        (
            PolicyTenantPhaseOneSurface::AuthorizedProjection,
            PolicyTenantSupportStatus::Deferred,
        ),
        (
            PolicyTenantPhaseOneSurface::ExecutionSeamParity,
            PolicyTenantSupportStatus::Deferred,
        ),
        (
            PolicyTenantPhaseOneSurface::DeliveryMetadata,
            PolicyTenantSupportStatus::Deferred,
        ),
        (
            PolicyTenantPhaseOneSurface::DurableStoreBackedArtifacts,
            PolicyTenantSupportStatus::Deferred,
        ),
    ];
    let profile_digest = hash_parts(&[
        format!(
            "admitted_modes:{}",
            admitted_execution_modes
                .iter()
                .map(PolicyExecutionModeRequest::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "deferred_modes:{}",
            deferred_execution_modes
                .iter()
                .map(PolicyExecutionModeRequest::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "surfaces:{}",
            surfaces
                .iter()
                .map(|(surface, status)| format!("{}:{}", surface.as_str(), status.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]);

    PolicyTenantAdmissionSupportProfile {
        admitted_execution_modes,
        deferred_execution_modes,
        surfaces,
        profile_digest,
    }
}
