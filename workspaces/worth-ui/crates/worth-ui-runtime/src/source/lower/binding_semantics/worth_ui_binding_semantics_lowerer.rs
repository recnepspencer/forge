use std::collections::BTreeMap;

use crate::capability::CapabilitySnapshot;
use crate::source::{
    WorthUiBindingDiagnostic, WorthUiBindingSemanticsReport, WorthUiBoundArtifactInput,
    WorthUiBoundArtifactInputModule, WorthUiBoundArtifactInputNode,
    WorthUiLegallyStructuredArtifactInput, WorthUiLegallyStructuredArtifactInputNode,
};

use super::worth_ui_binding_semantics_context::WorthUiBindingSemanticsContext;
use super::worth_ui_binding_semantics_node_lowering::{
    lower_binding_node, lower_component_node, lower_surface_node, lower_theme_token_node,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiBindingSemanticsLowerer;

impl WorthUiBindingSemanticsLowerer {
    pub(crate) fn lower(
        legally_structured_artifact_input: &WorthUiLegallyStructuredArtifactInput,
        capability_snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiBoundArtifactInput, WorthUiBindingSemanticsReport> {
        let mut context = WorthUiBindingSemanticsContext::new(capability_snapshot);
        let mut modules = BTreeMap::new();
        let mut diagnostics = Vec::<WorthUiBindingDiagnostic>::new();

        for module_id in legally_structured_artifact_input.module_ids() {
            let structured_module = legally_structured_artifact_input
                .module(module_id)
                .expect("legally structured artifact input should contain every canonical module");
            let mut nodes = Vec::new();

            for node in structured_module.nodes() {
                match lower_node(module_id, node, &mut context) {
                    Ok(node) => nodes.push(node),
                    Err(mut node_diagnostics) => diagnostics.append(&mut node_diagnostics),
                }
            }

            modules.insert(
                module_id.clone(),
                WorthUiBoundArtifactInputModule::new(module_id.clone(), nodes),
            );
        }

        let metrics = context.finish_metrics();
        if !diagnostics.is_empty() {
            return Err(WorthUiBindingSemanticsReport::new(diagnostics, metrics));
        }

        Ok(WorthUiBoundArtifactInput::new(
            modules,
            legally_structured_artifact_input.module_ids().to_vec(),
        ))
    }
}

fn lower_node(
    module_id: &crate::source::WorthUiSourceModuleId,
    node: &WorthUiLegallyStructuredArtifactInputNode,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundArtifactInputNode, Vec<WorthUiBindingDiagnostic>> {
    match node {
        WorthUiLegallyStructuredArtifactInputNode::Import(import_node) => {
            Ok(WorthUiBoundArtifactInputNode::Import(import_node.clone()))
        }
        WorthUiLegallyStructuredArtifactInputNode::Component(component_node) => Ok(
            WorthUiBoundArtifactInputNode::Component(lower_component_node(component_node)),
        ),
        WorthUiLegallyStructuredArtifactInputNode::Surface(surface_node) => {
            lower_surface_node(module_id, surface_node, context)
                .map(|node| WorthUiBoundArtifactInputNode::Surface(Box::new(node)))
        }
        WorthUiLegallyStructuredArtifactInputNode::Binding(binding_node) => {
            lower_binding_node(module_id, binding_node, context)
                .map(WorthUiBoundArtifactInputNode::Binding)
        }
        WorthUiLegallyStructuredArtifactInputNode::Token(token_node) => {
            lower_theme_token_node(module_id, token_node, context)
                .map(WorthUiBoundArtifactInputNode::Token)
        }
    }
}
