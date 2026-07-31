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
    ) -> Result<WorthUiArtifactInput, crate::source::WorthUiDslCompileReport> {
        let mut modules = BTreeMap::new();
        let canonical_module_order = parsed_source_package.module_ids().to_vec();
        let mut diagnostics = Vec::new();

        for module_id in parsed_source_package.module_ids() {
            let parsed_module = parsed_source_package
                .module(module_id)
                .expect("parsed source package should contain every canonical module");
            let nodes =
                parsed_module
                    .declarations()
                    .iter()
                    .enumerate()
                    .filter_map(|(declaration_index, declaration)| {
                        match lower_parsed_source_declaration(declaration, declaration_index) {
                            Ok(node) => Some(node),
                            Err(diagnostic) => {
                                diagnostics.push(diagnostic);
                                None
                            }
                        }
                    })
                    .collect();
            modules.insert(
                module_id.clone(),
                WorthUiArtifactInputModule::new(module_id.clone(), nodes),
            );
        }

        if !diagnostics.is_empty() {
            return Err(crate::source::WorthUiDslCompileReport::new(diagnostics));
        }
        Ok(WorthUiArtifactInputNormalizer::normalize(
            WorthUiArtifactInput::new(modules, canonical_module_order),
        ))
    }
}
