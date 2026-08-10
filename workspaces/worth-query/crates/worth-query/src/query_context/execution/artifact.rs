use crate::projection_consumption::ProjectionMaterializedFactPosture;

use super::super::basis::HistoricalAdmissionClass;
use super::super::performance::{
    HistoricalMaterializationCostClass, QueryContextBudgetClass, QueryContextCostClass,
    QueryContextPredictionDriftOutcome, QueryContextPredictionReport,
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
    pub(super) context_execution_count: usize,
    pub(super) materialized_row_count: usize,
    pub(super) result_shape_width: usize,
    pub(super) executor_rediscovery_count: usize,
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
    pub(super) query_digest: String,
    pub(super) basis_digest: String,
    pub(super) result_digest: String,
    pub(super) result_shape_digest: String,
    pub(super) rows: Vec<String>,
    pub(super) family: QueryContextExecutionFamily,
    pub(super) cost_class: QueryContextCostClass,
    pub(super) budget_class: QueryContextBudgetClass,
    pub(super) prediction_report: Option<QueryContextPredictionReport>,
    pub(super) prediction_drift_outcome: QueryContextPredictionDriftOutcome,
    pub(super) historical_admission_class: Option<HistoricalAdmissionClass>,
    pub(super) historical_materialization_cost_class: Option<HistoricalMaterializationCostClass>,
    pub(super) requested_path_identity: Option<String>,
    pub(super) admitted_path_identity: Option<String>,
    pub(super) resolved_path_identity: Option<String>,
    pub(super) materialization_path_identity: Option<String>,
    pub(super) preview_provenance_identity: Option<String>,
    pub(super) materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
    pub(super) counters: QueryContextExecutionCounters,
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
