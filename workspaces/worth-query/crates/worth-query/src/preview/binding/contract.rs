use crate::basis::ExecutionPreflightBundle;
use crate::identity::{
    CanonicalQueryDigest, CanonicalResultShapeDigest, ValidatedQueryDigest,
    ValidatedResultShapeDigest,
};
use crate::preview::binding::{PreviewBindingCounters, PreviewComplexityContract};
use crate::preview::evaluation::PreviewEvaluationClass;
use crate::preview::session_context::PreviewSessionQueryContext;
use crate::preview::workflow_context_identity;
use worth_runtime_bridge::facade::{
    BridgePreviewLifecycleStateKind, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewLifecycleMetadata {
    pub(super) lifecycle_state_kind: BridgePreviewLifecycleStateKind,
    pub(super) execution_record_identity: Option<PreviewExecutionRecordIdentity>,
    pub(super) replay_bundle_digest: Option<String>,
    pub(super) promotion_record_identity: Option<String>,
    pub(super) promotion_proof_digest: Option<String>,
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
    pub(super) fn new(
        canonical_query_digest: CanonicalQueryDigest,
        canonical_result_shape_digest: CanonicalResultShapeDigest,
        validated_query_digest: ValidatedQueryDigest,
        validated_result_shape_digest: ValidatedResultShapeDigest,
        evaluation_class: PreviewEvaluationClass,
        preview_session_identity: BridgePreviewSessionIdentity,
        declaration_identity: BridgePreviewSessionDeclarationIdentity,
        declaration_digest: String,
        lifecycle_state_kind: BridgePreviewLifecycleStateKind,
        execution_record_identity: Option<PreviewExecutionRecordIdentity>,
        replay_bundle_digest: Option<String>,
        promotion_record_identity: Option<String>,
        promotion_proof_digest: Option<String>,
    ) -> Self {
        let digest = workflow_context_identity::compose_preview_session_binding_tuple_digest(
            &canonical_query_digest,
            &canonical_result_shape_digest,
            &validated_query_digest,
            &validated_result_shape_digest,
            &evaluation_class,
            &preview_session_identity,
            &declaration_identity,
            &declaration_digest,
            lifecycle_state_kind,
            execution_record_identity.as_ref(),
            replay_bundle_digest.as_deref(),
            promotion_record_identity.as_deref(),
            promotion_proof_digest.as_deref(),
        );
        Self {
            digest,
            canonical_query_digest,
            canonical_result_shape_digest,
            validated_query_digest,
            validated_result_shape_digest,
            evaluation_class,
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
    pub(super) fn new(binding_tuple: PreviewSessionBindingTuple) -> Self {
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
    basis_binding_contract: PreviewComplexityContract,
    execution_metadata_contract: PreviewComplexityContract,
    counters: PreviewBindingCounters,
}

impl PreviewBindingReport {
    pub(super) fn new(
        binding_digest: String,
        evaluation_class: PreviewEvaluationClass,
        counters: PreviewBindingCounters,
    ) -> Self {
        Self {
            binding_digest,
            evaluation_class,
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
    pub(super) preflight: ExecutionPreflightBundle,
    pub(super) query_context: PreviewSessionQueryContext,
    pub(super) basis: PreviewSessionBasis,
    pub(super) lifecycle_metadata: PreviewLifecycleMetadata,
    pub(super) report: PreviewBindingReport,
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
    pub(in crate::preview) inner: PreviewSessionPlanBinding,
}

impl ReadOnlyPreviewSessionPlanBinding {
    pub fn preflight(&self) -> &ExecutionPreflightBundle {
        self.inner.preflight()
    }

    pub fn query_context(&self) -> &PreviewSessionQueryContext {
        self.inner.query_context()
    }

    pub fn basis(&self) -> &PreviewSessionBasis {
        self.inner.basis()
    }

    pub fn lifecycle_metadata(&self) -> &PreviewLifecycleMetadata {
        self.inner.lifecycle_metadata()
    }

    pub fn report(&self) -> &PreviewBindingReport {
        self.inner.report()
    }

    #[cfg(test)]
    pub(crate) fn as_preview_binding(&self) -> &PreviewSessionPlanBinding {
        &self.inner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionEligiblePreviewSessionPlanBinding {
    pub(in crate::preview) inner: PreviewSessionPlanBinding,
}

impl PromotionEligiblePreviewSessionPlanBinding {
    pub fn preflight(&self) -> &ExecutionPreflightBundle {
        self.inner.preflight()
    }

    pub fn query_context(&self) -> &PreviewSessionQueryContext {
        self.inner.query_context()
    }

    pub fn basis(&self) -> &PreviewSessionBasis {
        self.inner.basis()
    }

    pub fn lifecycle_metadata(&self) -> &PreviewLifecycleMetadata {
        self.inner.lifecycle_metadata()
    }

    pub fn report(&self) -> &PreviewBindingReport {
        self.inner.report()
    }

    #[cfg(test)]
    pub(crate) fn as_preview_binding(&self) -> &PreviewSessionPlanBinding {
        &self.inner
    }
}
