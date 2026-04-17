use crate::harness::certification::HostileExpectation;

use super::{PreviewFailureClass, PreviewLaneEvaluationClass, PreviewPerturbationClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewLaneSelector {
    ParityExecution,
    PromotionEligibleExecution,
    PromotionParity,
    PreviewLiveAdmission,
    PreviewLiveRebind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewRuntimeFailureSelector {
    UnsupportedPreviewFamily,
    InvalidBasis,
    BroadFallbackDenied,
    StaleLifecycle,
    DiscardedLifecycle,
    PreviewLiveDriftDenied,
    PreviewLiveBroadFallbackDenied,
    WorkflowFoundationScopeOverreach,
    PromotionLinkageDenied,
    ReplayLinkageDenied,
    ShapeMismatchDenied,
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
        row_name: "preview-basis-execution-parity",
        perturbation_class: PreviewPerturbationClass::ActiveBinding,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::ParityExecution,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::ReadOnly),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-lifecycle-explicitness",
        perturbation_class: PreviewPerturbationClass::LifecycleExplicitness,
        hostile_expectation: HostileExpectation::DistinctFromControl,
        hostile_lane_selector: PreviewLaneSelector::PromotionEligibleExecution,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-promotion-comparison-parity",
        perturbation_class: PreviewPerturbationClass::ActiveBinding,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::PromotionParity,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-lifecycle-no-rediscovery",
        perturbation_class: PreviewPerturbationClass::NoRediscovery,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::ParityExecution,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::ReadOnly),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-live-admission-parity",
        perturbation_class: PreviewPerturbationClass::PreviewLiveAdmission,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::PreviewLiveAdmission,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-live-drift-explicitness",
        perturbation_class: PreviewPerturbationClass::PreviewLiveDrift,
        hostile_expectation: HostileExpectation::DistinctFromControl,
        hostile_lane_selector: PreviewLaneSelector::PreviewLiveRebind,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-comparison-shape-proof-width",
        perturbation_class: PreviewPerturbationClass::NoRediscovery,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::PromotionParity,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-workflow-foundation-admission",
        perturbation_class: PreviewPerturbationClass::ActiveBinding,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::PromotionEligibleExecution,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-workflow-foundation-no-rescan",
        perturbation_class: PreviewPerturbationClass::NoRediscovery,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::PromotionEligibleExecution,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
    PreviewCanonicalRowSpec {
        row_name: "preview-work-avoided-counter-parity",
        perturbation_class: PreviewPerturbationClass::NoRediscovery,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        hostile_lane_selector: PreviewLaneSelector::PromotionEligibleExecution,
        hostile_evaluation_class: Some(PreviewLaneEvaluationClass::PromotionEligible),
    },
];

pub const PREVIEW_REJECTION_ROW_SPECS: &[PreviewRejectionRowSpec] = &[
    PreviewRejectionRowSpec {
        row_name: "unsupported-preview-family",
        perturbation_class: PreviewPerturbationClass::InvalidBasis,
        failure_class: PreviewFailureClass::UnsupportedPreviewFamily,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::UnsupportedPreviewFamily),
        compile_fail_case: None,
    },
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
        row_name: "discarded-preview-execution-denied",
        perturbation_class: PreviewPerturbationClass::StaleLifecycle,
        failure_class: PreviewFailureClass::StaleOrInactivePreviewLifecycle,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::DiscardedLifecycle),
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
        row_name: "unsupported-preview-promotion-comparison",
        perturbation_class: PreviewPerturbationClass::PromotionEligibilityBoolForbidden,
        failure_class: PreviewFailureClass::CompileFail,
        runtime_failure_selector: None,
        compile_fail_case: Some(
            "tests/ui/promotion_parity_comparison_requires_promotion_execution.rs",
        ),
    },
    PreviewRejectionRowSpec {
        row_name: "read-only-preview-denies-promotion-comparison",
        perturbation_class: PreviewPerturbationClass::PromotionEligibilityBoolForbidden,
        failure_class: PreviewFailureClass::CompileFail,
        runtime_failure_selector: None,
        compile_fail_case: Some("tests/ui/read_only_preview_denies_promotion_comparison.rs"),
    },
    PreviewRejectionRowSpec {
        row_name: "preview-live-drift-denied",
        perturbation_class: PreviewPerturbationClass::PreviewLiveDrift,
        failure_class: PreviewFailureClass::PreviewLiveDriftDenied,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::PreviewLiveDriftDenied),
        compile_fail_case: None,
    },
    PreviewRejectionRowSpec {
        row_name: "preview-live-broad-fallback-forbidden",
        perturbation_class: PreviewPerturbationClass::PreviewLiveDrift,
        failure_class: PreviewFailureClass::PreviewLiveBroadFallbackForbidden,
        runtime_failure_selector: Some(
            PreviewRuntimeFailureSelector::PreviewLiveBroadFallbackDenied,
        ),
        compile_fail_case: None,
    },
    PreviewRejectionRowSpec {
        row_name: "preview-broad-fallback-forbidden",
        perturbation_class: PreviewPerturbationClass::InvalidBasis,
        failure_class: PreviewFailureClass::InvalidPreviewBasis,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::BroadFallbackDenied),
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
        row_name: "preview-shape-mismatch-denied",
        perturbation_class: PreviewPerturbationClass::NoRediscovery,
        failure_class: PreviewFailureClass::PreviewShapeMismatchDenied,
        runtime_failure_selector: Some(PreviewRuntimeFailureSelector::ShapeMismatchDenied),
        compile_fail_case: None,
    },
    PreviewRejectionRowSpec {
        row_name: "promotion-eligibility-bool-forbidden",
        perturbation_class: PreviewPerturbationClass::PromotionEligibilityBoolForbidden,
        failure_class: PreviewFailureClass::CompileFail,
        runtime_failure_selector: None,
        compile_fail_case: Some("tests/ui/promotion_eligibility_bool_forbidden.rs"),
    },
    PreviewRejectionRowSpec {
        row_name: "preview-diagnostics-rescan-forbidden",
        perturbation_class: PreviewPerturbationClass::PromotionEligibilityBoolForbidden,
        failure_class: PreviewFailureClass::CompileFail,
        runtime_failure_selector: None,
        compile_fail_case: Some("tests/ui/preview_diagnostics_rescan_forbidden.rs"),
    },
    PreviewRejectionRowSpec {
        row_name: "fabricated-preview-lifecycle-forbidden",
        perturbation_class: PreviewPerturbationClass::PromotionEligibilityBoolForbidden,
        failure_class: PreviewFailureClass::CompileFail,
        runtime_failure_selector: None,
        compile_fail_case: Some("tests/ui/fabricated_preview_lifecycle_forbidden.rs"),
    },
    PreviewRejectionRowSpec {
        row_name: "out-of-scope-workflow-foundation-request",
        perturbation_class: PreviewPerturbationClass::WorkflowFoundationScopeOverreach,
        failure_class: PreviewFailureClass::WorkflowFoundationScopeOverreach,
        runtime_failure_selector: Some(
            PreviewRuntimeFailureSelector::WorkflowFoundationScopeOverreach,
        ),
        compile_fail_case: None,
    },
];

pub const PREVIEW_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "preview-basis-execution-parity",
    "preview-lifecycle-explicitness",
    "preview-promotion-comparison-parity",
    "preview-lifecycle-no-rediscovery",
    "preview-live-admission-parity",
    "preview-live-drift-explicitness",
    "preview-comparison-shape-proof-width",
    "preview-workflow-foundation-admission",
    "preview-workflow-foundation-no-rescan",
    "preview-work-avoided-counter-parity",
];

pub const PREVIEW_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "unsupported-preview-family",
    "invalid-preview-basis",
    "stale-preview-lifecycle-denied",
    "preview-live-drift-denied",
    "preview-live-broad-fallback-forbidden",
    "preview-broad-fallback-forbidden",
    "discarded-preview-execution-denied",
    "unsupported-preview-promotion-comparison",
    "read-only-preview-denies-promotion-comparison",
    "raw-branch-alias-preview-forbidden",
    "preview-promotion-linkage-denied",
    "preview-replay-linkage-denied",
    "preview-shape-mismatch-denied",
    "promotion-eligibility-bool-forbidden",
    "preview-diagnostics-rescan-forbidden",
    "fabricated-preview-lifecycle-forbidden",
    "out-of-scope-workflow-foundation-request",
];
