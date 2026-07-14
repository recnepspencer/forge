use worth_query::facade::runtime::WorthQueryGraphReadFamilyIndexContract;

fn main() {
    let _ = WorthQueryGraphReadFamilyIndexContract {
        digest: String::new(),
        read_graph_digest: String::new(),
        access_shape_digest: String::new(),
        selectivity_shape_digest: String::new(),
        requirement_set_digest: String::new(),
        persistent_requirement_digest: None,
        requirement_row_digests: Vec::new(),
    };
}
