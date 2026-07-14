use worth_query::facade::runtime::WorthQueryGraphReadAccessRequirementSet;

fn main() {
    let _ = WorthQueryGraphReadAccessRequirementSet {
        digest: unreachable!(),
        read_graph_digest: String::new(),
        access_shape_digest: String::new(),
        selectivity_shape_digest: String::new(),
        rows: Vec::new(),
        counters: unreachable!(),
    };
}
