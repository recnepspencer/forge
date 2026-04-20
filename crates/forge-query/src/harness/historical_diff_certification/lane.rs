use crate::harness::certification::{digest_parts, CertificationMatrix};
use crate::query_context::{
    DiffQueryMetadata, QueryBasisMetadata, QueryBasisResultBundle,
    QueryContextAdmissionFailureClass, QueryContextCounters, QueryDiffResultBundle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HistoricalDiffPerturbationClass {
    RuntimeBasis,
    HistoricalBasis,
    ComparisonFamily,
    MetadataShaping,
    PreviewDerivedBasis,
    DeferredHistorical,
    BasisSubstitution,
    BroadDiffDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistoricalDiffFailureClass {
    UnsupportedHistoricalBasis,
    DiffScopeMismatch,
    StoreBackedHistoricalDeferred,
    BasisSubstitutionForbidden,
    BroadComparisonForbidden,
    AmbiguousComparisonBasis,
    ComparisonShapeMismatch,
    ComparisonBroadeningRequired,
    HistoricalPathTooBroadDenied,
    RawStorageDeltaLeakageForbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalDiffLane {
    pub query_digest: String,
    pub basis_digest: String,
    pub comparison_basis_digest: String,
    pub result_digest: String,
    pub result_shape_digest: String,
    pub result_shape_width: usize,
    pub replay_digest: String,
    pub basis_family: String,
    pub comparison_basis_family: String,
    pub cost_class: String,
    pub budget_class: String,
    pub historical_admission_class: String,
    pub materialization_path_identity: String,
    pub preview_provenance_identity: String,
    pub prediction_drift_outcome: String,
    pub exact_counter_values: Vec<String>,
    pub counter_snapshot_digest: String,
}

impl HistoricalDiffLane {
    pub fn from_basis_result_bundle(bundle: &QueryBasisResultBundle) -> Self {
        let metadata = bundle.metadata();
        let execution = bundle.execution();
        let exact_counter_values = basis_counter_values(bundle);
        Self {
            query_digest: metadata.query_digest().to_string(),
            basis_digest: metadata.basis_digest().to_string(),
            comparison_basis_digest: "none".to_string(),
            result_digest: metadata.result_digest().to_string(),
            result_shape_digest: execution.result_shape_digest().to_string(),
            result_shape_width: execution.counters().result_shape_width(),
            replay_digest: bundle.replay_digest().to_string(),
            basis_family: metadata.basis_family().as_str().to_string(),
            comparison_basis_family: "none".to_string(),
            cost_class: metadata.cost_class().as_str().to_string(),
            budget_class: metadata.budget_class().as_str().to_string(),
            historical_admission_class: metadata
                .historical_admission_class()
                .map(|value| value.as_str().to_string())
                .unwrap_or_else(|| "none".to_string()),
            materialization_path_identity: metadata
                .materialization_path_identity()
                .unwrap_or("none")
                .to_string(),
            preview_provenance_identity: metadata
                .preview_provenance_identity()
                .unwrap_or("none")
                .to_string(),
            prediction_drift_outcome: metadata
                .prediction_drift_outcome()
                .map(|value| value.as_str().to_string())
                .unwrap_or_else(|| "none".to_string()),
            exact_counter_values,
            counter_snapshot_digest: digest_parts(&[
                bundle.counter_snapshot_digest().to_string(),
                format!(
                    "execution_count:{}",
                    execution.counters().context_execution_count()
                ),
                format!(
                    "executor_rediscovery:{}",
                    execution.counters().executor_rediscovery_count()
                ),
            ]),
        }
    }

    pub fn from_diff_result_bundle(
        bundle: &QueryDiffResultBundle,
        left_execution_count: usize,
        right_execution_count: usize,
        executor_rediscovery_count: usize,
    ) -> Self {
        let metadata = bundle.metadata();
        let change_set = bundle.change_set();
        let exact_counter_values = diff_counter_values(
            bundle,
            left_execution_count,
            right_execution_count,
            executor_rediscovery_count,
        );

        Self {
            query_digest: metadata.query_digest().to_string(),
            basis_digest: metadata.left_basis_digest().to_string(),
            comparison_basis_digest: metadata.right_basis_digest().to_string(),
            result_digest: metadata.comparison_result_digest().to_string(),
            result_shape_digest: metadata.result_shape_digest().to_string(),
            result_shape_width: change_set.rows().len(),
            replay_digest: bundle.replay_digest().to_string(),
            basis_family: bundle.context().left().family().as_str().to_string(),
            comparison_basis_family: metadata.comparison_basis_family().as_str().to_string(),
            cost_class: metadata.cost_class().as_str().to_string(),
            budget_class: metadata.budget_class().as_str().to_string(),
            historical_admission_class: bundle
                .context()
                .right()
                .historical_admission_class()
                .map(|value| value.as_str().to_string())
                .unwrap_or_else(|| "none".to_string()),
            materialization_path_identity: bundle
                .context()
                .right()
                .materialization_path_identity_source()
                .unwrap_or("none")
                .to_string(),
            preview_provenance_identity: bundle
                .context()
                .left()
                .preview_provenance_identity_source()
                .or(bundle
                    .context()
                    .right()
                    .preview_provenance_identity_source())
                .unwrap_or("none")
                .to_string(),
            prediction_drift_outcome: metadata.prediction_drift_outcome().as_str().to_string(),
            exact_counter_values,
            counter_snapshot_digest: digest_parts(&[
                bundle.counter_snapshot_digest().to_string(),
                format!("change_rows:{}", change_set.rows().len()),
            ]),
        }
    }
}

fn counter_values(counters: &QueryContextCounters) -> Vec<String> {
    vec![
        format!(
            "query_basis_bindings:{}",
            counters.query_basis_binding_count()
        ),
        format!(
            "historical_basis_lookups:{}",
            counters.historical_basis_lookup_count()
        ),
        format!(
            "comparison_basis_lookups:{}",
            counters.comparison_basis_lookup_count()
        ),
        format!(
            "materialization_path_compatibility_checks:{}",
            counters.materialization_path_compatibility_check_count()
        ),
        format!("basis_binding_width:{}", counters.basis_binding_width()),
        format!(
            "historical_lookup_width:{}",
            counters.historical_lookup_width()
        ),
        format!("comparison_binding_width:0"),
        format!(
            "comparison_scope_width:{}",
            counters.comparison_scope_width()
        ),
        format!("diff_input_breadth:{}", counters.diff_input_breadth()),
        format!(
            "diff_change_set_row_width:{}",
            counters.diff_change_set_row_width()
        ),
        format!("denial_width:{}", counters.denial_width()),
        format!(
            "unsupported_denials:{}",
            counters.unsupported_basis_denial_count()
        ),
        format!(
            "basis_substitution_denials:{}",
            counters.basis_substitution_denial_count()
        ),
        format!(
            "comparison_broadening_denials:{}",
            counters.comparison_broadening_denial_count()
        ),
        format!(
            "historical_broadening_denials:{}",
            counters.historical_broadening_denial_count()
        ),
        format!(
            "predicted_comparison_width:{}",
            counters.comparison_row_width()
        ),
        format!("realized_comparison_width:0"),
        format!("metadata_attachment_width:0"),
        format!("query_context_execution_count:0"),
        format!("query_context_metadata_attachment_count:0"),
        format!("query_context_executor_rediscovery:0"),
        format!("basis_rediscovery:{}", counters.basis_rediscovery_count()),
        format!(
            "historical_path_rediscovery:{}",
            counters.historical_path_rediscovery_count()
        ),
        format!(
            "comparison_family_rediscovery:{}",
            counters.comparison_family_rediscovery_count()
        ),
    ]
}

fn basis_counter_values(bundle: &QueryBasisResultBundle) -> Vec<String> {
    let context = bundle.context();
    let execution = bundle.execution();
    let metadata: &QueryBasisMetadata = bundle.metadata();
    let prediction = metadata.prediction_report();

    vec![
        format!(
            "query_basis_bindings:{}",
            context.counters().query_basis_binding_count()
        ),
        format!(
            "historical_basis_lookups:{}",
            context.counters().historical_basis_lookup_count()
        ),
        format!(
            "comparison_basis_lookups:{}",
            context.counters().comparison_basis_lookup_count()
        ),
        format!(
            "materialization_path_compatibility_checks:{}",
            context
                .counters()
                .materialization_path_compatibility_check_count()
        ),
        format!(
            "basis_binding_width:{}",
            context.counters().basis_binding_width()
        ),
        format!(
            "historical_lookup_width:{}",
            context.counters().historical_lookup_width()
        ),
        format!(
            "comparison_binding_width:{}",
            prediction
                .map(|value| value.comparison_binding_width())
                .unwrap_or(0)
        ),
        format!(
            "comparison_scope_width:{}",
            context.counters().comparison_scope_width()
        ),
        format!(
            "diff_input_breadth:{}",
            context.counters().diff_input_breadth()
        ),
        format!(
            "diff_change_set_row_width:{}",
            context.counters().diff_change_set_row_width()
        ),
        format!("denial_width:{}", context.counters().denial_width()),
        format!(
            "unsupported_denials:{}",
            context.counters().unsupported_basis_denial_count()
        ),
        format!(
            "basis_substitution_denials:{}",
            context.counters().basis_substitution_denial_count()
        ),
        format!(
            "comparison_broadening_denials:{}",
            context.counters().comparison_broadening_denial_count()
        ),
        format!(
            "historical_broadening_denials:{}",
            context.counters().historical_broadening_denial_count()
        ),
        format!(
            "predicted_comparison_width:{}",
            prediction
                .map(|value| value.comparison_row_width())
                .unwrap_or(0)
        ),
        "realized_comparison_width:0".to_string(),
        "metadata_attachment_width:1".to_string(),
        format!(
            "query_context_execution_count:{}",
            execution.counters().context_execution_count()
        ),
        "query_context_metadata_attachment_count:1".to_string(),
        format!(
            "query_context_executor_rediscovery:{}",
            execution.counters().executor_rediscovery_count()
        ),
        format!(
            "basis_rediscovery:{}",
            context.counters().basis_rediscovery_count()
        ),
        format!(
            "historical_path_rediscovery:{}",
            context.counters().historical_path_rediscovery_count()
        ),
        format!(
            "comparison_family_rediscovery:{}",
            context.counters().comparison_family_rediscovery_count()
        ),
    ]
}

fn diff_counter_values(
    bundle: &QueryDiffResultBundle,
    left_execution_count: usize,
    right_execution_count: usize,
    executor_rediscovery_count: usize,
) -> Vec<String> {
    let context = bundle.context();
    let metadata: &DiffQueryMetadata = bundle.metadata();

    vec![
        format!(
            "query_basis_bindings:{}",
            context.left().counters().query_basis_binding_count()
                + context.right().counters().query_basis_binding_count()
        ),
        format!(
            "historical_basis_lookups:{}",
            context.left().counters().historical_basis_lookup_count()
                + context.right().counters().historical_basis_lookup_count()
        ),
        format!(
            "comparison_basis_lookups:{}",
            context.counters().comparison_basis_lookup_count()
        ),
        format!(
            "materialization_path_compatibility_checks:{}",
            context
                .left()
                .counters()
                .materialization_path_compatibility_check_count()
                + context
                    .right()
                    .counters()
                    .materialization_path_compatibility_check_count()
        ),
        format!(
            "basis_binding_width:{}",
            context.left().counters().basis_binding_width()
                + context.right().counters().basis_binding_width()
        ),
        format!(
            "historical_lookup_width:{}",
            context.left().counters().historical_lookup_width()
                + context.right().counters().historical_lookup_width()
        ),
        format!(
            "comparison_binding_width:{}",
            metadata.prediction_report().comparison_binding_width()
        ),
        format!(
            "comparison_scope_width:{}",
            context.counters().comparison_scope_width()
        ),
        format!(
            "diff_input_breadth:{}",
            context.counters().diff_input_breadth()
        ),
        format!(
            "diff_change_set_row_width:{}",
            bundle.change_set().rows().len()
        ),
        format!("denial_width:{}", context.counters().denial_width()),
        format!(
            "unsupported_denials:{}",
            context.counters().unsupported_basis_denial_count()
        ),
        format!(
            "basis_substitution_denials:{}",
            context.counters().basis_substitution_denial_count()
        ),
        format!(
            "comparison_broadening_denials:{}",
            context.counters().comparison_broadening_denial_count()
        ),
        format!(
            "historical_broadening_denials:{}",
            context.counters().historical_broadening_denial_count()
        ),
        format!(
            "predicted_comparison_width:{}",
            metadata.prediction_report().comparison_row_width()
        ),
        format!(
            "realized_comparison_width:{}",
            bundle.change_set().rows().len()
        ),
        "metadata_attachment_width:1".to_string(),
        format!(
            "query_context_execution_count:{}",
            left_execution_count + right_execution_count
        ),
        "query_context_metadata_attachment_count:1".to_string(),
        format!(
            "query_context_executor_rediscovery:{}",
            executor_rediscovery_count
        ),
        format!(
            "basis_rediscovery:{}",
            context.counters().basis_rediscovery_count()
        ),
        format!(
            "historical_path_rediscovery:{}",
            context.counters().historical_path_rediscovery_count()
        ),
        format!(
            "comparison_family_rediscovery:{}",
            context.counters().comparison_family_rediscovery_count()
        ),
    ]
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalDiffRejection {
    pub failure_class: HistoricalDiffFailureClass,
    pub failure_digest: String,
    pub exact_counter_values: Vec<String>,
    pub counter_snapshot_digest: String,
}

impl HistoricalDiffRejection {
    pub fn from_error(error: &crate::query_context::QueryContextAdmissionError) -> Self {
        Self {
            failure_class: match error.failure_class() {
                QueryContextAdmissionFailureClass::UnsupportedHistoricalBasis => {
                    HistoricalDiffFailureClass::UnsupportedHistoricalBasis
                }
                QueryContextAdmissionFailureClass::DiffScopeMismatch => {
                    HistoricalDiffFailureClass::DiffScopeMismatch
                }
                QueryContextAdmissionFailureClass::StoreBackedHistoricalDeferred => {
                    HistoricalDiffFailureClass::StoreBackedHistoricalDeferred
                }
                QueryContextAdmissionFailureClass::BasisSubstitutionForbidden => {
                    HistoricalDiffFailureClass::BasisSubstitutionForbidden
                }
                QueryContextAdmissionFailureClass::BroadComparisonForbidden => {
                    HistoricalDiffFailureClass::BroadComparisonForbidden
                }
                QueryContextAdmissionFailureClass::AmbiguousComparisonBasis => {
                    HistoricalDiffFailureClass::AmbiguousComparisonBasis
                }
                QueryContextAdmissionFailureClass::ComparisonShapeMismatch => {
                    HistoricalDiffFailureClass::ComparisonShapeMismatch
                }
                QueryContextAdmissionFailureClass::ComparisonBroadeningRequired => {
                    HistoricalDiffFailureClass::ComparisonBroadeningRequired
                }
                QueryContextAdmissionFailureClass::HistoricalPathTooBroadDenied => {
                    HistoricalDiffFailureClass::HistoricalPathTooBroadDenied
                }
                QueryContextAdmissionFailureClass::RawStorageDeltaLeakageForbidden => {
                    HistoricalDiffFailureClass::RawStorageDeltaLeakageForbidden
                }
                QueryContextAdmissionFailureClass::NonQueryOwnedHistoricalArtifact => {
                    HistoricalDiffFailureClass::UnsupportedHistoricalBasis
                }
                other => panic!("unexpected historical diff failure class {other:?}"),
            },
            failure_digest: digest_parts(&[
                format!("failure_class:{:?}", error.failure_class()),
                format!("message:{}", error.message()),
                digest_parts(&counter_values(error.counters())),
            ]),
            exact_counter_values: counter_values(error.counters()),
            counter_snapshot_digest: digest_parts(&counter_values(error.counters())),
        }
    }
}

pub type HistoricalDiffCertificationMatrix = CertificationMatrix<
    HistoricalDiffPerturbationClass,
    HistoricalDiffLane,
    HistoricalDiffRejection,
>;
