use std::collections::BTreeMap;

use super::authoring_entry::{validate_authoring_entry, WorthUiAuthoringEntryReport};
use super::worth_ui_parsed_source_declaration_lowerer::lower_parsed_source_declaration;

use crate::source::{
    WorthUiArtifactInput, WorthUiArtifactInputModule, WorthUiArtifactInputNormalizer,
    WorthUiParsedSourceDeclaration, WorthUiParsedSourcePackage,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiParsedSourceToArtifactInputLowerer;

impl WorthUiParsedSourceToArtifactInputLowerer {
    pub(crate) fn lower(
        parsed_source_package: &WorthUiParsedSourcePackage,
    ) -> Result<WorthUiArtifactInput, WorthUiAuthoringEntryReport> {
        validate_authoring_entry(parsed_source_package)?;
        let mut modules = BTreeMap::new();
        let canonical_module_order = parsed_source_package.module_ids().to_vec();

        for module_id in parsed_source_package.module_ids() {
            let parsed_module = parsed_source_package
                .module(module_id)
                .expect("parsed source package should contain every canonical module");
            let nodes = parsed_module
                .declarations()
                .iter()
                .filter(|declaration| should_emit_ir_node(declaration))
                .map(lower_parsed_source_declaration)
                .collect();
            modules.insert(
                module_id.clone(),
                WorthUiArtifactInputModule::new(module_id.clone(), nodes),
            );
        }

        Ok(WorthUiArtifactInputNormalizer::normalize(
            WorthUiArtifactInput::new(modules, canonical_module_order),
        ))
    }
}

fn should_emit_ir_node(declaration: &WorthUiParsedSourceDeclaration) -> bool {
    matches!(
        declaration,
        WorthUiParsedSourceDeclaration::Import(_)
            | WorthUiParsedSourceDeclaration::Component(_)
            | WorthUiParsedSourceDeclaration::Surface(_)
            | WorthUiParsedSourceDeclaration::Binding(_)
            | WorthUiParsedSourceDeclaration::Token(_)
    )
}
