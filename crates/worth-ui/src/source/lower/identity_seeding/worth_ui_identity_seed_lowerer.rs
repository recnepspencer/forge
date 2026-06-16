use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use crate::source::{
    WorthUiArtifactIdentitySeedKind, WorthUiBoundArtifactInput, WorthUiBoundArtifactInputNode,
    WorthUiIdentitySeededArtifactInput, WorthUiIdentitySeededArtifactInputBindingNode,
    WorthUiIdentitySeededArtifactInputComponentNode, WorthUiIdentitySeededArtifactInputImportNode,
    WorthUiIdentitySeededArtifactInputModule, WorthUiIdentitySeededArtifactInputNode,
    WorthUiIdentitySeededArtifactInputPageNode, WorthUiIdentitySeededArtifactInputSurfaceNode,
    WorthUiIdentitySeededArtifactInputThemeTokenNode, WorthUiIdentitySeedingDiagnostic,
    WorthUiIdentitySeedingReport,
};

use super::worth_ui_identity_seed_basis::{
    binding_seed, classify_durable_state, component_seed, import_seed, no_durable_state_surface,
    page_seed, surface_seed, token_seed,
};
use super::worth_ui_identity_seeding_metrics::WorthUiIdentitySeedingMetrics;

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiIdentitySeedLowerer;

impl WorthUiIdentitySeedLowerer {
    pub(crate) fn lower(
        bound_artifact_input: &WorthUiBoundArtifactInput,
    ) -> Result<
        (
            WorthUiIdentitySeededArtifactInput,
            WorthUiIdentitySeedingMetrics,
        ),
        WorthUiIdentitySeedingReport,
    > {
        let mut modules = BTreeMap::new();
        let mut metrics = WorthUiIdentitySeedingMetrics::default();
        let mut diagnostics = Vec::new();
        let mut authored_seed_registry = BTreeMap::<String, String>::new();

        for module_id in bound_artifact_input.module_ids() {
            let module = bound_artifact_input
                .module(module_id)
                .expect("bound artifact input should contain every canonical module");
            let nodes = module
                .nodes()
                .iter()
                .map(|node| {
                    lower_node(
                        module_id,
                        node,
                        &mut metrics,
                        &mut authored_seed_registry,
                        &mut diagnostics,
                    )
                })
                .collect();
            modules.insert(
                module_id.clone(),
                WorthUiIdentitySeededArtifactInputModule::new(module_id.clone(), nodes),
            );
        }

        let identity_seeded_artifact_input = WorthUiIdentitySeededArtifactInput::new(
            modules,
            bound_artifact_input.module_ids().to_vec(),
        );

        if !diagnostics.is_empty() {
            return Err(WorthUiIdentitySeedingReport::new(diagnostics, metrics));
        }

        Ok((identity_seeded_artifact_input, metrics))
    }
}

fn lower_node(
    module_id: &crate::source::WorthUiSourceModuleId,
    node: &WorthUiBoundArtifactInputNode,
    metrics: &mut WorthUiIdentitySeedingMetrics,
    authored_seed_registry: &mut BTreeMap<String, String>,
    diagnostics: &mut Vec<WorthUiIdentitySeedingDiagnostic>,
) -> WorthUiIdentitySeededArtifactInputNode {
    let identity_seeded_node = match node {
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
        WorthUiBoundArtifactInputNode::Page(page_node) => {
            let seed = page_seed(module_id, page_node);
            let durable = classify_durable_state(page_node.structure());
            metrics.record_seed(
                page_node.authored_identity().is_some(),
                matches!(
                    durable,
                    crate::source::WorthUiDurableStateEligibility::Eligible { .. }
                ),
            );
            WorthUiIdentitySeededArtifactInputNode::Page(
                WorthUiIdentitySeededArtifactInputPageNode::new(page_node.clone(), seed, durable),
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
    };

    register_authored_seed_collision(
        module_id,
        authored_seed_registry,
        diagnostics,
        &identity_seeded_node,
    );

    identity_seeded_node
}

fn register_authored_seed_collision(
    module_id: &crate::source::WorthUiSourceModuleId,
    authored_seed_registry: &mut BTreeMap<String, String>,
    diagnostics: &mut Vec<WorthUiIdentitySeedingDiagnostic>,
    node: &WorthUiIdentitySeededArtifactInputNode,
) {
    let Some((seed_basis, semantic_locus, authored_identity)) = authored_seed_details(node) else {
        return;
    };

    match authored_seed_registry.entry(seed_basis.to_owned()) {
        Entry::Vacant(entry) => {
            entry.insert(semantic_locus.to_owned());
        }
        Entry::Occupied(entry) => {
            diagnostics.push(
                WorthUiIdentitySeedingDiagnostic::duplicate_authored_identity_seed(
                    module_id.clone(),
                    semantic_locus,
                    authored_identity,
                    entry.get().clone(),
                ),
            );
        }
    }
}

fn authored_seed_details(
    node: &WorthUiIdentitySeededArtifactInputNode,
) -> Option<(&str, String, &str)> {
    match node {
        WorthUiIdentitySeededArtifactInputNode::Import(_) => None,
        WorthUiIdentitySeededArtifactInputNode::Page(node) => authored_seed_details_for_kind(
            node.identity_seed().kind(),
            node.identity_seed().basis(),
            format!("page:{}", node.name_text()),
        ),
        WorthUiIdentitySeededArtifactInputNode::Component(node) => authored_seed_details_for_kind(
            node.identity_seed().kind(),
            node.identity_seed().basis(),
            format!("component:{}", node.component().id().as_str()),
        ),
        WorthUiIdentitySeededArtifactInputNode::Surface(node) => authored_seed_details_for_kind(
            node.identity_seed().kind(),
            node.identity_seed().basis(),
            format!("surface:{}", node.surface().id().as_str()),
        ),
        WorthUiIdentitySeededArtifactInputNode::Binding(node) => authored_seed_details_for_kind(
            node.identity_seed().kind(),
            node.identity_seed().basis(),
            format!(
                "binding:{}",
                node.view_binding_reference().view_binding().id().as_str()
            ),
        ),
        WorthUiIdentitySeededArtifactInputNode::Token(node) => authored_seed_details_for_kind(
            node.identity_seed().kind(),
            node.identity_seed().basis(),
            format!("token:{}", node.theme_token().id().as_str()),
        ),
    }
}

fn authored_seed_details_for_kind<'a>(
    seed_kind: &WorthUiArtifactIdentitySeedKind,
    seed_basis: &'a str,
    semantic_locus: String,
) -> Option<(&'a str, String, &'a str)> {
    if seed_kind != &WorthUiArtifactIdentitySeedKind::Authored {
        return None;
    }

    Some((
        seed_basis,
        semantic_locus,
        trailing_authored_identity(seed_basis),
    ))
}

fn trailing_authored_identity(seed_basis: &str) -> &str {
    seed_basis
        .split_once("|authored:")
        .map(|(_, authored_identity)| authored_identity)
        .expect("authored seed basis should include authored marker")
}
