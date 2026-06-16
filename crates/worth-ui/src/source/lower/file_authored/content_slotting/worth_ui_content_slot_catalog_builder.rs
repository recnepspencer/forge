use std::collections::BTreeMap;

use crate::source::{
    WorthUiAuthoringEntryReport, WorthUiContentSlotAssignment, WorthUiContentSlotCatalog,
    WorthUiLayoutTopologyCatalog, WorthUiLayoutTopologyChild, WorthUiLayoutTopologyNode,
    WorthUiPageContentSlots, WorthUiParsedSourcePackage,
};

use super::super::authoring_entry::WorthUiAuthoringSymbolTable;
use super::worth_ui_content_assignment_parser::parse_content_assignments;
use super::worth_ui_page_section_tokens::{page_section_tokens, PageSectionKind};

pub(crate) fn build_content_slot_catalog(
    parsed_package: &WorthUiParsedSourcePackage,
    layout_topology: &WorthUiLayoutTopologyCatalog,
) -> Result<WorthUiContentSlotCatalog, WorthUiAuthoringEntryReport> {
    let table = WorthUiAuthoringSymbolTable::build(parsed_package)?;
    let mut pages = Vec::new();

    for layout_page in layout_topology.pages() {
        let page = table
            .pages()
            .get(layout_page.page_name())
            .expect("layout topology catalog should only contain validated pages")
            .declaration;
        let content_tokens = page_section_tokens(page, &table, PageSectionKind::Content)?;
        let content = parse_content_assignments(page, content_tokens.as_slice())?;
        pages.push(WorthUiPageContentSlots::from_prepared_assignments(
            layout_page.page_name(),
            ordered_content_assignments(layout_page.root(), &content),
        ));
    }

    Ok(WorthUiContentSlotCatalog::from_prepared_pages(pages))
}

fn ordered_content_assignments(
    root: &WorthUiLayoutTopologyNode,
    content: &BTreeMap<String, String>,
) -> Vec<WorthUiContentSlotAssignment> {
    let mut assignments = Vec::new();
    collect_ordered_content_assignments(root, content, &mut assignments);
    assignments
}

fn collect_ordered_content_assignments(
    node: &WorthUiLayoutTopologyNode,
    content: &BTreeMap<String, String>,
    assignments: &mut Vec<WorthUiContentSlotAssignment>,
) {
    for child in node.children() {
        match child {
            WorthUiLayoutTopologyChild::Region(region) => {
                collect_ordered_content_assignments(region, content, assignments);
            }
            WorthUiLayoutTopologyChild::Slot(slot) => {
                let surface_id = content
                    .get(slot.slot_name())
                    .expect("authoring validation should fill every layout slot");
                assignments.push(WorthUiContentSlotAssignment::from_prepared_mount(
                    slot.slot_name(),
                    surface_id,
                ));
            }
        }
    }
}
