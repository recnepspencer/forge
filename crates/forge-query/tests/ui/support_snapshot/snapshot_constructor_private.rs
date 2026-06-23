use forge_query::facade::consumer_kit::{
    ForgeQuerySupportSnapshot, ForgeQuerySupportSnapshotSchemaVersion,
};

fn main() {
    let _ = ForgeQuerySupportSnapshot {
        schema_version: ForgeQuerySupportSnapshotSchemaVersion::current(),
        schema_identity: String::new(),
        backend_posture: String::new(),
        source_matrix_digest: String::new(),
        rows: Vec::new(),
        snapshot_digest: String::new(),
    };
}
