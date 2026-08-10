mod counter_values;

use counter_values::{basis_counter_values, counter_values, diff_counter_values};

use crate::harness::certification::{digest_parts, CertificationMatrix};
use crate::query_context::{
    QueryBasisResultBundle, QueryContextAdmissionFailureClass, QueryDiffResultBundle,
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
    pub basis_authority_family: String,
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
            basis_authority_family: metadata.basis_authority_family().as_str().to_string(),
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
            basis_authority_family: bundle
                .context()
                .left()
                .basis_authority_family()
                .as_str()
                .to_string(),
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
