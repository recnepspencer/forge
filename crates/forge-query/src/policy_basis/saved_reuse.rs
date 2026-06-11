use crate::query_context::QueryContextFamily;
use crate::saved_query::SavedQueryTemporalAsyncSurfacePosture;

use super::PolicyExecutionModeRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyReuseEquivalenceContract {
    same_policy_basis: bool,
    same_tenant_truth_basis: bool,
    same_tenant_schema_basis: bool,
    same_branch_access: bool,
    same_execution_mode: bool,
}

impl PolicyReuseEquivalenceContract {
    pub fn exact() -> Self {
        Self {
            same_policy_basis: true,
            same_tenant_truth_basis: true,
            same_tenant_schema_basis: true,
            same_branch_access: true,
            same_execution_mode: true,
        }
    }

    pub fn fresh_freeze_required() -> Self {
        Self {
            same_policy_basis: false,
            same_tenant_truth_basis: false,
            same_tenant_schema_basis: false,
            same_branch_access: true,
            same_execution_mode: true,
        }
    }

    pub(crate) fn exact_match(&self) -> bool {
        self.same_policy_basis
            && self.same_tenant_truth_basis
            && self.same_tenant_schema_basis
            && self.same_branch_access
            && self.same_execution_mode
    }

    pub(crate) fn compatible_rebind(&self) -> bool {
        self.same_branch_access && self.same_execution_mode
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedQueryPolicyReuseDisposition {
    LegalNoSemanticChange,
    LegalRequiresFreshFreeze,
    IllegalSemanticDrift,
}

impl SavedQueryPolicyReuseDisposition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LegalNoSemanticChange => "legal_no_semantic_change",
            Self::LegalRequiresFreshFreeze => "legal_requires_fresh_freeze",
            Self::IllegalSemanticDrift => "illegal_semantic_drift",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryPolicyReuseDescriptor {
    saved_query_digest: String,
    prior_policy_digest: String,
    prior_tenant_truth_basis_digest: String,
    prior_tenant_schema_basis_digest: String,
    prior_branch_access_digest: String,
    prior_execution_mode: PolicyExecutionModeRequest,
    new_policy_digest: String,
    new_tenant_truth_basis_digest: String,
    new_tenant_schema_basis_digest: String,
    new_branch_access_digest: String,
    new_execution_mode: PolicyExecutionModeRequest,
    temporal_async_surface_posture: SavedQueryTemporalAsyncSurfacePosture,
    prior_basis_family: Option<QueryContextFamily>,
    new_basis_family: Option<QueryContextFamily>,
    equivalence: Option<PolicyReuseEquivalenceContract>,
}

impl SavedQueryPolicyReuseDescriptor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        saved_query_digest: impl Into<String>,
        prior_policy_digest: impl Into<String>,
        prior_tenant_truth_basis_digest: impl Into<String>,
        prior_tenant_schema_basis_digest: impl Into<String>,
        prior_branch_access_digest: impl Into<String>,
        prior_execution_mode: PolicyExecutionModeRequest,
        new_policy_digest: impl Into<String>,
        new_tenant_truth_basis_digest: impl Into<String>,
        new_tenant_schema_basis_digest: impl Into<String>,
        new_branch_access_digest: impl Into<String>,
        new_execution_mode: PolicyExecutionModeRequest,
    ) -> Self {
        Self {
            saved_query_digest: saved_query_digest.into(),
            prior_policy_digest: prior_policy_digest.into(),
            prior_tenant_truth_basis_digest: prior_tenant_truth_basis_digest.into(),
            prior_tenant_schema_basis_digest: prior_tenant_schema_basis_digest.into(),
            prior_branch_access_digest: prior_branch_access_digest.into(),
            prior_execution_mode,
            new_policy_digest: new_policy_digest.into(),
            new_tenant_truth_basis_digest: new_tenant_truth_basis_digest.into(),
            new_tenant_schema_basis_digest: new_tenant_schema_basis_digest.into(),
            new_branch_access_digest: new_branch_access_digest.into(),
            new_execution_mode,
            temporal_async_surface_posture: SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly,
            prior_basis_family: None,
            new_basis_family: None,
            equivalence: None,
        }
    }

    pub fn with_equivalence(mut self, equivalence: PolicyReuseEquivalenceContract) -> Self {
        self.equivalence = Some(equivalence);
        self
    }

    pub fn with_temporal_async_surface(
        mut self,
        posture: SavedQueryTemporalAsyncSurfacePosture,
        prior_basis_family: Option<QueryContextFamily>,
        new_basis_family: Option<QueryContextFamily>,
    ) -> Self {
        self.temporal_async_surface_posture = posture;
        self.prior_basis_family = prior_basis_family;
        self.new_basis_family = new_basis_family;
        self
    }

    pub fn saved_query_digest(&self) -> &str {
        &self.saved_query_digest
    }

    pub(crate) fn exact_basis_match(&self) -> bool {
        self.prior_policy_digest == self.new_policy_digest
            && self.prior_tenant_truth_basis_digest == self.new_tenant_truth_basis_digest
            && self.prior_tenant_schema_basis_digest == self.new_tenant_schema_basis_digest
            && self.prior_branch_access_digest == self.new_branch_access_digest
            && self.prior_execution_mode == self.new_execution_mode
    }

    pub(crate) fn equivalence(&self) -> Option<&PolicyReuseEquivalenceContract> {
        self.equivalence.as_ref()
    }

    pub(crate) fn temporal_async_surface_posture(&self) -> SavedQueryTemporalAsyncSurfacePosture {
        self.temporal_async_surface_posture
    }

    pub(crate) fn prior_basis_family(&self) -> Option<&QueryContextFamily> {
        self.prior_basis_family.as_ref()
    }

    pub(crate) fn new_basis_family(&self) -> Option<&QueryContextFamily> {
        self.new_basis_family.as_ref()
    }

    pub(crate) fn prior_execution_mode(&self) -> PolicyExecutionModeRequest {
        self.prior_execution_mode
    }

    pub(crate) fn new_execution_mode(&self) -> PolicyExecutionModeRequest {
        self.new_execution_mode
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SavedQueryPolicyReuseEvaluation {
    descriptor: SavedQueryPolicyReuseDescriptor,
    disposition: SavedQueryPolicyReuseDisposition,
}

impl SavedQueryPolicyReuseEvaluation {
    fn saved_query_digest(&self) -> &str {
        self.descriptor.saved_query_digest()
    }

    fn disposition(&self) -> SavedQueryPolicyReuseDisposition {
        self.disposition
    }
}

fn evaluate_saved_query_policy_tenant_reuse(
    descriptor: SavedQueryPolicyReuseDescriptor,
) -> SavedQueryPolicyReuseEvaluation {
    let disposition = classify_saved_query_policy_tenant_reuse(&descriptor);
    SavedQueryPolicyReuseEvaluation {
        descriptor,
        disposition,
    }
}

pub(crate) fn build_saved_query_policy_reuse_evaluation(
    descriptor: SavedQueryPolicyReuseDescriptor,
) -> SavedQueryPolicyReuseEvaluation {
    evaluate_saved_query_policy_tenant_reuse(descriptor)
}

pub(crate) fn saved_query_policy_reuse_artifact_digest(
    evaluation: &SavedQueryPolicyReuseEvaluation,
) -> &str {
    evaluation.saved_query_digest()
}

pub(crate) fn saved_query_policy_reuse_disposition(
    evaluation: &SavedQueryPolicyReuseEvaluation,
) -> SavedQueryPolicyReuseDisposition {
    evaluation.disposition()
}

pub(crate) fn saved_query_policy_reuse_surface_posture(
    evaluation: &SavedQueryPolicyReuseEvaluation,
) -> SavedQueryTemporalAsyncSurfacePosture {
    evaluation.descriptor.temporal_async_surface_posture()
}

pub fn classify_saved_query_policy_tenant_reuse(
    descriptor: &SavedQueryPolicyReuseDescriptor,
) -> SavedQueryPolicyReuseDisposition {
    if !preserves_temporal_async_surface_meaning(descriptor) {
        return SavedQueryPolicyReuseDisposition::IllegalSemanticDrift;
    }

    if descriptor.exact_basis_match() {
        return SavedQueryPolicyReuseDisposition::LegalNoSemanticChange;
    }

    match descriptor.equivalence() {
        Some(equivalence) if equivalence.exact_match() => {
            SavedQueryPolicyReuseDisposition::LegalNoSemanticChange
        }
        Some(equivalence) if equivalence.compatible_rebind() => {
            SavedQueryPolicyReuseDisposition::LegalRequiresFreshFreeze
        }
        _ => SavedQueryPolicyReuseDisposition::IllegalSemanticDrift,
    }
}

fn preserves_temporal_async_surface_meaning(descriptor: &SavedQueryPolicyReuseDescriptor) -> bool {
    match descriptor.temporal_async_surface_posture() {
        SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly => true,
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked => {
            let Some(prior_basis_family) = descriptor.prior_basis_family() else {
                return false;
            };
            let Some(new_basis_family) = descriptor.new_basis_family() else {
                return false;
            };
            basis_family_matches_execution_mode(
                prior_basis_family,
                descriptor.prior_execution_mode(),
            ) && basis_family_matches_execution_mode(
                new_basis_family,
                descriptor.new_execution_mode(),
            )
        }
        SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred => false,
    }
}

fn basis_family_matches_execution_mode(
    basis_family: &QueryContextFamily,
    execution_mode: PolicyExecutionModeRequest,
) -> bool {
    match basis_family {
        QueryContextFamily::CurrentBranchHead => {
            execution_mode == PolicyExecutionModeRequest::CurrentRead
        }
        QueryContextFamily::BranchHead => execution_mode == PolicyExecutionModeRequest::BranchRead,
        QueryContextFamily::HistoricalSnapshot | QueryContextFamily::HistoricalCommit => {
            execution_mode == PolicyExecutionModeRequest::HistoricalRead
        }
        QueryContextFamily::PreviewDerivedHistorical | QueryContextFamily::DiffComparison => false,
    }
}
