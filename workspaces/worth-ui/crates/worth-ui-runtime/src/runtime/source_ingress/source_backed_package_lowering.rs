use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken, WorthUiDslPackage,
};

use crate::source::{
    WorthUiLegallyStructuredArtifactInput, WorthUiLegallyStructuredArtifactInputBindingNode,
    WorthUiLegallyStructuredArtifactInputNode,
};

pub(super) fn source_backed_package(
    structured: &WorthUiLegallyStructuredArtifactInput,
) -> WorthUiDslPackage {
    let package_name = source_backed_package_name(structured);
    structured.module_ids().iter().fold(
        WorthUiDslPackage::named(package_name),
        |package, module_id| {
            let Some(module) = structured.module(module_id) else {
                return package;
            };
            module
                .nodes()
                .iter()
                .filter_map(source_backed_semantic_spec)
                .fold(package, WorthUiDslPackage::with_semantic_artifact_spec)
        },
    )
}

fn source_backed_package_name(structured: &WorthUiLegallyStructuredArtifactInput) -> String {
    let primary_module_name = structured
        .module_ids()
        .first()
        .map(|module_id| sanitize_source_text(module_id.as_str()))
        .unwrap_or_else(|| "empty".to_owned());
    format!("worth-ui.runtime.source-ingress.{primary_module_name}")
}

fn source_backed_semantic_spec(
    node: &WorthUiLegallyStructuredArtifactInputNode,
) -> Option<UiDslSemanticArtifactSpec> {
    match node {
        WorthUiLegallyStructuredArtifactInputNode::Component(node) => Some(mosaic_semantic_spec(
            format!("component:{}", node.descriptor().id().as_str()),
            node.provenance().module_path(),
            node.provenance().declaration_index(),
            source_backed_structural_identity(
                "component",
                node.authored_identity(),
                node.descriptor().id().as_str(),
            ),
        )),
        WorthUiLegallyStructuredArtifactInputNode::Surface(node) => Some(mosaic_semantic_spec(
            format!("surface:{}", node.descriptor().id().as_str()),
            node.provenance().module_path(),
            node.provenance().declaration_index(),
            source_backed_structural_identity(
                "surface",
                node.authored_identity(),
                node.descriptor().id().as_str(),
            ),
        )),
        WorthUiLegallyStructuredArtifactInputNode::Binding(node) => {
            Some(mosaic_binding_semantic_spec(node))
        }
        WorthUiLegallyStructuredArtifactInputNode::Import(_)
        | WorthUiLegallyStructuredArtifactInputNode::Token(_) => None,
    }
}

fn mosaic_binding_semantic_spec(
    node: &WorthUiLegallyStructuredArtifactInputBindingNode,
) -> UiDslSemanticArtifactSpec {
    mosaic_semantic_spec(
        format!("binding:{}", node.view_binding().id().as_str()),
        node.provenance().module_path(),
        node.provenance().declaration_index(),
        source_backed_structural_identity(
            "binding",
            node.authored_identity(),
            node.view_binding().id().as_str(),
        ),
    )
}

fn mosaic_semantic_spec(
    key_basis: String,
    module_path: &str,
    declaration_index: usize,
    structural_identity: String,
) -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new(key_basis),
        UiDslSemanticFamily::Mosaic,
        UiDslSourceProvenance::file_authored(module_path, declaration_index),
    )
    .with_structural_token(UiDslStructuralToken::new(format!(
        "mosaic:{}|{}",
        module_path, structural_identity
    )))
}

fn source_backed_structural_identity(
    family: &str,
    authored_identity: Option<&str>,
    fallback_identity: &str,
) -> String {
    match authored_identity {
        Some(authored_identity) => format!("{family}:authored:{authored_identity}"),
        None => format!("{family}:identity:{fallback_identity}"),
    }
}

fn sanitize_source_text(text: &str) -> String {
    text.chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => character,
            _ => '.',
        })
        .collect()
}
