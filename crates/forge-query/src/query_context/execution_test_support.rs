use super::{
    HistoricalAdmissionClass, HistoricalMaterializationCostClass, QueryContextBudgetClass,
    QueryContextCostClass, QueryContextExecutionArtifact, QueryContextExecutionCounters,
    QueryContextExecutionFamily, QueryContextPredictionDriftOutcome,
};

impl QueryContextExecutionArtifact {
    pub(crate) fn test_only(
        family: QueryContextExecutionFamily,
        query_digest: &str,
        basis_digest: &str,
        result_digest: &str,
        result_shape_digest: &str,
        payload: Vec<String>,
        materialization_path_identity: Option<&str>,
        preview_provenance_identity: Option<&str>,
    ) -> Self {
        let payload_row_count = payload.len();
        Self {
            query_digest: query_digest.to_string(),
            basis_digest: basis_digest.to_string(),
            result_digest: result_digest.to_string(),
            result_shape_digest: result_shape_digest.to_string(),
            payload,
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
            counters: QueryContextExecutionCounters {
                context_execution_count: 1,
                payload_row_count,
                result_shape_width: 1,
                executor_rediscovery_count: 0,
            },
        }
    }
}
