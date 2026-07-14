use worth_query::facade::runtime::WorthQueryGraphReadMaterializationCheckpoint;

fn main() {
    let _ = WorthQueryGraphReadMaterializationCheckpoint {
        digest: String::new(),
        request_digest: String::new(),
        sequence: 0,
        touched_edges: 0,
        emitted_rows: 0,
        resident_bytes: 0,
    };
}
