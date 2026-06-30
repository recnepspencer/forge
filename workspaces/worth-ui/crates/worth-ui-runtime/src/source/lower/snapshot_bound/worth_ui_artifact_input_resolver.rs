use std::collections::BTreeMap;

use crate::capability::CapabilitySnapshot;
use crate::source::{
    WorthUiArtifactInput, WorthUiArtifactInputNode, WorthUiResolutionDiagnostic,
    WorthUiResolutionReport, WorthUiResolvedArtifactInput, WorthUiResolvedArtifactInputBindingNode,
    WorthUiResolvedArtifactInputComponentNode, WorthUiResolvedArtifactInputModule,
    WorthUiResolvedArtifactInputNode, WorthUiResolvedArtifactInputSurfaceNode,
    WorthUiResolvedArtifactInputThemeTokenNode,
};

use super::worth_ui_snapshot_resolution_context::WorthUiSnapshotResolutionContext;

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiArtifactInputResolver;

impl WorthUiArtifactInputResolver {
    pub(crate) fn resolve(
        artifact_input: &WorthUiArtifactInput,
        snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiResolvedArtifactInput, WorthUiResolutionReport> {
        let mut resolution_context = WorthUiSnapshotResolutionContext::new(snapshot);
        let mut modules = BTreeMap::new();
        let mut diagnostics = Vec::<WorthUiResolutionDiagnostic>::new();

        for module_id in artifact_input.module_ids() {
            let artifact_input_module = artifact_input
                .module(module_id)
                .expect("artifact input should contain every canonical module");
            let mut resolved_nodes = Vec::new();

            for node in artifact_input_module.nodes() {
                match resolve_node(module_id, node, &mut resolution_context) {
                    Ok(resolved_node) => resolved_nodes.push(resolved_node),
                    Err(diagnostic) => diagnostics.push(diagnostic),
                }
            }

            modules.insert(
                module_id.clone(),
                WorthUiResolvedArtifactInputModule::new(module_id.clone(), resolved_nodes),
            );
        }

        let metrics = resolution_context.finish_metrics();
        if !diagnostics.is_empty() {
            return Err(WorthUiResolutionReport::new(diagnostics, metrics));
        }

        Ok(WorthUiResolvedArtifactInput::new(
            modules,
            artifact_input.module_ids().to_vec(),
        ))
    }
}

fn resolve_node(
    module_id: &crate::source::WorthUiSourceModuleId,
    node: &WorthUiArtifactInputNode,
    resolution_context: &mut WorthUiSnapshotResolutionContext<'_>,
) -> Result<WorthUiResolvedArtifactInputNode, WorthUiResolutionDiagnostic> {
    match node {
        WorthUiArtifactInputNode::Import(import_node) => Ok(
            WorthUiResolvedArtifactInputNode::Import(import_node.clone()),
        ),
        WorthUiArtifactInputNode::Component(component_node) => {
            let (component, descriptor) = resolution_context.resolve_component(
                module_id,
                component_node.name_text(),
                component_node.provenance(),
            )?;
            Ok(WorthUiResolvedArtifactInputNode::Component(
                WorthUiResolvedArtifactInputComponentNode::new(
                    component,
                    descriptor,
                    component_node.authored_identity().map(str::to_owned),
                    component_node.body_atoms().to_vec(),
                    component_node.provenance().clone(),
                ),
            ))
        }
        WorthUiArtifactInputNode::Surface(surface_node) => {
            let (surface, descriptor) = resolution_context.resolve_surface(
                module_id,
                surface_node.name_text(),
                surface_node.provenance(),
            )?;
            Ok(WorthUiResolvedArtifactInputNode::Surface(
                WorthUiResolvedArtifactInputSurfaceNode::new(
                    surface,
                    descriptor,
                    surface_node.authored_identity().map(str::to_owned),
                    surface_node.body_atoms().to_vec(),
                    surface_node.provenance().clone(),
                ),
            ))
        }
        WorthUiArtifactInputNode::Binding(binding_node) => {
            let (view_binding, entry) = resolution_context.resolve_view_binding(
                module_id,
                binding_node.name_text(),
                binding_node.provenance(),
            )?;
            Ok(WorthUiResolvedArtifactInputNode::Binding(
                WorthUiResolvedArtifactInputBindingNode::new(
                    view_binding,
                    entry,
                    binding_node.authored_identity().map(str::to_owned),
                    binding_node.body_atoms().to_vec(),
                    binding_node.provenance().clone(),
                ),
            ))
        }
        WorthUiArtifactInputNode::Token(token_node) => {
            let (theme_token, entry) = resolution_context.resolve_theme_token(
                module_id,
                token_node.name_text(),
                token_node.provenance(),
            )?;
            Ok(WorthUiResolvedArtifactInputNode::Token(
                WorthUiResolvedArtifactInputThemeTokenNode::new(
                    theme_token,
                    entry,
                    token_node.authored_identity().map(str::to_owned),
                    token_node.provenance().clone(),
                ),
            ))
        }
    }
}
