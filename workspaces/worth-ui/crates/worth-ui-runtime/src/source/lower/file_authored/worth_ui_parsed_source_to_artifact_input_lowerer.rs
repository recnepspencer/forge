use std::collections::BTreeMap;

use super::worth_ui_parsed_source_declaration_lowerer::lower_parsed_source_declaration;

use crate::source::{
    WorthUiArtifactInput, WorthUiArtifactInputModule, WorthUiArtifactInputNormalizer,
    WorthUiParsedSourcePackage,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiParsedSourceToArtifactInputLowerer;

impl WorthUiParsedSourceToArtifactInputLowerer {
    pub(crate) fn lower(
        parsed_source_package: &WorthUiParsedSourcePackage,
    ) -> WorthUiArtifactInput {
        let mut modules = BTreeMap::new();
        let canonical_module_order = parsed_source_package.module_ids().to_vec();

        for module_id in parsed_source_package.module_ids() {
            let parsed_module = parsed_source_package
                .module(module_id)
                .expect("parsed source package should contain every canonical module");
            let nodes = parsed_module
                .declarations()
                .iter()
                .enumerate()
                .map(|(declaration_index, declaration)| {
                    lower_parsed_source_declaration(declaration, declaration_index)
                })
                .collect();
            modules.insert(
                module_id.clone(),
                WorthUiArtifactInputModule::new(module_id.clone(), nodes),
            );
        }

        WorthUiArtifactInputNormalizer::normalize(WorthUiArtifactInput::new(
            modules,
            canonical_module_order,
        ))
    }
}
