use super::WorkflowCounters;
use crate::basis::{BasisAuthorityFamily, ExecutionPreflightBundle};
use crate::correspondence_history::CorrespondenceHistoricalEnvelope;
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::identity::{CanonicalQueryDigest, PlanDigest, ValidatedQueryDigest};
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::preview::{
    preview_lifecycle_state_label, AdmittedPreviewWorkflowFoundation, PreviewEvaluationClass,
    PreviewWorkflowFoundationRequest, PromotionParityPreviewComparisonAdmission,
};
use worth_relational::facade::history::BranchId;
use worth_runtime_bridge::facade::BridgePreviewSessionIdentity;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowDeclarationFamily {
    ConflictInspectionNarrow,
    PostMergeInspectionNarrow,
    MutationLoweringNarrow,
    MergeLoweringNarrow,
    WritebackLoweringNarrow,
}

impl WorkflowDeclarationFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConflictInspectionNarrow => "conflict_inspection_narrow",
            Self::PostMergeInspectionNarrow => "post_merge_inspection_narrow",
            Self::MutationLoweringNarrow => "mutation_lowering_narrow",
            Self::MergeLoweringNarrow => "merge_lowering_narrow",
            Self::WritebackLoweringNarrow => "writeback_lowering_narrow",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowBasisFamily {
    RuntimePreflight,
    PreviewFoundation,
    PreviewPromotionComparison,
    CorrespondenceHistorical,
}

impl WorkflowBasisFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimePreflight => "runtime_preflight",
            Self::PreviewFoundation => "preview_foundation",
            Self::PreviewPromotionComparison => "preview_promotion_comparison",
            Self::CorrespondenceHistorical => "correspondence_historical",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowAuthorityTargetFamily {
    QueryInspection,
    RelationalMutation,
    RelationalMerge,
    BridgeWriteback,
}

impl WorkflowAuthorityTargetFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::QueryInspection => "query_inspection",
            Self::RelationalMutation => "relational_mutation",
            Self::RelationalMerge => "relational_merge",
            Self::BridgeWriteback => "bridge_writeback",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowCostClass {
    InspectionNarrow,
    MutationLoweringNarrow,
    MergeLoweringNarrow,
    WritebackLoweringNarrow,
}

impl WorkflowCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InspectionNarrow => "inspection_narrow",
            Self::MutationLoweringNarrow => "mutation_lowering_narrow",
            Self::MergeLoweringNarrow => "merge_lowering_narrow",
            Self::WritebackLoweringNarrow => "writeback_lowering_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowBudgetClass {
    InspectionBounded,
    AuthorityTargetBounded,
    CrossBoundaryExpansion,
}

impl WorkflowBudgetClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InspectionBounded => "inspection_bounded",
            Self::AuthorityTargetBounded => "authority_target_bounded",
            Self::CrossBoundaryExpansion => "cross_boundary_expansion",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowFreshnessPolicy {
    ExactBasis,
    AllowExplicitRebind,
}

impl WorkflowFreshnessPolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExactBasis => "exact_basis",
            Self::AllowExplicitRebind => "allow_explicit_rebind",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowPredictionDriftOutcome {
    WithinBudget,
    ExplicitBroadeningDenied,
    ExplicitRebindRequired,
}

impl WorkflowPredictionDriftOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::ExplicitBroadeningDenied => "explicit_broadening_denied",
            Self::ExplicitRebindRequired => "explicit_rebind_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowPreviewEvaluationClass {
    ReadOnly,
    PromotionEligible,
}

impl WorkflowPreviewEvaluationClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::PromotionEligible => "promotion_eligible",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkflowAdmissionFailureClass {
    UnsupportedWorkflowFamily,
    UnsupportedBasisFamily,
    InvalidBasisPairing,
    PreviewReadOnlyAuthorityRequestForbidden,
    UnsupportedAuthorityTargetFamily,
    ForbiddenWorkflowBroadening,
    ExplicitRebindRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowAdmissionError {
    failure_class: WorkflowAdmissionFailureClass,
    message: &'static str,
    drift_outcome: WorkflowPredictionDriftOutcome,
    counters: WorkflowCounters,
}

impl WorkflowAdmissionError {
    fn new(
        failure_class: WorkflowAdmissionFailureClass,
        message: &'static str,
        drift_outcome: WorkflowPredictionDriftOutcome,
        counters: WorkflowCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            drift_outcome,
            counters,
        }
    }

    pub fn failure_class(&self) -> &WorkflowAdmissionFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn drift_outcome(&self) -> &WorkflowPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn counters(&self) -> &WorkflowCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowDeclarationRequest {
    declaration_family: WorkflowDeclarationFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
    cost_class: WorkflowCostClass,
    budget_class: WorkflowBudgetClass,
    freshness_policy: WorkflowFreshnessPolicy,
}

impl WorkflowDeclarationRequest {
    pub fn new(
        declaration_family: WorkflowDeclarationFamily,
        authority_target_family: WorkflowAuthorityTargetFamily,
        cost_class: WorkflowCostClass,
        budget_class: WorkflowBudgetClass,
        freshness_policy: WorkflowFreshnessPolicy,
    ) -> Self {
        Self {
            declaration_family,
            authority_target_family,
            cost_class,
            budget_class,
            freshness_policy,
        }
    }

    pub fn declaration_family(&self) -> &WorkflowDeclarationFamily {
        &self.declaration_family
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn cost_class(&self) -> &WorkflowCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &WorkflowBudgetClass {
        &self.budget_class
    }

    pub fn freshness_policy(&self) -> &WorkflowFreshnessPolicy {
        &self.freshness_policy
    }
}

pub enum WorkflowBindingSource<'a> {
    RuntimePreflight(&'a ExecutionPreflightBundle),
    PreviewFoundation(&'a AdmittedPreviewWorkflowFoundation),
    PreviewPromotionComparison(&'a PromotionParityPreviewComparisonAdmission),
    CorrespondenceHistorical(&'a CorrespondenceHistoricalEnvelope),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowContextBinding {
    binding_identity: WorthQueryEvidenceIdentity,
    source_identity: WorthQueryEvidenceIdentity,
    query_identity: WorthQueryEvidenceIdentity,
    basis_family: WorkflowBasisFamily,
    basis_identity: WorthQueryEvidenceIdentity,
    runtime_snapshot_identity: Option<WorthQuerySnapshotIdentity>,
    runtime_target_branch: Option<BranchId>,
    preview_evaluation_class: Option<WorkflowPreviewEvaluationClass>,
    preview_request_family: Option<PreviewWorkflowFoundationRequest>,
    preview_session_identity: Option<BridgePreviewSessionIdentity>,
    counters: WorkflowCounters,
}

impl WorkflowContextBinding {
    pub fn binding_digest(&self) -> &str {
        self.binding_identity.as_str()
    }

    pub fn binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn source_for_reporting(&self) -> &str {
        self.source_identity.as_str()
    }

    pub fn source_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_identity
    }

    pub fn query_for_reporting(&self) -> &str {
        self.query_identity.as_str()
    }

    pub fn query_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.query_identity
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_for_reporting(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn runtime_snapshot_identity(&self) -> Option<&WorthQuerySnapshotIdentity> {
        self.runtime_snapshot_identity.as_ref()
    }

    pub fn runtime_target_branch(&self) -> Option<&BranchId> {
        self.runtime_target_branch.as_ref()
    }

    pub fn preview_evaluation_class(&self) -> Option<&WorkflowPreviewEvaluationClass> {
        self.preview_evaluation_class.as_ref()
    }

    pub fn preview_request_family(&self) -> Option<&PreviewWorkflowFoundationRequest> {
        self.preview_request_family.as_ref()
    }

    pub fn preview_session_identity(&self) -> Option<&BridgePreviewSessionIdentity> {
        self.preview_session_identity.as_ref()
    }

    pub fn counters(&self) -> &WorkflowCounters {
        &self.counters
    }
}

fn workflow_context_binding_identity(
    source_identity: &WorthQueryEvidenceIdentity,
    query_identity: &WorthQueryEvidenceIdentity,
    basis_family: WorkflowBasisFamily,
    basis_identity: &WorthQueryEvidenceIdentity,
    runtime_snapshot_identity: Option<&WorthQuerySnapshotIdentity>,
    binding_scope: Option<&WorkflowBindingScopeField<'_>>,
) -> WorthQueryEvidenceIdentity {
    let mut identity =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
            .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
            .field_evidence_identity(WorthQueryEvidenceTag::new("query"), query_identity)
            .field_shape(
                WorthQueryEvidenceTag::new("basis_family"),
                basis_family.as_str(),
            )
            .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_identity);
    if let Some(runtime_snapshot_identity) = runtime_snapshot_identity {
        identity = identity.field_evidence_identity(
            WorthQueryEvidenceTag::new("runtime_snapshot"),
            &runtime_snapshot_identity.evidence_identity(),
        );
    }
    if let Some(binding_scope) = binding_scope {
        identity = match binding_scope {
            WorkflowBindingScopeField::Unscoped => {
                identity.field_shape(WorthQueryEvidenceTag::new("scope"), "unscoped")
            }
            WorkflowBindingScopeField::Shape(label) => {
                identity.field_shape(WorthQueryEvidenceTag::new("scope"), *label)
            }
            WorkflowBindingScopeField::Identity(scope_identity) => identity
                .field_evidence_identity(WorthQueryEvidenceTag::new("scope"), scope_identity),
        };
    }
    identity.seal()
}

pub(crate) enum WorkflowBindingScopeField<'a> {
    Unscoped,
    Shape(&'a str),
    Identity(&'a WorthQueryEvidenceIdentity),
}

fn workflow_scope_from_label(label: &str) -> WorkflowBindingScopeField<'_> {
    if label == "unscoped" {
        WorkflowBindingScopeField::Unscoped
    } else {
        WorkflowBindingScopeField::Shape(label)
    }
}

fn apply_binding_scope_field(
    identity: crate::evidence_identity::WorthQueryEvidenceIdentityEncoder,
    scope: &WorkflowBindingScopeField<'_>,
) -> crate::evidence_identity::WorthQueryEvidenceIdentityEncoder {
    match scope {
        WorkflowBindingScopeField::Unscoped => {
            identity.field_shape(WorthQueryEvidenceTag::new("scope"), "unscoped")
        }
        WorkflowBindingScopeField::Shape(label) => {
            identity.field_shape(WorthQueryEvidenceTag::new("scope"), *label)
        }
        WorkflowBindingScopeField::Identity(scope_identity) => {
            identity.field_evidence_identity(WorthQueryEvidenceTag::new("scope"), scope_identity)
        }
    }
}

fn binding_scope_for_context_binding<'a>(
    scope: &'a WorkflowBindingScopeField<'a>,
) -> Option<&'a WorkflowBindingScopeField<'a>> {
    match scope {
        WorkflowBindingScopeField::Unscoped => None,
        _ => Some(scope),
    }
}

pub(crate) fn workflow_context_source_identity(
    source_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "workflow_context_source_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("source"), source_identity)
        .seal()
}

pub(crate) fn workflow_context_query_identity(
    query_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "workflow_context_query_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("query"), query_identity)
        .seal()
}

pub(crate) fn workflow_context_basis_identity(
    basis_family: &WorkflowBasisFamily,
    basis_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "workflow_context_basis_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("basis_family"),
            basis_family.as_str(),
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), basis_identity)
        .seal()
}

fn preview_workflow_foundation_binding_identity(
    foundation: &AdmittedPreviewWorkflowFoundation,
) -> WorthQueryEvidenceIdentity {
    let artifact = foundation.artifact();
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_workflow_foundation_binding_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("request"),
            foundation.request_family().as_str(),
        )
        .field_bridge_authority_identity(
            WorthQueryEvidenceTag::new("preview_session"),
            &foundation
                .preview_session_identity()
                .bridge_trust_boundary(),
        )
        .field_bridge_authority_identity(
            WorthQueryEvidenceTag::new("declaration"),
            &foundation.declaration_identity().bridge_trust_boundary(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("validated_query"),
            &workflow_validated_query_digest_evidence(foundation.validated_query_digest()),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("canonical_query"),
            &workflow_canonical_query_digest_evidence(foundation.canonical_query_digest()),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("lifecycle"),
            preview_lifecycle_state_label(artifact.lifecycle_state_kind()),
        )
        .field_bridge_authority_identity(
            WorthQueryEvidenceTag::new("execution_record"),
            &foundation
                .execution_record_identity()
                .bridge_trust_boundary(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("evaluation_class"),
            artifact.evaluation_class().as_str(),
        )
        .seal()
}

fn preview_workflow_foundation_source_identity(
    foundation: &AdmittedPreviewWorkflowFoundation,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_workflow_foundation_source_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("binding"),
            &preview_workflow_foundation_binding_identity(foundation),
        )
        .seal()
}

fn preview_workflow_foundation_basis_inner_identity(
    foundation: &AdmittedPreviewWorkflowFoundation,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_workflow_foundation_basis_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("binding"),
            &preview_workflow_foundation_binding_identity(foundation),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("validated_query"),
            &workflow_validated_query_digest_evidence(foundation.validated_query_digest()),
        )
        .field_bridge_authority_identity(
            WorthQueryEvidenceTag::new("preview_session"),
            &foundation
                .preview_session_identity()
                .bridge_trust_boundary(),
        )
        .seal()
}

fn preview_promotion_comparison_binding_identity(
    comparison: &PromotionParityPreviewComparisonAdmission,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_promotion_comparison_binding_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("validated_query"),
            &workflow_validated_query_digest_evidence(comparison.validated_query_digest()),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("canonical_query"),
            &workflow_canonical_query_digest_evidence(comparison.canonical_query_digest()),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("candidate_result"),
            &preview_candidate_result_identity(comparison.candidate_result_digest()),
        )
        .seal()
}

fn preview_candidate_result_identity(
    result_digest: &crate::identity::ResultDigest,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_candidate_result_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("result_label"),
            result_digest.as_str(),
        )
        .seal()
}

pub(crate) fn workflow_canonical_query_digest_evidence(
    digest: &CanonicalQueryDigest,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "workflow_canonical_query_digest_evidence_v1",
        )
        .field_value(
            WorthQueryEvidenceTag::new("canonical_query_digest"),
            digest.as_str(),
        )
        .seal()
}

pub(crate) fn workflow_validated_query_digest_evidence(
    digest: &ValidatedQueryDigest,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "workflow_validated_query_digest_evidence_v1",
        )
        .field_value(
            WorthQueryEvidenceTag::new("validated_query_digest"),
            digest.as_str(),
        )
        .seal()
}

fn workflow_plan_digest_evidence(digest: &PlanDigest) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "workflow_plan_digest_evidence_v1",
        )
        .field_value(WorthQueryEvidenceTag::new("plan_digest"), digest.as_str())
        .seal()
}

fn preview_promotion_comparison_source_identity(
    comparison: &PromotionParityPreviewComparisonAdmission,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_promotion_comparison_source_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("binding"),
            &preview_promotion_comparison_binding_identity(comparison),
        )
        .seal()
}

fn preview_promotion_comparison_basis_inner_identity(
    comparison: &PromotionParityPreviewComparisonAdmission,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "preview_promotion_comparison_basis_v1",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("binding"),
            &preview_promotion_comparison_binding_identity(comparison),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("validated_query"),
            &workflow_validated_query_digest_evidence(comparison.validated_query_digest()),
        )
        .seal()
}

fn synthetic_runtime_workflow_query_identity(
    source_label: &str,
    binding_scope: &WorkflowBindingScopeField<'_>,
    runtime_snapshot_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    apply_binding_scope_field(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "synthetic_runtime_workflow_query_v1",
            )
            .field_shape(WorthQueryEvidenceTag::new("source_label"), source_label),
        binding_scope,
    )
    .field_evidence_identity(
        WorthQueryEvidenceTag::new("runtime_snapshot"),
        runtime_snapshot_identity,
    )
    .seal()
}

fn synthetic_runtime_workflow_source_identity(
    source_label: &str,
    binding_scope: &WorkflowBindingScopeField<'_>,
    runtime_snapshot_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    apply_binding_scope_field(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "synthetic_runtime_workflow_source_v1",
            )
            .field_shape(WorthQueryEvidenceTag::new("source_label"), source_label),
        binding_scope,
    )
    .field_evidence_identity(
        WorthQueryEvidenceTag::new("runtime_snapshot"),
        runtime_snapshot_identity,
    )
    .seal()
}

fn synthetic_runtime_workflow_basis_identity(
    source_label: &str,
    binding_scope: &WorkflowBindingScopeField<'_>,
    runtime_snapshot_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    apply_binding_scope_field(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "synthetic_runtime_workflow_basis_v1",
            )
            .field_shape(
                WorthQueryEvidenceTag::new("basis_family"),
                WorkflowBasisFamily::RuntimePreflight.as_str(),
            )
            .field_shape(WorthQueryEvidenceTag::new("source_label"), source_label),
        binding_scope,
    )
    .field_evidence_identity(
        WorthQueryEvidenceTag::new("runtime_snapshot"),
        runtime_snapshot_identity,
    )
    .seal()
}

fn synthetic_preview_workflow_query_identity(
    source_label: &str,
    binding_scope: &WorkflowBindingScopeField<'_>,
    preview_session_identity: &BridgePreviewSessionIdentity,
) -> WorthQueryEvidenceIdentity {
    apply_binding_scope_field(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "synthetic_preview_workflow_query_v1",
            )
            .field_shape(WorthQueryEvidenceTag::new("source_label"), source_label),
        binding_scope,
    )
    .field_bridge_authority_identity(
        WorthQueryEvidenceTag::new("preview_session"),
        &preview_session_identity.bridge_trust_boundary(),
    )
    .seal()
}

fn synthetic_preview_workflow_source_identity(
    source_label: &str,
    binding_scope: &WorkflowBindingScopeField<'_>,
    evaluation_class: &WorkflowPreviewEvaluationClass,
    preview_session_identity: &BridgePreviewSessionIdentity,
) -> WorthQueryEvidenceIdentity {
    apply_binding_scope_field(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "synthetic_preview_workflow_source_v1",
            )
            .field_shape(WorthQueryEvidenceTag::new("source_label"), source_label)
            .field_shape(
                WorthQueryEvidenceTag::new("evaluation_class"),
                evaluation_class.as_str(),
            ),
        binding_scope,
    )
    .field_bridge_authority_identity(
        WorthQueryEvidenceTag::new("preview_session"),
        &preview_session_identity.bridge_trust_boundary(),
    )
    .seal()
}

fn synthetic_preview_workflow_basis_identity(
    source_label: &str,
    binding_scope: &WorkflowBindingScopeField<'_>,
    preview_session_identity: &BridgePreviewSessionIdentity,
) -> WorthQueryEvidenceIdentity {
    apply_binding_scope_field(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "synthetic_preview_workflow_basis_v1",
            )
            .field_shape(
                WorthQueryEvidenceTag::new("basis_family"),
                WorkflowBasisFamily::PreviewFoundation.as_str(),
            )
            .field_shape(WorthQueryEvidenceTag::new("source_label"), source_label),
        binding_scope,
    )
    .field_bridge_authority_identity(
        WorthQueryEvidenceTag::new("preview_session"),
        &preview_session_identity.bridge_trust_boundary(),
    )
    .seal()
}

fn workflow_declaration_identity(
    binding_identity: &WorthQueryEvidenceIdentity,
    request: &WorkflowDeclarationRequest,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "workflow_declaration_v1",
        )
        .field_evidence_identity(WorthQueryEvidenceTag::new("binding"), binding_identity)
        .field_shape(
            WorthQueryEvidenceTag::new("declaration_family"),
            request.declaration_family().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("authority_target_family"),
            request.authority_target_family().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("cost_class"),
            request.cost_class().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("budget_class"),
            request.budget_class().as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("freshness_policy"),
            request.freshness_policy().as_str(),
        )
        .seal()
}

pub(crate) fn synthetic_runtime_workflow_binding_for_snapshot_identity(
    source_label: &str,
    runtime_snapshot_identity: WorthQuerySnapshotIdentity,
) -> WorkflowContextBinding {
    synthetic_runtime_workflow_binding_scoped_for_snapshot_identity(
        source_label,
        "unscoped",
        runtime_snapshot_identity,
    )
}

pub(crate) fn synthetic_runtime_workflow_binding_scoped_for_snapshot_identity(
    source_label: &str,
    binding_scope_label: &str,
    runtime_snapshot_identity: WorthQuerySnapshotIdentity,
) -> WorkflowContextBinding {
    let binding_scope = workflow_scope_from_label(binding_scope_label);
    synthetic_runtime_workflow_binding_scoped_for_snapshot_binding_identity(
        source_label,
        &binding_scope,
        runtime_snapshot_identity,
    )
}

pub(crate) fn synthetic_runtime_workflow_binding_scoped_for_snapshot_binding_identity(
    source_label: &str,
    binding_scope: &WorkflowBindingScopeField<'_>,
    runtime_snapshot_identity: WorthQuerySnapshotIdentity,
) -> WorkflowContextBinding {
    synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_binding_identity(
        source_label,
        binding_scope,
        runtime_snapshot_identity,
        BranchId("main".to_string()),
    )
}

pub(crate) fn synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_identity(
    source_label: &str,
    binding_scope_label: &str,
    runtime_snapshot_identity: WorthQuerySnapshotIdentity,
    runtime_target_branch: BranchId,
) -> WorkflowContextBinding {
    let binding_scope = workflow_scope_from_label(binding_scope_label);
    synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_binding_identity(
        source_label,
        &binding_scope,
        runtime_snapshot_identity,
        runtime_target_branch,
    )
}

pub(crate) fn synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_binding_identity(
    source_label: &str,
    binding_scope: &WorkflowBindingScopeField<'_>,
    runtime_snapshot_identity: WorthQuerySnapshotIdentity,
    runtime_target_branch: BranchId,
) -> WorkflowContextBinding {
    let runtime_snapshot_evidence = runtime_snapshot_identity.evidence_identity();
    let source_identity = synthetic_runtime_workflow_source_identity(
        source_label,
        binding_scope,
        &runtime_snapshot_evidence,
    );
    let query_identity = synthetic_runtime_workflow_query_identity(
        source_label,
        binding_scope,
        &runtime_snapshot_evidence,
    );
    let basis_identity = synthetic_runtime_workflow_basis_identity(
        source_label,
        binding_scope,
        &runtime_snapshot_evidence,
    );
    let binding_identity = workflow_context_binding_identity(
        &source_identity,
        &query_identity,
        WorkflowBasisFamily::RuntimePreflight,
        &basis_identity,
        Some(&runtime_snapshot_identity),
        binding_scope_for_context_binding(binding_scope),
    );
    WorkflowContextBinding {
        binding_identity,
        source_identity,
        query_identity,
        basis_family: WorkflowBasisFamily::RuntimePreflight,
        basis_identity,
        runtime_snapshot_identity: Some(runtime_snapshot_identity),
        runtime_target_branch: Some(runtime_target_branch),
        preview_evaluation_class: None,
        preview_request_family: None,
        preview_session_identity: None,
        counters: WorkflowCounters {
            workflow_basis_binding_count: 1,
            workflow_basis_binding_width: 1,
            workflow_executor_rediscovery_count: 0,
            ..WorkflowCounters::default()
        },
    }
}

pub(crate) fn scoped_runtime_preflight_workflow_binding_for_binding_identity(
    preflight: &ExecutionPreflightBundle,
    binding_scope_identity: &WorthQueryEvidenceIdentity,
) -> Result<WorkflowContextBinding, WorkflowAdmissionError> {
    let mut binding = bind_runtime_preflight(preflight)?;
    let binding_scope = WorkflowBindingScopeField::Identity(binding_scope_identity);
    binding.binding_identity = workflow_context_binding_identity(
        &binding.source_identity,
        &binding.query_identity,
        binding.basis_family.clone(),
        &binding.basis_identity,
        binding.runtime_snapshot_identity.as_ref(),
        Some(&binding_scope),
    );
    Ok(binding)
}

pub(crate) fn synthetic_preview_workflow_binding(
    source_label: &str,
    preview_session_identity: BridgePreviewSessionIdentity,
    evaluation_class: WorkflowPreviewEvaluationClass,
) -> WorkflowContextBinding {
    synthetic_preview_workflow_binding_scoped(
        source_label,
        "unscoped",
        preview_session_identity,
        evaluation_class,
    )
}

pub(crate) fn synthetic_preview_workflow_binding_scoped(
    source_label: &str,
    binding_scope_digest: &str,
    preview_session_identity: BridgePreviewSessionIdentity,
    evaluation_class: WorkflowPreviewEvaluationClass,
) -> WorkflowContextBinding {
    synthetic_preview_workflow_binding_request_scoped(
        source_label,
        binding_scope_digest,
        preview_session_identity,
        evaluation_class,
        PreviewWorkflowFoundationRequest::compare_basis_pair(),
    )
}

pub(crate) fn synthetic_preview_workflow_binding_request_scoped(
    source_label: &str,
    binding_scope_label: &str,
    preview_session_identity: BridgePreviewSessionIdentity,
    evaluation_class: WorkflowPreviewEvaluationClass,
    request_family: PreviewWorkflowFoundationRequest,
) -> WorkflowContextBinding {
    let binding_scope = workflow_scope_from_label(binding_scope_label);
    let source_identity = synthetic_preview_workflow_source_identity(
        source_label,
        &binding_scope,
        &evaluation_class,
        &preview_session_identity,
    );
    let query_identity = synthetic_preview_workflow_query_identity(
        source_label,
        &binding_scope,
        &preview_session_identity,
    );
    let basis_identity = synthetic_preview_workflow_basis_identity(
        source_label,
        &binding_scope,
        &preview_session_identity,
    );
    let binding_identity = workflow_context_binding_identity(
        &source_identity,
        &query_identity,
        WorkflowBasisFamily::PreviewFoundation,
        &basis_identity,
        None,
        binding_scope_for_context_binding(&binding_scope),
    );
    WorkflowContextBinding {
        binding_identity,
        source_identity,
        query_identity,
        basis_family: WorkflowBasisFamily::PreviewFoundation,
        basis_identity,
        runtime_snapshot_identity: None,
        runtime_target_branch: None,
        preview_evaluation_class: Some(evaluation_class),
        preview_request_family: Some(request_family),
        preview_session_identity: Some(preview_session_identity),
        counters: WorkflowCounters {
            workflow_basis_binding_count: 1,
            workflow_basis_binding_width: 1,
            workflow_executor_rediscovery_count: 0,
            ..WorkflowCounters::default()
        },
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowAdmissionReport {
    binding_identity: WorthQueryEvidenceIdentity,
    declaration_identity: WorthQueryEvidenceIdentity,
    declaration_family: WorkflowDeclarationFamily,
    basis_family: WorkflowBasisFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
    cost_class: WorkflowCostClass,
    budget_class: WorkflowBudgetClass,
    freshness_policy: WorkflowFreshnessPolicy,
    drift_outcome: WorkflowPredictionDriftOutcome,
    counters: WorkflowCounters,
}

impl WorkflowAdmissionReport {
    pub fn binding_digest(&self) -> &str {
        self.binding_identity.as_str()
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_identity.as_str()
    }

    pub fn binding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.declaration_identity
    }

    pub fn declaration_family(&self) -> &WorkflowDeclarationFamily {
        &self.declaration_family
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn cost_class(&self) -> &WorkflowCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &WorkflowBudgetClass {
        &self.budget_class
    }

    pub fn freshness_policy(&self) -> &WorkflowFreshnessPolicy {
        &self.freshness_policy
    }

    pub fn drift_outcome(&self) -> &WorkflowPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn counters(&self) -> &WorkflowCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryWorkflowDeclaration {
    binding: WorkflowContextBinding,
    request: WorkflowDeclarationRequest,
    report: WorkflowAdmissionReport,
}

impl QueryWorkflowDeclaration {
    pub fn binding(&self) -> &WorkflowContextBinding {
        &self.binding
    }

    pub fn request(&self) -> &WorkflowDeclarationRequest {
        &self.request
    }

    pub fn report(&self) -> &WorkflowAdmissionReport {
        &self.report
    }
}

pub fn bind_workflow_context(
    source: WorkflowBindingSource<'_>,
) -> Result<WorkflowContextBinding, WorkflowAdmissionError> {
    match source {
        WorkflowBindingSource::RuntimePreflight(preflight) => bind_runtime_preflight(preflight),
        WorkflowBindingSource::PreviewFoundation(foundation) => bind_preview_foundation(foundation),
        WorkflowBindingSource::PreviewPromotionComparison(comparison) => {
            bind_preview_promotion_comparison(comparison)
        }
        WorkflowBindingSource::CorrespondenceHistorical(_historical) => {
            Err(WorkflowAdmissionError::new(
                WorkflowAdmissionFailureClass::UnsupportedBasisFamily,
                "correspondence/historical workflow binding remains explicitly denied in phase 1",
                WorkflowPredictionDriftOutcome::WithinBudget,
                WorkflowCounters {
                    workflow_basis_binding_count: 1,
                    workflow_basis_binding_width: 1,
                    workflow_denial_count: 1,
                    ..WorkflowCounters::default()
                },
            ))
        }
    }
}

pub fn admit_query_workflow_declaration(
    binding: &WorkflowContextBinding,
    request: WorkflowDeclarationRequest,
) -> Result<QueryWorkflowDeclaration, WorkflowAdmissionError> {
    if request.budget_class() == &WorkflowBudgetClass::CrossBoundaryExpansion {
        return Err(WorkflowAdmissionError::new(
            WorkflowAdmissionFailureClass::ForbiddenWorkflowBroadening,
            "workflow declarations that require cross-boundary expansion must deny in phase 1",
            WorkflowPredictionDriftOutcome::ExplicitBroadeningDenied,
            WorkflowCounters {
                workflow_declaration_count: 1,
                workflow_basis_binding_count: 1,
                workflow_basis_binding_width: 1,
                workflow_authority_target_check_count: 1,
                workflow_denial_count: 1,
                workflow_broadening_denial_count: 1,
                workflow_executor_rediscovery_count: 0,
            },
        ));
    }

    validate_target_for_family(
        request.declaration_family(),
        request.authority_target_family(),
    )?;
    validate_binding_for_request(binding, &request)?;

    let counters = WorkflowCounters {
        workflow_declaration_count: 1,
        workflow_basis_binding_count: binding.counters().workflow_basis_binding_count(),
        workflow_basis_binding_width: binding.counters().workflow_basis_binding_width(),
        workflow_authority_target_check_count: 1,
        workflow_denial_count: 0,
        workflow_broadening_denial_count: 0,
        workflow_executor_rediscovery_count: 0,
    };
    let declaration_identity = workflow_declaration_identity(binding.binding_identity(), &request);

    Ok(QueryWorkflowDeclaration {
        binding: binding.clone(),
        request: request.clone(),
        report: WorkflowAdmissionReport {
            binding_identity: binding.binding_identity().clone(),
            declaration_identity,
            declaration_family: request.declaration_family().clone(),
            basis_family: binding.basis_family().clone(),
            authority_target_family: request.authority_target_family().clone(),
            cost_class: request.cost_class().clone(),
            budget_class: request.budget_class().clone(),
            freshness_policy: request.freshness_policy().clone(),
            drift_outcome: WorkflowPredictionDriftOutcome::WithinBudget,
            counters,
        },
    })
}

fn bind_runtime_preflight(
    preflight: &ExecutionPreflightBundle,
) -> Result<WorkflowContextBinding, WorkflowAdmissionError> {
    if preflight.basis().identity().authority_family() != &BasisAuthorityFamily::Runtime {
        return Err(WorkflowAdmissionError::new(
            WorkflowAdmissionFailureClass::InvalidBasisPairing,
            "workflow binding requires a runtime-backed execution preflight basis",
            WorkflowPredictionDriftOutcome::WithinBudget,
            WorkflowCounters {
                workflow_basis_binding_count: 1,
                workflow_basis_binding_width: 1,
                workflow_denial_count: 1,
                workflow_executor_rediscovery_count: 0,
                ..WorkflowCounters::default()
            },
        ));
    }

    let plan_query = preflight.plan().query();
    let source_identity =
        workflow_context_source_identity(&workflow_plan_digest_evidence(plan_query.plan_digest()));
    let query_identity = workflow_context_query_identity(
        &workflow_canonical_query_digest_evidence(plan_query.canonical_query_digest()),
    );
    let basis_identity = workflow_context_basis_identity(
        &WorkflowBasisFamily::RuntimePreflight,
        preflight.basis().proof().identity(),
    );
    let runtime_snapshot_identity = WorthQuerySnapshotIdentity::preview(
        preflight.basis().identity().snapshot_identity().clone(),
    );
    let binding_identity = workflow_context_binding_identity(
        &source_identity,
        &query_identity,
        WorkflowBasisFamily::RuntimePreflight,
        &basis_identity,
        Some(&runtime_snapshot_identity),
        None,
    );

    Ok(WorkflowContextBinding {
        binding_identity,
        source_identity,
        query_identity,
        basis_family: WorkflowBasisFamily::RuntimePreflight,
        basis_identity,
        runtime_snapshot_identity: Some(runtime_snapshot_identity),
        runtime_target_branch: Some(BranchId("main".to_string())),
        preview_evaluation_class: None,
        preview_request_family: None,
        preview_session_identity: None,
        counters: WorkflowCounters {
            workflow_basis_binding_count: 1,
            workflow_basis_binding_width: 1,
            workflow_executor_rediscovery_count: 0,
            ..WorkflowCounters::default()
        },
    })
}

fn bind_preview_foundation(
    foundation: &AdmittedPreviewWorkflowFoundation,
) -> Result<WorkflowContextBinding, WorkflowAdmissionError> {
    let evaluation_class = match foundation.evaluation_class() {
        PreviewEvaluationClass::ReadOnly(_) => WorkflowPreviewEvaluationClass::ReadOnly,
        PreviewEvaluationClass::PromotionEligible(_) => {
            WorkflowPreviewEvaluationClass::PromotionEligible
        }
    };
    let source_identity =
        workflow_context_source_identity(&preview_workflow_foundation_source_identity(foundation));
    let query_identity = workflow_context_query_identity(
        &workflow_validated_query_digest_evidence(foundation.validated_query_digest()),
    );
    let basis_identity = workflow_context_basis_identity(
        &WorkflowBasisFamily::PreviewFoundation,
        &preview_workflow_foundation_basis_inner_identity(foundation),
    );
    let binding_identity = workflow_context_binding_identity(
        &source_identity,
        &query_identity,
        WorkflowBasisFamily::PreviewFoundation,
        &basis_identity,
        None,
        None,
    );

    Ok(WorkflowContextBinding {
        binding_identity,
        source_identity,
        query_identity,
        basis_family: WorkflowBasisFamily::PreviewFoundation,
        basis_identity,
        runtime_snapshot_identity: None,
        runtime_target_branch: None,
        preview_evaluation_class: Some(evaluation_class),
        preview_request_family: Some(foundation.request_family().clone()),
        preview_session_identity: Some(foundation.preview_session_identity().clone()),
        counters: WorkflowCounters {
            workflow_basis_binding_count: 1,
            workflow_basis_binding_width: 1,
            workflow_executor_rediscovery_count: 0,
            ..WorkflowCounters::default()
        },
    })
}

fn bind_preview_promotion_comparison(
    comparison: &PromotionParityPreviewComparisonAdmission,
) -> Result<WorkflowContextBinding, WorkflowAdmissionError> {
    let source_identity =
        workflow_context_source_identity(&preview_promotion_comparison_source_identity(comparison));
    let query_identity = workflow_context_query_identity(
        &workflow_validated_query_digest_evidence(comparison.validated_query_digest()),
    );
    let basis_identity = workflow_context_basis_identity(
        &WorkflowBasisFamily::PreviewPromotionComparison,
        &preview_promotion_comparison_basis_inner_identity(comparison),
    );
    let binding_identity = workflow_context_binding_identity(
        &source_identity,
        &query_identity,
        WorkflowBasisFamily::PreviewPromotionComparison,
        &basis_identity,
        None,
        None,
    );

    Ok(WorkflowContextBinding {
        binding_identity,
        source_identity,
        query_identity,
        basis_family: WorkflowBasisFamily::PreviewPromotionComparison,
        basis_identity,
        runtime_snapshot_identity: None,
        runtime_target_branch: None,
        preview_evaluation_class: Some(WorkflowPreviewEvaluationClass::PromotionEligible),
        preview_request_family: None,
        preview_session_identity: None,
        counters: WorkflowCounters {
            workflow_basis_binding_count: 1,
            workflow_basis_binding_width: 1,
            workflow_executor_rediscovery_count: 0,
            ..WorkflowCounters::default()
        },
    })
}

fn validate_target_for_family(
    family: &WorkflowDeclarationFamily,
    target: &WorkflowAuthorityTargetFamily,
) -> Result<(), WorkflowAdmissionError> {
    let supported = matches!(
        (family, target),
        (
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection
        ) | (
            WorkflowDeclarationFamily::MutationLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMutation
        ) | (
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge
        ) | (
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback
        ) | (
            WorkflowDeclarationFamily::PostMergeInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection
        )
    );
    if supported {
        Ok(())
    } else {
        Err(WorkflowAdmissionError::new(
            WorkflowAdmissionFailureClass::UnsupportedAuthorityTargetFamily,
            "workflow declaration family and authority target family must match exactly",
            WorkflowPredictionDriftOutcome::WithinBudget,
            WorkflowCounters {
                workflow_declaration_count: 1,
                workflow_basis_binding_count: 1,
                workflow_basis_binding_width: 1,
                workflow_authority_target_check_count: 1,
                workflow_denial_count: 1,
                workflow_executor_rediscovery_count: 0,
                ..WorkflowCounters::default()
            },
        ))
    }
}

fn validate_binding_for_request(
    binding: &WorkflowContextBinding,
    request: &WorkflowDeclarationRequest,
) -> Result<(), WorkflowAdmissionError> {
    match binding.basis_family() {
        WorkflowBasisFamily::RuntimePreflight => Ok(()),
        WorkflowBasisFamily::PreviewFoundation => {
            if request.declaration_family() == &WorkflowDeclarationFamily::PostMergeInspectionNarrow
            {
                return Err(WorkflowAdmissionError::new(
                    WorkflowAdmissionFailureClass::UnsupportedWorkflowFamily,
                    "post-merge inspection declarations require authoritative workflow basis, not preview foundation context",
                    WorkflowPredictionDriftOutcome::WithinBudget,
                    WorkflowCounters {
                        workflow_declaration_count: 1,
                        workflow_basis_binding_count: 1,
                        workflow_basis_binding_width: 1,
                        workflow_authority_target_check_count: 1,
                        workflow_denial_count: 1,
                        workflow_executor_rediscovery_count: 0,
                        ..WorkflowCounters::default()
                    },
                ));
            }
            if binding.preview_evaluation_class() == Some(&WorkflowPreviewEvaluationClass::ReadOnly)
                && !read_only_preview_request_allows_requested_authority(binding, request)
            {
                return Err(WorkflowAdmissionError::new(
                    WorkflowAdmissionFailureClass::PreviewReadOnlyAuthorityRequestForbidden,
                    "read-only preview workflow contexts may only author inspection declarations",
                    WorkflowPredictionDriftOutcome::WithinBudget,
                    WorkflowCounters {
                        workflow_declaration_count: 1,
                        workflow_basis_binding_count: 1,
                        workflow_basis_binding_width: 1,
                        workflow_authority_target_check_count: 1,
                        workflow_denial_count: 1,
                        workflow_executor_rediscovery_count: 0,
                        ..WorkflowCounters::default()
                    },
                ));
            }
            Ok(())
        }
        WorkflowBasisFamily::PreviewPromotionComparison => {
            if request.declaration_family() == &WorkflowDeclarationFamily::PostMergeInspectionNarrow
            {
                return Err(WorkflowAdmissionError::new(
                    WorkflowAdmissionFailureClass::UnsupportedWorkflowFamily,
                    "post-merge inspection declarations require authoritative workflow basis, not preview comparison context",
                    WorkflowPredictionDriftOutcome::WithinBudget,
                    WorkflowCounters {
                        workflow_declaration_count: 1,
                        workflow_basis_binding_count: 1,
                        workflow_basis_binding_width: 1,
                        workflow_authority_target_check_count: 1,
                        workflow_denial_count: 1,
                        workflow_executor_rediscovery_count: 0,
                        ..WorkflowCounters::default()
                    },
                ));
            }
            if matches!(
                request.authority_target_family(),
                WorkflowAuthorityTargetFamily::RelationalMutation
                    | WorkflowAuthorityTargetFamily::BridgeWriteback
            ) {
                return Err(WorkflowAdmissionError::new(
                    WorkflowAdmissionFailureClass::ExplicitRebindRequired,
                    "preview promotion comparison contexts require explicit rebind before mutation or writeback intent",
                    WorkflowPredictionDriftOutcome::ExplicitRebindRequired,
                    WorkflowCounters {
                        workflow_declaration_count: 1,
                        workflow_basis_binding_count: 1,
                        workflow_basis_binding_width: 1,
                        workflow_authority_target_check_count: 1,
                        workflow_denial_count: 1,
                        workflow_executor_rediscovery_count: 0,
                        ..WorkflowCounters::default()
                    },
                ));
            }
            Ok(())
        }
        WorkflowBasisFamily::CorrespondenceHistorical => Err(WorkflowAdmissionError::new(
            WorkflowAdmissionFailureClass::UnsupportedBasisFamily,
            "correspondence/historical workflow declarations remain denied in phase 1",
            WorkflowPredictionDriftOutcome::WithinBudget,
            WorkflowCounters {
                workflow_declaration_count: 1,
                workflow_basis_binding_count: 1,
                workflow_basis_binding_width: 1,
                workflow_authority_target_check_count: 1,
                workflow_denial_count: 1,
                workflow_executor_rediscovery_count: 0,
                ..WorkflowCounters::default()
            },
        )),
    }
}

fn read_only_preview_request_allows_requested_authority(
    binding: &WorkflowContextBinding,
    request: &WorkflowDeclarationRequest,
) -> bool {
    if request.authority_target_family() == &WorkflowAuthorityTargetFamily::QueryInspection {
        return true;
    }

    request.authority_target_family() == &WorkflowAuthorityTargetFamily::BridgeWriteback
        && request.declaration_family() == &WorkflowDeclarationFamily::WritebackLoweringNarrow
        && binding.preview_request_family()
            == Some(&PreviewWorkflowFoundationRequest::DeferredMutationWriteback)
}
