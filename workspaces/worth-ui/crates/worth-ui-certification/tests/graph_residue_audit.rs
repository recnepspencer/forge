use worth_ui_certification::topology::{
    audit_phase5_graph_lookup_lane_does_not_reopen_declaration_source,
    audit_phase5_graph_lookup_lane_is_indexed_not_scan_first,
    audit_phase6_aspect_lookup_lane_does_not_reopen_declaration_source,
    audit_phase6_aspect_lookup_lane_is_indexed_not_scan_first,
};

fn workspace_root() -> &'static worth_ui_certification::topology::WorkspaceSourceInventory {
    super::workspace_source_inventory()
}

#[test]
fn phase5_graph_lookup_lane_does_not_reopen_declaration_source() {
    let violations =
        audit_phase5_graph_lookup_lane_does_not_reopen_declaration_source(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn phase5_graph_lookup_lane_is_indexed_not_scan_first() {
    let violations = audit_phase5_graph_lookup_lane_is_indexed_not_scan_first(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn phase6_aspect_lookup_lane_does_not_reopen_declaration_source() {
    let violations =
        audit_phase6_aspect_lookup_lane_does_not_reopen_declaration_source(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn phase6_aspect_lookup_lane_is_indexed_not_scan_first() {
    let violations = audit_phase6_aspect_lookup_lane_is_indexed_not_scan_first(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
