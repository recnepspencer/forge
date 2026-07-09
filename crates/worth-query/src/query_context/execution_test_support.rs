use super::{
    HistoricalAdmissionClass, HistoricalMaterializationCostClass, QueryContextBudgetClass,
    QueryContextCostClass, QueryContextExecutionArtifact, QueryContextExecutionCounters,
    QueryContextExecutionFamily, QueryContextPredictionDriftOutcome,
};
use crate::projection_consumption::ProjectionMaterializedFactPosture;

impl QueryContextExecutionArtifact {
    pub(crate) fn test_only(
        family: QueryContextExecutionFamily,
        query_digest: &str,
        basis_digest: &str,
        result_digest: &str,
        result_shape_digest: &str,
        rows: Vec<String>,
        materialization_path_identity: Option<&str>,
        preview_provenance_identity: Option<&str>,
    ) -> Self {
        let materialized_row_count = rows.len();
        Self {
            query_digest: query_digest.to_string(),
            basis_digest: basis_digest.to_string(),
            result_digest: result_digest.to_string(),
            result_shape_digest: result_shape_digest.to_string(),
            rows,
            family,
            cost_class: QueryContextCostClass::CurrentHeadNarrow,
            budget_class: QueryContextBudgetClass::NarrowSingleBasis,
            prediction_report: None,
            prediction_drift_outcome: QueryContextPredictionDriftOutcome::WithinBudget,
            historical_admission_class: materialization_path_identity
                .map(|_| HistoricalAdmissionClass::RuntimeRetained),
            historical_materialization_cost_class: materialization_path_identity
                .map(|_| HistoricalMaterializationCostClass::RetainedBounded),
            requested_path_identity: materialization_path_identity.map(str::to_string),
            admitted_path_identity: materialization_path_identity.map(str::to_string),
            resolved_path_identity: materialization_path_identity.map(str::to_string),
            materialization_path_identity: materialization_path_identity.map(str::to_string),
            preview_provenance_identity: preview_provenance_identity.map(str::to_string),
            materialized_fact_posture: None,
            counters: QueryContextExecutionCounters {
                context_execution_count: 1,
                materialized_row_count,
                result_shape_width: 1,
                executor_rediscovery_count: 0,
            },
        }
    }

    pub(crate) fn test_only_with_materialized_fact_posture(
        mut self,
        posture: ProjectionMaterializedFactPosture,
    ) -> Self {
        self.materialized_fact_posture = Some(posture);
        self
    }
}
