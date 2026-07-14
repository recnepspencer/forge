use forge_store_layout_indexes::LayoutMutationPlan;

fn main() {
    let _ = LayoutMutationPlan {
        family: panic!("private fields prevent raw plan construction"),
        maintenance_mode: panic!("private fields prevent raw plan construction"),
        mutation_shape: panic!("private fields prevent raw plan construction"),
        kind: panic!("private fields prevent raw plan construction"),
    };
}
