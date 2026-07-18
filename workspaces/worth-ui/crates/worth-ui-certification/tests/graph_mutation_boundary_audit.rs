use worth_ui_certification::topology::audit_graph_mutation_boundary_owns_snapshot_and_index_commit;

fn workspace_root() -> &'static worth_ui_certification::topology::WorkspaceSourceInventory {
    super::workspace_source_inventory()
}

#[test]
fn graph_mutation_boundary_owns_snapshot_and_index_commit() {
    let violations = audit_graph_mutation_boundary_owns_snapshot_and_index_commit(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
