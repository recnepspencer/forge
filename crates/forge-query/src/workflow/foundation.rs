use super::WorkflowCounters;
use crate::basis::{BasisAuthorityFamily, ExecutionPreflightBundle};
use crate::correspondence_history::CorrespondenceHistoricalEnvelope;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::preview::{
    AdmittedPreviewWorkflowFoundation, PreviewEvaluationClass, PreviewWorkflowFoundationRequest,
    PromotionParityPreviewComparisonAdmission,
};
use forge_relational::facade::history::BranchId;
use forge_runtime_bridge::facade::BridgePreviewSessionIdentity;

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
    digest: String,
    source_digest: String,
    query_identity_digest: String,
    basis_family: WorkflowBasisFamily,
    basis_digest: String,
    runtime_snapshot_identity: Option<ForgeQuerySnapshotIdentity>,
    runtime_target_branch: Option<BranchId>,
    preview_evaluation_class: Option<WorkflowPreviewEvaluationClass>,
    preview_request_family: Option<PreviewWorkflowFoundationRequest>,
    preview_session_identity: Option<BridgePreviewSessionIdentity>,
    counters: WorkflowCounters,
}

impl WorkflowContextBinding {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn binding_identity(&self) -> ForgeQueryEvidenceIdentity {
        workflow_context_binding_identity(
            &self.source_digest,
            &self.query_identity_digest,
            self.basis_family.clone(),
            &self.basis_digest,
            self.runtime_snapshot_identity.as_ref(),
            None,
        )
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn query_identity_digest(&self) -> &str {
        &self.query_identity_digest
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn basis_identity(&self) -> ForgeQueryEvidenceIdentity {
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowContextBinding)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "workflow_context_basis_v1",
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("basis_family"),
                self.basis_family.as_str(),
            )
            .field_identity(ForgeQueryEvidenceTag::new("basis"), &self.basis_digest)
            .seal()
    }

    pub fn runtime_snapshot_identity(&self) -> Option<&ForgeQuerySnapshotIdentity> {
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
    source_digest: &str,
    query_identity_digest: &str,
    basis_family: WorkflowBasisFamily,
    basis_digest: &str,
    runtime_snapshot_identity: Option<&ForgeQuerySnapshotIdentity>,
    binding_scope_identity: Option<&ForgeQueryEvidenceIdentity>,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowContextBinding)
            .field_identity(ForgeQueryEvidenceTag::new("source"), source_digest)
            .field_identity(ForgeQueryEvidenceTag::new("query"), query_identity_digest)
            .field_shape(
                ForgeQueryEvidenceTag::new("basis_family"),
                basis_family.as_str(),
            )
            .field_identity(ForgeQueryEvidenceTag::new("basis"), basis_digest);
    if let Some(runtime_snapshot_identity) = runtime_snapshot_identity {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime_snapshot"),
            &runtime_snapshot_identity.evidence_identity(),
        );
    }
    if let Some(binding_scope_identity) = binding_scope_identity {
        identity = identity
            .field_evidence_identity(ForgeQueryEvidenceTag::new("scope"), binding_scope_identity);
    }
    identity.seal()
}

fn workflow_scope_digest_identity(binding_scope_digest: &str) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowContextBinding)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "workflow_context_binding_scope_digest_v1",
        )
        .field_identity(ForgeQueryEvidenceTag::new("scope"), binding_scope_digest)
        .seal()
}

pub(crate) fn synthetic_runtime_workflow_binding_for_snapshot_identity(
    source_label: &str,
    runtime_snapshot_identity: ForgeQuerySnapshotIdentity,
) -> WorkflowContextBinding {
    synthetic_runtime_workflow_binding_scoped_for_snapshot_identity(
        source_label,
        "unscoped",
        runtime_snapshot_identity,
    )
}

pub(crate) fn synthetic_runtime_workflow_binding_scoped_for_snapshot_identity(
    source_label: &str,
    binding_scope_digest: &str,
    runtime_snapshot_identity: ForgeQuerySnapshotIdentity,
) -> WorkflowContextBinding {
    let binding_scope_identity = workflow_scope_digest_identity(binding_scope_digest);
    synthetic_runtime_workflow_binding_scoped_for_snapshot_binding_identity(
        source_label,
        &binding_scope_identity,
        runtime_snapshot_identity,
    )
}

pub(crate) fn synthetic_runtime_workflow_binding_scoped_for_snapshot_binding_identity(
    source_label: &str,
    binding_scope_identity: &ForgeQueryEvidenceIdentity,
    runtime_snapshot_identity: ForgeQuerySnapshotIdentity,
) -> WorkflowContextBinding {
    synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_binding_identity(
        source_label,
        binding_scope_identity,
        runtime_snapshot_identity,
        BranchId("main".to_string()),
    )
}

pub(crate) fn synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_identity(
    source_label: &str,
    binding_scope_digest: &str,
    runtime_snapshot_identity: ForgeQuerySnapshotIdentity,
    runtime_target_branch: BranchId,
) -> WorkflowContextBinding {
    let binding_scope_identity = workflow_scope_digest_identity(binding_scope_digest);
    synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_binding_identity(
        source_label,
        &binding_scope_identity,
        runtime_snapshot_identity,
        runtime_target_branch,
    )
}

pub(crate) fn synthetic_runtime_workflow_binding_scoped_for_branch_snapshot_binding_identity(
    source_label: &str,
    binding_scope_identity: &ForgeQueryEvidenceIdentity,
    runtime_snapshot_identity: ForgeQuerySnapshotIdentity,
    runtime_target_branch: BranchId,
) -> WorkflowContextBinding {
    let runtime_snapshot_evidence = runtime_snapshot_identity.evidence_identity();
    let query_identity_digest = hash_parts(&[
        format!("synthetic_query:{source_label}"),
        binding_scope_identity.as_ref().to_string(),
        "basis:runtime".to_string(),
    ]);
    let source_digest = hash_parts(&[
        format!("synthetic_source:{source_label}"),
        binding_scope_identity.as_ref().to_string(),
        runtime_snapshot_evidence.as_ref().to_string(),
    ]);
    let basis_digest = hash_parts(&[
        format!("synthetic_basis:{source_label}"),
        binding_scope_identity.as_ref().to_string(),
        runtime_snapshot_evidence.as_ref().to_string(),
    ]);
    let digest = workflow_context_binding_identity(
        &source_digest,
        &query_identity_digest,
        WorkflowBasisFamily::RuntimePreflight,
        &basis_digest,
        Some(&runtime_snapshot_identity),
        Some(binding_scope_identity),
    )
    .as_ref()
    .to_string();
    WorkflowContextBinding {
        digest,
        source_digest,
        query_identity_digest,
        basis_family: WorkflowBasisFamily::RuntimePreflight,
        basis_digest,
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
    binding_scope_identity: &ForgeQueryEvidenceIdentity,
) -> Result<WorkflowContextBinding, WorkflowAdmissionError> {
    let mut binding = bind_runtime_preflight(preflight)?;
    binding.digest = workflow_context_binding_identity(
        &binding.source_digest,
        &binding.query_identity_digest,
        binding.basis_family.clone(),
        &binding.basis_digest,
        binding.runtime_snapshot_identity.as_ref(),
        Some(binding_scope_identity),
    )
    .as_ref()
    .to_string();
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
    binding_scope_digest: &str,
    preview_session_identity: BridgePreviewSessionIdentity,
    evaluation_class: WorkflowPreviewEvaluationClass,
    request_family: PreviewWorkflowFoundationRequest,
) -> WorkflowContextBinding {
    let query_identity_digest = hash_parts(&[
        format!("synthetic_query:{source_label}"),
        format!("scope:{binding_scope_digest}"),
        format!(
            "preview_session:{}",
            preview_session_identity.evidence_identity().as_str()
        ),
    ]);
    let source_digest = hash_parts(&[
        format!("synthetic_source:{source_label}"),
        format!("scope:{binding_scope_digest}"),
        format!("evaluation:{}", evaluation_class.as_str()),
        format!(
            "preview_session:{}",
            preview_session_identity.evidence_identity().as_str()
        ),
    ]);
    let basis_digest = hash_parts(&[
        format!("synthetic_basis:{source_label}"),
        format!("scope:{binding_scope_digest}"),
        format!(
            "preview_session:{}",
            preview_session_identity.evidence_identity().as_str()
        ),
    ]);
    let digest = hash_parts(&[
        format!("source:{source_digest}"),
        format!("query:{query_identity_digest}"),
        format!(
            "basis_family:{}",
            WorkflowBasisFamily::PreviewFoundation.as_str()
        ),
        format!("basis:{basis_digest}"),
        format!("evaluation:{}", evaluation_class.as_str()),
        format!("request_family:{}", request_family.as_str()),
        format!(
            "preview_session:{}",
            preview_session_identity.evidence_identity().as_str()
        ),
    ]);
    WorkflowContextBinding {
        digest,
        source_digest,
        query_identity_digest,
        basis_family: WorkflowBasisFamily::PreviewFoundation,
        basis_digest,
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
    binding_digest: String,
    declaration_digest: String,
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
        &self.binding_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
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
    let declaration_digest = hash_parts(&[
        format!("binding:{}", binding.digest()),
        format!("family:{}", request.declaration_family().as_str()),
        format!("target:{}", request.authority_target_family().as_str()),
        format!("cost:{}", request.cost_class().as_str()),
        format!("budget:{}", request.budget_class().as_str()),
        format!("freshness:{}", request.freshness_policy().as_str()),
    ]);

    Ok(QueryWorkflowDeclaration {
        binding: binding.clone(),
        request: request.clone(),
        report: WorkflowAdmissionReport {
            binding_digest: binding.digest().to_string(),
            declaration_digest,
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

    let query_identity_digest = preflight.plan().query().canonical_query_digest().as_str();
    let basis_digest = preflight.basis().proof().digest().as_str();
    let runtime_snapshot_identity = ForgeQuerySnapshotIdentity::preview(
        preflight.basis().identity().snapshot_identity().clone(),
    );
    let source_digest = preflight.plan().query().plan_digest().as_str();
    let digest = workflow_context_binding_identity(
        source_digest,
        query_identity_digest,
        WorkflowBasisFamily::RuntimePreflight,
        basis_digest,
        Some(&runtime_snapshot_identity),
        None,
    )
    .as_ref()
    .to_string();

    Ok(WorkflowContextBinding {
        digest,
        source_digest: source_digest.to_string(),
        query_identity_digest: query_identity_digest.to_string(),
        basis_family: WorkflowBasisFamily::RuntimePreflight,
        basis_digest: basis_digest.to_string(),
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
    let source_digest = foundation.digest();
    let query_identity_digest = foundation.validated_query_digest().as_str();
    let basis_digest = foundation.binding_digest();
    let digest = hash_parts(&[
        format!("source:{source_digest}"),
        format!("query:{query_identity_digest}"),
        format!(
            "basis_family:{}",
            WorkflowBasisFamily::PreviewFoundation.as_str()
        ),
        format!("basis:{basis_digest}"),
        format!("evaluation:{}", evaluation_class.as_str()),
        format!(
            "preview_session:{}",
            foundation
                .preview_session_identity()
                .evidence_identity()
                .as_str()
        ),
    ]);

    Ok(WorkflowContextBinding {
        digest,
        source_digest: source_digest.to_string(),
        query_identity_digest: query_identity_digest.to_string(),
        basis_family: WorkflowBasisFamily::PreviewFoundation,
        basis_digest: basis_digest.to_string(),
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
    let source_digest = comparison.digest();
    let query_identity_digest = comparison.validated_query_digest().as_str();
    let basis_digest = comparison.candidate_basis_digest();
    let digest = hash_parts(&[
        format!("source:{source_digest}"),
        format!("query:{query_identity_digest}"),
        format!(
            "basis_family:{}",
            WorkflowBasisFamily::PreviewPromotionComparison.as_str()
        ),
        format!("basis:{basis_digest}"),
    ]);

    Ok(WorkflowContextBinding {
        digest,
        source_digest: source_digest.to_string(),
        query_identity_digest: query_identity_digest.to_string(),
        basis_family: WorkflowBasisFamily::PreviewPromotionComparison,
        basis_digest: basis_digest.to_string(),
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
