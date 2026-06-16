use std::collections::{BTreeMap, BTreeSet};

use crate::source::{
    WorthUiArtifactInputBodyAtom, WorthUiAuthoringEntryDiagnosticCode, WorthUiAuthoringEntryReport,
    WorthUiLayoutAxis, WorthUiLayoutTopologyChild, WorthUiLayoutTopologyNode,
    WorthUiParsedPageDeclaration,
};

use super::worth_ui_content_slotting_diagnostic::page_content_slotting_diagnostic;

const COLUMN_REGION_ID: &str = "worth.ui.layout.column";
const ROW_REGION_ID: &str = "worth.ui.layout.row";
const SLOT_REGION_ID: &str = "worth.ui.layout.slot";

pub(super) fn compose_slot_structure_body_atoms(
    page: &WorthUiParsedPageDeclaration,
    root: &WorthUiLayoutTopologyNode,
    content: &BTreeMap<String, String>,
) -> Result<Vec<WorthUiArtifactInputBodyAtom>, WorthUiAuthoringEntryReport> {
    let mut used_slots = BTreeSet::new();
    let mut atoms = Vec::new();
    push_region_atoms(root, content, &mut used_slots, &mut atoms);

    let layout_slots = collect_layout_slots(root);
    let diagnostics = slot_coverage_diagnostics(page, content, &layout_slots, &used_slots);
    if diagnostics.is_empty() {
        Ok(atoms)
    } else {
        Err(WorthUiAuthoringEntryReport::new(diagnostics))
    }
}

fn push_region_atoms(
    node: &WorthUiLayoutTopologyNode,
    content: &BTreeMap<String, String>,
    used_slots: &mut BTreeSet<String>,
    atoms: &mut Vec<WorthUiArtifactInputBodyAtom>,
) {
    push_ident(atoms, "region");
    push_ident(atoms, region_id_for_axis(node.axis()));
    atoms.push(WorthUiArtifactInputBodyAtom::LeftBrace);
    for child in node.children() {
        match child {
            WorthUiLayoutTopologyChild::Region(region) => {
                push_region_atoms(region, content, used_slots, atoms);
            }
            WorthUiLayoutTopologyChild::Slot(slot) => {
                push_slot_region_atoms(slot.slot_name(), content, used_slots, atoms);
            }
        }
    }
    atoms.push(WorthUiArtifactInputBodyAtom::RightBrace);
}

fn push_slot_region_atoms(
    slot_name: &str,
    content: &BTreeMap<String, String>,
    used_slots: &mut BTreeSet<String>,
    atoms: &mut Vec<WorthUiArtifactInputBodyAtom>,
) {
    push_ident(atoms, "region");
    push_ident(atoms, SLOT_REGION_ID);
    atoms.push(WorthUiArtifactInputBodyAtom::LeftBrace);
    if let Some(surface_name) = content.get(slot_name) {
        used_slots.insert(slot_name.to_owned());
        push_ident(atoms, "mount");
        push_ident(atoms, surface_name);
        atoms.push(WorthUiArtifactInputBodyAtom::Semicolon);
    }
    atoms.push(WorthUiArtifactInputBodyAtom::RightBrace);
}

fn slot_coverage_diagnostics(
    page: &WorthUiParsedPageDeclaration,
    content: &BTreeMap<String, String>,
    layout_slots: &BTreeSet<String>,
    used_slots: &BTreeSet<String>,
) -> Vec<crate::source::WorthUiAuthoringEntryDiagnostic> {
    let mut diagnostics = Vec::new();
    for slot in layout_slots.difference(used_slots) {
        diagnostics.push(page_content_slotting_diagnostic(
            WorthUiAuthoringEntryDiagnosticCode::MissingContentSlotAssignment,
            page,
            format!(
                "page '{}' content does not fill slot '{slot}'",
                page.name_text()
            ),
        ));
    }
    for slot in content.keys() {
        if !layout_slots.contains(slot) {
            diagnostics.push(page_content_slotting_diagnostic(
                WorthUiAuthoringEntryDiagnosticCode::UnknownContentSlotAssignment,
                page,
                format!(
                    "page '{}' content targets unknown slot '{slot}'",
                    page.name_text()
                ),
            ));
        }
    }
    diagnostics
}

fn collect_layout_slots(root: &WorthUiLayoutTopologyNode) -> BTreeSet<String> {
    let mut slots = BTreeSet::new();
    collect_region_slots(root, &mut slots);
    slots
}

fn collect_region_slots(node: &WorthUiLayoutTopologyNode, slots: &mut BTreeSet<String>) {
    for child in node.children() {
        match child {
            WorthUiLayoutTopologyChild::Region(region) => collect_region_slots(region, slots),
            WorthUiLayoutTopologyChild::Slot(slot) => {
                slots.insert(slot.slot_name().to_owned());
            }
        }
    }
}

fn push_ident(atoms: &mut Vec<WorthUiArtifactInputBodyAtom>, text: &str) {
    atoms.push(WorthUiArtifactInputBodyAtom::Identifier(text.to_owned()));
}

fn region_id_for_axis(axis: &WorthUiLayoutAxis) -> &'static str {
    match axis {
        WorthUiLayoutAxis::Column => COLUMN_REGION_ID,
        WorthUiLayoutAxis::Row => ROW_REGION_ID,
    }
}
