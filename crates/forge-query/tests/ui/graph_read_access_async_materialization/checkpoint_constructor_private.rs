use forge_query::facade::runtime::ForgeQueryGraphReadMaterializationCheckpoint;

fn main() {
    let _ = ForgeQueryGraphReadMaterializationCheckpoint {
        digest: String::new(),
        request_digest: String::new(),
        sequence: 0,
        touched_edges: 0,
        emitted_rows: 0,
        resident_bytes: 0,
    };
}
