use std::collections::BTreeMap;

use crate::capability::{CapabilitySnapshot, FrozenViewBindingEntry};
use crate::source::{
    WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer, WorthUiBoundArtifactInput,
    WorthUiLegallyStructuredArtifactInput, WorthUiLegallyStructuredArtifactInputBindingNode,
    WorthUiLegallyStructuredArtifactInputModule, WorthUiLegallyStructuredArtifactInputNode,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredToArtifactInputLowerer,
    WorthUiStructuralLegalityLowerer,
};

use super::binding_app_fixture::standard_artifact_input;

pub(super) fn legally_structured_artifact_input(
    snapshot: &CapabilitySnapshot,
) -> WorthUiLegallyStructuredArtifactInput {
    legally_structured_artifact_input_for(snapshot, &standard_artifact_input())
}

pub(super) fn legally_structured_artifact_input_for(
    snapshot: &CapabilitySnapshot,
    artifact_input: &WorthUiRustAuthoredArtifactInput,
) -> WorthUiLegallyStructuredArtifactInput {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(artifact_input);
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution should succeed");
    WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("phase 5 structural legality should succeed")
}

pub(super) fn bound_artifact_input(snapshot: &CapabilitySnapshot) -> WorthUiBoundArtifactInput {
    bound_artifact_input_for(snapshot, &standard_artifact_input())
}

pub(super) fn bound_artifact_input_for(
    snapshot: &CapabilitySnapshot,
    artifact_input: &WorthUiRustAuthoredArtifactInput,
) -> WorthUiBoundArtifactInput {
    let legally_structured = legally_structured_artifact_input_for(snapshot, artifact_input);
    WorthUiBindingSemanticsLowerer::lower(&legally_structured, snapshot)
        .expect("phase 6 binding semantics should succeed")
}

pub(super) fn legally_structured_with_binding_entry(
    snapshot: &CapabilitySnapshot,
    binding_entry: FrozenViewBindingEntry,
) -> WorthUiLegallyStructuredArtifactInput {
    let baseline = legally_structured_artifact_input(snapshot);
    let mut modules = BTreeMap::new();

    for module_id in baseline.module_ids() {
        let module = baseline.module(module_id).unwrap();
        let nodes = module
            .nodes()
            .iter()
            .cloned()
            .map(|node| match node {
                WorthUiLegallyStructuredArtifactInputNode::Binding(binding_node) => {
                    WorthUiLegallyStructuredArtifactInputNode::Binding(
                        WorthUiLegallyStructuredArtifactInputBindingNode::new(
                            binding_node.view_binding().clone(),
                            binding_entry.clone(),
                            binding_node.authored_identity().map(str::to_owned),
                            binding_node.structure().clone(),
                            binding_node.provenance().clone(),
                        ),
                    )
                }
                other => other,
            })
            .collect();
        modules.insert(
            module_id.clone(),
            WorthUiLegallyStructuredArtifactInputModule::new(module_id.clone(), nodes),
        );
    }

    WorthUiLegallyStructuredArtifactInput::new(modules, baseline.module_ids().to_vec())
}
