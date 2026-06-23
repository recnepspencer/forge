use std::collections::BTreeSet;

use crate::source::{
    WorthUiLayoutTopologyChild, WorthUiLayoutTopologyDiagnostic,
    WorthUiLayoutTopologyDiagnosticCode, WorthUiLayoutTopologyNode,
};

pub(super) fn validate_slots_in_node(
    node: &WorthUiLayoutTopologyNode,
    seen: &mut BTreeSet<String>,
    layout_locus: &str,
) -> Vec<WorthUiLayoutTopologyDiagnostic> {
    let mut diagnostics = Vec::new();
    push_slot_denials(node, seen, &mut diagnostics, layout_locus);
    diagnostics
}

fn push_slot_denials(
    node: &WorthUiLayoutTopologyNode,
    seen: &mut BTreeSet<String>,
    diagnostics: &mut Vec<WorthUiLayoutTopologyDiagnostic>,
    layout_locus: &str,
) {
    for child in node.children() {
        match child {
            WorthUiLayoutTopologyChild::Region(child_node) => {
                push_slot_denials(child_node, seen, diagnostics, layout_locus);
            }
            WorthUiLayoutTopologyChild::Slot(slot) => {
                if !seen.insert(slot.slot_name().to_owned()) {
                    diagnostics.push(WorthUiLayoutTopologyDiagnostic::new(
                        WorthUiLayoutTopologyDiagnosticCode::DuplicateLayoutSlot,
                        layout_locus,
                        format!(
                            "layout topology cannot declare slot '{}' more than once",
                            slot.slot_name()
                        ),
                    ));
                }
            }
        }
    }
}
