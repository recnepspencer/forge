use super::{
    PolicyAwareExecutionSeamError, PolicyAwareExecutionSeamFailureClass, PolicyAwareSeamCounters,
};
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyExecutionSeamSurface {
    CurrentPlan,
    BranchPlan,
    RuntimeHistoricalPlan,
    RuntimeHistoricalDiffPlan,
    LiveAdmission,
    DeliveryShape,
    OptimizerInput,
    StoreBackedRetainedHistoricalExecution,
    DurablePolicyCursor,
    DurablePolicyArtifactReload,
}

impl PolicyExecutionSeamSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentPlan => "policy_aware_current_plan",
            Self::BranchPlan => "policy_aware_branch_plan",
            Self::RuntimeHistoricalPlan => "policy_aware_runtime_historical_plan",
            Self::RuntimeHistoricalDiffPlan => "policy_aware_runtime_historical_diff_plan",
            Self::LiveAdmission => "policy_aware_live_admission",
            Self::DeliveryShape => "policy_aware_delivery_shape",
            Self::OptimizerInput => "policy_aware_optimizer_input",
            Self::StoreBackedRetainedHistoricalExecution => {
                "policy_aware_store_backed_retained_historical_execution"
            }
            Self::DurablePolicyCursor => "durable_policy_cursor",
            Self::DurablePolicyArtifactReload => "durable_policy_artifact_reload",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyExecutionSeamSupportStatus {
    Verified,
    LimitedAdmission,
    Deferred,
    BlockedOnForgeStore,
}

impl PolicyExecutionSeamSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::LimitedAdmission => "limited_admission",
            Self::Deferred => "deferred",
            Self::BlockedOnForgeStore => "blocked_on_forge_store",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyExecutionSeamSupportProfile {
    surfaces: Vec<(PolicyExecutionSeamSurface, PolicyExecutionSeamSupportStatus)>,
    profile_digest: String,
}

impl PolicyExecutionSeamSupportProfile {
    pub(crate) fn new(
        surfaces: Vec<(PolicyExecutionSeamSurface, PolicyExecutionSeamSupportStatus)>,
    ) -> Self {
        let profile_digest = hash_parts(
            &surfaces
                .iter()
                .map(|(surface, status)| format!("{}:{}", surface.as_str(), status.as_str()))
                .collect::<Vec<_>>(),
        );
        Self {
            surfaces,
            profile_digest,
        }
    }

    pub fn surfaces(&self) -> &[(PolicyExecutionSeamSurface, PolicyExecutionSeamSupportStatus)] {
        &self.surfaces
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyExecutionSeamHandoffReport {
    milestone_ten_store_backed_handoff: Vec<&'static str>,
    milestone_eleven_durable_handoff: Vec<&'static str>,
    runtime_backed_verified_surface_count: usize,
    limited_admission_surface_count: usize,
    blocked_or_deferred_surface_count: usize,
    handoff_digest: String,
}

impl PolicyExecutionSeamHandoffReport {
    pub(crate) fn new(profile: &PolicyExecutionSeamSupportProfile) -> Self {
        let milestone_ten_store_backed_handoff = vec![
            "store-backed historical restore breadth expansion",
            "store-backed diff execution parity",
            "runtime/store parity over broader policy-aware query plans",
        ];
        let milestone_eleven_durable_handoff = vec![
            "durable tenant/query artifacts",
            "durable saved-query policy reload",
            "durable delivery cursors",
            "durable policy delivery metadata reload",
            "restart-stable subscription metadata",
        ];
        let runtime_backed_verified_surface_count = profile
            .surfaces()
            .iter()
            .filter(|(_, status)| *status == PolicyExecutionSeamSupportStatus::Verified)
            .count();
        let limited_admission_surface_count = profile
            .surfaces()
            .iter()
            .filter(|(_, status)| *status == PolicyExecutionSeamSupportStatus::LimitedAdmission)
            .count();
        let blocked_or_deferred_surface_count = profile
            .surfaces()
            .iter()
            .filter(|(_, status)| {
                matches!(
                    status,
                    PolicyExecutionSeamSupportStatus::Deferred
                        | PolicyExecutionSeamSupportStatus::BlockedOnForgeStore
                )
            })
            .count();
        let mut parts = Vec::new();
        parts.extend(
            milestone_ten_store_backed_handoff
                .iter()
                .map(|item| format!("m10:{item}")),
        );
        parts.extend(
            milestone_eleven_durable_handoff
                .iter()
                .map(|item| format!("m11:{item}")),
        );
        parts.push(format!(
            "runtime_verified:{runtime_backed_verified_surface_count}"
        ));
        parts.push(format!(
            "limited_admission:{limited_admission_surface_count}"
        ));
        parts.push(format!(
            "blocked_or_deferred:{blocked_or_deferred_surface_count}"
        ));
        parts.push(format!("profile:{}", profile.profile_digest()));
        let handoff_digest = hash_parts(&parts);
        Self {
            milestone_ten_store_backed_handoff,
            milestone_eleven_durable_handoff,
            runtime_backed_verified_surface_count,
            limited_admission_surface_count,
            blocked_or_deferred_surface_count,
            handoff_digest,
        }
    }

    pub fn milestone_ten_store_backed_handoff(&self) -> &[&'static str] {
        &self.milestone_ten_store_backed_handoff
    }

    pub fn milestone_eleven_durable_handoff(&self) -> &[&'static str] {
        &self.milestone_eleven_durable_handoff
    }

    pub fn runtime_backed_verified_surface_count(&self) -> usize {
        self.runtime_backed_verified_surface_count
    }

    pub fn limited_admission_surface_count(&self) -> usize {
        self.limited_admission_surface_count
    }

    pub fn blocked_or_deferred_surface_count(&self) -> usize {
        self.blocked_or_deferred_surface_count
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

pub fn runtime_backed_policy_execution_seam_support_profile() -> PolicyExecutionSeamSupportProfile {
    PolicyExecutionSeamSupportProfile::new(vec![
        (
            PolicyExecutionSeamSurface::CurrentPlan,
            PolicyExecutionSeamSupportStatus::Verified,
        ),
        (
            PolicyExecutionSeamSurface::BranchPlan,
            PolicyExecutionSeamSupportStatus::Verified,
        ),
        (
            PolicyExecutionSeamSurface::RuntimeHistoricalPlan,
            PolicyExecutionSeamSupportStatus::Verified,
        ),
        (
            PolicyExecutionSeamSurface::RuntimeHistoricalDiffPlan,
            PolicyExecutionSeamSupportStatus::Verified,
        ),
        (
            PolicyExecutionSeamSurface::LiveAdmission,
            PolicyExecutionSeamSupportStatus::Verified,
        ),
        (
            PolicyExecutionSeamSurface::DeliveryShape,
            PolicyExecutionSeamSupportStatus::Verified,
        ),
        (
            PolicyExecutionSeamSurface::OptimizerInput,
            PolicyExecutionSeamSupportStatus::Verified,
        ),
        (
            PolicyExecutionSeamSurface::StoreBackedRetainedHistoricalExecution,
            PolicyExecutionSeamSupportStatus::LimitedAdmission,
        ),
        (
            PolicyExecutionSeamSurface::DurablePolicyCursor,
            PolicyExecutionSeamSupportStatus::Deferred,
        ),
        (
            PolicyExecutionSeamSurface::DurablePolicyArtifactReload,
            PolicyExecutionSeamSupportStatus::Deferred,
        ),
    ])
}

pub fn runtime_backed_policy_execution_seam_handoff_report() -> PolicyExecutionSeamHandoffReport {
    PolicyExecutionSeamHandoffReport::new(&runtime_backed_policy_execution_seam_support_profile())
}

pub fn deny_durable_policy_cursor_claim() -> PolicyAwareExecutionSeamError {
    PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::DurablePolicyCursorDeferred,
        "durable policy cursor resume is deferred until the Milestone 11 durable query surface",
        PolicyAwareSeamCounters::deferred_durable_cursor(),
    )
}

pub fn deny_durable_policy_artifact_reload_claim() -> PolicyAwareExecutionSeamError {
    PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::DurablePolicyArtifactReloadDeferred,
        "durable policy artifact reload is deferred until the Milestone 11 durable query surface",
        PolicyAwareSeamCounters::deferred_durable_artifact_reload(),
    )
}

pub fn deny_durable_policy_delivery_metadata_reload_claim() -> PolicyAwareExecutionSeamError {
    PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::DurablePolicyDeliveryMetadataDeferred,
        "durable policy delivery metadata reload is deferred until the Milestone 11 durable query surface",
        PolicyAwareSeamCounters::deferred_durable_delivery_metadata(),
    )
}

pub fn deny_policy_per_row_allocation_claim() -> PolicyAwareExecutionSeamError {
    PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::PerRowPolicyAllocationForbidden,
        "policy-aware hot paths must not allocate or rediscover policy semantics per row",
        PolicyAwareSeamCounters::denied_per_row_allocation(),
    )
}

pub fn deny_policy_cross_tenant_fanout_claim() -> PolicyAwareExecutionSeamError {
    PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::CrossTenantPolicyFanoutForbidden,
        "policy-aware execution cannot fan out across tenant bases without explicit tenant-scoped admission",
        PolicyAwareSeamCounters::denied_cross_tenant_fanout(),
    )
}

pub fn deny_saved_query_policy_bypass_claim() -> PolicyAwareExecutionSeamError {
    PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::SavedQueryPolicyBypassForbidden,
        "saved-query reuse cannot bypass fresh policy narrowing before plan lowering",
        PolicyAwareSeamCounters::denied_saved_query_policy_bypass(),
    )
}

pub fn deny_unsupported_policy_workflow_composition_claim() -> PolicyAwareExecutionSeamError {
    PolicyAwareExecutionSeamError::new(
        PolicyAwareExecutionSeamFailureClass::UnsupportedPolicyWorkflowComposition,
        "unsupported policy/workflow composition is denied before execution",
        PolicyAwareSeamCounters::denied_unsupported_policy_workflow_composition(),
    )
}
