use std::collections::{BTreeMap, BTreeSet};

use crate::declaration::stable_text_digest;
use crate::source::{
    WorthUiArtifact, WorthUiArtifactAssemblyDiagnostic, WorthUiArtifactAssemblyMetrics,
    WorthUiArtifactAssemblyReport, WorthUiArtifactBindingHandle, WorthUiArtifactBindingNode,
    WorthUiArtifactComponentHandle, WorthUiArtifactComponentNode, WorthUiArtifactHandle,
    WorthUiArtifactImportHandle, WorthUiArtifactImportNode, WorthUiArtifactModule,
    WorthUiArtifactNode, WorthUiArtifactSurfaceHandle, WorthUiArtifactSurfaceNode,
    WorthUiArtifactThemeTokenHandle, WorthUiArtifactThemeTokenNode,
    WorthUiIdentitySeededArtifactInput, WorthUiIdentitySeededArtifactInputNode,
    WorthUiSourceModuleId,
};

use super::{
    worth_ui_canonical_node_key, worth_ui_canonical_node_sort_key, worth_ui_semantic_locus,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiCanonicalArtifactAssembler;

impl WorthUiCanonicalArtifactAssembler {
    pub(crate) fn assemble(
        identity_seeded_artifact_input: &WorthUiIdentitySeededArtifactInput,
    ) -> Result<WorthUiArtifact, WorthUiArtifactAssemblyReport> {
        Self::assemble_with_metrics(identity_seeded_artifact_input).map(|(artifact, _)| artifact)
    }

    pub(crate) fn assemble_with_metrics(
        identity_seeded_artifact_input: &WorthUiIdentitySeededArtifactInput,
    ) -> Result<(WorthUiArtifact, WorthUiArtifactAssemblyMetrics), WorthUiArtifactAssemblyReport>
    {
        let mut modules = BTreeMap::new();
        let mut metrics = WorthUiArtifactAssemblyMetrics::default();
        let mut diagnostics = Vec::new();

        for module_id in identity_seeded_artifact_input.module_ids() {
            metrics.record_module_assembled();
            let module = identity_seeded_artifact_input
                .module(module_id)
                .expect("identity-seeded artifact input should contain every canonical module");
            let nodes =
                assemble_module_nodes(module_id, module.nodes(), &mut metrics, &mut diagnostics);
            modules.insert(
                module_id.clone(),
                WorthUiArtifactModule::new(module_id.clone(), nodes),
            );
        }

        if !diagnostics.is_empty() {
            return Err(WorthUiArtifactAssemblyReport::new(diagnostics, metrics));
        }

        Ok((
            WorthUiArtifact::new(
                modules,
                identity_seeded_artifact_input.module_ids().to_vec(),
            ),
            metrics,
        ))
    }
}

fn assemble_module_nodes(
    module_id: &WorthUiSourceModuleId,
    nodes: &[WorthUiIdentitySeededArtifactInputNode],
    metrics: &mut WorthUiArtifactAssemblyMetrics,
    diagnostics: &mut Vec<WorthUiArtifactAssemblyDiagnostic>,
) -> Vec<WorthUiArtifactNode> {
    let original_keys = node_order_keys(nodes.iter());
    let mut ordered_nodes = nodes.iter().collect::<Vec<_>>();
    ordered_nodes.sort_by_key(|node| worth_ui_canonical_node_sort_key(node));

    if original_keys != node_order_keys(ordered_nodes.iter().copied()) {
        metrics.record_module_with_reordered_nodes();
    }

    let mut seen_node_keys = BTreeSet::new();
    let mut assembled_nodes = Vec::with_capacity(ordered_nodes.len());

    for (node_index, node) in ordered_nodes.into_iter().enumerate() {
        metrics.record_node_assembled();
        let node_key = worth_ui_canonical_node_key(node);

        if !seen_node_keys.insert(node_key.clone()) {
            diagnostics.push(
                WorthUiArtifactAssemblyDiagnostic::duplicate_canonical_artifact_node_key(
                    module_id.clone(),
                    worth_ui_semantic_locus(node),
                    node_key,
                ),
            );
        }

        assembled_nodes.push(assemble_node(module_id, node_index, node));
    }

    assembled_nodes
}

fn assemble_node(
    module_id: &WorthUiSourceModuleId,
    node_index: usize,
    node: &WorthUiIdentitySeededArtifactInputNode,
) -> WorthUiArtifactNode {
    match node {
        WorthUiIdentitySeededArtifactInputNode::Import(node) => {
            WorthUiArtifactNode::Import(WorthUiArtifactImportNode::new(
                WorthUiArtifactHandle::Import(WorthUiArtifactImportHandle::new(
                    module_id.clone(),
                    node_index,
                )),
                node.target().clone(),
                authored_provenance_digest(node.provenance()),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
        WorthUiIdentitySeededArtifactInputNode::Component(node) => {
            WorthUiArtifactNode::Component(WorthUiArtifactComponentNode::new(
                WorthUiArtifactHandle::Component(WorthUiArtifactComponentHandle::new(
                    module_id.clone(),
                    node_index,
                )),
                node.component().clone(),
                node.bound_node().descriptor().clone(),
                node.bound_node().structure().clone(),
                authored_provenance_digest(node.provenance()),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
        WorthUiIdentitySeededArtifactInputNode::Surface(node) => {
            WorthUiArtifactNode::Surface(WorthUiArtifactSurfaceNode::new(
                WorthUiArtifactHandle::Surface(WorthUiArtifactSurfaceHandle::new(
                    module_id.clone(),
                    node_index,
                )),
                node.surface().clone(),
                node.bound_node().descriptor().clone(),
                node.bound_node().structure().clone(),
                node.bound_node().semantics().clone(),
                authored_provenance_digest(node.provenance()),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
        WorthUiIdentitySeededArtifactInputNode::Binding(node) => {
            WorthUiArtifactNode::Binding(WorthUiArtifactBindingNode::new(
                WorthUiArtifactHandle::Binding(WorthUiArtifactBindingHandle::new(
                    module_id.clone(),
                    node_index,
                )),
                node.view_binding_reference().clone(),
                node.bound_node().structure().clone(),
                authored_provenance_digest(node.provenance()),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
        WorthUiIdentitySeededArtifactInputNode::Token(node) => {
            WorthUiArtifactNode::Token(WorthUiArtifactThemeTokenNode::new(
                WorthUiArtifactHandle::Token(WorthUiArtifactThemeTokenHandle::new(
                    module_id.clone(),
                    node_index,
                )),
                node.theme_token().clone(),
                node.bound_node().entry().clone(),
                node.bound_node().semantics().clone(),
                authored_provenance_digest(node.provenance()),
                node.identity_seed().clone(),
                node.durable_state_eligibility().clone(),
            ))
        }
    }
}

fn authored_provenance_digest(provenance: &crate::source::WorthUiArtifactInputProvenance) -> u64 {
    stable_text_digest(provenance.module_path())
        ^ (provenance.declaration_index() as u64).rotate_left(13)
}

fn node_order_keys<'a>(
    nodes: impl IntoIterator<Item = &'a WorthUiIdentitySeededArtifactInputNode>,
) -> Vec<String> {
    nodes.into_iter().map(worth_ui_canonical_node_key).collect()
}
