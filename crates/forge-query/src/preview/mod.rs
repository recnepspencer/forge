use crate::basis::{BasisAuthorityFamily, ExecutionPreflightBundle};
use crate::collection::{
    AggregateFunctionFamily, CollectionResultFamily, DerivedFieldComputationClass,
    MaterializationBreadthClass,
};
use crate::execution::{
    execute_preflight_bundle, ExecutionCounters, ExecutionError, ExecutionResultEnvelope,
};
use crate::identity::{
    hash_parts, CanonicalQueryDigest, CanonicalResultShapeDigest, ValidatedQueryDigest,
    ValidatedResultShapeDigest,
};
use crate::identity::{CollectionPlanDigest, ResultDigest};
use forge_runtime_bridge::facade::{
    BridgePreviewExecutionRecord, BridgePreviewLifecycleStateKind, BridgePreviewPromotionRecord,
    BridgePreviewReplayBundle, BridgePreviewSession, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, PreviewActive, PreviewAdmitted, PreviewDeclared,
    PreviewDiscarded, PreviewExecutionRecordIdentity, PreviewPromoted,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadOnlyPreviewEvaluation(());

impl ReadOnlyPreviewEvaluation {
    fn new() -> Self {
        Self(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionEligiblePreviewEvaluation(());

impl PromotionEligiblePreviewEvaluation {
    fn new() -> Self {
        Self(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewEvaluationClass {
    ReadOnly(ReadOnlyPreviewEvaluation),
    PromotionEligible(PromotionEligiblePreviewEvaluation),
}

impl PreviewEvaluationClass {
    pub fn read_only() -> Self {
        Self::ReadOnly(ReadOnlyPreviewEvaluation::new())
    }

    pub fn promotion_eligible() -> Self {
        Self::PromotionEligible(PromotionEligiblePreviewEvaluation::new())
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly(_) => "read_only",
            Self::PromotionEligible(_) => "promotion_eligible",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewBindingIntent {
    PreviewOnly,
    PreviewWithLiveLane,
}

impl PreviewBindingIntent {
    pub fn preview_only() -> Self {
        Self::PreviewOnly
    }

    pub fn preview_with_live_lane() -> Self {
        Self::PreviewWithLiveLane
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PreviewOnly => "preview_only",
            Self::PreviewWithLiveLane => "preview_with_live_lane",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewPerformanceStatusMarker {
    ConstantTimeCertified,
    RescanForbidden,
}

impl PreviewPerformanceStatusMarker {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConstantTimeCertified => "constant_time_certified",
            Self::RescanForbidden => "rescan_forbidden",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewComplexityContract {
    contract_name: &'static str,
    status_marker: PreviewPerformanceStatusMarker,
}

impl PreviewComplexityContract {
    fn preview_basis_binding_contract() -> Self {
        Self {
            contract_name: "preview_basis_binding_contract",
            status_marker: PreviewPerformanceStatusMarker::ConstantTimeCertified,
        }
    }

    fn preview_execution_metadata_contract() -> Self {
        Self {
            contract_name: "preview_execution_metadata_contract",
            status_marker: PreviewPerformanceStatusMarker::RescanForbidden,
        }
    }

    pub fn contract_name(&self) -> &'static str {
        self.contract_name
    }

    pub fn status_marker(&self) -> &PreviewPerformanceStatusMarker {
        &self.status_marker
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PreviewBindingCounters {
    preview_session_admission_count: usize,
    preview_basis_resolution_count: usize,
    preview_lifecycle_lookup_count: usize,
    preview_lifecycle_rediscovery_count: usize,
    preview_invalid_basis_denial_count: usize,
    preview_invalid_lifecycle_denial_count: usize,
    preview_broad_fallback_denial_count: usize,
    preview_executor_rediscovery_count: usize,
}

impl PreviewBindingCounters {
    pub fn preview_session_admission_count(&self) -> usize {
        self.preview_session_admission_count
    }

    pub fn preview_basis_resolution_count(&self) -> usize {
        self.preview_basis_resolution_count
    }

    pub fn preview_lifecycle_lookup_count(&self) -> usize {
        self.preview_lifecycle_lookup_count
    }

    pub fn preview_lifecycle_rediscovery_count(&self) -> usize {
        self.preview_lifecycle_rediscovery_count
    }

    pub fn preview_invalid_basis_denial_count(&self) -> usize {
        self.preview_invalid_basis_denial_count
    }

    pub fn preview_invalid_lifecycle_denial_count(&self) -> usize {
        self.preview_invalid_lifecycle_denial_count
    }

    pub fn preview_broad_fallback_denial_count(&self) -> usize {
        self.preview_broad_fallback_denial_count
    }

    pub fn preview_executor_rediscovery_count(&self) -> usize {
        self.preview_executor_rediscovery_count
    }

    #[cfg(test)]
    pub(crate) fn absorb(&mut self, other: &Self) {
        self.preview_session_admission_count += other.preview_session_admission_count;
        self.preview_basis_resolution_count += other.preview_basis_resolution_count;
        self.preview_lifecycle_lookup_count += other.preview_lifecycle_lookup_count;
        self.preview_lifecycle_rediscovery_count += other.preview_lifecycle_rediscovery_count;
        self.preview_invalid_basis_denial_count += other.preview_invalid_basis_denial_count;
        self.preview_invalid_lifecycle_denial_count += other.preview_invalid_lifecycle_denial_count;
        self.preview_broad_fallback_denial_count += other.preview_broad_fallback_denial_count;
        self.preview_executor_rediscovery_count += other.preview_executor_rediscovery_count;
    }

    fn for_admitted_path() -> Self {
        Self {
            preview_session_admission_count: 1,
            preview_basis_resolution_count: 1,
            preview_lifecycle_lookup_count: 1,
            preview_lifecycle_rediscovery_count: 0,
            preview_invalid_basis_denial_count: 0,
            preview_invalid_lifecycle_denial_count: 0,
            preview_broad_fallback_denial_count: 0,
            preview_executor_rediscovery_count: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewBindingFailureClass {
    InvalidPreviewBasis,
    UnsupportedPreviewQueryFamily,
    StaleOrInactivePreviewLifecycle,
    PreviewLiveDeniedInPhaseTwo,
    RawBranchAliasPreviewForbidden,
    MissingExecutionRecordIdentity,
    PromotionLinkageMismatch,
    StoreBackedRouteForbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewBindingError {
    failure_class: PreviewBindingFailureClass,
    message: &'static str,
    counters: PreviewBindingCounters,
}

impl PreviewBindingError {
    fn new(
        failure_class: PreviewBindingFailureClass,
        message: &'static str,
        counters: PreviewBindingCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> &PreviewBindingFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &PreviewBindingCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewLifecycleMetadata {
    lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    execution_record_identity: Option<PreviewExecutionRecordIdentity>,
    replay_bundle_digest: Option<String>,
    promotion_record_identity: Option<String>,
    promotion_proof_digest: Option<String>,
}

impl PreviewLifecycleMetadata {
    pub fn lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.lifecycle_state_kind
    }

    pub fn execution_record_identity(&self) -> Option<&PreviewExecutionRecordIdentity> {
        self.execution_record_identity.as_ref()
    }

    pub fn replay_bundle_digest(&self) -> Option<&str> {
        self.replay_bundle_digest.as_deref()
    }

    pub fn promotion_record_identity(&self) -> Option<&str> {
        self.promotion_record_identity.as_deref()
    }

    pub fn promotion_proof_digest(&self) -> Option<&str> {
        self.promotion_proof_digest.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSessionBindingTuple {
    digest: String,
    canonical_query_digest: CanonicalQueryDigest,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    validated_query_digest: ValidatedQueryDigest,
    validated_result_shape_digest: ValidatedResultShapeDigest,
    evaluation_class: PreviewEvaluationClass,
    binding_intent: PreviewBindingIntent,
    preview_session_identity: BridgePreviewSessionIdentity,
    declaration_identity: BridgePreviewSessionDeclarationIdentity,
    declaration_digest: String,
    lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    execution_record_identity: Option<PreviewExecutionRecordIdentity>,
    replay_bundle_digest: Option<String>,
    promotion_record_identity: Option<String>,
    promotion_proof_digest: Option<String>,
}

impl PreviewSessionBindingTuple {
    #[allow(clippy::too_many_arguments)]
    fn new(
        canonical_query_digest: CanonicalQueryDigest,
        canonical_result_shape_digest: CanonicalResultShapeDigest,
        validated_query_digest: ValidatedQueryDigest,
        validated_result_shape_digest: ValidatedResultShapeDigest,
        evaluation_class: PreviewEvaluationClass,
        binding_intent: PreviewBindingIntent,
        preview_session_identity: BridgePreviewSessionIdentity,
        declaration_identity: BridgePreviewSessionDeclarationIdentity,
        declaration_digest: String,
        lifecycle_state_kind: BridgePreviewLifecycleStateKind,
        execution_record_identity: Option<PreviewExecutionRecordIdentity>,
        replay_bundle_digest: Option<String>,
        promotion_record_identity: Option<String>,
        promotion_proof_digest: Option<String>,
    ) -> Self {
        let digest = hash_parts(&[
            format!("canonical_query:{}", canonical_query_digest.as_str()),
            format!(
                "canonical_result_shape:{}",
                canonical_result_shape_digest.as_str()
            ),
            format!("validated_query:{}", validated_query_digest.as_str()),
            format!(
                "validated_result_shape:{}",
                validated_result_shape_digest.as_str()
            ),
            format!("evaluation_class:{}", evaluation_class.as_str()),
            format!("binding_intent:{}", binding_intent.as_str()),
            format!("preview_session:{}", preview_session_identity.as_str()),
            format!("declaration_identity:{}", declaration_identity.as_str()),
            format!("declaration_digest:{declaration_digest}"),
            format!("lifecycle:{lifecycle_state_kind:?}"),
            format!(
                "execution_record:{}",
                execution_record_identity
                    .as_ref()
                    .map(PreviewExecutionRecordIdentity::as_str)
                    .unwrap_or("none")
            ),
            format!(
                "replay_bundle:{}",
                replay_bundle_digest.as_deref().unwrap_or("none")
            ),
            format!(
                "promotion_record:{}",
                promotion_record_identity.as_deref().unwrap_or("none")
            ),
            format!(
                "promotion_proof:{}",
                promotion_proof_digest.as_deref().unwrap_or("none")
            ),
        ]);
        Self {
            digest,
            canonical_query_digest,
            canonical_result_shape_digest,
            validated_query_digest,
            validated_result_shape_digest,
            evaluation_class,
            binding_intent,
            preview_session_identity,
            declaration_identity,
            declaration_digest,
            lifecycle_state_kind,
            execution_record_identity,
            replay_bundle_digest,
            promotion_record_identity,
            promotion_proof_digest,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn validated_query_digest(&self) -> &ValidatedQueryDigest {
        &self.validated_query_digest
    }

    pub fn validated_result_shape_digest(&self) -> &ValidatedResultShapeDigest {
        &self.validated_result_shape_digest
    }

    pub fn evaluation_class(&self) -> &PreviewEvaluationClass {
        &self.evaluation_class
    }

    pub fn binding_intent(&self) -> &PreviewBindingIntent {
        &self.binding_intent
    }

    pub fn preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.preview_session_identity
    }

    pub fn declaration_identity(&self) -> &BridgePreviewSessionDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.lifecycle_state_kind
    }

    pub fn execution_record_identity(&self) -> Option<&PreviewExecutionRecordIdentity> {
        self.execution_record_identity.as_ref()
    }

    pub fn replay_bundle_digest(&self) -> Option<&str> {
        self.replay_bundle_digest.as_deref()
    }

    pub fn promotion_record_identity(&self) -> Option<&str> {
        self.promotion_record_identity.as_deref()
    }

    pub fn promotion_proof_digest(&self) -> Option<&str> {
        self.promotion_proof_digest.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSessionBasis {
    binding_tuple: PreviewSessionBindingTuple,
}

impl PreviewSessionBasis {
    fn new(binding_tuple: PreviewSessionBindingTuple) -> Self {
        Self { binding_tuple }
    }

    pub fn binding_tuple(&self) -> &PreviewSessionBindingTuple {
        &self.binding_tuple
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewBindingReport {
    binding_digest: String,
    evaluation_class: PreviewEvaluationClass,
    binding_intent: PreviewBindingIntent,
    basis_binding_contract: PreviewComplexityContract,
    execution_metadata_contract: PreviewComplexityContract,
    counters: PreviewBindingCounters,
}

impl PreviewBindingReport {
    fn new(
        binding_digest: String,
        evaluation_class: PreviewEvaluationClass,
        binding_intent: PreviewBindingIntent,
        counters: PreviewBindingCounters,
    ) -> Self {
        Self {
            binding_digest,
            evaluation_class,
            binding_intent,
            basis_binding_contract: PreviewComplexityContract::preview_basis_binding_contract(),
            execution_metadata_contract:
                PreviewComplexityContract::preview_execution_metadata_contract(),
            counters,
        }
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn evaluation_class(&self) -> &PreviewEvaluationClass {
        &self.evaluation_class
    }

    pub fn binding_intent(&self) -> &PreviewBindingIntent {
        &self.binding_intent
    }

    pub fn basis_binding_contract(&self) -> &PreviewComplexityContract {
        &self.basis_binding_contract
    }

    pub fn execution_metadata_contract(&self) -> &PreviewComplexityContract {
        &self.execution_metadata_contract
    }

    pub fn counters(&self) -> &PreviewBindingCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSessionPlanBinding {
    preflight: ExecutionPreflightBundle,
    query_context: PreviewSessionQueryContext,
    basis: PreviewSessionBasis,
    lifecycle_metadata: PreviewLifecycleMetadata,
    report: PreviewBindingReport,
}

impl PreviewSessionPlanBinding {
    pub fn preflight(&self) -> &ExecutionPreflightBundle {
        &self.preflight
    }

    pub fn query_context(&self) -> &PreviewSessionQueryContext {
        &self.query_context
    }

    pub fn basis(&self) -> &PreviewSessionBasis {
        &self.basis
    }

    pub fn lifecycle_metadata(&self) -> &PreviewLifecycleMetadata {
        &self.lifecycle_metadata
    }

    pub fn report(&self) -> &PreviewBindingReport {
        &self.report
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadOnlyPreviewSessionPlanBinding {
    inner: PreviewSessionPlanBinding,
}

impl ReadOnlyPreviewSessionPlanBinding {
    pub fn as_preview_binding(&self) -> &PreviewSessionPlanBinding {
        &self.inner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionEligiblePreviewSessionPlanBinding {
    inner: PreviewSessionPlanBinding,
}

impl PromotionEligiblePreviewSessionPlanBinding {
    pub fn as_preview_binding(&self) -> &PreviewSessionPlanBinding {
        &self.inner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewExecutionCounters {
    binding_counters: PreviewBindingCounters,
    execution_counters: ExecutionCounters,
    preview_execution_envelope_count: usize,
    preview_execution_count: usize,
    preview_promotable_execution_count: usize,
    preview_read_only_execution_count: usize,
    preview_comparison_eligibility_proof_count: usize,
    preview_comparison_shape_check_width: usize,
    preview_workflow_foundation_artifact_lookup_count: usize,
}

impl PreviewExecutionCounters {
    pub fn binding_counters(&self) -> &PreviewBindingCounters {
        &self.binding_counters
    }

    pub fn execution_counters(&self) -> &ExecutionCounters {
        &self.execution_counters
    }

    pub fn preview_execution_envelope_count(&self) -> usize {
        self.preview_execution_envelope_count
    }

    pub fn preview_execution_count(&self) -> usize {
        self.preview_execution_count
    }

    pub fn preview_promotable_execution_count(&self) -> usize {
        self.preview_promotable_execution_count
    }

    pub fn preview_read_only_execution_count(&self) -> usize {
        self.preview_read_only_execution_count
    }

    pub fn preview_comparison_eligibility_proof_count(&self) -> usize {
        self.preview_comparison_eligibility_proof_count
    }

    pub fn preview_comparison_shape_check_width(&self) -> usize {
        self.preview_comparison_shape_check_width
    }

    pub fn preview_workflow_foundation_artifact_lookup_count(&self) -> usize {
        self.preview_workflow_foundation_artifact_lookup_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewComparisonCounters {
    preview_promotion_comparison_count: usize,
    preview_promotion_comparison_denial_count: usize,
    preview_comparison_eligibility_proof_count: usize,
    preview_comparison_shape_check_width: usize,
    preview_basis_pair_width: usize,
}

impl PreviewComparisonCounters {
    pub fn preview_promotion_comparison_count(&self) -> usize {
        self.preview_promotion_comparison_count
    }

    pub fn preview_promotion_comparison_denial_count(&self) -> usize {
        self.preview_promotion_comparison_denial_count
    }

    pub fn preview_comparison_eligibility_proof_count(&self) -> usize {
        self.preview_comparison_eligibility_proof_count
    }

    pub fn preview_comparison_shape_check_width(&self) -> usize {
        self.preview_comparison_shape_check_width
    }

    pub fn preview_basis_pair_width(&self) -> usize {
        self.preview_basis_pair_width
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewExecutionReport {
    preview_execution_digest: String,
    binding_digest: String,
    basis_digest: String,
    preview_session_identity: BridgePreviewSessionIdentity,
    lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    execution_record_identity: PreviewExecutionRecordIdentity,
    query_digest: ValidatedQueryDigest,
    result_digest: ResultDigest,
    comparison_eligibility_digest: String,
    workflow_foundation_digest: String,
}

impl PreviewExecutionReport {
    pub fn preview_execution_digest(&self) -> &str {
        &self.preview_execution_digest
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.preview_session_identity
    }

    pub fn lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.lifecycle_state_kind
    }

    pub fn execution_record_identity(&self) -> &PreviewExecutionRecordIdentity {
        &self.execution_record_identity
    }

    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &ResultDigest {
        &self.result_digest
    }

    pub fn comparison_eligibility_digest(&self) -> &str {
        &self.comparison_eligibility_digest
    }

    pub fn workflow_foundation_digest(&self) -> &str {
        &self.workflow_foundation_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewExecutionFailureClass {
    UnderlyingExecutionFailure,
    InvalidExecutionClass,
    InternalInvariantBreak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewExecutionError {
    ExecutionFailure(ExecutionError),
    EvaluationClassMismatch {
        expected: &'static str,
        actual: &'static str,
    },
    PreviewExecutionInvariantViolation {
        message: &'static str,
    },
}

impl PreviewExecutionError {
    pub fn failure_class(&self) -> PreviewExecutionFailureClass {
        match self {
            Self::ExecutionFailure(_) => PreviewExecutionFailureClass::UnderlyingExecutionFailure,
            Self::EvaluationClassMismatch { .. } => {
                PreviewExecutionFailureClass::InvalidExecutionClass
            }
            Self::PreviewExecutionInvariantViolation { .. } => {
                PreviewExecutionFailureClass::InternalInvariantBreak
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewExecutionEnvelope {
    binding: PreviewSessionPlanBinding,
    execution: ExecutionResultEnvelope,
    comparison_eligibility: PreviewComparisonEligibilityArtifact,
    workflow_foundation: AdmittedPreviewWorkflowFoundation,
    report: PreviewExecutionReport,
    counters: PreviewExecutionCounters,
}

impl PreviewExecutionEnvelope {
    pub fn binding(&self) -> &PreviewSessionPlanBinding {
        &self.binding
    }

    pub fn execution(&self) -> &ExecutionResultEnvelope {
        &self.execution
    }

    pub fn comparison_eligibility(&self) -> &PreviewComparisonEligibilityArtifact {
        &self.comparison_eligibility
    }

    pub fn workflow_foundation(&self) -> &PreviewWorkflowFoundationArtifact {
        self.workflow_foundation.artifact()
    }

    pub fn report(&self) -> &PreviewExecutionReport {
        &self.report
    }

    pub fn counters(&self) -> &PreviewExecutionCounters {
        &self.counters
    }

    pub fn check_invariants(&self) -> Result<(), PreviewExecutionError> {
        self.execution
            .check_invariants()
            .map_err(PreviewExecutionError::ExecutionFailure)?;

        if self
            .binding
            .report()
            .counters()
            .preview_lifecycle_rediscovery_count()
            != 0
        {
            return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                message: "preview lifecycle rediscovery must remain zero after execution",
            });
        }

        if self
            .binding
            .report()
            .counters()
            .preview_executor_rediscovery_count()
            != 0
        {
            return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                message: "preview executor rediscovery must remain zero after execution",
            });
        }

        if self
            .counters
            .binding_counters()
            .preview_lifecycle_rediscovery_count()
            != 0
            || self
                .counters
                .binding_counters()
                .preview_executor_rediscovery_count()
                != 0
        {
            return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                message: "preview execution counters must preserve zero rediscovery guarantees",
            });
        }

        if self
            .counters
            .execution_counters()
            .executor_semantic_rediscovery_count()
            != 0
        {
            return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                message: "preview execution cannot introduce executor semantic rediscovery",
            });
        }

        if self.counters.preview_execution_envelope_count() != 1 {
            return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                message: "preview execution must emit exactly one execution envelope",
            });
        }

        if self.counters.preview_execution_count() != 1 {
            return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                message: "preview execution must record exactly one preview execution",
            });
        }

        match self.binding.basis().binding_tuple().evaluation_class() {
            PreviewEvaluationClass::ReadOnly(_) => {
                if self.counters.preview_read_only_execution_count() != 1
                    || self.counters.preview_promotable_execution_count() != 0
                {
                    return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                        message: "read-only preview execution counters must remain class-explicit",
                    });
                }
            }
            PreviewEvaluationClass::PromotionEligible(_) => {
                if self.counters.preview_promotable_execution_count() != 1
                    || self.counters.preview_read_only_execution_count() != 0
                {
                    return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                        message:
                            "promotion-eligible preview execution counters must remain class-explicit",
                    });
                }
            }
        }

        if self.counters.preview_comparison_eligibility_proof_count() != 1 {
            return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                message: "preview execution must emit exactly one comparison eligibility proof",
            });
        }

        if self
            .counters
            .preview_workflow_foundation_artifact_lookup_count()
            != 1
        {
            return Err(PreviewExecutionError::PreviewExecutionInvariantViolation {
                message: "preview execution must resolve exactly one workflow foundation artifact",
            });
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadOnlyPreviewExecutionEnvelope {
    inner: PreviewExecutionEnvelope,
}

impl ReadOnlyPreviewExecutionEnvelope {
    pub fn as_preview_execution(&self) -> &PreviewExecutionEnvelope {
        &self.inner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionEligiblePreviewExecutionEnvelope {
    inner: PreviewExecutionEnvelope,
}

impl PromotionEligiblePreviewExecutionEnvelope {
    pub fn as_preview_execution(&self) -> &PreviewExecutionEnvelope {
        &self.inner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSessionQueryContext {
    source: PreviewContextSource,
    evaluation_class: PreviewEvaluationClass,
    binding_intent: PreviewBindingIntent,
    replay_bundle: Option<PreviewReplaySnapshot>,
    promotion_record: Option<PreviewPromotionSnapshot>,
}

impl PreviewSessionQueryContext {
    pub fn active(
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: &BridgePreviewExecutionRecord,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_active(session, Some(execution_record)),
            evaluation_class,
            binding_intent: PreviewBindingIntent::preview_only(),
            replay_bundle: None,
            promotion_record: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn active_without_execution_record(
        session: &BridgePreviewSession<PreviewActive>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_active(session, None),
            evaluation_class,
            binding_intent: PreviewBindingIntent::preview_only(),
            replay_bundle: None,
            promotion_record: None,
        }
    }

    pub fn declared(
        session: &BridgePreviewSession<PreviewDeclared>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_declared(session),
            evaluation_class,
            binding_intent: PreviewBindingIntent::preview_only(),
            replay_bundle: None,
            promotion_record: None,
        }
    }

    pub fn admitted(
        session: &BridgePreviewSession<PreviewAdmitted>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_admitted(session),
            evaluation_class,
            binding_intent: PreviewBindingIntent::preview_only(),
            replay_bundle: None,
            promotion_record: None,
        }
    }

    pub fn discarded(
        session: &BridgePreviewSession<PreviewDiscarded>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_discarded(session),
            evaluation_class,
            binding_intent: PreviewBindingIntent::preview_only(),
            replay_bundle: None,
            promotion_record: None,
        }
    }

    pub fn promoted(
        session: &BridgePreviewSession<PreviewPromoted>,
        evaluation_class: PreviewEvaluationClass,
    ) -> Self {
        Self {
            source: PreviewContextSource::from_promoted(session),
            evaluation_class,
            binding_intent: PreviewBindingIntent::preview_only(),
            replay_bundle: None,
            promotion_record: None,
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn with_binding_intent(mut self, binding_intent: PreviewBindingIntent) -> Self {
        self.binding_intent = binding_intent;
        self
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn with_replay_bundle(mut self, replay_bundle: &BridgePreviewReplayBundle) -> Self {
        self.replay_bundle = Some(PreviewReplaySnapshot::from_bundle(replay_bundle));
        self
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn with_promotion_record(
        mut self,
        promotion_record: &BridgePreviewPromotionRecord,
    ) -> Self {
        self.promotion_record = Some(PreviewPromotionSnapshot::from_record(promotion_record));
        self
    }

    pub fn evaluation_class(&self) -> &PreviewEvaluationClass {
        &self.evaluation_class
    }

    pub fn binding_intent(&self) -> &PreviewBindingIntent {
        &self.binding_intent
    }

    pub fn lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.source.lifecycle_state_kind()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PreviewContextSource {
    Active(PreviewSessionSnapshot),
    Declared(PreviewSessionSnapshot),
    Admitted(PreviewSessionSnapshot),
    Discarded(PreviewSessionSnapshot),
    Promoted(PreviewSessionSnapshot),
}

impl PreviewContextSource {
    fn from_active(
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: Option<&BridgePreviewExecutionRecord>,
    ) -> Self {
        Self::Active(PreviewSessionSnapshot::from_active(
            session,
            execution_record,
        ))
    }

    fn from_declared(session: &BridgePreviewSession<PreviewDeclared>) -> Self {
        Self::Declared(PreviewSessionSnapshot::from_declared(session))
    }

    fn from_admitted(session: &BridgePreviewSession<PreviewAdmitted>) -> Self {
        Self::Admitted(PreviewSessionSnapshot::from_admitted(session))
    }

    fn from_discarded(session: &BridgePreviewSession<PreviewDiscarded>) -> Self {
        Self::Discarded(PreviewSessionSnapshot::from_discarded(session))
    }

    fn from_promoted(session: &BridgePreviewSession<PreviewPromoted>) -> Self {
        Self::Promoted(PreviewSessionSnapshot::from_promoted(session))
    }

    fn snapshot(&self) -> &PreviewSessionSnapshot {
        match self {
            Self::Active(snapshot)
            | Self::Declared(snapshot)
            | Self::Admitted(snapshot)
            | Self::Discarded(snapshot)
            | Self::Promoted(snapshot) => snapshot,
        }
    }

    fn lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.snapshot().lifecycle_state_kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreviewSessionSnapshot {
    preview_session_identity: BridgePreviewSessionIdentity,
    declaration_identity: BridgePreviewSessionDeclarationIdentity,
    declaration_digest: String,
    lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    execution_record_identity: Option<PreviewExecutionRecordIdentity>,
    session_execution_record_identity: Option<PreviewExecutionRecordIdentity>,
    execution_record_digest: Option<String>,
    execution_record_preview_session_identity: Option<String>,
    execution_record_declaration_digest: Option<String>,
}

impl PreviewSessionSnapshot {
    fn from_declared(session: &BridgePreviewSession<PreviewDeclared>) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: None,
            session_execution_record_identity: None,
            execution_record_digest: None,
            execution_record_preview_session_identity: None,
            execution_record_declaration_digest: None,
        }
    }

    fn from_admitted(session: &BridgePreviewSession<PreviewAdmitted>) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: None,
            session_execution_record_identity: None,
            execution_record_digest: None,
            execution_record_preview_session_identity: None,
            execution_record_declaration_digest: None,
        }
    }

    fn from_active(
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: Option<&BridgePreviewExecutionRecord>,
    ) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: execution_record
                .map(|record| record.record_identity().clone())
                .or_else(|| session.execution_record_identity().cloned()),
            session_execution_record_identity: session.execution_record_identity().cloned(),
            execution_record_digest: execution_record.map(|record| record.digest().to_string()),
            execution_record_preview_session_identity: execution_record
                .map(|record| record.preview_session_identity().to_string()),
            execution_record_declaration_digest: execution_record
                .map(|record| record.preview_declaration_digest().to_string()),
        }
    }

    fn from_discarded(session: &BridgePreviewSession<PreviewDiscarded>) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: session.execution_record_identity().cloned(),
            session_execution_record_identity: session.execution_record_identity().cloned(),
            execution_record_digest: None,
            execution_record_preview_session_identity: None,
            execution_record_declaration_digest: None,
        }
    }

    fn from_promoted(session: &BridgePreviewSession<PreviewPromoted>) -> Self {
        Self {
            preview_session_identity: session.session_identity().clone(),
            declaration_identity: session
                .declaration()
                .declaration()
                .declaration_identity()
                .clone(),
            declaration_digest: session.declaration().digest().to_string(),
            lifecycle_state_kind: session.lifecycle_state_kind(),
            execution_record_identity: session.execution_record_identity().cloned(),
            session_execution_record_identity: session.execution_record_identity().cloned(),
            execution_record_digest: None,
            execution_record_preview_session_identity: None,
            execution_record_declaration_digest: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreviewReplaySnapshot {
    digest: String,
}

impl PreviewReplaySnapshot {
    fn from_bundle(bundle: &BridgePreviewReplayBundle) -> Self {
        Self {
            digest: bundle.digest().to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreviewPromotionSnapshot {
    record_identity: String,
    proof_digest: String,
}

impl PreviewPromotionSnapshot {
    fn from_record(record: &BridgePreviewPromotionRecord) -> Self {
        Self {
            record_identity: record.record_identity().as_str().to_string(),
            proof_digest: record.promotion_proof_digest().to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreviewComparisonShapeContract {
    collection_digest: Option<CollectionPlanDigest>,
    result_family: String,
    ordering_digest: String,
    materialization_boundary_digest: String,
    shape_check_width: usize,
}

impl PreviewComparisonShapeContract {
    fn from_preflight(preflight: &ExecutionPreflightBundle) -> Self {
        let collection = preflight.plan().collection();
        let ordering_digest = hash_parts(
            &collection
                .map(|collection| collection.ordering_basis().digest_parts())
                .unwrap_or_else(|| vec!["detail_ordering:root_entity_identity".to_string()]),
        );
        let materialization_boundary_digest = hash_parts(
            &collection
                .map(|collection| {
                    let mut parts = vec![
                        format!("window_policy:{:?}", collection.window_policy()),
                        collection.cursor_contract().digest_part(),
                    ];
                    parts.extend(collection.traversal_bound().digest_parts());
                    parts.extend(collection.post_read_shaping().digest_parts());
                    parts
                })
                .unwrap_or_else(|| {
                    vec![
                        "window_policy:detail_single_read".to_string(),
                        "cursor_contract:not_applicable".to_string(),
                        "materialization_breadth:scalar_only".to_string(),
                        "detail_result_family:detail".to_string(),
                    ]
                }),
        );
        let shape_check_width = collection
            .map(|collection| {
                collection.ordering_basis().entries().len()
                    + collection.traversal_bound().edge_classes().len()
                    + usize::from(matches!(
                        collection.traversal_bound().materialization_breadth(),
                        MaterializationBreadthClass::RootPlusTraversal
                    ))
                    + preflight.plan().result_shape().binding_count()
            })
            .unwrap_or_else(|| preflight.plan().result_shape().binding_count().max(1));
        let result_family = collection
            .map(
                |collection| match collection.planning_context().result_family() {
                    CollectionResultFamily::OrdinaryCollection => "ordinary_collection",
                    CollectionResultFamily::CdcCollection => "cdc_collection",
                },
            )
            .unwrap_or("detail")
            .to_string();

        Self {
            collection_digest: collection.map(|collection| collection.digest().clone()),
            result_family,
            ordering_digest,
            materialization_boundary_digest,
            shape_check_width,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewComparisonEligibilityArtifact {
    digest: String,
    canonical_query_digest: CanonicalQueryDigest,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    collection_digest: Option<CollectionPlanDigest>,
    result_family: String,
    ordering_digest: String,
    materialization_boundary_digest: String,
    shape_check_width: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewWorkflowFoundationArtifact {
    digest: String,
    binding_digest: String,
    preview_session_identity: BridgePreviewSessionIdentity,
    declaration_identity: BridgePreviewSessionDeclarationIdentity,
    declaration_digest: String,
    lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    evaluation_class: PreviewEvaluationClass,
    execution_record_identity: PreviewExecutionRecordIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedPreviewWorkflowFoundation {
    artifact: PreviewWorkflowFoundationArtifact,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewComparisonCandidateArtifact {
    digest: String,
    validated_query_digest: ValidatedQueryDigest,
    basis_digest: String,
    result_digest: ResultDigest,
    canonical_query_digest: CanonicalQueryDigest,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    collection_digest: Option<CollectionPlanDigest>,
    result_family: String,
    ordering_digest: String,
    materialization_boundary_digest: String,
    shape_check_width: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoritativePreviewComparisonCandidate {
    artifact: PreviewComparisonCandidateArtifact,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewComparisonFailureClass {
    CandidateBasisAuthorityMismatch,
    CandidateExecutionPlanMismatch,
    CandidateExecutionBasisMismatch,
    QueryDigestMismatch,
    ResultShapeMismatch,
    ResultFamilyMismatch,
    OrderingBasisMismatch,
    MaterializationBoundaryMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewComparisonError {
    failure_class: PreviewComparisonFailureClass,
    message: &'static str,
    preview_digest: String,
    candidate_digest: String,
    counters: PreviewComparisonCounters,
}

impl PreviewComparisonError {
    pub fn failure_class(&self) -> &PreviewComparisonFailureClass {
        &self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn preview_digest(&self) -> &str {
        &self.preview_digest
    }

    pub fn candidate_digest(&self) -> &str {
        &self.candidate_digest
    }

    pub fn counters(&self) -> &PreviewComparisonCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewExecutionComparisonAdmission {
    digest: String,
    preview_execution_digest: String,
    preview_comparison_digest: String,
    candidate_comparison_digest: String,
    candidate_basis_digest: String,
    candidate_result_digest: ResultDigest,
    shape_check_width: usize,
    counters: PreviewComparisonCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionParityPreviewComparisonAdmission {
    inner: PreviewExecutionComparisonAdmission,
}

impl PreviewComparisonEligibilityArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn collection_digest(&self) -> Option<&CollectionPlanDigest> {
        self.collection_digest.as_ref()
    }

    pub fn result_family(&self) -> &str {
        &self.result_family
    }

    pub fn ordering_digest(&self) -> &str {
        &self.ordering_digest
    }

    pub fn materialization_boundary_digest(&self) -> &str {
        &self.materialization_boundary_digest
    }

    pub fn shape_check_width(&self) -> usize {
        self.shape_check_width
    }
}

impl PreviewWorkflowFoundationArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn preview_session_identity(&self) -> &BridgePreviewSessionIdentity {
        &self.preview_session_identity
    }

    pub fn declaration_identity(&self) -> &BridgePreviewSessionDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn lifecycle_state_kind(&self) -> BridgePreviewLifecycleStateKind {
        self.lifecycle_state_kind
    }

    pub fn evaluation_class(&self) -> &PreviewEvaluationClass {
        &self.evaluation_class
    }

    pub fn execution_record_identity(&self) -> &PreviewExecutionRecordIdentity {
        &self.execution_record_identity
    }
}

impl AdmittedPreviewWorkflowFoundation {
    pub fn artifact(&self) -> &PreviewWorkflowFoundationArtifact {
        &self.artifact
    }
}

impl PreviewComparisonCandidateArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn validated_query_digest(&self) -> &ValidatedQueryDigest {
        &self.validated_query_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn result_digest(&self) -> &ResultDigest {
        &self.result_digest
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn collection_digest(&self) -> Option<&CollectionPlanDigest> {
        self.collection_digest.as_ref()
    }

    pub fn result_family(&self) -> &str {
        &self.result_family
    }

    pub fn ordering_digest(&self) -> &str {
        &self.ordering_digest
    }

    pub fn materialization_boundary_digest(&self) -> &str {
        &self.materialization_boundary_digest
    }

    pub fn shape_check_width(&self) -> usize {
        self.shape_check_width
    }
}

impl AuthoritativePreviewComparisonCandidate {
    pub fn artifact(&self) -> &PreviewComparisonCandidateArtifact {
        &self.artifact
    }
}

impl PreviewExecutionComparisonAdmission {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn preview_execution_digest(&self) -> &str {
        &self.preview_execution_digest
    }

    pub fn preview_comparison_digest(&self) -> &str {
        &self.preview_comparison_digest
    }

    pub fn candidate_comparison_digest(&self) -> &str {
        &self.candidate_comparison_digest
    }

    pub fn candidate_basis_digest(&self) -> &str {
        &self.candidate_basis_digest
    }

    pub fn candidate_result_digest(&self) -> &ResultDigest {
        &self.candidate_result_digest
    }

    pub fn shape_check_width(&self) -> usize {
        self.shape_check_width
    }

    pub fn counters(&self) -> &PreviewComparisonCounters {
        &self.counters
    }
}

impl PromotionParityPreviewComparisonAdmission {
    pub fn as_preview_comparison(&self) -> &PreviewExecutionComparisonAdmission {
        &self.inner
    }
}

pub fn derive_preview_comparison_eligibility(
    binding: &PreviewSessionPlanBinding,
) -> PreviewComparisonEligibilityArtifact {
    let shape_contract = PreviewComparisonShapeContract::from_preflight(binding.preflight());

    let digest = hash_parts(&[
        format!(
            "canonical_query:{}",
            binding
                .basis()
                .binding_tuple()
                .canonical_query_digest()
                .as_str()
        ),
        format!(
            "canonical_result_shape:{}",
            binding
                .basis()
                .binding_tuple()
                .canonical_result_shape_digest()
                .as_str()
        ),
        format!(
            "collection:{}",
            shape_contract
                .collection_digest
                .as_ref()
                .map(CollectionPlanDigest::as_str)
                .unwrap_or("detail")
        ),
        format!("result_family:{}", shape_contract.result_family),
        format!("ordering:{}", shape_contract.ordering_digest),
        format!(
            "materialization_boundary:{}",
            shape_contract.materialization_boundary_digest
        ),
        format!("shape_check_width:{}", shape_contract.shape_check_width),
    ]);

    PreviewComparisonEligibilityArtifact {
        digest,
        canonical_query_digest: binding
            .basis()
            .binding_tuple()
            .canonical_query_digest()
            .clone(),
        canonical_result_shape_digest: binding
            .basis()
            .binding_tuple()
            .canonical_result_shape_digest()
            .clone(),
        collection_digest: shape_contract.collection_digest,
        result_family: shape_contract.result_family,
        ordering_digest: shape_contract.ordering_digest,
        materialization_boundary_digest: shape_contract.materialization_boundary_digest,
        shape_check_width: shape_contract.shape_check_width,
    }
}

fn derive_preview_workflow_foundation(
    binding: &PreviewSessionPlanBinding,
) -> PreviewWorkflowFoundationArtifact {
    let binding_tuple = binding.basis().binding_tuple();
    let execution_record_identity = binding_tuple
        .execution_record_identity()
        .cloned()
        .expect("active preview bindings must carry an execution record identity");
    let digest = hash_parts(&[
        format!("binding:{}", binding_tuple.digest()),
        format!(
            "preview_session:{}",
            binding_tuple.preview_session_identity().as_str()
        ),
        format!(
            "declaration_identity:{}",
            binding_tuple.declaration_identity().as_str()
        ),
        format!("declaration_digest:{}", binding_tuple.declaration_digest()),
        format!("lifecycle:{:?}", binding_tuple.lifecycle_state_kind()),
        format!("execution_record:{}", execution_record_identity.as_str()),
        format!(
            "evaluation_class:{}",
            binding_tuple.evaluation_class().as_str()
        ),
    ]);

    PreviewWorkflowFoundationArtifact {
        digest,
        binding_digest: binding_tuple.digest().to_string(),
        preview_session_identity: binding_tuple.preview_session_identity().clone(),
        declaration_identity: binding_tuple.declaration_identity().clone(),
        declaration_digest: binding_tuple.declaration_digest().to_string(),
        lifecycle_state_kind: binding_tuple.lifecycle_state_kind(),
        evaluation_class: binding_tuple.evaluation_class().clone(),
        execution_record_identity,
    }
}

pub fn admit_preview_workflow_foundation(
    binding: &PreviewSessionPlanBinding,
) -> AdmittedPreviewWorkflowFoundation {
    AdmittedPreviewWorkflowFoundation {
        artifact: derive_preview_workflow_foundation(binding),
    }
}

fn derive_preview_comparison_candidate(
    preflight: &ExecutionPreflightBundle,
    execution: &ExecutionResultEnvelope,
) -> PreviewComparisonCandidateArtifact {
    let shape_contract = PreviewComparisonShapeContract::from_preflight(preflight);
    let digest = hash_parts(&[
        format!(
            "validated_query:{}",
            preflight.plan().query().validated_query_digest().as_str()
        ),
        format!("result:{}", execution.report().result_digest().as_str()),
        format!(
            "canonical_query:{}",
            preflight.plan().query().canonical_query_digest().as_str()
        ),
        format!(
            "canonical_result_shape:{}",
            preflight
                .plan()
                .result_shape()
                .canonical_result_shape_digest()
                .as_str()
        ),
        format!(
            "collection:{}",
            shape_contract
                .collection_digest
                .as_ref()
                .map(CollectionPlanDigest::as_str)
                .unwrap_or("detail")
        ),
        format!("result_family:{}", shape_contract.result_family),
        format!("ordering:{}", shape_contract.ordering_digest),
        format!(
            "materialization_boundary:{}",
            shape_contract.materialization_boundary_digest
        ),
        format!("shape_check_width:{}", shape_contract.shape_check_width),
    ]);

    PreviewComparisonCandidateArtifact {
        digest,
        validated_query_digest: preflight.plan().query().validated_query_digest().clone(),
        basis_digest: preflight.basis().proof().digest().as_str().to_string(),
        result_digest: execution.report().result_digest().clone(),
        canonical_query_digest: preflight.plan().query().canonical_query_digest().clone(),
        canonical_result_shape_digest: preflight
            .plan()
            .result_shape()
            .canonical_result_shape_digest()
            .clone(),
        collection_digest: shape_contract.collection_digest,
        result_family: shape_contract.result_family,
        ordering_digest: shape_contract.ordering_digest,
        materialization_boundary_digest: shape_contract.materialization_boundary_digest,
        shape_check_width: shape_contract.shape_check_width,
    }
}

pub fn admit_authoritative_preview_comparison_candidate(
    candidate_preflight: &ExecutionPreflightBundle,
    candidate_execution: &ExecutionResultEnvelope,
) -> Result<AuthoritativePreviewComparisonCandidate, PreviewComparisonError> {
    let candidate = derive_preview_comparison_candidate(candidate_preflight, candidate_execution);
    let denial_counters = || PreviewComparisonCounters {
        preview_promotion_comparison_count: 0,
        preview_promotion_comparison_denial_count: 1,
        preview_comparison_eligibility_proof_count: 1,
        preview_comparison_shape_check_width: candidate.shape_check_width(),
        preview_basis_pair_width: 1,
    };

    if matches!(
        candidate_preflight.basis().identity().authority_family(),
        BasisAuthorityFamily::Store
    ) {
        return Err(PreviewComparisonError {
            failure_class: PreviewComparisonFailureClass::CandidateBasisAuthorityMismatch,
            message: "preview comparison only admits runtime-backed authoritative candidates",
            preview_digest: String::new(),
            candidate_digest: candidate.digest().to_string(),
            counters: denial_counters(),
        });
    }

    if candidate_execution.report().query_digest()
        != candidate_preflight.plan().query().validated_query_digest()
        || candidate_execution.report().plan_digest()
            != candidate_preflight.plan().query().plan_digest()
    {
        return Err(PreviewComparisonError {
            failure_class: PreviewComparisonFailureClass::CandidateExecutionPlanMismatch,
            message:
                "preview comparison candidates require execution from the same admitted query plan",
            preview_digest: String::new(),
            candidate_digest: candidate.digest().to_string(),
            counters: denial_counters(),
        });
    }

    if candidate_execution.report().basis_digest() != candidate_preflight.basis().proof().digest() {
        return Err(PreviewComparisonError {
            failure_class: PreviewComparisonFailureClass::CandidateExecutionBasisMismatch,
            message:
                "preview comparison candidates require execution from the same authoritative basis",
            preview_digest: String::new(),
            candidate_digest: candidate.digest().to_string(),
            counters: denial_counters(),
        });
    }

    Ok(AuthoritativePreviewComparisonCandidate { artifact: candidate })
}

fn admit_preview_execution_comparison(
    preview_execution: &PreviewExecutionEnvelope,
    candidate: &AuthoritativePreviewComparisonCandidate,
) -> Result<PreviewExecutionComparisonAdmission, PreviewComparisonError> {
    let preview = preview_execution.comparison_eligibility();
    let candidate = candidate.artifact();
    let denial_counters = |shape_width: usize| PreviewComparisonCounters {
        preview_promotion_comparison_count: 0,
        preview_promotion_comparison_denial_count: 1,
        preview_comparison_eligibility_proof_count: 1,
        preview_comparison_shape_check_width: shape_width,
        preview_basis_pair_width: 2,
    };

    if preview_execution.report().query_digest() != candidate.validated_query_digest() {
        return Err(PreviewComparisonError {
            failure_class: PreviewComparisonFailureClass::QueryDigestMismatch,
            message: "preview comparison requires the same validated query digest on both sides",
            preview_digest: preview.digest().to_string(),
            candidate_digest: candidate.digest().to_string(),
            counters: denial_counters(candidate.shape_check_width()),
        });
    }

    if preview.canonical_query_digest() != candidate.canonical_query_digest()
        || preview.canonical_result_shape_digest() != candidate.canonical_result_shape_digest()
    {
        return Err(PreviewComparisonError {
            failure_class: PreviewComparisonFailureClass::ResultShapeMismatch,
            message:
                "preview comparison requires the same canonical query and result-shape digests",
            preview_digest: preview.digest().to_string(),
            candidate_digest: candidate.digest().to_string(),
            counters: denial_counters(preview.shape_check_width().max(candidate.shape_check_width())),
        });
    }

    if preview.result_family() != candidate.result_family() {
        return Err(PreviewComparisonError {
            failure_class: PreviewComparisonFailureClass::ResultFamilyMismatch,
            message: "preview comparison requires the same result family on both sides",
            preview_digest: preview.digest().to_string(),
            candidate_digest: candidate.digest().to_string(),
            counters: denial_counters(preview.shape_check_width().max(candidate.shape_check_width())),
        });
    }

    if preview.ordering_digest() != candidate.ordering_digest() {
        return Err(PreviewComparisonError {
            failure_class: PreviewComparisonFailureClass::OrderingBasisMismatch,
            message: "preview comparison requires identical ordering basis proofs",
            preview_digest: preview.digest().to_string(),
            candidate_digest: candidate.digest().to_string(),
            counters: denial_counters(preview.shape_check_width().max(candidate.shape_check_width())),
        });
    }

    if preview.materialization_boundary_digest() != candidate.materialization_boundary_digest() {
        return Err(PreviewComparisonError {
            failure_class: PreviewComparisonFailureClass::MaterializationBoundaryMismatch,
            message: "preview comparison requires identical materialization boundary proofs",
            preview_digest: preview.digest().to_string(),
            candidate_digest: candidate.digest().to_string(),
            counters: denial_counters(preview.shape_check_width().max(candidate.shape_check_width())),
        });
    }

    let shape_check_width = preview
        .shape_check_width()
        .max(candidate.shape_check_width());
    Ok(PreviewExecutionComparisonAdmission {
        digest: hash_parts(&[
            format!(
                "preview_execution:{}",
                preview_execution.report().preview_execution_digest()
            ),
            format!("preview_comparison:{}", preview.digest()),
            format!("candidate_comparison:{}", candidate.digest()),
            format!("candidate_basis:{}", candidate.basis_digest()),
            format!("candidate_result:{}", candidate.result_digest().as_str()),
        ]),
        preview_execution_digest: preview_execution
            .report()
            .preview_execution_digest()
            .to_string(),
        preview_comparison_digest: preview.digest().to_string(),
        candidate_comparison_digest: candidate.digest().to_string(),
        candidate_basis_digest: candidate.basis_digest().to_string(),
        candidate_result_digest: candidate.result_digest().clone(),
        shape_check_width,
        counters: PreviewComparisonCounters {
            preview_promotion_comparison_count: 1,
            preview_promotion_comparison_denial_count: 0,
            preview_comparison_eligibility_proof_count: 1,
            preview_comparison_shape_check_width: shape_check_width,
            preview_basis_pair_width: 2,
        },
    })
}

pub fn admit_preview_promotion_parity_comparison(
    preview_execution: &PromotionEligiblePreviewExecutionEnvelope,
    candidate: &AuthoritativePreviewComparisonCandidate,
) -> Result<PromotionParityPreviewComparisonAdmission, PreviewComparisonError> {
    let admission = admit_preview_execution_comparison(
        preview_execution.as_preview_execution(),
        candidate,
    )?;

    Ok(PromotionParityPreviewComparisonAdmission { inner: admission })
}

pub fn execute_preview_session_plan(
    binding: &PreviewSessionPlanBinding,
) -> Result<PreviewExecutionEnvelope, PreviewExecutionError> {
    let execution = execute_preflight_bundle(binding.preflight())
        .map_err(PreviewExecutionError::ExecutionFailure)?;
    let comparison_eligibility = derive_preview_comparison_eligibility(binding);
    let workflow_foundation = admit_preview_workflow_foundation(binding);
    let binding_tuple = binding.basis().binding_tuple();
    let execution_record_identity = binding_tuple
        .execution_record_identity()
        .cloned()
        .expect("active preview bindings must carry an execution record identity");
    let is_promotion_eligible = matches!(
        binding_tuple.evaluation_class(),
        PreviewEvaluationClass::PromotionEligible(_)
    );
    let report = PreviewExecutionReport {
        preview_execution_digest: hash_parts(&[
            format!("binding:{}", binding_tuple.digest()),
            format!("basis:{}", execution.report().basis_digest().as_str()),
            format!(
                "preview_session:{}",
                binding_tuple.preview_session_identity().as_str()
            ),
            format!("lifecycle:{:?}", binding_tuple.lifecycle_state_kind()),
            format!("execution_record:{}", execution_record_identity.as_str()),
            format!("result:{}", execution.report().result_digest().as_str()),
            format!("comparison:{}", comparison_eligibility.digest()),
            format!("workflow:{}", workflow_foundation.artifact().digest()),
        ]),
        binding_digest: binding_tuple.digest().to_string(),
        basis_digest: execution.report().basis_digest().as_str().to_string(),
        preview_session_identity: binding_tuple.preview_session_identity().clone(),
        lifecycle_state_kind: binding_tuple.lifecycle_state_kind(),
        execution_record_identity,
        query_digest: execution.report().query_digest().clone(),
        result_digest: execution.report().result_digest().clone(),
        comparison_eligibility_digest: comparison_eligibility.digest().to_string(),
        workflow_foundation_digest: workflow_foundation.artifact().digest().to_string(),
    };
    let envelope = PreviewExecutionEnvelope {
        binding: binding.clone(),
        counters: PreviewExecutionCounters {
            binding_counters: binding.report().counters().clone(),
            execution_counters: execution.counters().clone(),
            preview_execution_envelope_count: 1,
            preview_execution_count: 1,
            preview_promotable_execution_count: usize::from(is_promotion_eligible),
            preview_read_only_execution_count: usize::from(!is_promotion_eligible),
            preview_comparison_eligibility_proof_count: 1,
            preview_comparison_shape_check_width: comparison_eligibility.shape_check_width(),
            preview_workflow_foundation_artifact_lookup_count: 1,
        },
        execution,
        comparison_eligibility,
        workflow_foundation,
        report,
    };
    envelope.check_invariants()?;
    Ok(envelope)
}

pub fn admit_read_only_preview_session_plan_binding(
    binding: PreviewSessionPlanBinding,
) -> Result<ReadOnlyPreviewSessionPlanBinding, PreviewExecutionError> {
    if !matches!(
        binding.basis().binding_tuple().evaluation_class(),
        PreviewEvaluationClass::ReadOnly(_)
    ) {
        return Err(PreviewExecutionError::EvaluationClassMismatch {
            expected: "read_only",
            actual: binding.basis().binding_tuple().evaluation_class().as_str(),
        });
    }

    Ok(ReadOnlyPreviewSessionPlanBinding { inner: binding })
}

pub fn admit_promotion_eligible_preview_session_plan_binding(
    binding: PreviewSessionPlanBinding,
) -> Result<PromotionEligiblePreviewSessionPlanBinding, PreviewExecutionError> {
    if !matches!(
        binding.basis().binding_tuple().evaluation_class(),
        PreviewEvaluationClass::PromotionEligible(_)
    ) {
        return Err(PreviewExecutionError::EvaluationClassMismatch {
            expected: "promotion_eligible",
            actual: binding.basis().binding_tuple().evaluation_class().as_str(),
        });
    }

    Ok(PromotionEligiblePreviewSessionPlanBinding { inner: binding })
}

pub fn execute_read_only_preview_session_plan(
    binding: &ReadOnlyPreviewSessionPlanBinding,
) -> Result<ReadOnlyPreviewExecutionEnvelope, PreviewExecutionError> {
    Ok(ReadOnlyPreviewExecutionEnvelope {
        inner: execute_preview_session_plan(binding.as_preview_binding())?,
    })
}

pub fn execute_promotion_eligible_preview_session_plan(
    binding: &PromotionEligiblePreviewSessionPlanBinding,
) -> Result<PromotionEligiblePreviewExecutionEnvelope, PreviewExecutionError> {
    Ok(PromotionEligiblePreviewExecutionEnvelope {
        inner: execute_preview_session_plan(binding.as_preview_binding())?,
    })
}

pub fn bind_preflight_to_preview_session(
    preflight: ExecutionPreflightBundle,
    query_context: PreviewSessionQueryContext,
) -> Result<PreviewSessionPlanBinding, PreviewBindingError> {
    reject_unsupported_preview_family(&preflight)?;

    if matches!(
        preflight.basis().identity().authority_family(),
        BasisAuthorityFamily::Store
    ) {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_basis_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::StoreBackedRouteForbidden,
            "preview binding requires runtime basis authority",
            counters,
        ));
    }

    if matches!(
        query_context.binding_intent(),
        PreviewBindingIntent::PreviewWithLiveLane
    ) {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_broad_fallback_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::PreviewLiveDeniedInPhaseTwo,
            "preview-bound live lanes are denied in milestone 5.2 phases 1-2",
            counters,
        ));
    }

    let source = query_context.source.snapshot();
    if source.lifecycle_state_kind != BridgePreviewLifecycleStateKind::Active {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_lifecycle_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::StaleOrInactivePreviewLifecycle,
            "preview lifecycle must be active before binding",
            counters,
        ));
    }

    if source.execution_record_identity.is_none() || source.execution_record_digest.is_none() {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_lifecycle_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::MissingExecutionRecordIdentity,
            "active preview binding requires an explicit preview execution record",
            counters,
        ));
    }

    if let Some(execution_record_digest) = source.execution_record_digest.as_ref() {
        if execution_record_digest.is_empty() {
            let mut counters = PreviewBindingCounters::default();
            counters.preview_invalid_basis_denial_count = 1;
            return Err(PreviewBindingError::new(
                PreviewBindingFailureClass::InvalidPreviewBasis,
                "preview execution record digest must not be empty",
                counters,
            ));
        }
    }

    if let Some(execution_record_session_identity) =
        source.execution_record_preview_session_identity.as_ref()
    {
        if execution_record_session_identity != source.preview_session_identity.as_str() {
            let mut counters = PreviewBindingCounters::default();
            counters.preview_invalid_basis_denial_count = 1;
            return Err(PreviewBindingError::new(
                PreviewBindingFailureClass::InvalidPreviewBasis,
                "preview execution record must belong to the requested preview session",
                counters,
            ));
        }
    }

    if let Some(execution_record_declaration_digest) =
        source.execution_record_declaration_digest.as_ref()
    {
        if execution_record_declaration_digest != &source.declaration_digest {
            let mut counters = PreviewBindingCounters::default();
            counters.preview_invalid_basis_denial_count = 1;
            return Err(PreviewBindingError::new(
                PreviewBindingFailureClass::InvalidPreviewBasis,
                "preview execution record must match the requested preview declaration digest",
                counters,
            ));
        }
    }

    if let (Some(execution_record_identity), Some(session_execution_record_identity)) = (
        source.execution_record_identity.as_ref(),
        source.session_execution_record_identity.as_ref(),
    ) {
        if execution_record_identity != session_execution_record_identity {
            let mut counters = PreviewBindingCounters::default();
            counters.preview_invalid_basis_denial_count = 1;
            return Err(PreviewBindingError::new(
                PreviewBindingFailureClass::InvalidPreviewBasis,
                "preview execution record identity must match the active preview session identity",
                counters,
            ));
        }
    }

    if matches!(
        query_context.evaluation_class(),
        PreviewEvaluationClass::ReadOnly(_)
    ) && query_context.promotion_record.is_some()
    {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_basis_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::PromotionLinkageMismatch,
            "read-only preview evaluation cannot carry promotion linkage",
            counters,
        ));
    }

    if query_context.promotion_record.is_some() || query_context.replay_bundle.is_some() {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_basis_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::PromotionLinkageMismatch,
            "phase 1-2 preview binding does not admit replay or promotion linkage on active sessions",
            counters,
        ));
    }

    let counters = PreviewBindingCounters::for_admitted_path();
    let lifecycle_metadata = PreviewLifecycleMetadata {
        lifecycle_state_kind: source.lifecycle_state_kind,
        execution_record_identity: source.execution_record_identity.clone(),
        replay_bundle_digest: query_context
            .replay_bundle
            .as_ref()
            .map(|bundle| bundle.digest.clone()),
        promotion_record_identity: query_context
            .promotion_record
            .as_ref()
            .map(|record| record.record_identity.clone()),
        promotion_proof_digest: query_context
            .promotion_record
            .as_ref()
            .map(|record| record.proof_digest.clone()),
    };
    let binding_tuple = PreviewSessionBindingTuple::new(
        preflight.plan().query().canonical_query_digest().clone(),
        preflight
            .plan()
            .result_shape()
            .canonical_result_shape_digest()
            .clone(),
        preflight.plan().query().validated_query_digest().clone(),
        preflight
            .plan()
            .result_shape()
            .validated_result_shape_digest()
            .clone(),
        query_context.evaluation_class.clone(),
        query_context.binding_intent.clone(),
        source.preview_session_identity.clone(),
        source.declaration_identity.clone(),
        source.declaration_digest.clone(),
        source.lifecycle_state_kind,
        source.execution_record_identity.clone(),
        lifecycle_metadata.replay_bundle_digest.clone(),
        lifecycle_metadata.promotion_record_identity.clone(),
        lifecycle_metadata.promotion_proof_digest.clone(),
    );
    let basis = PreviewSessionBasis::new(binding_tuple.clone());
    let report = PreviewBindingReport::new(
        binding_tuple.digest().to_string(),
        query_context.evaluation_class.clone(),
        query_context.binding_intent.clone(),
        counters,
    );

    Ok(PreviewSessionPlanBinding {
        preflight,
        query_context,
        basis,
        lifecycle_metadata,
        report,
    })
}

fn reject_unsupported_preview_family(
    preflight: &ExecutionPreflightBundle,
) -> Result<(), PreviewBindingError> {
    let Some(collection) = preflight.plan().collection() else {
        return Ok(());
    };

    let unsupported_family = matches!(
        collection.planning_context().result_family(),
        CollectionResultFamily::CdcCollection
    ) || !matches!(
        collection
            .post_read_shaping()
            .aggregate_shape()
            .function_family(),
        AggregateFunctionFamily::NoneAdmittedYet
    ) || !matches!(
        collection
            .post_read_shaping()
            .derived_field_plan()
            .computation_class(),
        DerivedFieldComputationClass::NoneAdmittedYet
    );

    if unsupported_family {
        let mut counters = PreviewBindingCounters::default();
        counters.preview_invalid_basis_denial_count = 1;
        return Err(PreviewBindingError::new(
            PreviewBindingFailureClass::UnsupportedPreviewQueryFamily,
            "preview binding only admits detail, ordinary collection, and bounded materialization families in phases 1-2",
            counters,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        admit_authoritative_preview_comparison_candidate,
        admit_preview_promotion_parity_comparison,
        admit_promotion_eligible_preview_session_plan_binding,
        admit_preview_workflow_foundation, admit_read_only_preview_session_plan_binding,
        bind_preflight_to_preview_session, derive_preview_comparison_eligibility,
        execute_preview_session_plan, execute_promotion_eligible_preview_session_plan,
        execute_read_only_preview_session_plan, PreviewBindingFailureClass, PreviewBindingIntent,
        PreviewComparisonFailureClass, PreviewEvaluationClass, PreviewExecutionFailureClass,
        PreviewSessionQueryContext,
    };
    use crate::harness::fixtures::{
        execution_preflights,
        preview_bridge::{
            active_preview_artifacts, admitted_preview_session, declared_preview_session,
            discarded_preview_artifacts, promoted_preview_artifacts,
            promoted_preview_replay_bundle,
        },
    };
    use forge_runtime_bridge::facade::BridgePreviewLifecycleStateKind;

    #[test]
    fn active_preview_binding_succeeds_with_required_tuple_fields() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-active-success");
        let binding = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("active preview should bind");

        assert_eq!(
            binding.basis().binding_tuple().preview_session_identity(),
            active.session_identity()
        );
        assert_eq!(
            binding.basis().binding_tuple().lifecycle_state_kind(),
            BridgePreviewLifecycleStateKind::Active
        );
        assert_eq!(
            binding.basis().binding_tuple().canonical_query_digest(),
            preflight.plan().query().canonical_query_digest()
        );
        assert_eq!(
            binding
                .basis()
                .binding_tuple()
                .canonical_result_shape_digest(),
            preflight
                .plan()
                .result_shape()
                .canonical_result_shape_digest()
        );
        assert_eq!(
            binding.basis().binding_tuple().evaluation_class(),
            &PreviewEvaluationClass::read_only()
        );
        assert_eq!(
            binding
                .report()
                .counters()
                .preview_lifecycle_rediscovery_count(),
            0
        );
        assert_eq!(
            binding
                .report()
                .counters()
                .preview_executor_rediscovery_count(),
            0
        );
    }

    #[test]
    fn non_active_lifecycle_bindings_are_rejected() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_declared_runtime, declared) = declared_preview_session("preview-declared-reject");
        let declared_error = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::declared(&declared, PreviewEvaluationClass::read_only()),
        )
        .expect_err("declared preview should reject");
        assert_eq!(
            declared_error.failure_class(),
            &PreviewBindingFailureClass::StaleOrInactivePreviewLifecycle
        );

        let (_admitted_runtime, admitted) = admitted_preview_session("preview-admitted-reject");
        let admitted_error = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::admitted(&admitted, PreviewEvaluationClass::read_only()),
        )
        .expect_err("admitted preview should reject");
        assert_eq!(
            admitted_error.failure_class(),
            &PreviewBindingFailureClass::StaleOrInactivePreviewLifecycle
        );

        let (_discarded_runtime, discarded, _) =
            discarded_preview_artifacts("preview-discarded-reject");
        let discarded_error = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::discarded(&discarded, PreviewEvaluationClass::read_only()),
        )
        .expect_err("discarded preview should reject");
        assert_eq!(
            discarded_error.failure_class(),
            &PreviewBindingFailureClass::StaleOrInactivePreviewLifecycle
        );

        let (_promoted_runtime, promoted, _, _) =
            promoted_preview_artifacts("preview-promoted-reject");
        let promoted_error = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::promoted(&promoted, PreviewEvaluationClass::read_only()),
        )
        .expect_err("promoted preview should reject");
        assert_eq!(
            promoted_error.failure_class(),
            &PreviewBindingFailureClass::StaleOrInactivePreviewLifecycle
        );
    }

    #[test]
    fn preview_evaluation_classes_remain_distinct() {
        let read_only = PreviewEvaluationClass::read_only();
        let promotable = PreviewEvaluationClass::promotion_eligible();

        assert_ne!(read_only, promotable);
        assert_eq!(read_only.as_str(), "read_only");
        assert_eq!(promotable.as_str(), "promotion_eligible");
    }

    #[test]
    fn missing_execution_record_identity_for_active_preview_rejects() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, _execution_record) =
            active_preview_artifacts("preview-missing-execution-record");
        let error = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active_without_execution_record(
                &active,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect_err("active preview without execution record should reject");

        assert_eq!(
            error.failure_class(),
            &PreviewBindingFailureClass::MissingExecutionRecordIdentity
        );
    }

    #[test]
    fn store_backed_preflight_plus_preview_binding_rejects() {
        let preflight = execution_preflights::store_detail_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-store-preflight");
        let error = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect_err("store-backed preflight should reject");

        assert_eq!(
            error.failure_class(),
            &PreviewBindingFailureClass::StoreBackedRouteForbidden
        );
    }

    #[test]
    fn preview_live_request_rejects_in_phase_one_and_two() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) = active_preview_artifacts("preview-live-denied");
        let error = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            )
            .with_binding_intent(PreviewBindingIntent::preview_with_live_lane()),
        )
        .expect_err("preview live request should reject");

        assert_eq!(
            error.failure_class(),
            &PreviewBindingFailureClass::PreviewLiveDeniedInPhaseTwo
        );
    }

    #[test]
    fn binding_tuple_digest_is_stable_for_equivalent_admitted_inputs() {
        let left_preflight = execution_preflights::direct_runtime_preflight();
        let right_preflight = execution_preflights::replay_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-digest-stability");

        let left = bind_preflight_to_preview_session(
            left_preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("left preview binding should succeed");
        let right = bind_preflight_to_preview_session(
            right_preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("right preview binding should succeed");

        assert_eq!(
            left.basis().binding_tuple().digest(),
            right.basis().binding_tuple().digest()
        );
    }

    #[test]
    fn evaluation_class_changes_binding_tuple_digest() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-evaluation-class-digest");

        let read_only = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("read-only preview binding should succeed");
        let promotable = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("promotion-eligible preview binding should succeed");

        assert_ne!(
            read_only.basis().binding_tuple().digest(),
            promotable.basis().binding_tuple().digest()
        );
    }

    #[test]
    fn promotion_linkage_rejects_even_for_promotion_eligible_active_binding() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-promotion-linkage-denied");
        let (_promoted_runtime, _promoted, _promoted_execution, promotion_record) =
            promoted_preview_artifacts("preview-promotion-linkage-source");

        let error = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            )
            .with_promotion_record(&promotion_record),
        )
        .expect_err("promotion linkage should reject for phase 1-2 active binding");

        assert_eq!(
            error.failure_class(),
            &PreviewBindingFailureClass::PromotionLinkageMismatch
        );
    }

    #[test]
    fn replay_bundle_rejects_for_phase_one_and_two_active_binding() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-replay-linkage-denied");
        let (_promoted_runtime, _promoted, _promoted_execution, _promotion_record, replay_bundle) =
            promoted_preview_replay_bundle("preview-replay-linkage-source");

        let error = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            )
            .with_replay_bundle(&replay_bundle),
        )
        .expect_err("replay linkage should reject for phase 1-2 active binding");

        assert_eq!(
            error.failure_class(),
            &PreviewBindingFailureClass::PromotionLinkageMismatch
        );
    }

    #[test]
    fn unsupported_preview_query_families_are_rejected() {
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-unsupported-families");

        for preflight in [
            execution_preflights::cdc_collection_preflight(),
            execution_preflights::aggregate_rollup_collection_preflight(),
            execution_preflights::derived_field_collection_preflight(),
        ] {
            let error = bind_preflight_to_preview_session(
                preflight,
                PreviewSessionQueryContext::active(
                    &active,
                    &execution_record,
                    PreviewEvaluationClass::read_only(),
                ),
            )
            .expect_err("unsupported preview family should reject");

            assert_eq!(
                error.failure_class(),
                &PreviewBindingFailureClass::UnsupportedPreviewQueryFamily
            );
        }
    }

    #[test]
    fn preview_execution_envelope_preserves_zero_rediscovery_invariants() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-execution-envelope");
        let binding = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("preview binding should succeed");

        let execution =
            execute_preview_session_plan(&binding).expect("preview execution should succeed");

        assert_eq!(
            execution
                .counters()
                .binding_counters()
                .preview_lifecycle_rediscovery_count(),
            0
        );
        assert_eq!(
            execution
                .counters()
                .binding_counters()
                .preview_executor_rediscovery_count(),
            0
        );
        assert_eq!(
            execution
                .counters()
                .execution_counters()
                .executor_semantic_rediscovery_count(),
            0
        );
        assert_eq!(execution.counters().preview_execution_envelope_count(), 1);
        assert_eq!(execution.counters().preview_execution_count(), 1);
        assert_eq!(execution.counters().preview_read_only_execution_count(), 1);
        assert_eq!(execution.counters().preview_promotable_execution_count(), 0);
        assert_eq!(
            execution.counters().preview_comparison_shape_check_width,
            execution.comparison_eligibility().shape_check_width()
        );
        assert_eq!(
            execution.binding.basis().binding_tuple().digest(),
            binding.basis().binding_tuple().digest()
        );
        assert_eq!(
            execution.execution.report().result_digest(),
            &execution.report().result_digest
        );
        assert_eq!(
            execution.report().binding_digest,
            binding.basis().binding_tuple().digest()
        );
        assert_eq!(
            execution.report().basis_digest(),
            execution.execution.report().basis_digest().as_str()
        );
        assert_eq!(
            execution.report().preview_session_identity(),
            binding.basis().binding_tuple().preview_session_identity()
        );
        assert_eq!(
            execution.report().lifecycle_state_kind(),
            binding.basis().binding_tuple().lifecycle_state_kind()
        );
        assert_eq!(
            execution.report().execution_record_identity(),
            binding
                .basis()
                .binding_tuple()
                .execution_record_identity()
                .expect("binding should carry execution record identity")
        );
        assert_eq!(
            &execution.report().query_digest,
            binding.preflight().plan().query().validated_query_digest()
        );
        assert!(!execution.report().preview_execution_digest.is_empty());
        assert_eq!(
            execution.report().comparison_eligibility_digest(),
            execution.comparison_eligibility().digest()
        );
        assert_eq!(
            execution.report().workflow_foundation_digest(),
            execution.workflow_foundation().digest()
        );
    }

    #[test]
    fn read_only_preview_execution_entrypoint_requires_read_only_binding() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-read-only-entrypoint");
        let read_only_binding = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("read-only binding should succeed");
        let promotion_binding = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("promotion-eligible binding should succeed");

        let read_only_binding = admit_read_only_preview_session_plan_binding(read_only_binding)
            .expect("read-only binding should admit to the read-only execution class");
        let read_only_execution = execute_read_only_preview_session_plan(&read_only_binding)
            .expect("read-only execution entrypoint should accept read-only binding");
        assert_eq!(
            read_only_execution
                .as_preview_execution()
                .binding
                .basis()
                .binding_tuple()
                .evaluation_class(),
            &PreviewEvaluationClass::read_only()
        );

        let mismatch = admit_read_only_preview_session_plan_binding(promotion_binding)
            .expect_err("read-only witness admission should reject promotion-eligible binding");
        assert_eq!(
            mismatch.failure_class(),
            PreviewExecutionFailureClass::InvalidExecutionClass
        );
    }

    #[test]
    fn promotion_eligible_preview_execution_entrypoint_requires_promotion_binding() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-promotion-entrypoint");
        let read_only_binding = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("read-only binding should succeed");
        let promotion_binding = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("promotion-eligible binding should succeed");

        let promotion_binding =
            admit_promotion_eligible_preview_session_plan_binding(promotion_binding)
                .expect("promotion binding should admit to the promotion execution class");
        let promotion_execution =
            execute_promotion_eligible_preview_session_plan(&promotion_binding)
                .expect("promotion entrypoint should accept promotion binding");
        assert_eq!(
            promotion_execution
                .as_preview_execution()
                .binding
                .basis()
                .binding_tuple()
                .evaluation_class(),
            &PreviewEvaluationClass::promotion_eligible()
        );
        assert_eq!(
            promotion_execution
                .as_preview_execution()
                .counters()
                .preview_promotable_execution_count(),
            1
        );
        assert_eq!(
            promotion_execution
                .as_preview_execution()
                .counters()
                .preview_read_only_execution_count(),
            0
        );

        let mismatch = admit_promotion_eligible_preview_session_plan_binding(read_only_binding)
            .expect_err("promotion witness admission should reject read-only binding");
        assert_eq!(
            mismatch.failure_class(),
            PreviewExecutionFailureClass::InvalidExecutionClass
        );
    }

    #[test]
    fn preview_execution_comparison_admits_shape_compatible_runtime_result() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-comparison-admitted");
        let binding = bind_preflight_to_preview_session(
            preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("preview binding should succeed");
        let preview_execution =
            admit_promotion_eligible_preview_session_plan_binding(binding)
                .expect("promotion-eligible binding should admit");
        let preview_execution = execute_promotion_eligible_preview_session_plan(&preview_execution)
            .expect("preview execution should succeed");
        let candidate_execution = crate::execution::execute_preflight_bundle(&preflight)
            .expect("candidate execution should succeed");
        let candidate =
            admit_authoritative_preview_comparison_candidate(&preflight, &candidate_execution)
                .expect("runtime candidate should admit");

        let admission =
            admit_preview_promotion_parity_comparison(&preview_execution, &candidate)
            .expect("shape-compatible runtime result should admit comparison");
        let admission = admission.as_preview_comparison();

        assert!(!admission.digest().is_empty());
        assert_eq!(
            admission.preview_execution_digest(),
            preview_execution
                .as_preview_execution()
                .report()
                .preview_execution_digest()
        );
        assert_eq!(
            admission.candidate_result_digest(),
            candidate_execution.report().result_digest()
        );
        assert_eq!(
            admission.candidate_basis_digest(),
            preflight.basis().proof().digest().as_str()
        );
        assert!(admission.shape_check_width() > 0);
        assert_eq!(admission.counters().preview_promotion_comparison_count(), 1);
        assert_eq!(
            admission.counters().preview_promotion_comparison_denial_count(),
            0
        );
        assert_eq!(admission.counters().preview_basis_pair_width(), 2);
    }

    #[test]
    fn preview_execution_comparison_rejects_query_digest_mismatch_before_shape_checks() {
        let preview_preflight =
            execution_preflights::ordered_collection_without_traversal_preflight();
        let candidate_preflight = execution_preflights::ordered_collection_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-comparison-ordering-mismatch");
        let binding = bind_preflight_to_preview_session(
            preview_preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("ordered preview binding should succeed");
        let preview_execution =
            admit_promotion_eligible_preview_session_plan_binding(binding)
                .expect("promotion binding should admit");
        let preview_execution = execute_promotion_eligible_preview_session_plan(&preview_execution)
            .expect("preview execution should succeed");
        let candidate_execution = crate::execution::execute_preflight_bundle(&candidate_preflight)
            .expect("candidate execution should succeed");
        let candidate = admit_authoritative_preview_comparison_candidate(
            &candidate_preflight,
            &candidate_execution,
        )
        .expect("shape-mismatched runtime candidate should still admit as authoritative");

        let error = admit_preview_promotion_parity_comparison(&preview_execution, &candidate)
            .expect_err("materially different collection shape should reject comparison");

        assert_eq!(
            error.failure_class(),
            &PreviewComparisonFailureClass::QueryDigestMismatch
        );
        assert!(!error.preview_digest().is_empty());
        assert!(!error.candidate_digest().is_empty());
        assert_eq!(
            error.counters().preview_promotion_comparison_denial_count(),
            1
        );
    }

    #[test]
    fn preview_execution_comparison_rejects_store_backed_candidates() {
        let preview_preflight = execution_preflights::direct_runtime_preflight();
        let candidate_preflight = execution_preflights::store_detail_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-comparison-store-candidate");
        let binding = bind_preflight_to_preview_session(
            preview_preflight.clone(),
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("preview binding should succeed");
        let _preview_execution =
            execute_preview_session_plan(&binding).expect("preview execution should succeed");
        let candidate_execution = crate::execution::execute_preflight_bundle(&candidate_preflight)
            .expect("candidate execution should succeed");
        let error = admit_authoritative_preview_comparison_candidate(
            &candidate_preflight,
            &candidate_execution,
        )
        .expect_err("store-backed comparison candidates must reject before comparison admission");

        assert_eq!(
            error.failure_class(),
            &PreviewComparisonFailureClass::CandidateBasisAuthorityMismatch
        );
    }

    #[test]
    fn read_only_preview_execution_stays_read_only_at_comparison_boundary() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-comparison-read-only-boundary");
        let binding = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("read-only preview binding should succeed");
        let read_only_binding = admit_read_only_preview_session_plan_binding(binding)
            .expect("read-only binding should admit");
        let read_only_execution = execute_read_only_preview_session_plan(&read_only_binding)
            .expect("read-only execution should succeed");

        assert_eq!(
            read_only_execution
                .as_preview_execution()
                .binding()
                .basis()
                .binding_tuple()
                .evaluation_class(),
            &PreviewEvaluationClass::read_only()
        );
    }

    #[test]
    fn preview_comparison_candidate_rejects_inconsistent_execution_preflight_pair() {
        let candidate_preflight = execution_preflights::direct_runtime_preflight();
        let mismatched_execution_preflight = execution_preflights::ordered_collection_preflight();
        let mismatched_execution =
            crate::execution::execute_preflight_bundle(&mismatched_execution_preflight)
                .expect("mismatched execution should succeed");

        let error = admit_authoritative_preview_comparison_candidate(
            &candidate_preflight,
            &mismatched_execution,
        )
        .expect_err("candidate proof should reject execution from a different admitted plan");

        assert!(
            matches!(
                error.failure_class(),
                PreviewComparisonFailureClass::CandidateExecutionPlanMismatch
                    | PreviewComparisonFailureClass::CandidateExecutionBasisMismatch
            ),
            "expected candidate proof to reject inconsistent execution pair, got {:?}",
            error.failure_class()
        );
    }

    #[test]
    fn preview_comparison_candidate_tracks_candidate_shape_contracts() {
        let preflight = execution_preflights::ordered_collection_without_traversal_preflight();
        let execution = crate::execution::execute_preflight_bundle(&preflight)
            .expect("candidate execution should succeed");
        let candidate = admit_authoritative_preview_comparison_candidate(&preflight, &execution)
            .expect("authoritative runtime candidate should admit");
        let artifact = candidate.artifact();

        assert_eq!(
            artifact.validated_query_digest(),
            preflight.plan().query().validated_query_digest()
        );
        assert_eq!(
            artifact.result_digest(),
            execution.report().result_digest()
        );
        assert_eq!(
            artifact.basis_digest(),
            preflight.basis().proof().digest().as_str()
        );
        assert!(artifact.collection_digest().is_some());
        assert_eq!(artifact.result_family(), "ordinary_collection");
        assert!(artifact.shape_check_width() > 0);
    }

    #[test]
    fn preview_comparison_eligibility_uses_collection_shape_contracts() {
        let preflight = execution_preflights::ordered_collection_without_traversal_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-comparison-eligibility");
        let binding = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("ordered collection preview should bind");

        let artifact = derive_preview_comparison_eligibility(&binding);

        assert_eq!(artifact.result_family(), "ordinary_collection");
        assert_eq!(
            &artifact.canonical_query_digest,
            binding.basis().binding_tuple().canonical_query_digest()
        );
        assert_eq!(
            &artifact.canonical_result_shape_digest,
            binding
                .basis()
                .binding_tuple()
                .canonical_result_shape_digest()
        );
        assert!(artifact.collection_digest().is_some());
        assert!(artifact.shape_check_width() > 0);
        assert!(!artifact.ordering_digest().is_empty());
        assert!(!artifact.materialization_boundary_digest().is_empty());
    }

    #[test]
    fn preview_workflow_foundation_is_bound_to_the_admitted_preview_tuple() {
        let preflight = execution_preflights::direct_runtime_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-workflow-foundation");
        let binding = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::promotion_eligible(),
            ),
        )
        .expect("promotion-eligible preview binding should succeed");

        let workflow = admit_preview_workflow_foundation(&binding);

        assert_eq!(
            workflow.artifact().preview_session_identity(),
            binding.basis().binding_tuple().preview_session_identity()
        );
        assert_eq!(
            workflow.artifact().declaration_identity(),
            binding.basis().binding_tuple().declaration_identity()
        );
        assert_eq!(
            workflow.artifact().execution_record_identity(),
            binding
                .basis()
                .binding_tuple()
                .execution_record_identity()
                .expect("active binding should carry execution record identity")
        );
        assert_eq!(
            workflow.artifact().evaluation_class(),
            &PreviewEvaluationClass::promotion_eligible()
        );
        assert_eq!(
            workflow.artifact().binding_digest(),
            binding.basis().binding_tuple().digest()
        );
        assert_eq!(
            workflow.artifact().declaration_digest(),
            binding.basis().binding_tuple().declaration_digest()
        );
        assert_eq!(
            workflow.artifact().lifecycle_state_kind(),
            BridgePreviewLifecycleStateKind::Active
        );
    }

    #[test]
    fn preview_execution_failure_classifies_underlying_execution_errors() {
        let preflight = execution_preflights::cdc_collection_preflight();
        let (_runtime, active, execution_record) =
            active_preview_artifacts("preview-execution-underlying-failure");
        let error = bind_preflight_to_preview_session(
            preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect_err("unsupported preview families should reject before execution");

        assert_eq!(
            error.failure_class(),
            &PreviewBindingFailureClass::UnsupportedPreviewQueryFamily
        );

        let supported_preflight = execution_preflights::direct_runtime_preflight();
        let binding = bind_preflight_to_preview_session(
            supported_preflight,
            PreviewSessionQueryContext::active(
                &active,
                &execution_record,
                PreviewEvaluationClass::read_only(),
            ),
        )
        .expect("supported preview binding should succeed");
        let execution = execute_preview_session_plan(&binding)
            .expect("supported preview execution should work");

        assert_eq!(
            execution
                .check_invariants()
                .map(|_| ())
                .map_err(|err| err.failure_class()),
            Ok(())
        );
        assert_ne!(
            PreviewExecutionFailureClass::UnderlyingExecutionFailure,
            PreviewExecutionFailureClass::InternalInvariantBreak
        );
    }
}
