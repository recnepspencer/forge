use worth_query::facade::consumer_kit::{
    WorthQuerySupportSnapshot, WorthQuerySupportSnapshotSchemaVersion,
};

fn main() {
    let _ = WorthQuerySupportSnapshot {
        schema_version: WorthQuerySupportSnapshotSchemaVersion::current(),
        schema_identity: String::new(),
        backend_posture: String::new(),
        source_matrix_digest: String::new(),
        rows: Vec::new(),
        snapshot_digest: String::new(),
    };
}
