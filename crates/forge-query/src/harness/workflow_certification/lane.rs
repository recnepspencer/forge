use crate::harness::certification::{digest_parts, CertificationMatrix};
use crate::workflow::{
    QueryWorkflowDeclaration, WorkflowAdmissionError, WorkflowAdmissionFailureClass,
    WorkflowCounters, WorkflowInspectionError, WorkflowInspectionFailureClass,
    WorkflowLoweringError, WorkflowLoweringFailureClass, WorkflowPredictionDriftOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowPerturbationClass {
    DeclarationFamily,
    BasisFamily,
    AuthorityTargetFamily,
    NoRediscovery,
    BudgetClass,
    MutationParity,
    Freshness,
    PredictionWidth,
    RealizedWidth,
    RediscoveryZero,
    UnsupportedWorkflowFamily,
    InvalidBasisPairing,
    PreviewReadOnlyAuthority,
    UnsupportedAuthorityTarget,
    ForbiddenBroadening,
    LoweringParity,
    WritebackParity,
    ConflictInspection,
    PostMergeInspection,
    DeniedMergeClass,
    ExplicitRebindRequired,
    PostMergeOutcomeForbidden,
    CompileFailBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowFailureClass {
    UnsupportedWorkflowFamily,
    InvalidBasisPairing,
    PreviewReadOnlyAuthorityRequestForbidden,
    UnsupportedAuthorityTargetFamily,
    ForbiddenWorkflowBroadening,
    StaleWorkflowDenied,
    ExplicitRebindRequired,
    AmbientBasisFallbackForbidden,
    PostMergeOutcomeForbidden,
    CompileFail,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowCertificationLane {
    pub query_digest: String,
    pub plan_digest: String,
    pub result_digest: String,
    pub delivery_digest: String,
    pub failure_digest: String,
    pub counter_snapshot_digest: String,
    pub binding_digest: String,
    pub declaration_digest: String,
    pub basis_family: String,
    pub declaration_family: String,
    pub authority_target_family: String,
    pub cost_class: String,
    pub budget_class: String,
    pub freshness_policy: String,
    pub preview_session_identity: Option<String>,
    pub lowered_request_digest: Option<String>,
    pub lowered_freshness_binding: Option<String>,
    pub authority_outcome_family: Option<String>,
    pub inspection_family: Option<String>,
    pub replay_bundle_digest: Option<String>,
    pub prediction_drift_outcome: Option<String>,
    pub budget_outcome: Option<String>,
    pub predicted_declaration_width: Option<usize>,
    pub predicted_inspection_width: Option<usize>,
    pub predicted_lowering_width: Option<usize>,
    pub realized_width: Option<usize>,
    pub lowering_width: Option<usize>,
    pub inspection_row_width: Option<usize>,
    pub declaration_executor_rediscovery_count: usize,
    pub lowering_executor_rediscovery_count: Option<usize>,
    pub inspection_executor_rediscovery_count: Option<usize>,
    pub replay_executor_rediscovery_count: Option<usize>,
    pub counters: WorkflowCounters,
}

impl WorkflowCertificationLane {
    pub(crate) fn from_declaration(declaration: &QueryWorkflowDeclaration) -> Self {
        let counters = declaration.report().counters().clone();
        Self {
            query_digest: declaration.binding().query_for_reporting().to_string(),
            plan_digest: declaration.binding().source_for_reporting().to_string(),
            result_digest: declaration.report().declaration_digest().to_string(),
            delivery_digest: declaration.report().binding_digest().to_string(),
            failure_digest: "none".to_string(),
            counter_snapshot_digest: digest_parts(&[
                format!("declarations:{}", counters.workflow_declaration_count()),
                format!("basis_bindings:{}", counters.workflow_basis_binding_count()),
                format!("basis_width:{}", counters.workflow_basis_binding_width()),
                format!(
                    "authority_checks:{}",
                    counters.workflow_authority_target_check_count()
                ),
                format!("denials:{}", counters.workflow_denial_count()),
                format!(
                    "broadening_denials:{}",
                    counters.workflow_broadening_denial_count()
                ),
                format!(
                    "executor_rediscovery:{}",
                    counters.workflow_executor_rediscovery_count()
                ),
            ]),
            binding_digest: declaration.binding().binding_digest().to_string(),
            declaration_digest: declaration.report().declaration_digest().to_string(),
            basis_family: declaration.report().basis_family().as_str().to_string(),
            declaration_family: declaration
                .report()
                .declaration_family()
                .as_str()
                .to_string(),
            authority_target_family: declaration
                .report()
                .authority_target_family()
                .as_str()
                .to_string(),
            cost_class: declaration.report().cost_class().as_str().to_string(),
            budget_class: declaration.report().budget_class().as_str().to_string(),
            freshness_policy: declaration.report().freshness_policy().as_str().to_string(),
            preview_session_identity: declaration
                .binding()
                .preview_session_identity()
                .map(|identity| identity.evidence_identity().as_str().to_string()),
            lowered_request_digest: None,
            lowered_freshness_binding: None,
            authority_outcome_family: None,
            inspection_family: None,
            replay_bundle_digest: None,
            prediction_drift_outcome: Some(
                declaration.report().drift_outcome().as_str().to_string(),
            ),
            budget_outcome: None,
            predicted_declaration_width: None,
            predicted_inspection_width: None,
            predicted_lowering_width: None,
            realized_width: None,
            lowering_width: None,
            inspection_row_width: None,
            declaration_executor_rediscovery_count: declaration
                .report()
                .counters()
                .workflow_executor_rediscovery_count(),
            lowering_executor_rediscovery_count: None,
            inspection_executor_rediscovery_count: None,
            replay_executor_rediscovery_count: None,
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowCertificationRejection {
    pub failure_class: WorkflowFailureClass,
    pub failure_digest: String,
    pub counter_snapshot_digest: String,
    pub drift_outcome: Option<WorkflowPredictionDriftOutcome>,
    pub counters: Option<WorkflowCounters>,
    pub compile_fail_case: Option<&'static str>,
}

impl WorkflowCertificationRejection {
    pub(crate) fn from_error(error: &WorkflowAdmissionError) -> Self {
        let failure_class = match error.failure_class() {
            WorkflowAdmissionFailureClass::UnsupportedWorkflowFamily => {
                WorkflowFailureClass::UnsupportedWorkflowFamily
            }
            WorkflowAdmissionFailureClass::InvalidBasisPairing => {
                WorkflowFailureClass::InvalidBasisPairing
            }
            WorkflowAdmissionFailureClass::PreviewReadOnlyAuthorityRequestForbidden => {
                WorkflowFailureClass::PreviewReadOnlyAuthorityRequestForbidden
            }
            WorkflowAdmissionFailureClass::UnsupportedAuthorityTargetFamily => {
                WorkflowFailureClass::UnsupportedAuthorityTargetFamily
            }
            WorkflowAdmissionFailureClass::ForbiddenWorkflowBroadening => {
                WorkflowFailureClass::ForbiddenWorkflowBroadening
            }
            WorkflowAdmissionFailureClass::ExplicitRebindRequired => {
                WorkflowFailureClass::ExplicitRebindRequired
            }
            other => panic!("unexpected workflow certification failure: {other:?}"),
        };
        Self {
            failure_class,
            failure_digest: digest_parts(&[
                format!("failure:{failure_class:?}"),
                format!("message:{}", error.message()),
                format!("drift:{}", error.drift_outcome().as_str()),
            ]),
            counter_snapshot_digest: digest_parts(&[
                format!(
                    "declarations:{}",
                    error.counters().workflow_declaration_count()
                ),
                format!(
                    "basis_bindings:{}",
                    error.counters().workflow_basis_binding_count()
                ),
                format!(
                    "basis_width:{}",
                    error.counters().workflow_basis_binding_width()
                ),
                format!(
                    "authority_checks:{}",
                    error.counters().workflow_authority_target_check_count()
                ),
                format!("denials:{}", error.counters().workflow_denial_count()),
                format!(
                    "broadening_denials:{}",
                    error.counters().workflow_broadening_denial_count()
                ),
                format!(
                    "executor_rediscovery:{}",
                    error.counters().workflow_executor_rediscovery_count()
                ),
            ]),
            drift_outcome: Some(error.drift_outcome().clone()),
            counters: Some(error.counters().clone()),
            compile_fail_case: None,
        }
    }

    pub(crate) fn compile_fail(case: &'static str) -> Self {
        Self {
            failure_class: WorkflowFailureClass::CompileFail,
            failure_digest: digest_parts(&[
                "failure:CompileFail".to_string(),
                format!("case:{case}"),
            ]),
            counter_snapshot_digest: digest_parts(
                &["compile_fail:no_runtime_counters".to_string()],
            ),
            drift_outcome: None,
            counters: None,
            compile_fail_case: Some(case),
        }
    }

    pub(crate) fn from_lowering_error(error: &WorkflowLoweringError) -> Self {
        let failure_class = match error.failure_class() {
            WorkflowLoweringFailureClass::InvalidWorkflowDeclarationFamily => {
                WorkflowFailureClass::UnsupportedWorkflowFamily
            }
            WorkflowLoweringFailureClass::UnsupportedMergeFamily
            | WorkflowLoweringFailureClass::UnsupportedWritebackFamily => {
                WorkflowFailureClass::UnsupportedWorkflowFamily
            }
            WorkflowLoweringFailureClass::StaleWorkflowDenied => {
                WorkflowFailureClass::StaleWorkflowDenied
            }
            WorkflowLoweringFailureClass::ExplicitRebindRequired => {
                WorkflowFailureClass::ExplicitRebindRequired
            }
            WorkflowLoweringFailureClass::UnsupportedRelationalStrategyTarget
            | WorkflowLoweringFailureClass::UnsupportedWritebackCausality => {
                WorkflowFailureClass::AmbientBasisFallbackForbidden
            }
            other => panic!("unexpected workflow lowering certification failure: {other:?}"),
        };
        Self {
            failure_class,
            failure_digest: digest_parts(&[
                format!("failure:{failure_class:?}"),
                format!("message:{}", error.message()),
                format!("staleness:{}", error.staleness_class().as_str()),
            ]),
            counter_snapshot_digest: digest_parts(&[
                format!("lowerings:{}", error.counters().workflow_lowering_count()),
                format!(
                    "mutation_lowerings:{}",
                    error.counters().workflow_mutation_lowering_count()
                ),
                format!(
                    "merge_lowerings:{}",
                    error.counters().workflow_merge_lowering_count()
                ),
                format!("width:{}", error.counters().workflow_lowering_width()),
                format!(
                    "denials:{}",
                    error.counters().workflow_lowering_denial_count()
                ),
                format!(
                    "staleness_denials:{}",
                    error.counters().workflow_lowering_staleness_denial_count()
                ),
                format!(
                    "merge_denials:{}",
                    error.counters().workflow_merge_denial_count()
                ),
                format!(
                    "writeback_declarations:{}",
                    error.counters().workflow_writeback_declaration_count()
                ),
                format!(
                    "writeback_denials:{}",
                    error.counters().workflow_writeback_denial_count()
                ),
                format!(
                    "writeback_causality_bindings:{}",
                    error
                        .counters()
                        .workflow_writeback_causality_binding_count()
                ),
                format!(
                    "staleness_checks:{}",
                    error.counters().workflow_staleness_check_count()
                ),
                format!(
                    "stale_denials:{}",
                    error.counters().workflow_stale_denial_count()
                ),
                format!(
                    "explicit_rebinds:{}",
                    error.counters().workflow_explicit_rebind_required_count()
                ),
                format!(
                    "authority_override_denials:{}",
                    error.counters().workflow_authority_override_denial_count()
                ),
                format!(
                    "ambient_basis_denials:{}",
                    error
                        .counters()
                        .workflow_ambient_basis_fallback_denial_count()
                ),
                format!(
                    "replay_bundle_count:{}",
                    error.counters().workflow_replay_bundle_count()
                ),
                format!(
                    "budget_crosses:{}",
                    error.counters().workflow_budget_cross_count()
                ),
                format!(
                    "work_avoided:{}",
                    error
                        .counters()
                        .workflow_work_avoided_by_query_lowering_count()
                ),
                format!(
                    "executor_rediscovery:{}",
                    error.counters().workflow_executor_rediscovery_count()
                ),
            ]),
            drift_outcome: None,
            counters: None,
            compile_fail_case: None,
        }
    }

    pub(crate) fn from_inspection_error(error: &WorkflowInspectionError) -> Self {
        let failure_class = match error.failure_class() {
            WorkflowInspectionFailureClass::NonAuthoritativeOutcomeForbidden => {
                WorkflowFailureClass::PostMergeOutcomeForbidden
            }
            other => panic!("unexpected workflow inspection certification failure: {other:?}"),
        };
        Self {
            failure_class,
            failure_digest: digest_parts(&[
                format!("failure:{failure_class:?}"),
                format!("message:{}", error.message()),
            ]),
            counter_snapshot_digest: digest_parts(&[
                format!(
                    "inspection_denial_width:{}",
                    error.counters().workflow_inspection_denial_width()
                ),
                format!(
                    "executor_rediscovery:{}",
                    error.counters().workflow_executor_rediscovery_count()
                ),
            ]),
            drift_outcome: None,
            counters: None,
            compile_fail_case: None,
        }
    }
}

pub type WorkflowCertificationMatrix = CertificationMatrix<
    WorkflowPerturbationClass,
    WorkflowCertificationLane,
    WorkflowCertificationRejection,
>;
