use forge_store_physical_isolation::admit_physical_isolation_entry;

struct SemanticVisibilitySnapshotToken {
    snapshot_id: u64,
}

fn main() {
    let semantic_snapshot = SemanticVisibilitySnapshotToken { snapshot_id: 1 };
    let _ = admit_physical_isolation_entry(semantic_snapshot);
}
