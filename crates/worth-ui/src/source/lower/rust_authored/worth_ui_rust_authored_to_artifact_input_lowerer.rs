use std::collections::BTreeMap;
use std::path::Path;

use crate::source::{
    WorthUiArtifactInput, WorthUiArtifactInputBlockNode, WorthUiArtifactInputImportNode,
    WorthUiArtifactInputModule, WorthUiArtifactInputNode, WorthUiArtifactInputNormalizer,
    WorthUiArtifactInputProvenance, WorthUiArtifactInputReference, WorthUiArtifactInputTokenNode,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiSourceModuleId,
};

use super::worth_ui_rust_authored_artifact_input_module::WorthUiRustAuthoredDeclaration;

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiRustAuthoredToArtifactInputLowerer;

impl WorthUiRustAuthoredToArtifactInputLowerer {
    pub(crate) fn lower(
        rust_authored_input: &WorthUiRustAuthoredArtifactInput,
    ) -> WorthUiArtifactInput {
        let mut modules = BTreeMap::new();
        let mut canonical_module_order = Vec::new();

        for module in rust_authored_input.modules() {
            let module_id =
                WorthUiSourceModuleId::from_relative_path(Path::new(module.relative_module_path()))
                    .expect("rust-authored module path should be valid");
            let nodes = lower_rust_authored_module(module);
            canonical_module_order.push(module_id.clone());
            let previous_module = modules.insert(
                module_id.clone(),
                WorthUiArtifactInputModule::new(module_id, nodes),
            );
            assert!(
                previous_module.is_none(),
                "rust-authored module identity must be unique after canonicalization"
            );
        }

        WorthUiArtifactInputNormalizer::normalize(WorthUiArtifactInput::new(
            modules,
            canonical_module_order,
        ))
    }
}

fn lower_rust_authored_module(
    module: &WorthUiRustAuthoredArtifactInputModule,
) -> Vec<WorthUiArtifactInputNode> {
    module
        .declarations()
        .iter()
        .enumerate()
        .map(|(declaration_index, declaration)| {
            let provenance = WorthUiArtifactInputProvenance::rust_authored(
                module.relative_module_path(),
                declaration_index,
            );
            match declaration {
                WorthUiRustAuthoredDeclaration::Import { target_module_path } => {
                    WorthUiArtifactInputNode::Import(WorthUiArtifactInputImportNode::new(
                        WorthUiArtifactInputReference::new(target_module_path),
                        provenance,
                    ))
                }
                WorthUiRustAuthoredDeclaration::Component {
                    name_text,
                    authored_identity,
                    body_atoms,
                } => WorthUiArtifactInputNode::Component(WorthUiArtifactInputBlockNode::new(
                    name_text,
                    authored_identity.clone(),
                    body_atoms.clone(),
                    provenance,
                )),
                WorthUiRustAuthoredDeclaration::Surface {
                    name_text,
                    authored_identity,
                    body_atoms,
                } => WorthUiArtifactInputNode::Surface(WorthUiArtifactInputBlockNode::new(
                    name_text,
                    authored_identity.clone(),
                    body_atoms.clone(),
                    provenance,
                )),
                WorthUiRustAuthoredDeclaration::Binding {
                    name_text,
                    authored_identity,
                    body_atoms,
                } => WorthUiArtifactInputNode::Binding(WorthUiArtifactInputBlockNode::new(
                    name_text,
                    authored_identity.clone(),
                    body_atoms.clone(),
                    provenance,
                )),
                WorthUiRustAuthoredDeclaration::Token {
                    name_text,
                    authored_identity,
                    value_text,
                } => WorthUiArtifactInputNode::Token(WorthUiArtifactInputTokenNode::new(
                    name_text,
                    authored_identity.clone(),
                    value_text,
                    provenance,
                )),
            }
        })
        .collect()
}
