use crate::harness::certification::HostileExpectation;

use super::{CorrespondenceHistoryFailureClass, CorrespondenceHistoryPerturbationClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoryCanonicalRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: CorrespondenceHistoryPerturbationClass,
    pub hostile_expectation: HostileExpectation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CorrespondenceHistoryRejectionRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: CorrespondenceHistoryPerturbationClass,
    pub failure_class: CorrespondenceHistoryFailureClass,
    pub compile_fail_case: Option<&'static str>,
}

pub const CORRESPONDENCE_HISTORY_CANONICAL_ROW_SPECS: &[CorrespondenceHistoryCanonicalRowSpec] = &[
    CorrespondenceHistoryCanonicalRowSpec {
        row_name: "lineage-correspondence-authoritative",
        perturbation_class: CorrespondenceHistoryPerturbationClass::LineageAuthoritativeParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    CorrespondenceHistoryCanonicalRowSpec {
        row_name: "structural-correspondence-advisory",
        perturbation_class: CorrespondenceHistoryPerturbationClass::StructuralAdvisoryBoundary,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    CorrespondenceHistoryCanonicalRowSpec {
        row_name: "lineage-structural-disagreement-explicit",
        perturbation_class: CorrespondenceHistoryPerturbationClass::LineageStructuralDisagreement,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    CorrespondenceHistoryCanonicalRowSpec {
        row_name: "structural-ambiguity-explicit",
        perturbation_class: CorrespondenceHistoryPerturbationClass::StructuralAmbiguityBoundary,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    CorrespondenceHistoryCanonicalRowSpec {
        row_name: "historical-retained-snapshot-path",
        perturbation_class: CorrespondenceHistoryPerturbationClass::HistoricalRetainedPathParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    CorrespondenceHistoryCanonicalRowSpec {
        row_name: "historical-delta-replay-path",
        perturbation_class: CorrespondenceHistoryPerturbationClass::HistoricalReplayPathParity,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    CorrespondenceHistoryCanonicalRowSpec {
        row_name: "historical-full-reconstruction-path",
        perturbation_class:
            CorrespondenceHistoryPerturbationClass::HistoricalReconstructionPathParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    CorrespondenceHistoryCanonicalRowSpec {
        row_name: "prediction-drift-explicit",
        perturbation_class: CorrespondenceHistoryPerturbationClass::PredictionDriftExplicitness,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
];

pub const CORRESPONDENCE_HISTORY_REJECTION_ROW_SPECS: &[CorrespondenceHistoryRejectionRowSpec] = &[
    CorrespondenceHistoryRejectionRowSpec {
        row_name: "structural-as-authoritative-forbidden",
        perturbation_class:
            CorrespondenceHistoryPerturbationClass::StructuralAuthorityPromotionForbidden,
        failure_class: CorrespondenceHistoryFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/advisory_structural_unique_is_not_lineage_continuity.rs"),
    },
    CorrespondenceHistoryRejectionRowSpec {
        row_name: "ambiguous-correspondence-not-collapsed",
        perturbation_class: CorrespondenceHistoryPerturbationClass::AmbiguityCollapseForbidden,
        failure_class: CorrespondenceHistoryFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/naked_best_match_accessor_forbidden.rs"),
    },
    CorrespondenceHistoryRejectionRowSpec {
        row_name: "unsupported-correspondence-family",
        perturbation_class: CorrespondenceHistoryPerturbationClass::UnsupportedCorrespondenceFamily,
        failure_class: CorrespondenceHistoryFailureClass::CorrespondenceDenied,
        compile_fail_case: None,
    },
    CorrespondenceHistoryRejectionRowSpec {
        row_name: "unsupported-historical-materialization-path",
        perturbation_class:
            CorrespondenceHistoryPerturbationClass::UnsupportedHistoricalMaterializationPath,
        failure_class: CorrespondenceHistoryFailureClass::HistoricalPathDenied,
        compile_fail_case: None,
    },
    CorrespondenceHistoryRejectionRowSpec {
        row_name: "hidden-materialization-path-substitution-forbidden",
        perturbation_class:
            CorrespondenceHistoryPerturbationClass::HiddenMaterializationSubstitutionForbidden,
        failure_class: CorrespondenceHistoryFailureClass::HistoricalPathDenied,
        compile_fail_case: None,
    },
    CorrespondenceHistoryRejectionRowSpec {
        row_name: "broad-candidate-scan-success-forbidden",
        perturbation_class: CorrespondenceHistoryPerturbationClass::BroadCandidateScanForbidden,
        failure_class: CorrespondenceHistoryFailureClass::CorrespondenceDenied,
        compile_fail_case: None,
    },
    CorrespondenceHistoryRejectionRowSpec {
        row_name: "no-executor-path-mutation-after-planning",
        perturbation_class: CorrespondenceHistoryPerturbationClass::ExecutorPathMutationForbidden,
        failure_class: CorrespondenceHistoryFailureClass::HistoricalPathDenied,
        compile_fail_case: None,
    },
    CorrespondenceHistoryRejectionRowSpec {
        row_name: "host-cache-history-authority-forbidden",
        perturbation_class: CorrespondenceHistoryPerturbationClass::HostCacheHistoryAuthorityForbidden,
        failure_class: CorrespondenceHistoryFailureClass::HistoricalPathDenied,
        compile_fail_case: None,
    },
];

pub const CORRESPONDENCE_HISTORY_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "lineage-correspondence-authoritative",
    "structural-correspondence-advisory",
    "lineage-structural-disagreement-explicit",
    "structural-ambiguity-explicit",
    "historical-retained-snapshot-path",
    "historical-delta-replay-path",
    "historical-full-reconstruction-path",
    "prediction-drift-explicit",
];

pub const CORRESPONDENCE_HISTORY_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "structural-as-authoritative-forbidden",
    "ambiguous-correspondence-not-collapsed",
    "unsupported-correspondence-family",
    "unsupported-historical-materialization-path",
    "hidden-materialization-path-substitution-forbidden",
    "broad-candidate-scan-success-forbidden",
    "no-executor-path-mutation-after-planning",
    "host-cache-history-authority-forbidden",
];
