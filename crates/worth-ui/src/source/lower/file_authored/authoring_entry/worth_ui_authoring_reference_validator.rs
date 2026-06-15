use crate::source::WorthUiParsedSourcePackage;

use super::{
    worth_ui_authoring_hierarchy_validator::validate_authoring_hierarchy,
    worth_ui_authoring_page_validator::validate_page_sections,
    worth_ui_authoring_symbol_table::WorthUiAuthoringSymbolTable, WorthUiAuthoringEntryReport,
};

pub(crate) fn validate_authoring_entry(
    parsed_package: &WorthUiParsedSourcePackage,
) -> Result<(), WorthUiAuthoringEntryReport> {
    let table = WorthUiAuthoringSymbolTable::build(parsed_package)?;
    if !table.has_authoring_roots() {
        return Ok(());
    }

    let mut diagnostics = Vec::new();
    let Some(referenced_pages) = validate_authoring_hierarchy(&table, &mut diagnostics) else {
        return Err(WorthUiAuthoringEntryReport::new(diagnostics));
    };

    for (name, page) in table.pages() {
        if !referenced_pages.contains(*name) {
            diagnostics.push(super::WorthUiAuthoringEntryDiagnostic::new(
                super::WorthUiAuthoringEntryDiagnosticCode::UnownedPageDeclaration,
                format!("page '{name}' is not referenced by the declared workspace"),
                page.declaration.span().clone(),
            ));
        }
        validate_page_sections(page.declaration, &table, &mut diagnostics);
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(WorthUiAuthoringEntryReport::new(diagnostics))
    }
}
