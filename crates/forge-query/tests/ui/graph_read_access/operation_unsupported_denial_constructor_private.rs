use forge_query::facade::runtime::{
    ForgeQueryGraphReadOperationUnsupportedDenial,
    ForgeQueryGraphReadOperationUnsupportedDenialKind,
};

fn main() {
    let _ = ForgeQueryGraphReadOperationUnsupportedDenial {
        kind: ForgeQueryGraphReadOperationUnsupportedDenialKind::DeniedUnsupportedShape,
        shape_name: String::new(),
        explanation: String::new(),
        read_graph_digest: String::new(),
        matched_relations: Vec::new(),
    };
}

