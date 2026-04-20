use crate::harness::certification::HostileExpectation;

use super::{WorkflowFailureClass, WorkflowPerturbationClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkflowCanonicalRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: WorkflowPerturbationClass,
    pub hostile_expectation: HostileExpectation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkflowRejectionRowSpec {
    pub row_name: &'static str,
    pub perturbation_class: WorkflowPerturbationClass,
    pub failure_class: WorkflowFailureClass,
    pub compile_fail_case: Option<&'static str>,
}

pub const WORKFLOW_CANONICAL_ROW_SPECS: &[WorkflowCanonicalRowSpec] = &[
    WorkflowCanonicalRowSpec {
        row_name: "workflow-declaration-family-explicitness",
        perturbation_class: WorkflowPerturbationClass::DeclarationFamily,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "workflow-basis-family-explicitness",
        perturbation_class: WorkflowPerturbationClass::BasisFamily,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "workflow-authority-target-explicitness",
        perturbation_class: WorkflowPerturbationClass::AuthorityTargetFamily,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "workflow-preview-foundation-no-rediscovery",
        perturbation_class: WorkflowPerturbationClass::NoRediscovery,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "workflow-budget-class-explicitness",
        perturbation_class: WorkflowPerturbationClass::BudgetClass,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "query-authored-mutation-lowering-parity",
        perturbation_class: WorkflowPerturbationClass::MutationParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "query-authored-merge-lowering-parity",
        perturbation_class: WorkflowPerturbationClass::LoweringParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "query-triggered-writeback-lowering-parity",
        perturbation_class: WorkflowPerturbationClass::WritebackParity,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "conflict-inspection-explicitness",
        perturbation_class: WorkflowPerturbationClass::ConflictInspection,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "unsupported-deletion-topology-merge-class",
        perturbation_class: WorkflowPerturbationClass::DeniedMergeClass,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "post-merge-inspection-explicitness",
        perturbation_class: WorkflowPerturbationClass::PostMergeInspection,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "workflow-freshness-explicitness",
        perturbation_class: WorkflowPerturbationClass::Freshness,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "workflow-prediction-width-explicitness",
        perturbation_class: WorkflowPerturbationClass::PredictionWidth,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "workflow-realized-width-explicitness",
        perturbation_class: WorkflowPerturbationClass::RealizedWidth,
        hostile_expectation: HostileExpectation::DistinctFromControl,
    },
    WorkflowCanonicalRowSpec {
        row_name: "workflow-rediscovery-zero-parity",
        perturbation_class: WorkflowPerturbationClass::RediscoveryZero,
        hostile_expectation: HostileExpectation::EquivalentToControl,
    },
];

pub const WORKFLOW_REJECTION_ROW_SPECS: &[WorkflowRejectionRowSpec] = &[
    WorkflowRejectionRowSpec {
        row_name: "unsupported-workflow-family",
        perturbation_class: WorkflowPerturbationClass::UnsupportedWorkflowFamily,
        failure_class: WorkflowFailureClass::UnsupportedWorkflowFamily,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "invalid-basis-pairing",
        perturbation_class: WorkflowPerturbationClass::InvalidBasisPairing,
        failure_class: WorkflowFailureClass::InvalidBasisPairing,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "preview-read-only-authority-request-forbidden",
        perturbation_class: WorkflowPerturbationClass::PreviewReadOnlyAuthority,
        failure_class: WorkflowFailureClass::PreviewReadOnlyAuthorityRequestForbidden,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "unsupported-authority-target",
        perturbation_class: WorkflowPerturbationClass::UnsupportedAuthorityTarget,
        failure_class: WorkflowFailureClass::UnsupportedAuthorityTargetFamily,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "forbidden-workflow-broadening",
        perturbation_class: WorkflowPerturbationClass::ForbiddenBroadening,
        failure_class: WorkflowFailureClass::ForbiddenWorkflowBroadening,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "unsupported-merge-family",
        perturbation_class: WorkflowPerturbationClass::UnsupportedWorkflowFamily,
        failure_class: WorkflowFailureClass::UnsupportedWorkflowFamily,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "unsupported-writeback-family",
        perturbation_class: WorkflowPerturbationClass::UnsupportedWorkflowFamily,
        failure_class: WorkflowFailureClass::UnsupportedWorkflowFamily,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "explicit-rebind-required",
        perturbation_class: WorkflowPerturbationClass::ExplicitRebindRequired,
        failure_class: WorkflowFailureClass::ExplicitRebindRequired,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "stale-workflow-denied",
        perturbation_class: WorkflowPerturbationClass::Freshness,
        failure_class: WorkflowFailureClass::StaleWorkflowDenied,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "post-merge-non-authoritative-outcome-forbidden",
        perturbation_class: WorkflowPerturbationClass::PostMergeOutcomeForbidden,
        failure_class: WorkflowFailureClass::PostMergeOutcomeForbidden,
        compile_fail_case: None,
    },
    WorkflowRejectionRowSpec {
        row_name: "query-workflow-declaration-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/query_workflow_declaration_constructor_private.rs"),
    },
    WorkflowRejectionRowSpec {
        row_name: "workflow-context-binding-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/workflow_context_binding_constructor_private.rs"),
    },
    WorkflowRejectionRowSpec {
        row_name: "workflow-authority-target-override-forbidden",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/workflow_authority_target_override_forbidden.rs"),
    },
    WorkflowRejectionRowSpec {
        row_name: "workflow-admission-bool-shortcut-forbidden",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/workflow_admission_bool_shortcut_forbidden.rs"),
    },
    WorkflowRejectionRowSpec {
        row_name: "raw-preflight-is-not-workflow-declaration",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/raw_preflight_is_not_workflow_declaration.rs"),
    },
    WorkflowRejectionRowSpec {
        row_name: "query-conflict-inspection-artifact-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some(
            "tests/ui/query_conflict_inspection_artifact_constructor_private.rs",
        ),
    },
    WorkflowRejectionRowSpec {
        row_name: "query-post-merge-inspection-artifact-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some(
            "tests/ui/query_post_merge_inspection_artifact_constructor_private.rs",
        ),
    },
    WorkflowRejectionRowSpec {
        row_name: "workflow-authority-outcome-artifact-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some(
            "tests/ui/workflow_authority_outcome_artifact_constructor_private.rs",
        ),
    },
    WorkflowRejectionRowSpec {
        row_name: "workflow-replay-bundle-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/workflow_replay_bundle_constructor_private.rs"),
    },
    WorkflowRejectionRowSpec {
        row_name: "lowered-mutation-intent-declaration-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some(
            "tests/ui/lowered_mutation_intent_declaration_constructor_private.rs",
        ),
    },
    WorkflowRejectionRowSpec {
        row_name: "lowered-merge-workflow-declaration-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some(
            "tests/ui/lowered_merge_workflow_declaration_constructor_private.rs",
        ),
    },
    WorkflowRejectionRowSpec {
        row_name: "query-writeback-declaration-constructor-private",
        perturbation_class: WorkflowPerturbationClass::CompileFailBoundary,
        failure_class: WorkflowFailureClass::CompileFail,
        compile_fail_case: Some("tests/ui/query_writeback_declaration_constructor_private.rs"),
    },
];

pub const WORKFLOW_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "workflow-declaration-family-explicitness",
    "workflow-basis-family-explicitness",
    "workflow-authority-target-explicitness",
    "workflow-preview-foundation-no-rediscovery",
    "workflow-budget-class-explicitness",
    "query-authored-mutation-lowering-parity",
    "query-authored-merge-lowering-parity",
    "query-triggered-writeback-lowering-parity",
    "conflict-inspection-explicitness",
    "unsupported-deletion-topology-merge-class",
    "post-merge-inspection-explicitness",
    "workflow-freshness-explicitness",
    "workflow-prediction-width-explicitness",
    "workflow-realized-width-explicitness",
    "workflow-rediscovery-zero-parity",
];

pub const WORKFLOW_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "unsupported-workflow-family",
    "invalid-basis-pairing",
    "preview-read-only-authority-request-forbidden",
    "unsupported-authority-target",
    "forbidden-workflow-broadening",
    "unsupported-merge-family",
    "unsupported-writeback-family",
    "stale-workflow-denied",
    "explicit-rebind-required",
    "post-merge-non-authoritative-outcome-forbidden",
    "query-workflow-declaration-constructor-private",
    "workflow-context-binding-constructor-private",
    "workflow-authority-target-override-forbidden",
    "workflow-admission-bool-shortcut-forbidden",
    "raw-preflight-is-not-workflow-declaration",
];
