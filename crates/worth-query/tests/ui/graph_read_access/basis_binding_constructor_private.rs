use worth_query::facade::runtime::{WorthQueryGraphReadBasisBinding, WorthQueryGraphReadBasisPosture};

fn main() {
    let _ = WorthQueryGraphReadBasisBinding {
        read_graph_digest: String::new(),
        schema_basis_digest: String::new(),
        posture: WorthQueryGraphReadBasisPosture::RuntimeCurrent,
    };
}
