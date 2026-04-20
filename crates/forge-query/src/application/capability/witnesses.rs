use crate::basis::ExecutionPreflightBundle;
use crate::execution::{execute_preflight_bundle, ExecutionError, ExecutionResultEnvelope};
use crate::historical::{
    admit_historical_evaluation_path, HistoricalCapabilityDescriptor,
    HistoricalEvaluationAdmission, HistoricalEvaluationError, HistoricalEvaluationRequest,
};
use crate::live::{promote_preflight_bundle_to_live, LivePromotionError, LiveQueryPlan};
use crate::preview::{
    bind_preflight_to_preview_session, PreviewBindingError, PreviewSessionPlanBinding,
    PreviewSessionQueryContext,
};
use crate::query_context::{
    admit_query_basis_context, attach_diff_query_metadata, attach_query_basis_metadata,
    bind_diff_query_context, bind_query_basis_context, build_query_basis_result_bundle,
    build_query_diff_result_bundle, execute_query_basis_context, shape_query_diff_change_set,
    AdmittedDiffQueryContext, AdmittedQueryBasisContext, DiffQueryMetadata,
    QueryBasisContextBinding, QueryBasisContextRequest, QueryBasisMetadata,
    QueryBasisResultBundle, QueryContextAdmissionError, QueryContextBindingSource,
    QueryContextExecutionArtifact, QueryDiffChangeSetArtifact, QueryDiffResultBundle,
};
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, QueryWorkflowDeclaration,
    WorkflowAdmissionError, WorkflowBindingSource, WorkflowContextBinding,
    WorkflowDeclarationRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryReadCapability {
    facade_digest: String,
}

impl QueryReadCapability {
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn execute_preflight(
        &self,
        preflight: &ExecutionPreflightBundle,
    ) -> Result<ExecutionResultEnvelope, ExecutionError> {
        let _ = &self.facade_digest;
        execute_preflight_bundle(preflight)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveQueryCapability {
    facade_digest: String,
}

impl LiveQueryCapability {
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn promote_preflight(
        &self,
        preflight: &ExecutionPreflightBundle,
    ) -> Result<LiveQueryPlan, LivePromotionError> {
        let _ = &self.facade_digest;
        promote_preflight_bundle_to_live(preflight)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSessionCapability {
    facade_digest: String,
}

impl PreviewSessionCapability {
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn bind_preflight(
        &self,
        preflight: ExecutionPreflightBundle,
        context: PreviewSessionQueryContext,
    ) -> Result<PreviewSessionPlanBinding, PreviewBindingError> {
        let _ = &self.facade_digest;
        bind_preflight_to_preview_session(preflight, context)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowOrchestrationCapability {
    facade_digest: String,
}

impl WorkflowOrchestrationCapability {
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn bind_context(
        &self,
        source: WorkflowBindingSource,
    ) -> Result<WorkflowContextBinding, WorkflowAdmissionError> {
        let _ = &self.facade_digest;
        bind_workflow_context(source)
    }

    pub fn admit_declaration(
        &self,
        binding: &WorkflowContextBinding,
        request: WorkflowDeclarationRequest,
    ) -> Result<QueryWorkflowDeclaration, WorkflowAdmissionError> {
        let _ = &self.facade_digest;
        admit_query_workflow_declaration(binding, request)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalEvaluationCapability {
    facade_digest: String,
}

impl HistoricalEvaluationCapability {
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn admit_path(
        &self,
        request: HistoricalEvaluationRequest,
        capability: HistoricalCapabilityDescriptor,
    ) -> Result<HistoricalEvaluationAdmission, HistoricalEvaluationError> {
        let _ = &self.facade_digest;
        admit_historical_evaluation_path(request, capability)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryContextCapability {
    facade_digest: String,
}

impl QueryContextCapability {
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn bind_basis_context(
        &self,
        request: QueryBasisContextRequest,
        source: QueryContextBindingSource<'_>,
    ) -> Result<QueryBasisContextBinding, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        bind_query_basis_context(request, source)
    }

    pub fn admit_basis_context(
        &self,
        binding: QueryBasisContextBinding,
    ) -> Result<AdmittedQueryBasisContext, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        admit_query_basis_context(binding)
    }

    pub fn bind_diff_context(
        &self,
        left: &AdmittedQueryBasisContext,
        right: &AdmittedQueryBasisContext,
    ) -> Result<AdmittedDiffQueryContext, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        bind_diff_query_context(left, right)
    }

    pub fn execute_basis_context(
        &self,
        context: &AdmittedQueryBasisContext,
    ) -> Result<QueryContextExecutionArtifact, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        execute_query_basis_context(context)
    }

    pub fn execute_basis_result_bundle(
        &self,
        context: &AdmittedQueryBasisContext,
    ) -> Result<QueryBasisResultBundle, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        let execution = execute_query_basis_context(context)?;
        build_query_basis_result_bundle(context, execution)
    }

    pub fn attach_basis_metadata(
        &self,
        context: &AdmittedQueryBasisContext,
        result: &QueryContextExecutionArtifact,
    ) -> Result<QueryBasisMetadata, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        attach_query_basis_metadata(context, result)
    }

    pub fn attach_diff_metadata(
        &self,
        context: &AdmittedDiffQueryContext,
        left_result: &QueryContextExecutionArtifact,
        right_result: &QueryContextExecutionArtifact,
        change_set: &QueryDiffChangeSetArtifact,
    ) -> Result<DiffQueryMetadata, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        attach_diff_query_metadata(context, left_result, right_result, change_set)
    }

    pub fn shape_diff_change_set(
        &self,
        context: &AdmittedDiffQueryContext,
        left_result: &QueryContextExecutionArtifact,
        right_result: &QueryContextExecutionArtifact,
    ) -> Result<QueryDiffChangeSetArtifact, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        shape_query_diff_change_set(context, left_result, right_result)
    }

    pub fn shape_diff_result_bundle(
        &self,
        context: &AdmittedDiffQueryContext,
        left_result: &QueryContextExecutionArtifact,
        right_result: &QueryContextExecutionArtifact,
    ) -> Result<QueryDiffResultBundle, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        let change_set = shape_query_diff_change_set(context, left_result, right_result)?;
        build_query_diff_result_bundle(context, change_set, left_result, right_result)
    }
}
