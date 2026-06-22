use forge_query::facade::runtime::ForgeQueryGraphReadAccessRequirementSet;

fn main() {
    let _ = ForgeQueryGraphReadAccessRequirementSet {
        digest: unreachable!(),
        read_graph_digest: String::new(),
        access_shape_digest: String::new(),
        selectivity_shape_digest: String::new(),
        rows: Vec::new(),
        counters: unreachable!(),
    };
}
