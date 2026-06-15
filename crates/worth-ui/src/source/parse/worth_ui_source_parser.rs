use std::collections::BTreeMap;

use super::worth_ui_source_declaration_parser::parse_module_declarations;
use super::worth_ui_source_tokenizer::tokenize_module_source;

use crate::source::{
    WorthUiParseReport, WorthUiParsedSourceModule, WorthUiParsedSourcePackage,
    WorthUiSourceModuleRecord, WorthUiSourcePackage,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiSourceParser;

impl WorthUiSourceParser {
    pub(crate) fn parse_package(
        source_package: &WorthUiSourcePackage,
    ) -> Result<WorthUiParsedSourcePackage, WorthUiParseReport> {
        let mut diagnostics = Vec::new();
        let mut modules = BTreeMap::new();

        for module_id in source_package.module_ids() {
            let module_record = source_package
                .module_record(module_id)
                .expect("canonical source package should contain every module record");
            match parse_source_module(module_record) {
                Ok(parsed_module) => {
                    modules.insert(module_id.clone(), parsed_module);
                }
                Err(mut module_diagnostics) => diagnostics.append(&mut module_diagnostics),
            }
        }

        if !diagnostics.is_empty() {
            return Err(WorthUiParseReport::new(diagnostics));
        }

        Ok(WorthUiParsedSourcePackage::new(
            modules,
            source_package.module_ids().to_vec(),
        ))
    }
}

fn parse_source_module(
    module_record: &WorthUiSourceModuleRecord,
) -> Result<WorthUiParsedSourceModule, Vec<crate::source::WorthUiParseDiagnostic>> {
    let tokens = tokenize_module_source(module_record)?;
    let declarations = parse_module_declarations(
        module_record.module_id(),
        module_record.source_text().len(),
        tokens,
    )?;
    Ok(WorthUiParsedSourceModule::new(
        module_record.module_id().clone(),
        declarations,
    ))
}
