use super::{HistoricalDiffFailureClass, HistoricalDiffPerturbationClass};
use crate::harness::certification::HostileExpectation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HistoricalDiffCanonicalRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: HistoricalDiffPerturbationClass,
    pub hostile_expectation: HostileExpectation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HistoricalDiffRejectionRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: HistoricalDiffPerturbationClass,
    pub failure_class: HistoricalDiffFailureClass,
}

pub const HISTORICAL_DIFF_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "current-vs-branch-basis-explicitness",
    "current-vs-historical-basis-explicitness",
    "historical-materialization-path-explicitness",
    "diff-comparison-family-explicitness",
    "branch-to-branch-diff-shaped",
    "current-to-historical-diff-shaped",
    "result-shape-parity-across-basis-variants",
    "preview-derived-historical-basis-explicitness",
    "admitted-diff-cost-class-explicitness",
    "prediction-versus-realization-explicitness",
];

pub const HISTORICAL_DIFF_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "unsupported-historical-basis",
    "ambiguous-comparison-basis",
    "diff-scope-mismatch",
    "store-backed-historical-deferred-debt",
    "forbidden-basis-substitution",
    "raw-storage-delta-leakage-forbidden",
    "broadening-required-comparison-denial",
    "declared-result-shape-mismatch",
];

pub const HISTORICAL_DIFF_CANONICAL_ROW_SPECS: &[HistoricalDiffCanonicalRowSpec] = &[
    HistoricalDiffCanonicalRowSpec {
        row_name: "current-vs-branch-basis-explicitness",
        perturbation_class: HistoricalDiffPerturbationClass::RuntimeBasis,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "current-vs-historical-basis-explicitness",
        perturbation_class: HistoricalDiffPerturbationClass::HistoricalBasis,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "historical-materialization-path-explicitness",
        perturbation_class: HistoricalDiffPerturbationClass::MetadataShaping,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "diff-comparison-family-explicitness",
        perturbation_class: HistoricalDiffPerturbationClass::ComparisonFamily,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "branch-to-branch-diff-shaped",
        perturbation_class: HistoricalDiffPerturbationClass::ComparisonFamily,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "current-to-historical-diff-shaped",
        perturbation_class: HistoricalDiffPerturbationClass::ComparisonFamily,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "result-shape-parity-across-basis-variants",
        perturbation_class: HistoricalDiffPerturbationClass::MetadataShaping,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "preview-derived-historical-basis-explicitness",
        perturbation_class: HistoricalDiffPerturbationClass::PreviewDerivedBasis,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "admitted-diff-cost-class-explicitness",
        perturbation_class: HistoricalDiffPerturbationClass::ComparisonFamily,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    HistoricalDiffCanonicalRowSpec {
        row_name: "prediction-versus-realization-explicitness",
        perturbation_class: HistoricalDiffPerturbationClass::ComparisonFamily,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
];

pub const HISTORICAL_DIFF_REJECTION_ROW_SPECS: &[HistoricalDiffRejectionRowSpec] = &[
    HistoricalDiffRejectionRowSpec {
        row_name: "unsupported-historical-basis",
        perturbation_class: HistoricalDiffPerturbationClass::DeferredHistorical,
        failure_class: HistoricalDiffFailureClass::UnsupportedHistoricalBasis,
    },
    HistoricalDiffRejectionRowSpec {
        row_name: "ambiguous-comparison-basis",
        perturbation_class: HistoricalDiffPerturbationClass::BroadDiffDenied,
        failure_class: HistoricalDiffFailureClass::AmbiguousComparisonBasis,
    },
    HistoricalDiffRejectionRowSpec {
        row_name: "diff-scope-mismatch",
        perturbation_class: HistoricalDiffPerturbationClass::BasisSubstitution,
        failure_class: HistoricalDiffFailureClass::DiffScopeMismatch,
    },
    HistoricalDiffRejectionRowSpec {
        row_name: "store-backed-historical-deferred-debt",
        perturbation_class: HistoricalDiffPerturbationClass::DeferredHistorical,
        failure_class: HistoricalDiffFailureClass::StoreBackedHistoricalDeferred,
    },
    HistoricalDiffRejectionRowSpec {
        row_name: "forbidden-basis-substitution",
        perturbation_class: HistoricalDiffPerturbationClass::BasisSubstitution,
        failure_class: HistoricalDiffFailureClass::BasisSubstitutionForbidden,
    },
    HistoricalDiffRejectionRowSpec {
        row_name: "raw-storage-delta-leakage-forbidden",
        perturbation_class: HistoricalDiffPerturbationClass::BroadDiffDenied,
        failure_class: HistoricalDiffFailureClass::RawStorageDeltaLeakageForbidden,
    },
    HistoricalDiffRejectionRowSpec {
        row_name: "broadening-required-comparison-denial",
        perturbation_class: HistoricalDiffPerturbationClass::BroadDiffDenied,
        failure_class: HistoricalDiffFailureClass::ComparisonBroadeningRequired,
    },
    HistoricalDiffRejectionRowSpec {
        row_name: "declared-result-shape-mismatch",
        perturbation_class: HistoricalDiffPerturbationClass::BroadDiffDenied,
        failure_class: HistoricalDiffFailureClass::ComparisonShapeMismatch,
    },
];
