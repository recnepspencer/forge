use worth_query::facade::runtime::{WorthQueryGraphReadOperationUnsupportedDenial, WorthQueryGraphReadOperationUnsupportedDenialKind};

fn main() {
    let _ = WorthQueryGraphReadOperationUnsupportedDenial {
        kind: WorthQueryGraphReadOperationUnsupportedDenialKind::DeniedUnsupportedShape,
        shape_name: String::new(),
        explanation: String::new(),
        read_graph_digest: String::new(),
        matched_relations: Vec::new(),
    };
}

