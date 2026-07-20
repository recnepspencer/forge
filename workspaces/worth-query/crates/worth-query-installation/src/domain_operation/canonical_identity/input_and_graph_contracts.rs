use sha2::Sha256;

use crate::canonical_hash_encoding::hash_text_field;

use super::{bool_name, hash_sequence};
use crate::domain_operation::*;

pub(super) fn hash_input_and_graph_contracts(
    hasher: &mut Sha256,
    semantics: &WorthQueryDomainOperationSemanticClosure,
) {
    hash_parameters(hasher, &semantics.parameters);
    hash_sequence(
        hasher,
        "required-domain",
        semantics.required_domains.iter().map(|role| role.as_str()),
    );
    hash_native_projection(hasher, &semantics.native_projection);
    hash_text_field(
        hasher,
        "query-intent",
        semantics.canonical_query.query().digest().as_str(),
    );
    hash_text_field(
        hasher,
        "result-shape",
        semantics.canonical_query.result_shape().digest().as_str(),
    );
    hash_collection(hasher, &semantics.collection);
    hash_sequence(
        hasher,
        "required-capability",
        semantics
            .required_capabilities
            .iter()
            .map(|capability| capability.as_str()),
    );
    hash_workflow(hasher, &semantics.workflow);
    hash_conditional_nodes(hasher, &semantics.conditional_nodes, "operation-condition");
    hash_graph_reads(hasher, &semantics.graph_reads);
    hash_touches(hasher, &semantics.touches);
    hash_effects(hasher, &semantics.effects);
    hash_invariants(hasher, &semantics.invariants);
}

fn hash_parameters(hasher: &mut Sha256, contract: &WorthQueryOperationParameterContract) {
    match contract {
        WorthQueryOperationParameterContract::NotRequired => {
            hash_text_field(hasher, "parameters", "not-required");
        }
        WorthQueryOperationParameterContract::Declared { fields } => {
            hash_text_field(hasher, "parameters", "declared");
            for field in fields {
                hash_text_field(hasher, "parameter-name", &field.name);
                hash_text_field(hasher, "parameter-required", bool_name(field.required));
                match &field.value_family {
                    WorthQueryOperationValueFamily::NativeAspect { key, identity } => {
                        hash_text_field(hasher, "parameter-family", "native-aspect");
                        hash_text_field(hasher, "parameter-aspect-key", key.as_str());
                        hash_text_field(
                            hasher,
                            "parameter-aspect-identity",
                            &identity.0.to_string(),
                        );
                    }
                    family => {
                        hash_text_field(hasher, "parameter-family", value_family_name(family));
                    }
                }
            }
        }
    }
}

fn hash_native_projection(
    hasher: &mut Sha256,
    contract: &WorthQueryOperationNativeProjectionContract,
) {
    hash_text_field(hasher, "native-aspect-key", contract.aspect_key.as_str());
    hash_text_field(
        hasher,
        "native-aspect-identity",
        &contract.aspect_identity.0.to_string(),
    );
    hash_text_field(
        hasher,
        "native-contract-revision",
        &contract.contract_revision.0.to_string(),
    );
    if contract.mask.is_whole_aspect() {
        hash_text_field(hasher, "native-mask", "whole-aspect");
    } else {
        for path in contract.mask.paths() {
            let path = path
                .fields()
                .iter()
                .map(|field| field.as_str())
                .collect::<Vec<_>>()
                .join(".");
            hash_text_field(hasher, "native-mask-field", &path);
        }
    }
}

fn hash_collection(hasher: &mut Sha256, contract: &WorthQueryOperationCollectionContract) {
    match contract {
        WorthQueryOperationCollectionContract::NotCollection => {
            hash_text_field(hasher, "collection", "not-collection");
        }
        WorthQueryOperationCollectionContract::Collection {
            row_identity_field,
            ordering_fields,
            continuation,
        } => {
            hash_text_field(hasher, "collection", "collection");
            hash_text_field(hasher, "row-identity-field", row_identity_field);
            hash_sequence(
                hasher,
                "ordering-field",
                ordering_fields.iter().map(String::as_str),
            );
            hash_text_field(hasher, "continuation", continuation_name(*continuation));
        }
    }
}

fn hash_workflow(hasher: &mut Sha256, contract: &WorthQueryOperationWorkflowContract) {
    match contract {
        WorthQueryOperationWorkflowContract::NotRequired => {
            hash_text_field(hasher, "workflow", "not-required");
        }
        WorthQueryOperationWorkflowContract::Declared(workflow) => {
            hash_text_field(hasher, "workflow", "declared");
            hash_text_field(hasher, "workflow-entry", workflow.entry_stage());
            for stage in workflow.stages() {
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
                let semantics = stage.semantics();
                hash_text_field(
                    hasher,
                    "workflow-input",
                    workflow_value_name(semantics.input),
                );
                hash_text_field(
                    hasher,
                    "workflow-output",
                    workflow_value_name(semantics.output),
                );
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
        }
    }
}

fn hash_conditional_nodes(
    hasher: &mut Sha256,
    nodes: &[WorthQueryPortableConditionalNodeDeclaration],
    tag: &'static str,
) {
    if nodes.is_empty() {
        hash_text_field(hasher, tag, "not-required");
        return;
    }
    for node in nodes {
        hash_text_field(hasher, tag, &node.canonical_token());
    }
}

fn workflow_value_name(value: WorthQueryWorkflowValueContract) -> &'static str {
    match value {
        WorthQueryWorkflowValueContract::NotRequired => "not-required",
        WorthQueryWorkflowValueContract::Bool => "bool",
        WorthQueryWorkflowValueContract::I64 => "i64",
        WorthQueryWorkflowValueContract::U64 => "u64",
        WorthQueryWorkflowValueContract::Text => "text",
        WorthQueryWorkflowValueContract::EntityIdentity => "entity-identity",
        WorthQueryWorkflowValueContract::Projection => "projection",
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

fn hash_graph_reads(hasher: &mut Sha256, contract: &WorthQueryOperationGraphReadContract) {
    let WorthQueryOperationGraphReadContract::Declared { roles } = contract else {
        hash_text_field(hasher, "graph-read", "not-required");
        return;
    };
    hash_text_field(hasher, "graph-read", "declared");
    for role in roles {
        hash_text_field(hasher, "graph-read-role", &role.role);
        match &role.participation {
            WorthQueryOperationGraphParticipation::PrimaryLogicalGraph => {
                hash_text_field(hasher, "graph-participation", "primary");
            }
            WorthQueryOperationGraphParticipation::SeparateAuthority { role } => {
                hash_text_field(hasher, "graph-participation", "separate");
                hash_text_field(hasher, "graph-participation-role", role);
            }
        }
        hash_text_field(hasher, "graph-access", graph_access_name(role.access));
        for read in &role.semantic_reads {
            hash_text_field(hasher, "graph-semantic-read", &read.canonical_key());
        }
    }
}

fn hash_touches(hasher: &mut Sha256, contract: &WorthQueryOperationTouchContract) {
    match contract {
        WorthQueryOperationTouchContract::NotRequired => {
            hash_text_field(hasher, "touch", "not-required");
        }
        WorthQueryOperationTouchContract::Declared {
            graph_roles,
            scopes,
        } => {
            hash_text_field(hasher, "touch", "declared");
            hash_sequence(
                hasher,
                "touch-graph",
                graph_roles.iter().map(String::as_str),
            );
            hash_sequence(hasher, "touch-scope", scopes.iter().map(String::as_str));
        }
    }
}

fn hash_effects(hasher: &mut Sha256, contract: &WorthQueryOperationEffectContract) {
    match contract {
        WorthQueryOperationEffectContract::NotRequired => {
            hash_text_field(hasher, "effect", "not-required");
        }
        WorthQueryOperationEffectContract::Declared { effect_families } => {
            hash_text_field(hasher, "effect", "declared");
            hash_sequence(
                hasher,
                "effect-family",
                effect_families.iter().map(|family| family.as_str()),
            );
        }
    }
}

fn hash_invariants(hasher: &mut Sha256, contract: &WorthQueryOperationInvariantContract) {
    match contract {
        WorthQueryOperationInvariantContract::NotRequired => {
            hash_text_field(hasher, "invariant", "not-required");
        }
        WorthQueryOperationInvariantContract::Declared { invariant_slots } => hash_sequence(
            hasher,
            "invariant-slot",
            invariant_slots.iter().map(String::as_str),
        ),
    }
}

fn value_family_name(family: &WorthQueryOperationValueFamily) -> &'static str {
    match family {
        WorthQueryOperationValueFamily::Bool => "bool",
        WorthQueryOperationValueFamily::I64 => "i64",
        WorthQueryOperationValueFamily::U64 => "u64",
        WorthQueryOperationValueFamily::Text => "text",
        WorthQueryOperationValueFamily::EntityIdentity => "entity-identity",
        WorthQueryOperationValueFamily::NativeAspect { .. } => "native-aspect",
    }
}

fn continuation_name(posture: WorthQueryOperationContinuationPosture) -> &'static str {
    match posture {
        WorthQueryOperationContinuationPosture::NotRequired => "not-required",
        WorthQueryOperationContinuationPosture::SnapshotCursor => "snapshot-cursor",
        WorthQueryOperationContinuationPosture::LiveCursor => "live-cursor",
    }
}

fn graph_access_name(access: WorthQueryOperationGraphAccess) -> &'static str {
    match access {
        WorthQueryOperationGraphAccess::Observe => "observe",
        WorthQueryOperationGraphAccess::Project => "project",
    }
}
