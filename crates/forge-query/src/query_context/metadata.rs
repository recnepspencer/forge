use super::basis::{
    historical_admission_of, materialization_identity_of, preview_identity_of,
    AdmittedQueryBasisContext, ComparisonBasisFamily, HistoricalAdmissionClass,
    QueryContextAdmissionError, QueryContextAdmissionFailureClass, QueryContextDriftOutcome,
    QueryContextFamily,
};
use super::comparison::{AdmittedDiffQueryContext, QueryDiffChangeSetArtifact};
use super::execution::QueryContextExecutionArtifact;
use super::performance::{
    HistoricalMaterializationCostClass, QueryContextBudgetClass, QueryContextCostClass,
    QueryContextCounters, QueryContextPredictionDriftOutcome, QueryContextPredictionReport,
};
use crate::basis::BasisAuthorityFamily;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBasisMetadata {
    query_digest: String,
    basis_digest: String,
    basis_authority_family: BasisAuthorityFamily,
    basis_family: QueryContextFamily,
    cost_class: QueryContextCostClass,
    budget_class: QueryContextBudgetClass,
    historical_admission_class: Option<HistoricalAdmissionClass>,
    historical_materialization_cost_class: Option<HistoricalMaterializationCostClass>,
    requested_path_identity: Option<String>,
    admitted_path_identity: Option<String>,
    resolved_path_identity: Option<String>,
    materialization_path_identity: Option<String>,
    preview_provenance_identity: Option<String>,
    result_digest: String,
    drift_outcome: QueryContextDriftOutcome,
    prediction_report: Option<QueryContextPredictionReport>,
    prediction_drift_outcome: Option<QueryContextPredictionDriftOutcome>,
}

impl QueryBasisMetadata {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn basis_authority_family(&self) -> &BasisAuthorityFamily {
        &self.basis_authority_family
    }

    pub fn basis_family(&self) -> &QueryContextFamily {
        &self.basis_family
    }

    pub fn cost_class(&self) -> &QueryContextCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &QueryContextBudgetClass {
        &self.budget_class
    }

    pub fn historical_admission_class(&self) -> Option<&HistoricalAdmissionClass> {
        self.historical_admission_class.as_ref()
    }

    pub fn historical_materialization_cost_class(
        &self,
    ) -> Option<&HistoricalMaterializationCostClass> {
        self.historical_materialization_cost_class.as_ref()
    }

    pub fn requested_path_identity(&self) -> Option<&str> {
        self.requested_path_identity.as_deref()
    }

    pub fn admitted_path_identity(&self) -> Option<&str> {
        self.admitted_path_identity.as_deref()
    }

    pub fn resolved_path_identity(&self) -> Option<&str> {
        self.resolved_path_identity.as_deref()
    }

    pub fn materialization_path_identity(&self) -> Option<&str> {
        self.materialization_path_identity.as_deref()
    }

    pub fn preview_provenance_identity(&self) -> Option<&str> {
        self.preview_provenance_identity.as_deref()
    }

    pub fn preview_identity(&self) -> Option<&str> {
        self.preview_provenance_identity()
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn drift_outcome(&self) -> &QueryContextDriftOutcome {
        &self.drift_outcome
    }

    pub fn prediction_report(&self) -> Option<&QueryContextPredictionReport> {
        self.prediction_report.as_ref()
    }

    pub fn prediction_drift_outcome(&self) -> Option<&QueryContextPredictionDriftOutcome> {
        self.prediction_drift_outcome.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffQueryMetadata {
    query_digest: String,
    comparison_basis_family: ComparisonBasisFamily,
    left_basis_digest: String,
    right_basis_digest: String,
    result_shape_digest: String,
    left_result_digest: String,
    right_result_digest: String,
    comparison_result_digest: String,
    cost_class: QueryContextCostClass,
    budget_class: QueryContextBudgetClass,
    prediction_report: QueryContextPredictionReport,
    prediction_drift_outcome: QueryContextPredictionDriftOutcome,
    drift_outcome: QueryContextDriftOutcome,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBasisResultBundle {
    context: AdmittedQueryBasisContext,
    execution: QueryContextExecutionArtifact,
    metadata: QueryBasisMetadata,
    replay_digest: String,
    counter_snapshot_digest: String,
}

impl QueryBasisResultBundle {
    pub fn context(&self) -> &AdmittedQueryBasisContext {
        &self.context
    }

    pub fn execution(&self) -> &QueryContextExecutionArtifact {
        &self.execution
    }

    pub fn metadata(&self) -> &QueryBasisMetadata {
        &self.metadata
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryDiffResultBundle {
    context: AdmittedDiffQueryContext,
    change_set: QueryDiffChangeSetArtifact,
    metadata: DiffQueryMetadata,
    replay_digest: String,
    counter_snapshot_digest: String,
}

impl QueryDiffResultBundle {
    pub fn context(&self) -> &AdmittedDiffQueryContext {
        &self.context
    }

    pub fn change_set(&self) -> &QueryDiffChangeSetArtifact {
        &self.change_set
    }

    pub fn metadata(&self) -> &DiffQueryMetadata {
        &self.metadata
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }
}

impl DiffQueryMetadata {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn comparison_basis_family(&self) -> &ComparisonBasisFamily {
        &self.comparison_basis_family
    }

    pub fn left_basis_digest(&self) -> &str {
        &self.left_basis_digest
    }

    pub fn right_basis_digest(&self) -> &str {
        &self.right_basis_digest
    }

    pub fn left_result_digest(&self) -> &str {
        &self.left_result_digest
    }

    pub fn result_shape_digest(&self) -> &str {
        &self.result_shape_digest
    }

    pub fn right_result_digest(&self) -> &str {
        &self.right_result_digest
    }

    pub fn comparison_result_digest(&self) -> &str {
        &self.comparison_result_digest
    }

    pub fn cost_class(&self) -> &QueryContextCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &QueryContextBudgetClass {
        &self.budget_class
    }

    pub fn prediction_report(&self) -> &QueryContextPredictionReport {
        &self.prediction_report
    }

    pub fn prediction_drift_outcome(&self) -> &QueryContextPredictionDriftOutcome {
        &self.prediction_drift_outcome
    }

    pub fn drift_outcome(&self) -> &QueryContextDriftOutcome {
        &self.drift_outcome
    }
}

pub fn attach_query_basis_metadata(
    context: &AdmittedQueryBasisContext,
    result: &QueryContextExecutionArtifact,
) -> Result<QueryBasisMetadata, QueryContextAdmissionError> {
    if context.query_digest() != result.query_digest() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::BasisSubstitutionForbidden,
            "result metadata attachment requires the original admitted query digest",
            QueryContextCounters::for_denial(false, true),
        ));
    }

    Ok(QueryBasisMetadata {
        query_digest: context.query_digest().to_string(),
        basis_digest: context.basis_digest().to_string(),
        basis_authority_family: context.basis_authority_family().clone(),
        basis_family: context.family().clone(),
        cost_class: context.cost_class().clone(),
        budget_class: context.budget_class().clone(),
        historical_admission_class: result
            .historical_admission_class()
            .cloned()
            .or_else(|| historical_admission_of(context)),
        historical_materialization_cost_class: result
            .historical_materialization_cost_class()
            .cloned()
            .or_else(|| context.historical_materialization_cost_class().cloned()),
        requested_path_identity: result.requested_path_identity().map(ToString::to_string),
        admitted_path_identity: result.admitted_path_identity().map(ToString::to_string),
        resolved_path_identity: result.resolved_path_identity().map(ToString::to_string),
        materialization_path_identity: result
            .materialization_path_identity()
            .map(ToString::to_string)
            .or_else(|| materialization_identity_of(context)),
        preview_provenance_identity: result
            .preview_provenance_identity()
            .map(ToString::to_string)
            .or_else(|| preview_identity_of(context)),
        result_digest: result.result_digest().to_string(),
        drift_outcome: context.drift_outcome().clone(),
        prediction_report: context.prediction_report().cloned(),
        prediction_drift_outcome: Some(result.prediction_drift_outcome().clone()),
    })
}

pub fn build_query_basis_result_bundle(
    context: &AdmittedQueryBasisContext,
    execution: QueryContextExecutionArtifact,
) -> Result<QueryBasisResultBundle, QueryContextAdmissionError> {
    let metadata = attach_query_basis_metadata(context, &execution)?;
    let replay_digest = hash_parts(&[
        format!("query:{}", context.query_digest()),
        format!("basis:{}", context.basis_digest()),
        format!("family:{}", context.family().as_str()),
        format!("result:{}", execution.result_digest()),
        format!("metadata_result:{}", metadata.result_digest()),
        format!(
            "prediction:{}",
            metadata
                .prediction_drift_outcome()
                .map(QueryContextPredictionDriftOutcome::as_str)
                .unwrap_or("none")
        ),
    ]);
    let counter_snapshot_digest = hash_parts(&[
        format!(
            "binding_count:{}",
            context.counters().query_basis_binding_count()
        ),
        format!(
            "historical_lookup:{}",
            context.counters().historical_basis_lookup_count()
        ),
        format!("binding_width:{}", context.counters().basis_binding_width()),
        format!(
            "historical_width:{}",
            context.counters().historical_lookup_width()
        ),
        format!(
            "execution_count:{}",
            execution.counters().context_execution_count()
        ),
        format!(
            "materialized_rows:{}",
            execution.counters().materialized_row_count()
        ),
        format!(
            "result_shape_width:{}",
            execution.counters().result_shape_width()
        ),
    ]);

    Ok(QueryBasisResultBundle {
        context: context.clone(),
        execution,
        metadata,
        replay_digest,
        counter_snapshot_digest,
    })
}

pub fn attach_diff_query_metadata(
    context: &AdmittedDiffQueryContext,
    left_result: &QueryContextExecutionArtifact,
    right_result: &QueryContextExecutionArtifact,
    change_set: &QueryDiffChangeSetArtifact,
) -> Result<DiffQueryMetadata, QueryContextAdmissionError> {
    if left_result.query_digest() != context.left().query_digest()
        || right_result.query_digest() != context.right().query_digest()
    {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::DiffScopeMismatch,
            "diff metadata requires result envelopes that match the admitted context pair",
            QueryContextCounters::for_diff_denial(true, false),
        ));
    }

    if left_result.basis_digest() != context.left().basis_digest()
        || right_result.basis_digest() != context.right().basis_digest()
    {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::ComparisonShapeMismatch,
            "diff metadata requires execution artifacts bound to the admitted basis pair",
            QueryContextCounters::for_diff_denial(false, false),
        ));
    }

    if left_result.result_shape_digest() != right_result.result_shape_digest() {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::ComparisonShapeMismatch,
            "diff metadata requires execution artifacts that preserve one declared result-shape identity",
            QueryContextCounters::for_diff_denial(false, false),
        ));
    }

    if change_set.left_basis_digest() != context.left().basis_digest()
        || change_set.right_basis_digest() != context.right().basis_digest()
        || change_set.query_digest() != context.left().query_digest()
    {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::ComparisonShapeMismatch,
            "diff metadata requires the realized query-shaped change set for the admitted basis pair",
            QueryContextCounters::for_diff_denial(false, false),
        ));
    }

    Ok(DiffQueryMetadata {
        query_digest: context.left().query_digest().to_string(),
        comparison_basis_family: context.family().clone(),
        left_basis_digest: context.left().basis_digest().to_string(),
        right_basis_digest: context.right().basis_digest().to_string(),
        result_shape_digest: left_result.result_shape_digest().to_string(),
        left_result_digest: left_result.result_digest().to_string(),
        right_result_digest: right_result.result_digest().to_string(),
        comparison_result_digest: change_set.result_digest().to_string(),
        cost_class: context.cost_class().clone(),
        budget_class: context.budget_class().clone(),
        prediction_report: context.prediction_report().clone(),
        prediction_drift_outcome: change_set.prediction_drift_outcome().clone(),
        drift_outcome: context.drift_outcome().clone(),
    })
}

pub fn build_query_diff_result_bundle(
    context: &AdmittedDiffQueryContext,
    change_set: QueryDiffChangeSetArtifact,
    left_result: &QueryContextExecutionArtifact,
    right_result: &QueryContextExecutionArtifact,
) -> Result<QueryDiffResultBundle, QueryContextAdmissionError> {
    let metadata = attach_diff_query_metadata(context, left_result, right_result, &change_set)?;
    let replay_digest = hash_parts(&[
        format!("query:{}", context.left().query_digest()),
        format!("comparison_family:{}", context.family().as_str()),
        format!("left_basis:{}", context.left().basis_digest()),
        format!("right_basis:{}", context.right().basis_digest()),
        format!("comparison_result:{}", change_set.result_digest()),
        format!(
            "prediction:{}",
            change_set.prediction_drift_outcome().as_str()
        ),
    ]);
    let counter_snapshot_digest = hash_parts(&[
        format!(
            "comparison_lookups:{}",
            context.counters().comparison_basis_lookup_count()
        ),
        format!(
            "comparison_scope_width:{}",
            context.counters().comparison_scope_width()
        ),
        format!(
            "comparison_row_width:{}",
            context.counters().comparison_row_width()
        ),
        format!(
            "diff_input_breadth:{}",
            context.counters().diff_input_breadth()
        ),
        format!(
            "comparison_broadening_denials:{}",
            context.counters().comparison_broadening_denial_count()
        ),
        format!("change_rows:{}", change_set.rows().len()),
    ]);

    Ok(QueryDiffResultBundle {
        context: context.clone(),
        change_set,
        metadata,
        replay_digest,
        counter_snapshot_digest,
    })
}
