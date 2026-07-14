use worth_query::facade::runtime::ValidatedTraversalEntry;

fn main() {
    let entry = traversal_entry_fixture();
    let _ = entry.terminal_relation_projection();
}

fn traversal_entry_fixture() -> ValidatedTraversalEntry {
    panic!("fixture only")
}
