use std::path::Path;

use worth_ui_certification::topology::audit_graph_mutation_boundary_owns_snapshot_and_index_commit;

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root")
}

#[test]
fn graph_mutation_boundary_owns_snapshot_and_index_commit() {
    let violations = audit_graph_mutation_boundary_owns_snapshot_and_index_commit(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}
