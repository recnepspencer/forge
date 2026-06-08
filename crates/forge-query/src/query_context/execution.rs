use crate::execution::execute_preflight_bundle;
use crate::identity::{hash_parts, ResultDigest};
use crate::projection_consumption::ProjectionMaterializedFactPosture;

#[path = "execution_synthetic.rs"]
mod synthetic;

use super::basis::{
    AdmittedQueryBasisContext, HistoricalAdmissionClass, QueryContextAdmissionError,
    QueryContextAdmissionFailureClass,
};
use super::historical::{
    admitted_path_identity, materialization_path_identity, requested_path_identity,
    resolved_path_identity,
};
use super::performance::{
    HistoricalMaterializationCostClass, QueryContextBudgetClass, QueryContextCostClass,
    QueryContextCounters, QueryContextPredictionDriftOutcome, QueryContextPredictionReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryContextExecutionFamily {
    RuntimeCurrent,
    RuntimeBranch,
    HistoricalMaterialized,
    PreviewDerivedHistorical,
}

impl QueryContextExecutionFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeCurrent => "runtime_current",
            Self::RuntimeBranch => "runtime_branch",
            Self::HistoricalMaterialized => "historical_materialized",
            Self::PreviewDerivedHistorical => "preview_derived_historical",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryContextExecutionCounters {
    context_execution_count: usize,
    materialized_row_count: usize,
    result_shape_width: usize,
    executor_rediscovery_count: usize,
}

impl QueryContextExecutionCounters {
    pub fn context_execution_count(&self) -> usize {
        self.context_execution_count
    }

    pub fn materialized_row_count(&self) -> usize {
        self.materialized_row_count
    }

    pub fn result_shape_width(&self) -> usize {
        self.result_shape_width
    }

    pub fn executor_rediscovery_count(&self) -> usize {
        self.executor_rediscovery_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryContextExecutionArtifact {
    query_digest: String,
    basis_digest: String,
    result_digest: String,
    result_shape_digest: String,
    rows: Vec<String>,
    family: QueryContextExecutionFamily,
    cost_class: QueryContextCostClass,
    budget_class: QueryContextBudgetClass,
    prediction_report: Option<QueryContextPredictionReport>,
    prediction_drift_outcome: QueryContextPredictionDriftOutcome,
    historical_admission_class: Option<HistoricalAdmissionClass>,
    historical_materialization_cost_class: Option<HistoricalMaterializationCostClass>,
    requested_path_identity: Option<String>,
    admitted_path_identity: Option<String>,
    resolved_path_identity: Option<String>,
    materialization_path_identity: Option<String>,
    preview_provenance_identity: Option<String>,
    materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
    counters: QueryContextExecutionCounters,
}

impl QueryContextExecutionArtifact {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn result_shape_digest(&self) -> &str {
        &self.result_shape_digest
    }

    pub fn rows(&self) -> &[String] {
        &self.rows
    }

    pub fn family(&self) -> &QueryContextExecutionFamily {
        &self.family
    }

    pub fn cost_class(&self) -> &QueryContextCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &QueryContextBudgetClass {
        &self.budget_class
    }

    pub fn prediction_report(&self) -> Option<&QueryContextPredictionReport> {
        self.prediction_report.as_ref()
    }

    pub fn prediction_drift_outcome(&self) -> &QueryContextPredictionDriftOutcome {
        &self.prediction_drift_outcome
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

    pub fn materialized_fact_posture(&self) -> Option<&ProjectionMaterializedFactPosture> {
        self.materialized_fact_posture.as_ref()
    }

    pub(crate) fn with_materialized_fact_posture(
        mut self,
        posture: Option<ProjectionMaterializedFactPosture>,
    ) -> Self {
        self.materialized_fact_posture = posture;
        self
    }

    pub fn counters(&self) -> &QueryContextExecutionCounters {
        &self.counters
    }
}

pub fn execute_query_basis_context(
    context: &AdmittedQueryBasisContext,
) -> Result<QueryContextExecutionArtifact, QueryContextAdmissionError> {
    match context.binding().evidence() {
        super::basis::QueryBasisBindingEvidenceView::Runtime { preflight } => {
            let execution = execute_preflight_bundle(preflight)
                .expect("runtime preflight execution should succeed");
            let family = if context.family().as_str() == "current_branch_head" {
                QueryContextExecutionFamily::RuntimeCurrent
            } else {
                QueryContextExecutionFamily::RuntimeBranch
            };
            Ok(QueryContextExecutionArtifact {
                query_digest: execution.report().query_digest().as_str().to_string(),
                basis_digest: execution.report().basis_digest().as_str().to_string(),
                result_digest: execution.report().result_digest().as_str().to_string(),
                result_shape_digest: preflight
                    .plan()
                    .result_shape()
                    .canonical_result_shape_digest()
                    .as_str()
                    .to_string(),
                rows: execution.rows().to_vec(),
                family,
                cost_class: context.cost_class().clone(),
                budget_class: context.budget_class().clone(),
                prediction_report: context.prediction_report().cloned(),
                prediction_drift_outcome: QueryContextPredictionDriftOutcome::WithinBudget,
                historical_admission_class: None,
                historical_materialization_cost_class: None,
                requested_path_identity: None,
                admitted_path_identity: None,
                resolved_path_identity: None,
                materialization_path_identity: None,
                preview_provenance_identity: None,
                materialized_fact_posture: None,
                counters: QueryContextExecutionCounters {
                    context_execution_count: 1,
                    materialized_row_count: execution.rows().len(),
                    result_shape_width: preflight.plan().result_shape().binding_count(),
                    executor_rediscovery_count: 0,
                },
            })
        }
        super::basis::QueryBasisBindingEvidenceView::Historical {
            query_preflight,
            admission,
            metadata,
        } => {
            let requested_path_class = admission.requested_path().requested_path_class().as_str();
            let materialization_identity = materialization_path_identity(metadata);
            let requested_path_identity = requested_path_identity(metadata);
            let admitted_path_identity = admitted_path_identity(metadata);
            let resolved_path_identity = resolved_path_identity(metadata);
            let historical_admission_class = context
                .historical_admission_class()
                .cloned()
                .ok_or_else(|| {
                    QueryContextAdmissionError::new(
                        QueryContextAdmissionFailureClass::UnsupportedHistoricalBasis,
                        "historical execution requires an admitted historical basis class",
                        QueryContextCounters::for_denial(true, false),
                    )
                })?;
            let historical_materialization_cost_class = context
                .historical_materialization_cost_class()
                .cloned()
                .ok_or_else(|| {
                    QueryContextAdmissionError::new(
                        QueryContextAdmissionFailureClass::UnsupportedHistoricalMaterializationPathClass,
                        "historical execution requires explicit materialization cost posture",
                        QueryContextCounters::for_denial(true, false),
                    )
                })?;
            if historical_admission_class == HistoricalAdmissionClass::RuntimeReconstruction
                && query_preflight.plan().result_shape().binding_count() > 1
            {
                return Err(QueryContextAdmissionError::new(
                    QueryContextAdmissionFailureClass::HistoricalPathTooBroadDenied,
                    "historical execution denies reconstruction lanes that would broaden beyond the admitted narrow result shape",
                    QueryContextCounters::for_historical_broadening_denial(),
                ));
            }
            let rows = synthetic::synthetic_rows(
                query_preflight,
                context.basis_digest(),
                Some(requested_path_class),
                Some(materialization_identity.as_str()),
            );
            let result_digest = synthetic::synthetic_result_digest(
                query_preflight,
                context.basis_digest(),
                &rows,
                Some(requested_path_class),
                Some(materialization_identity.as_str()),
            );
            Ok(QueryContextExecutionArtifact {
                query_digest: context.query_digest().to_string(),
                basis_digest: context.basis_digest().to_string(),
                result_digest: result_digest.as_str().to_string(),
                result_shape_digest: query_preflight
                    .plan()
                    .result_shape()
                    .canonical_result_shape_digest()
                    .as_str()
                    .to_string(),
                rows,
                family: QueryContextExecutionFamily::HistoricalMaterialized,
                cost_class: context.cost_class().clone(),
                budget_class: context.budget_class().clone(),
                prediction_report: context.prediction_report().cloned(),
                prediction_drift_outcome: QueryContextPredictionDriftOutcome::WithinBudget,
                historical_admission_class: Some(historical_admission_class),
                historical_materialization_cost_class: Some(historical_materialization_cost_class),
                requested_path_identity: Some(requested_path_identity),
                admitted_path_identity: Some(admitted_path_identity),
                resolved_path_identity: Some(resolved_path_identity),
                materialization_path_identity: Some(materialization_identity),
                preview_provenance_identity: None,
                materialized_fact_posture: None,
                counters: QueryContextExecutionCounters {
                    context_execution_count: 1,
                    materialized_row_count: query_preflight.plan().result_shape().binding_count(),
                    result_shape_width: query_preflight.plan().result_shape().binding_count(),
                    executor_rediscovery_count: 0,
                },
            })
        }
        super::basis::QueryBasisBindingEvidenceView::PreviewDerived { foundation } => {
            let preview_identity = context
                .preview_provenance_identity_source()
                .map(ToString::to_string)
                .ok_or_else(|| {
                    QueryContextAdmissionError::new(
                        QueryContextAdmissionFailureClass::PreviewProvenanceRequired,
                        "preview-derived execution requires explicit admitted preview provenance",
                        QueryContextCounters::for_denial(false, false),
                    )
                })?;
            let rows = (0..foundation.shape_check_width())
                .map(|index| {
                    format!(
                        "preview:{}:{}:{}:{}",
                        foundation.validated_query_digest().as_str(),
                        context.basis_digest(),
                        preview_identity,
                        index
                    )
                })
                .collect::<Vec<_>>();
            let materialized_row_count = rows.len();
            let result_digest = ResultDigest::from_parts(
                &rows
                    .iter()
                    .enumerate()
                    .map(|(index, value)| format!("row:{}:{}", index, value))
                    .chain(std::iter::once(format!("query:{}", context.query_digest())))
                    .chain(std::iter::once(format!("basis:{}", context.basis_digest())))
                    .chain(std::iter::once(format!("preview:{}", preview_identity)))
                    .chain(std::iter::once(format!(
                        "foundation:{}",
                        foundation.digest()
                    )))
                    .collect::<Vec<_>>(),
            );
            Ok(QueryContextExecutionArtifact {
                query_digest: context.query_digest().to_string(),
                basis_digest: context.basis_digest().to_string(),
                result_digest: result_digest.as_str().to_string(),
                result_shape_digest: hash_parts(&[
                    format!(
                        "preview_query:{}",
                        foundation.validated_query_digest().as_str()
                    ),
                    format!("shape_check_width:{}", foundation.shape_check_width()),
                    "preview_query_context_shape".to_string(),
                ]),
                rows,
                family: QueryContextExecutionFamily::PreviewDerivedHistorical,
                cost_class: context.cost_class().clone(),
                budget_class: context.budget_class().clone(),
                prediction_report: context.prediction_report().cloned(),
                prediction_drift_outcome: QueryContextPredictionDriftOutcome::WithinBudget,
                historical_admission_class: None,
                historical_materialization_cost_class: None,
                requested_path_identity: None,
                admitted_path_identity: None,
                resolved_path_identity: None,
                materialization_path_identity: None,
                preview_provenance_identity: Some(preview_identity),
                materialized_fact_posture: None,
                counters: QueryContextExecutionCounters {
                    context_execution_count: 1,
                    materialized_row_count,
                    result_shape_width: foundation.shape_check_width(),
                    executor_rediscovery_count: 0,
                },
            })
        }
    }
}

#[cfg(test)]
#[path = "execution_test_support.rs"]
mod test_support;
