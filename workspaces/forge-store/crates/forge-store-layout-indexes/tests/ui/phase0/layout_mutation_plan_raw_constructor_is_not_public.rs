use forge_store_layout_indexes::layout_rebuild::S8LayoutMutationPlan;

fn main() {
    let _ = S8LayoutMutationPlan {
        admitted_strategy: unsafe { std::mem::zeroed() },
        request: unsafe { std::mem::zeroed() },
    };
}
