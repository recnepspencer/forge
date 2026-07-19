use crate::basis::ExecutionPreflightBundle;
use crate::basis_lifecycle::BasisLifecycleIntentDraft;
use crate::composition::{
    ExpandedComposedIntent, GuidedCompositionPath, QueryCompositionError, QueryScopeDescriptor,
    QueryTemplateDescriptor, TemplateBindingSet,
};
use crate::execution::{execute_preflight_bundle, ExecutionError, ExecutionResultEnvelope};
use crate::historical::{
    admit_historical_evaluation_path, HistoricalCapabilityDescriptor,
    HistoricalEvaluationAdmission, HistoricalEvaluationError, HistoricalEvaluationRequest,
};
use crate::identity_evolution::{
    admit_identity_evolution_query, execute_admitted_identity_evolution_query,
    AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError,
    IdentityEvolutionExecutionArtifact, IdentityEvolutionQueryContext,
};
use crate::live::{promote_preflight_bundle_to_live, LivePromotionError, LiveQueryPlan};
use crate::preview::{
    bind_preflight_to_preview_session, PreviewBindingError, PreviewSessionPlanBinding,
    PreviewSessionQueryContext,
};
use crate::query_context::{
    admit_query_basis_context, attach_diff_query_metadata, attach_query_basis_metadata,
    bind_diff_query_context, build_query_basis_result_bundle, build_query_diff_result_bundle,
    execute_query_basis_context, shape_query_diff_change_set, AdmittedDiffQueryContext,
    DiffQueryMetadata, QueryBasisMetadata, QueryBasisResultBundle, QueryContextAdmissionError,
    QueryContextBindingSource, QueryContextExecutionArtifact, QueryDiffChangeSetArtifact,
    QueryDiffResultBundle, ScopedQueryBasisContext, ScopedQueryContextAdmissionError,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCompositionCapability {
    facade_digest: String,
}

impl QueryCompositionCapability {
    #[cfg(test)]
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn expand_detail_scopes(
        &self,
        query: crate::authoring::DetailAuthoredQuery,
        result_shape: crate::authoring::DetailAuthoredResultShape,
        scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    ) -> Result<
        (
            crate::composition::ExpandedScopeArtifact,
            ExpandedComposedIntent,
        ),
        QueryCompositionError,
    > {
        let _ = &self.facade_digest;
        GuidedCompositionPath::expand_detail_scopes(query, result_shape, scopes)
    }

    pub fn expand_collection_scopes(
        &self,
        query: crate::authoring::CollectionAuthoredQuery,
        result_shape: crate::authoring::CollectionAuthoredResultShape,
        scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    ) -> Result<
        (
            crate::composition::ExpandedScopeArtifact,
            ExpandedComposedIntent,
        ),
        QueryCompositionError,
    > {
        let _ = &self.facade_digest;
        GuidedCompositionPath::expand_collection_scopes(query, result_shape, scopes)
    }

    pub fn instantiate_detail_template(
        &self,
        template: QueryTemplateDescriptor<
            crate::authoring::DetailFamily,
            crate::authoring::DetailResultShapeFamily,
        >,
        bindings: TemplateBindingSet,
    ) -> Result<
        (
            crate::composition::TemplateInstantiationArtifact,
            ExpandedComposedIntent,
        ),
        QueryCompositionError,
    > {
        let _ = &self.facade_digest;
        GuidedCompositionPath::instantiate_detail_template(template, bindings)
    }

    pub fn instantiate_collection_template(
        &self,
        template: QueryTemplateDescriptor<
            crate::authoring::CollectionFamily,
            crate::authoring::CollectionResultShapeFamily,
        >,
        bindings: TemplateBindingSet,
    ) -> Result<
        (
            crate::composition::TemplateInstantiationArtifact,
            ExpandedComposedIntent,
        ),
        QueryCompositionError,
    > {
        let _ = &self.facade_digest;
        GuidedCompositionPath::instantiate_collection_template(template, bindings)
    }
}

impl QueryReadCapability {
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn admit_basis_context(
        &self,
        declaration: BasisLifecycleIntentDraft,
        source: QueryContextBindingSource<'_>,
    ) -> Result<ScopedQueryBasisContext, ScopedQueryContextAdmissionError> {
        let _ = &self.facade_digest;
        admit_query_basis_context(declaration, source)
    }

    pub fn bind_diff_context(
        &self,
        left: &ScopedQueryBasisContext,
        right: &ScopedQueryBasisContext,
    ) -> Result<AdmittedDiffQueryContext, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        bind_diff_query_context(left, right)
    }

    pub fn execute_basis_context(
        &self,
        context: &ScopedQueryBasisContext,
    ) -> Result<QueryContextExecutionArtifact, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        execute_query_basis_context(context)
    }

    pub fn execute_basis_result_bundle(
        &self,
        context: &ScopedQueryBasisContext,
    ) -> Result<QueryBasisResultBundle, QueryContextAdmissionError> {
        let _ = &self.facade_digest;
        let execution = execute_query_basis_context(context)?;
        build_query_basis_result_bundle(context, execution)
    }

    pub fn attach_basis_metadata(
        &self,
        context: &ScopedQueryBasisContext,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionCapability {
    facade_digest: String,
}

impl IdentityEvolutionCapability {
    #[cfg(test)]
    pub(crate) fn new(facade_digest: String) -> Self {
        Self { facade_digest }
    }

    pub fn admit_query(
        &self,
        context: IdentityEvolutionQueryContext,
    ) -> Result<AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError> {
        let _ = &self.facade_digest;
        admit_identity_evolution_query(context)
    }

    pub fn execute_query(
        &self,
        admitted: &AdmittedIdentityEvolutionQuery,
    ) -> Result<IdentityEvolutionExecutionArtifact, IdentityEvolutionAdmissionError> {
        let _ = &self.facade_digest;
        execute_admitted_identity_evolution_query(admitted)
    }
}
