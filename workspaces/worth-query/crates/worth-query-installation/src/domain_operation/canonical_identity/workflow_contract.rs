use sha2::Sha256;

use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_operation::*;

use super::{bool_name, conditional_nodes::hash_conditional_nodes, hash_sequence};

pub(super) fn hash_workflow_contract(
    hasher: &mut Sha256,
    contract: &WorthQueryOperationWorkflowContract,
) {
    match contract {
        WorthQueryOperationWorkflowContract::NotRequired => {
            hash_text_field(hasher, "workflow", "not-required");
        }
        WorthQueryOperationWorkflowContract::Declared(workflow) => {
            hash_text_field(hasher, "workflow", "declared");
            hash_text_field(hasher, "workflow-entry", workflow.entry_stage());
            for stage in workflow.stages() {
                hash_stage(hasher, stage);
            }
        }
    }
}

fn hash_stage(hasher: &mut Sha256, stage: &WorthQueryPortableWorkflowStage) {
    hash_text_field(hasher, "workflow-stage", stage.identity());
    hash_sequence(
        hasher,
        "workflow-predecessor",
        stage.predecessors().iter().map(String::as_str),
    );
    hash_text_field(hasher, "workflow-terminal", bool_name(stage.is_terminal()));
    hash_text_field(
        hasher,
        "workflow-publishable",
        bool_name(stage.is_publishable()),
    );
    hash_sequence(
        hasher,
        "workflow-capability",
        stage
            .required_capabilities()
            .iter()
            .map(|capability| capability.as_str()),
    );
    hash_stage_semantics(hasher, stage.semantics());
}

fn hash_stage_semantics(hasher: &mut Sha256, semantics: &WorthQueryWorkflowStageSemantics) {
    hash_workflow_value(hasher, "workflow-input", &semantics.input);
    hash_workflow_value(hasher, "workflow-output", &semantics.output);
    hash_sequence(
        hasher,
        "workflow-required-domain",
        semantics
            .required_domain_roles
            .iter()
            .map(|role| role.as_str()),
    );
    hash_sequence(
        hasher,
        "workflow-graph-read",
        semantics.graph_read_roles.iter().map(String::as_str),
    );
    hash_sequence(
        hasher,
        "workflow-touch",
        semantics.touch_roles.iter().map(String::as_str),
    );
    hash_sequence(
        hasher,
        "workflow-effect",
        semantics.effect_roles.iter().map(|family| family.as_str()),
    );
    hash_sequence(
        hasher,
        "workflow-invariant",
        semantics.invariant_roles.iter().map(String::as_str),
    );
    hash_sequence(
        hasher,
        "workflow-cost",
        semantics.cost_roles.iter().map(|role| role.as_str()),
    );
    for state in &semantics.terminal_result_states {
        hash_text_field(
            hasher,
            "workflow-result-state",
            workflow_result_state_name(*state),
        );
    }
    for failure in &semantics.failure_classes {
        hash_text_field(hasher, "workflow-failure", workflow_failure_name(failure));
    }
    hash_conditional_nodes(
        hasher,
        &semantics.conditional_nodes,
        "workflow-stage-condition",
    );
}

fn hash_workflow_value(
    hasher: &mut Sha256,
    label: &'static str,
    value: &WorthQueryWorkflowValueContract,
) {
    hash_text_field(hasher, label, value.canonical_kind());
    if let WorthQueryWorkflowValueContract::InstalledArtifact(reference) = value {
        hash_text_field(hasher, "artifact-family", reference.family().as_str());
        hash_text_field(
            hasher,
            "artifact-schema",
            &reference.schema_version().get().to_string(),
        );
        hash_text_field(
            hasher,
            "artifact-protocol",
            &reference.protocol_version().get().to_string(),
        );
    }
}

fn workflow_result_state_name(state: WorthQueryOperationResultState) -> &'static str {
    match state {
        WorthQueryOperationResultState::Ready => "ready",
        WorthQueryOperationResultState::Advisory => "advisory",
        WorthQueryOperationResultState::Pending => "pending",
        WorthQueryOperationResultState::Partial => "partial",
        WorthQueryOperationResultState::Violation => "violation",
    }
}

fn workflow_failure_name(failure: &WorthQueryOperationFailureClass) -> &str {
    match failure {
        WorthQueryOperationFailureClass::InvalidInput => "invalid-input",
        WorthQueryOperationFailureClass::Unsupported => "unsupported",
        WorthQueryOperationFailureClass::Conflict => "conflict",
        WorthQueryOperationFailureClass::Dependency => "dependency",
        WorthQueryOperationFailureClass::Indeterminate => "indeterminate",
        WorthQueryOperationFailureClass::Domain(name) => name,
    }
}
