use std::collections::BTreeMap;

use crate::source::{
    WorthUiArtifactInput, WorthUiArtifactInputModule, WorthUiArtifactInputNode,
    WorthUiArtifactInputNodeKind,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiArtifactInputNormalizer;

impl WorthUiArtifactInputNormalizer {
    pub(crate) fn normalize(artifact_input: WorthUiArtifactInput) -> WorthUiArtifactInput {
        let mut modules = BTreeMap::new();
        let mut canonical_module_order = artifact_input.module_ids().to_vec();
        canonical_module_order.sort();

        for module_id in &canonical_module_order {
            let module = artifact_input
                .module(module_id)
                .expect("artifact input should contain every canonical module");
            let mut normalized_nodes = module.nodes().to_vec();
            normalized_nodes.sort_by_key(node_normalization_key);
            modules.insert(
                module_id.clone(),
                WorthUiArtifactInputModule::new(module_id.clone(), normalized_nodes),
            );
        }

        WorthUiArtifactInput::new(modules, canonical_module_order)
    }
}

fn node_normalization_key(
    node: &WorthUiArtifactInputNode,
) -> (WorthUiArtifactInputNodeKind, String) {
    let detail = match node {
        WorthUiArtifactInputNode::Import(import_node) => {
            format!("import:{}", import_node.target().authored_text())
        }
        WorthUiArtifactInputNode::Page(page_node) => {
            format!(
                "page:{}:{}:{}",
                page_node.name_text(),
                template_parameters_key(page_node.template_parameters()),
                body_atoms_key(page_node.body_atoms())
            )
        }
        WorthUiArtifactInputNode::Component(block_node)
        | WorthUiArtifactInputNode::Surface(block_node)
        | WorthUiArtifactInputNode::Binding(block_node) => {
            format!(
                "block:{}:{}",
                block_node.name_text(),
                body_atoms_key(block_node.body_atoms())
            )
        }
        WorthUiArtifactInputNode::Token(token_node) => {
            format!(
                "token:{}:{}",
                token_node.name_text(),
                token_node.value_text()
            )
        }
    };
    (node.kind(), detail)
}

fn template_parameters_key(template_parameters: &[(String, String)]) -> String {
    template_parameters
        .iter()
        .map(|(name, kind)| format!("{name}:{kind}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn body_atoms_key(body_atoms: &[crate::source::WorthUiArtifactInputBodyAtom]) -> String {
    body_atoms
        .iter()
        .map(body_atom_key)
        .collect::<Vec<_>>()
        .join("|")
}

fn body_atom_key(body_atom: &crate::source::WorthUiArtifactInputBodyAtom) -> String {
    match body_atom {
        crate::source::WorthUiArtifactInputBodyAtom::Identifier(text) => format!("id:{text}"),
        crate::source::WorthUiArtifactInputBodyAtom::NumberLiteral(value) => {
            format!("num:{value}")
        }
        crate::source::WorthUiArtifactInputBodyAtom::StringLiteral(text) => format!("str:{text}"),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordImport => "kw:import".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordApp => "kw:app".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordWorkspace => "kw:workspace".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordPage => "kw:page".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordRuntime => "kw:runtime".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordLayout => "kw:layout".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordContent => "kw:content".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordAppearance => {
            "kw:appearance".to_owned()
        }
        crate::source::WorthUiArtifactInputBodyAtom::KeywordComponent => "kw:component".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordSurface => "kw:surface".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordBinding => "kw:binding".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::KeywordToken => "kw:token".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::LeftBrace => "{".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::RightBrace => "}".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::LeftParen => "(".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::RightParen => ")".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::LeftBracket => "[".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::RightBracket => "]".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::Comma => ",".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::Colon => ":".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::Semicolon => ";".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::Equals => "=".to_owned(),
        crate::source::WorthUiArtifactInputBodyAtom::Arrow => "->".to_owned(),
    }
}
