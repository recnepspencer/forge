use crate::source::{
    validate_layout_topology_tokens, WorthUiArtifactInputNode, WorthUiArtifactInputPageNode,
    WorthUiArtifactInputProvenance, WorthUiAuthoringEntryDiagnosticCode,
    WorthUiAuthoringEntryReport, WorthUiLayoutTopologyReport, WorthUiParsedPageDeclaration,
};

use super::super::authoring_entry::WorthUiAuthoringSymbolTable;
use super::worth_ui_content_assignment_parser::parse_content_assignments;
use super::worth_ui_content_slotting_diagnostic::page_content_slotting_diagnostic;
use super::worth_ui_page_section_tokens::{page_section_tokens, PageSectionKind};
use super::worth_ui_slot_structure_body_atom_composer::compose_slot_structure_body_atoms;

pub(crate) fn compose_page_structure_node(
    page: &WorthUiParsedPageDeclaration,
    table: &WorthUiAuthoringSymbolTable<'_>,
) -> Result<WorthUiArtifactInputNode, WorthUiAuthoringEntryReport> {
    let layout_tokens = page_section_tokens(page, table, PageSectionKind::Layout)?;
    let content_tokens = page_section_tokens(page, table, PageSectionKind::Content)?;
    let layout_root = validate_layout_topology_tokens(layout_tokens.as_slice(), page.name_text())
        .map_err(|report| map_layout_report(page, report))?;
    let content = parse_content_assignments(page, content_tokens.as_slice())?;
    let body_atoms = compose_slot_structure_body_atoms(page, &layout_root, &content)?;

    Ok(WorthUiArtifactInputNode::Page(
        WorthUiArtifactInputPageNode::new(
            page.name_text(),
            page.template_parameters()
                .iter()
                .map(|parameter| {
                    (
                        parameter.name_text().to_owned(),
                        parameter.type_text().to_owned(),
                    )
                })
                .collect(),
            None,
            body_atoms,
            WorthUiArtifactInputProvenance::parsed_source(page.span().clone(), None),
        ),
    ))
}

fn map_layout_report(
    page: &WorthUiParsedPageDeclaration,
    report: WorthUiLayoutTopologyReport,
) -> WorthUiAuthoringEntryReport {
    let message = report
        .diagnostics()
        .first()
        .map(|diagnostic| diagnostic.message().to_owned())
        .unwrap_or_else(|| "layout topology is invalid".to_owned());
    WorthUiAuthoringEntryReport::new(vec![page_content_slotting_diagnostic(
        WorthUiAuthoringEntryDiagnosticCode::InvalidLayoutTopology,
        page,
        message,
    )])
}
