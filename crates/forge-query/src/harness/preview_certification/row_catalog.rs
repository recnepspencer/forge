use crate::harness::certification::HostileExpectation;

use super::{PreviewFailureClass, PreviewLaneEvaluationClass, PreviewPerturbationClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewLaneSelector {
    Parity,
    PromotionEligible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewRuntimeFailureSelector {
    InvalidBasis,
    StaleLifecycle,
    PreviewLiveDenied,
    PromotionLinkageDenied,
    ReplayLinkageDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreviewCanonicalRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: PreviewPerturbationClass,
    pub hostile_expectation: HostileExpectation,
    pub hostile_lane_selector: PreviewLaneSelector,
    pub hostile_evaluation_class: Option<PreviewLaneEvaluationClass>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreviewRejectionRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: PreviewPerturbationClass,
    pub failure_class: PreviewFailureClass,
    pub runtime_failure_selector: Option<PreviewRuntimeFailureSelector>,
    pub compile_fail_case: Option<&'static str>,
}

pub const PREVIEW_CANONICAL_ROW_SPECS: &[PreviewCanonicalRowSpec] = &[
    PreviewCanonicalRowSpec {
        row_name: "preview-basis-binding-active",
        perturbation_class: PreviewPerturbationClass::ActiveBinding,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::Parity,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::ReadOnly),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-lifecycle-explicitness",
        perturbation_class: PreviewPerturbationClass::LifecycleExplicitness,
        hostile_expectation: HostileExpectation::DistinctFromControl,
        hostile_lane_selector: PreviewLaneSelector::PromotionEligible,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-lifecycle-no-rediscovery",
        perturbation_class: PreviewPerturbationClass::NoRediscovery,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::Parity,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::ReadOnly),
    },
];

pub const PREVIEW_REJECTION_ROW_SPECS: &[PreviewRejectionRowSpec] = &[
    PreviewRejectionRowSpec {
        row_name: "invalid-preview-basis",
        perturbation_class: PreviewPerturbationClass::InvalidBasis,
        failure_class: PreviewFailureClass::InvalidPreviewBasis,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::InvalidBasis),
        compile_fail_case: None,
    },
    PreviewRejectionRowSpec {
        row_name: "stale-preview-lifecycle-denied",
        perturbation_class: PreviewPerturbationClass::StaleLifecycle,
        failure_class: PreviewFailureClass::StaleOrInactivePreviewLifecycle,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::StaleLifecycle),
        compile_fail_case: None,
    },
    PreviewRejectionRowSpec {
        row_name: "raw-branch-alias-preview-forbidden",
        perturbation_class: PreviewPerturbationClass::RawBranchAliasForbidden,
        failure_class: PreviewFailureClass::CompileFail,
        runtime_failure_selector: None,
        compile_fail_case: Some("tests/ui/raw_branch_alias_preview_forbidden.rs"),
    },
    PreviewRejectionRowSpec {
        row_name: "preview-live-denied-phase-2",
        perturbation_class: PreviewPerturbationClass::PreviewLiveDenied,
        failure_class: PreviewFailureClass::PreviewLiveDeniedInPhaseTwo,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::PreviewLiveDenied),
        compile_fail_case: None,
    },
    PreviewRejectionRowSpec {
        row_name: "preview-promotion-linkage-denied",
        perturbation_class: PreviewPerturbationClass::PromotionLinkageDenied,
        failure_class: PreviewFailureClass::PromotionLinkageMismatch,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::PromotionLinkageDenied),
        compile_fail_case: None,
    },
    PreviewRejectionRowSpec {
        row_name: "preview-replay-linkage-denied",
        perturbation_class: PreviewPerturbationClass::ReplayLinkageDenied,
        failure_class: PreviewFailureClass::PromotionLinkageMismatch,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::ReplayLinkageDenied),
        compile_fail_case: None,
    },
    PreviewRejectionRowSpec {
        row_name: "promotion-eligibility-bool-forbidden",
        perturbation_class: PreviewPerturbationClass::PromotionEligibilityBoolForbidden,
        failure_class: PreviewFailureClass::CompileFail,
        runtime_failure_selector: None,
        compile_fail_case: Some("tests/ui/promotion_eligibility_bool_forbidden.rs"),
    },
];

pub const PREVIEW_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "preview-basis-binding-active",
    "preview-lifecycle-explicitness",
    "preview-lifecycle-no-rediscovery",
];

pub const PREVIEW_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "invalid-preview-basis",
    "stale-preview-lifecycle-denied",
    "raw-branch-alias-preview-forbidden",
    "preview-live-denied-phase-2",
    "preview-promotion-linkage-denied",
    "preview-replay-linkage-denied",
    "promotion-eligibility-bool-forbidden",
];

pub const PREVIEW_MINIMUM_SPEC_CANONICAL_ROW_NAMES: &[&str] = &[
    "preview-basis-execution-parity",
    "preview-lifecycle-explicitness",
    "preview-promotion-comparison-parity",
    "preview-lifecycle-no-rediscovery",
    "preview-comparison-shape-proof-width",
    "preview-workflow-foundation-admission",
    "preview-workflow-foundation-no-rescan",
    "preview-work-avoided-counter-parity",
];

pub const PREVIEW_MINIMUM_SPEC_REJECTION_ROW_NAMES: &[&str] = &[
    "unsupported-preview-family",
    "invalid-preview-basis",
    "stale-preview-lifecycle-denied",
    "unsupported-preview-promotion-comparison",
    "promotion-eligibility-bool-forbidden",
    "preview-shape-mismatch-denied",
    "preview-broad-fallback-forbidden",
    "preview-diagnostics-rescan-forbidden",
    "raw-branch-alias-preview-forbidden",
    "fabricated-preview-lifecycle-forbidden",
    "out-of-scope-workflow-foundation-request",
];
