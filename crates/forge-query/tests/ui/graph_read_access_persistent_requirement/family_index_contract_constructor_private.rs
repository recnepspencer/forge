use forge_query::facade::runtime::ForgeQueryGraphReadFamilyIndexContract;

fn main() {
    let _ = ForgeQueryGraphReadFamilyIndexContract {
        digest: String::new(),
        read_graph_digest: String::new(),
        access_shape_digest: String::new(),
        selectivity_shape_digest: String::new(),
        requirement_set_digest: String::new(),
        persistent_requirement_digest: None,
        requirement_row_digests: Vec::new(),
    };
}
