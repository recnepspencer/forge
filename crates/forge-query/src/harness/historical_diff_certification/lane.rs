use crate::harness::certification::{digest_parts, CertificationMatrix};
use crate::query_context::{
    QueryBasisMetadata, QueryContextAdmissionFailureClass, QueryContextCounters,
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
    RawStorageDeltaLeakageForbidden,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalDiffLane {
    pub query_digest: String,
    pub basis_digest: String,
    pub result_digest: String,
    pub replay_digest: String,
    pub basis_family: String,
    pub cost_class: String,
    pub budget_class: String,
    pub historical_admission_class: String,
    pub comparison_family: String,
    pub prediction_drift_outcome: String,
    pub exact_counter_values: Vec<String>,
    pub counter_snapshot_digest: String,
}

impl HistoricalDiffLane {
    pub fn from_basis_metadata(
        metadata: &QueryBasisMetadata,
        counters: &QueryContextCounters,
    ) -> Self {
        Self {
            query_digest: metadata.query_digest().to_string(),
            basis_digest: metadata.basis_digest().to_string(),
            result_digest: metadata.result_digest().to_string(),
            replay_digest: digest_parts(&[
                format!("query:{}", metadata.query_digest()),
                format!("basis:{}", metadata.basis_digest()),
                format!("result:{}", metadata.result_digest()),
                format!("drift:{}", metadata.drift_outcome().as_str()),
            ]),
            basis_family: metadata.basis_family().as_str().to_string(),
            cost_class: metadata.cost_class().as_str().to_string(),
            budget_class: metadata.budget_class().as_str().to_string(),
            historical_admission_class: metadata
                .historical_admission_class()
                .map(|value| value.as_str().to_string())
                .unwrap_or_else(|| "none".to_string()),
            comparison_family: "none".to_string(),
            prediction_drift_outcome: metadata
                .prediction_drift_outcome()
                .map(|value| value.as_str().to_string())
                .unwrap_or_else(|| "none".to_string()),
            exact_counter_values: counter_values(counters),
            counter_snapshot_digest: digest_parts(&counter_values(counters)),
        }
    }
}

fn counter_values(counters: &QueryContextCounters) -> Vec<String> {
    vec![
        format!("bindings:{}", counters.query_basis_binding_count()),
        format!(
            "historical_lookups:{}",
            counters.historical_basis_lookup_count()
        ),
        format!("binding_width:{}", counters.basis_binding_width()),
        format!("historical_width:{}", counters.historical_lookup_width()),
        format!("denial_width:{}", counters.denial_width()),
        format!(
            "comparison_lookups:{}",
            counters.comparison_basis_lookup_count()
        ),
        format!("scope_width:{}", counters.comparison_scope_width()),
        format!("row_width:{}", counters.comparison_row_width()),
        format!("diff_breadth:{}", counters.diff_input_breadth()),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalDiffRejection {
    pub failure_class: HistoricalDiffFailureClass,
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
                QueryContextAdmissionFailureClass::RawStorageDeltaLeakageForbidden => {
                    HistoricalDiffFailureClass::RawStorageDeltaLeakageForbidden
                }
                QueryContextAdmissionFailureClass::NonQueryOwnedHistoricalArtifact => {
                    HistoricalDiffFailureClass::UnsupportedHistoricalBasis
                }
                other => panic!("unexpected historical diff failure class {other:?}"),
            },
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
