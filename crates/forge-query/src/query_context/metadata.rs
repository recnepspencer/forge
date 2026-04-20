use super::basis::{
    historical_admission_of, materialization_identity_of, preview_identity_of,
    AdmittedQueryBasisContext, ComparisonBasisFamily, HistoricalAdmissionClass,
    QueryContextAdmissionError, QueryContextAdmissionFailureClass, QueryContextDriftOutcome,
    QueryContextFamily,
};
use super::comparison::AdmittedDiffQueryContext;
use super::execution::QueryContextExecutionArtifact;
use super::performance::{
    HistoricalMaterializationCostClass, QueryContextBudgetClass, QueryContextCostClass,
    QueryContextCounters, QueryContextPredictionDriftOutcome, QueryContextPredictionReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBasisMetadata {
    query_digest: String,
    basis_digest: String,
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
    left_result_digest: String,
    right_result_digest: String,
    drift_outcome: QueryContextDriftOutcome,
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

    pub fn right_result_digest(&self) -> &str {
        &self.right_result_digest
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

pub fn attach_diff_query_metadata(
    context: &AdmittedDiffQueryContext,
    left_result: &QueryContextExecutionArtifact,
    right_result: &QueryContextExecutionArtifact,
) -> Result<DiffQueryMetadata, QueryContextAdmissionError> {
    if left_result.query_digest() != context.left().query_digest()
        || right_result.query_digest() != context.right().query_digest()
    {
        return Err(QueryContextAdmissionError::new(
            QueryContextAdmissionFailureClass::DiffScopeMismatch,
            "diff metadata requires result envelopes that match the admitted context pair",
            QueryContextCounters::for_diff_denial(true),
        ));
    }

    Ok(DiffQueryMetadata {
        query_digest: context.left().query_digest().to_string(),
        comparison_basis_family: context.family().clone(),
        left_basis_digest: context.left().basis_digest().to_string(),
        right_basis_digest: context.right().basis_digest().to_string(),
        left_result_digest: left_result.result_digest().to_string(),
        right_result_digest: right_result.result_digest().to_string(),
        drift_outcome: context.drift_outcome().clone(),
    })
}
