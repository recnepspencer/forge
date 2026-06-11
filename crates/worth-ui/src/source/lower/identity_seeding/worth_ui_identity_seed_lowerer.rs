use std::collections::BTreeMap;

use crate::source::{
    WorthUiBoundArtifactInput, WorthUiBoundArtifactInputNode, WorthUiIdentitySeededArtifactInput,
    WorthUiIdentitySeededArtifactInputBindingNode, WorthUiIdentitySeededArtifactInputComponentNode,
    WorthUiIdentitySeededArtifactInputImportNode, WorthUiIdentitySeededArtifactInputModule,
    WorthUiIdentitySeededArtifactInputNode, WorthUiIdentitySeededArtifactInputSurfaceNode,
    WorthUiIdentitySeededArtifactInputThemeTokenNode,
};

use super::worth_ui_identity_seed_basis::{
    binding_seed, classify_durable_state, component_seed, import_seed, no_durable_state_surface,
    surface_seed, token_seed,
};
use super::worth_ui_identity_seeding_metrics::WorthUiIdentitySeedingMetrics;

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiIdentitySeedLowerer;

impl WorthUiIdentitySeedLowerer {
    pub(crate) fn lower(
        bound_artifact_input: &WorthUiBoundArtifactInput,
    ) -> (
        WorthUiIdentitySeededArtifactInput,
        WorthUiIdentitySeedingMetrics,
    ) {
        let mut modules = BTreeMap::new();
        let mut metrics = WorthUiIdentitySeedingMetrics::default();

        for module_id in bound_artifact_input.module_ids() {
            let module = bound_artifact_input
                .module(module_id)
                .expect("bound artifact input should contain every canonical module");
            let nodes = module
                .nodes()
                .iter()
                .map(|node| lower_node(module_id, node, &mut metrics))
                .collect();
            modules.insert(
                module_id.clone(),
                WorthUiIdentitySeededArtifactInputModule::new(module_id.clone(), nodes),
            );
        }

        (
            WorthUiIdentitySeededArtifactInput::new(
                modules,
                bound_artifact_input.module_ids().to_vec(),
            ),
            metrics,
        )
    }
}

fn lower_node(
    module_id: &crate::source::WorthUiSourceModuleId,
    node: &WorthUiBoundArtifactInputNode,
    metrics: &mut WorthUiIdentitySeedingMetrics,
) -> WorthUiIdentitySeededArtifactInputNode {
    match node {
        WorthUiBoundArtifactInputNode::Import(import_node) => {
            let seed = import_seed(module_id, import_node);
            let durable = no_durable_state_surface();
            metrics.record_seed(false, false);
            WorthUiIdentitySeededArtifactInputNode::Import(
                WorthUiIdentitySeededArtifactInputImportNode::new(
                    import_node.clone(),
                    seed,
                    durable,
                ),
            )
        }
        WorthUiBoundArtifactInputNode::Component(component_node) => {
            let seed = component_seed(module_id, component_node);
            let durable = classify_durable_state(component_node.structure());
            metrics.record_seed(
                component_node.authored_identity().is_some(),
                matches!(
                    durable,
                    crate::source::WorthUiDurableStateEligibility::Eligible { .. }
                ),
            );
            WorthUiIdentitySeededArtifactInputNode::Component(
                WorthUiIdentitySeededArtifactInputComponentNode::new(
                    component_node.clone(),
                    seed,
                    durable,
                ),
            )
        }
        WorthUiBoundArtifactInputNode::Surface(surface_node) => {
            let seed = surface_seed(module_id, surface_node);
            let durable = classify_durable_state(surface_node.structure());
            metrics.record_seed(
                surface_node.authored_identity().is_some(),
                matches!(
                    durable,
                    crate::source::WorthUiDurableStateEligibility::Eligible { .. }
                ),
            );
            WorthUiIdentitySeededArtifactInputNode::Surface(
                WorthUiIdentitySeededArtifactInputSurfaceNode::new(
                    surface_node.clone(),
                    seed,
                    durable,
                ),
            )
        }
        WorthUiBoundArtifactInputNode::Binding(binding_node) => {
            let seed = binding_seed(module_id, binding_node);
            let durable = no_durable_state_surface();
            metrics.record_seed(binding_node.authored_identity().is_some(), false);
            WorthUiIdentitySeededArtifactInputNode::Binding(
                WorthUiIdentitySeededArtifactInputBindingNode::new(
                    binding_node.clone(),
                    seed,
                    durable,
                ),
            )
        }
        WorthUiBoundArtifactInputNode::Token(token_node) => {
            let seed = token_seed(module_id, token_node);
            let durable = no_durable_state_surface();
            metrics.record_seed(token_node.authored_identity().is_some(), false);
            WorthUiIdentitySeededArtifactInputNode::Token(
                WorthUiIdentitySeededArtifactInputThemeTokenNode::new(
                    token_node.clone(),
                    seed,
                    durable,
                ),
            )
        }
    }
}
