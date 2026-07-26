use std::collections::BTreeMap;

use crate::source::{
    WorthUiArtifact, WorthUiArtifactInspectionBasis, WorthUiArtifactInspectionDiagnostic,
    WorthUiArtifactInspectionMetrics, WorthUiArtifactInspectionReport, WorthUiArtifactNodeKind,
    WorthUiArtifactSourceOrigin, WorthUiIdentitySeededArtifactInput,
    WorthUiIdentitySeededArtifactInputNode,
};

use super::super::artifact_assembly::worth_ui_canonical_node_sort_key;

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiArtifactInspectionBasisBuilder;

impl WorthUiArtifactInspectionBasisBuilder {
    pub(crate) fn build(
        artifact: &WorthUiArtifact,
        identity_seeded_artifact_input: &WorthUiIdentitySeededArtifactInput,
    ) -> Result<WorthUiArtifactInspectionBasis, WorthUiArtifactInspectionReport> {
        let mut diagnostics = Vec::new();
        let mut source_origins = BTreeMap::new();

        for module_id in artifact.module_ids() {
            let Some(artifact_module) = artifact.module(module_id) else {
                continue;
            };
            let Some(identity_seeded_module) = identity_seeded_artifact_input.module(module_id)
            else {
                diagnostics.push(
                    WorthUiArtifactInspectionDiagnostic::artifact_basis_alignment_mismatch(
                        format!("missing identity-seeded module for {}", module_id.as_str()),
                    ),
                );
                continue;
            };

            let mut ordered_seeded_nodes = identity_seeded_module.nodes().to_vec();
            ordered_seeded_nodes.sort_by_key(worth_ui_canonical_node_sort_key);

            if artifact_module.nodes().len() != ordered_seeded_nodes.len() {
                diagnostics.push(
                    WorthUiArtifactInspectionDiagnostic::artifact_basis_alignment_mismatch(
                        format!("node count mismatch for {}", module_id.as_str()),
                    ),
                );
                continue;
            }

            for (artifact_node, seeded_node) in artifact_module
                .nodes()
                .iter()
                .zip(ordered_seeded_nodes.iter())
            {
                if artifact_node.handle().kind() != seeded_node_kind(seeded_node) {
                    diagnostics.push(
                        WorthUiArtifactInspectionDiagnostic::artifact_basis_alignment_mismatch(
                            format!("node kind mismatch for {}", module_id.as_str()),
                        ),
                    );
                    continue;
                }

                let artifact_key = artifact_node_alignment_key(artifact_node);
                let seeded_key = worth_ui_canonical_node_sort_key(seeded_node);
                if artifact_key != seeded_key {
                    diagnostics.push(
                        WorthUiArtifactInspectionDiagnostic::artifact_basis_alignment_mismatch(
                            format!(
                                "node key mismatch for {}: artifact={:?}, seeded={:?}",
                                module_id.as_str(),
                                artifact_key,
                                seeded_key
                            ),
                        ),
                    );
                    continue;
                }

                source_origins.insert(
                    artifact_node.handle().clone(),
                    WorthUiArtifactSourceOrigin::from_provenance(seeded_node_provenance(
                        seeded_node,
                    )),
                );
            }
        }

        if !diagnostics.is_empty() {
            return Err(WorthUiArtifactInspectionReport::new(
                diagnostics,
                WorthUiArtifactInspectionMetrics::default(),
            ));
        }

        Ok(WorthUiArtifactInspectionBasis::new(source_origins))
    }
}

fn artifact_node_alignment_key(node: &crate::source::WorthUiArtifactNode) -> (u8, String, String) {
    match node {
        crate::source::WorthUiArtifactNode::Import(node) => (
            0,
            node.target().authored_text().to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
        crate::source::WorthUiArtifactNode::Component(node) => (
            1,
            node.component().id().as_str().to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
        crate::source::WorthUiArtifactNode::Surface(node) => (
            2,
            node.surface().id().as_str().to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
        crate::source::WorthUiArtifactNode::Binding(node) => (
            3,
            node.view_binding_reference()
                .view_binding()
                .id()
                .as_str()
                .to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
        crate::source::WorthUiArtifactNode::Token(node) => (
            4,
            node.theme_token().id().as_str().to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
    }
}

fn seeded_node_kind(node: &WorthUiIdentitySeededArtifactInputNode) -> WorthUiArtifactNodeKind {
    match node {
        WorthUiIdentitySeededArtifactInputNode::Import(_) => WorthUiArtifactNodeKind::Import,
        WorthUiIdentitySeededArtifactInputNode::Component(_) => WorthUiArtifactNodeKind::Component,
        WorthUiIdentitySeededArtifactInputNode::Surface(_) => WorthUiArtifactNodeKind::Surface,
        WorthUiIdentitySeededArtifactInputNode::Binding(_) => WorthUiArtifactNodeKind::Binding,
        WorthUiIdentitySeededArtifactInputNode::Token(_) => WorthUiArtifactNodeKind::Token,
    }
}

fn seeded_node_provenance(
    node: &WorthUiIdentitySeededArtifactInputNode,
) -> &worth_ui_dsl::WorthUiArtifactInputProvenance {
    match node {
        WorthUiIdentitySeededArtifactInputNode::Import(node) => node.provenance(),
        WorthUiIdentitySeededArtifactInputNode::Component(node) => node.provenance(),
        WorthUiIdentitySeededArtifactInputNode::Surface(node) => node.provenance(),
        WorthUiIdentitySeededArtifactInputNode::Binding(node) => node.provenance(),
        WorthUiIdentitySeededArtifactInputNode::Token(node) => node.provenance(),
    }
}
