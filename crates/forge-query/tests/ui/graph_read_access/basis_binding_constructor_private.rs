use forge_query::facade::runtime::{
    ForgeQueryGraphReadBasisBinding, ForgeQueryGraphReadBasisPosture,
};

fn main() {
    let _ = ForgeQueryGraphReadBasisBinding {
        read_graph_digest: String::new(),
        schema_basis_digest: String::new(),
        posture: ForgeQueryGraphReadBasisPosture::RuntimeCurrent,
    };
}
