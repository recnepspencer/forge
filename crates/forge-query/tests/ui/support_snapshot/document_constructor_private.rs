use forge_query::facade::consumer_kit::ForgeQuerySupportSnapshotDocument;

fn main() {
    let _ = ForgeQuerySupportSnapshotDocument {
        schema_version: 1,
        schema_identity: String::new(),
        backend_posture: String::new(),
        source_matrix_digest: String::new(),
        snapshot_digest: String::new(),
        rows: Vec::new(),
    };
}
