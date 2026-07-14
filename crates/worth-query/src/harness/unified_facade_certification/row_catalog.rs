use super::{UnifiedFacadeFailureClass, UnifiedFacadePerturbationClass};
use crate::harness::certification::HostileExpectation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnifiedFacadeCanonicalRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: UnifiedFacadePerturbationClass,
    pub hostile_expectation: HostileExpectation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnifiedFacadeRejectionRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: UnifiedFacadePerturbationClass,
    pub failure_class: UnifiedFacadeFailureClass,
}

pub const UNIFIED_FACADE_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "unified-query-read-capability",
    "unified-query-context-capability",
    "unified-identity-evolution-capability",
    "unified-query-context-basis-result-bundle",
    "unified-query-context-diff-result-bundle",
    "unified-live-capability",
    "unified-preview-capability",
    "unified-workflow-capability",
    "unified-historical-capability",
    "unified-config-section-explicitness",
    "capability-support-metadata-sync",
    "query-context-support-profile-sync",
    "identity-evolution-support-profile-sync",
];

pub const UNIFIED_FACADE_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "missing-owning-live-section",
    "invalid-workflow-support-posture",
    "deferred-durable-artifacts",
    "invalid-unified-configuration",
    "broad-collection-diff-denied",
];

pub const UNIFIED_FACADE_REQUIRED_COMPILE_FAIL_BOUNDARY_NAMES: &[&str] = &[
    "query_read_capability_constructor_private",
    "live_query_capability_constructor_private",
    "preview_session_capability_constructor_private",
    "workflow_orchestration_capability_constructor_private",
    "historical_evaluation_capability_constructor_private",
    "query_context_capability_constructor_private",
    "identity_evolution_capability_constructor_private",
    "validated_worth_query_config_constructor_private",
    "worth_query_support_report_constructor_private",
    "capability_admission_decision_constructor_private",
    "facade_query_read_capability_has_no_live_promote",
    "facade_preview_capability_cannot_admit_workflow",
    "facade_historical_capability_cannot_bind_query_context",
    "facade_has_no_dynamic_capability_routing",
    "legacy_broad_facade_has_no_preview_workflow_shortcut",
    "query_basis_result_bundle_constructor_private",
    "query_diff_result_bundle_constructor_private",
    "facade_query_read_capability_has_no_query_context_basis_bundle",
    "facade_historical_capability_has_no_query_context_diff_bundle",
];

pub const UNIFIED_FACADE_CANONICAL_ROW_SPECS: &[UnifiedFacadeCanonicalRowSpec] = &[
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-query-read-capability",
        perturbation_class: UnifiedFacadePerturbationClass::ApplicationCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-query-context-capability",
        perturbation_class: UnifiedFacadePerturbationClass::QueryContextCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-identity-evolution-capability",
        perturbation_class: UnifiedFacadePerturbationClass::ApplicationCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-query-context-basis-result-bundle",
        perturbation_class: UnifiedFacadePerturbationClass::QueryContextCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-query-context-diff-result-bundle",
        perturbation_class: UnifiedFacadePerturbationClass::QueryContextCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-live-capability",
        perturbation_class: UnifiedFacadePerturbationClass::ApplicationCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-preview-capability",
        perturbation_class: UnifiedFacadePerturbationClass::ApplicationCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-workflow-capability",
        perturbation_class: UnifiedFacadePerturbationClass::ApplicationCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-historical-capability",
        perturbation_class: UnifiedFacadePerturbationClass::ApplicationCapability,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "unified-config-section-explicitness",
        perturbation_class: UnifiedFacadePerturbationClass::ConfigurationSection,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "capability-support-metadata-sync",
        perturbation_class: UnifiedFacadePerturbationClass::SupportMetadata,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "query-context-support-profile-sync",
        perturbation_class: UnifiedFacadePerturbationClass::SupportMetadata,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    UnifiedFacadeCanonicalRowSpec {
        row_name: "identity-evolution-support-profile-sync",
        perturbation_class: UnifiedFacadePerturbationClass::SupportMetadata,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
];

pub const UNIFIED_FACADE_REJECTION_ROW_SPECS: &[UnifiedFacadeRejectionRowSpec] = &[
    UnifiedFacadeRejectionRowSpec {
        row_name: "missing-owning-live-section",
        perturbation_class: UnifiedFacadePerturbationClass::UnsupportedComposition,
        failure_class: UnifiedFacadeFailureClass::MissingOwningSection,
    },
    UnifiedFacadeRejectionRowSpec {
        row_name: "invalid-workflow-support-posture",
        perturbation_class: UnifiedFacadePerturbationClass::UnsupportedComposition,
        failure_class: UnifiedFacadeFailureClass::InvalidComposedSupportPosture,
    },
    UnifiedFacadeRejectionRowSpec {
        row_name: "deferred-durable-artifacts",
        perturbation_class: UnifiedFacadePerturbationClass::DeferredComposition,
        failure_class: UnifiedFacadeFailureClass::DeferredCapability,
    },
    UnifiedFacadeRejectionRowSpec {
        row_name: "invalid-unified-configuration",
        perturbation_class: UnifiedFacadePerturbationClass::ConfigurationSection,
        failure_class: UnifiedFacadeFailureClass::InvalidConfiguration,
    },
    UnifiedFacadeRejectionRowSpec {
        row_name: "broad-collection-diff-denied",
        perturbation_class: UnifiedFacadePerturbationClass::QueryContextCapability,
        failure_class: UnifiedFacadeFailureClass::QueryContextBroadeningDenied,
    },
];
