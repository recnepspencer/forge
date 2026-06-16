use std::collections::BTreeMap;

use crate::capability::CapabilitySnapshot;
use crate::source::{
    WorthUiLegallyStructuredArtifactInput, WorthUiLegallyStructuredArtifactInputModule,
    WorthUiLegallyStructuredArtifactInputNode, WorthUiLegallyStructuredArtifactInputThemeTokenNode,
    WorthUiResolvedArtifactInput, WorthUiResolvedArtifactInputNode,
    WorthUiStructuralLegalityDiagnostic, WorthUiStructuralLegalityReport,
};

use super::worth_ui_structural_legality_context::WorthUiStructuralLegalityContext;
use super::worth_ui_structural_legality_node_lowering::{
    lower_binding_node, lower_component_node, lower_page_node, lower_surface_node,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiStructuralLegalityLowerer;

impl WorthUiStructuralLegalityLowerer {
    pub(crate) fn lower(
        resolved_artifact_input: &WorthUiResolvedArtifactInput,
        capability_snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiLegallyStructuredArtifactInput, WorthUiStructuralLegalityReport> {
        let mut context = WorthUiStructuralLegalityContext::new(capability_snapshot);
        let mut modules = BTreeMap::new();
        let mut diagnostics = Vec::<WorthUiStructuralLegalityDiagnostic>::new();

        for module_id in resolved_artifact_input.module_ids() {
            let resolved_module = resolved_artifact_input
                .module(module_id)
                .expect("resolved artifact input should contain every canonical module");
            let mut nodes = Vec::new();

            for node in resolved_module.nodes() {
                match lower_node(module_id, node, &mut context) {
                    Ok(node) => nodes.push(node),
                    Err(mut node_diagnostics) => diagnostics.append(&mut node_diagnostics),
                }
            }

            modules.insert(
                module_id.clone(),
                WorthUiLegallyStructuredArtifactInputModule::new(module_id.clone(), nodes),
            );
        }

        let metrics = context.finish_metrics();
        if !diagnostics.is_empty() {
            return Err(WorthUiStructuralLegalityReport::new(diagnostics, metrics));
        }

        Ok(WorthUiLegallyStructuredArtifactInput::new(
            modules,
            resolved_artifact_input.module_ids().to_vec(),
        ))
    }
}

fn lower_node(
    module_id: &crate::source::WorthUiSourceModuleId,
    node: &WorthUiResolvedArtifactInputNode,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<WorthUiLegallyStructuredArtifactInputNode, Vec<WorthUiStructuralLegalityDiagnostic>> {
    match node {
        WorthUiResolvedArtifactInputNode::Import(import_node) => Ok(
            WorthUiLegallyStructuredArtifactInputNode::Import(import_node.clone()),
        ),
        WorthUiResolvedArtifactInputNode::Page(page_node) => {
            lower_page_node(module_id, page_node, context)
                .map(WorthUiLegallyStructuredArtifactInputNode::Page)
        }
        WorthUiResolvedArtifactInputNode::Component(component_node) => {
            lower_component_node(module_id, component_node, context)
                .map(WorthUiLegallyStructuredArtifactInputNode::Component)
        }
        WorthUiResolvedArtifactInputNode::Surface(surface_node) => {
            lower_surface_node(module_id, surface_node, context)
                .map(WorthUiLegallyStructuredArtifactInputNode::Surface)
        }
        WorthUiResolvedArtifactInputNode::Binding(binding_node) => {
            lower_binding_node(module_id, binding_node, context)
                .map(WorthUiLegallyStructuredArtifactInputNode::Binding)
        }
        WorthUiResolvedArtifactInputNode::Token(token_node) => {
            Ok(WorthUiLegallyStructuredArtifactInputNode::Token(
                WorthUiLegallyStructuredArtifactInputThemeTokenNode::new(
                    token_node.theme_token().clone(),
                    token_node.entry().clone(),
                    token_node.authored_identity().map(str::to_owned),
                    token_node.provenance().clone(),
                ),
            ))
        }
    }
}
