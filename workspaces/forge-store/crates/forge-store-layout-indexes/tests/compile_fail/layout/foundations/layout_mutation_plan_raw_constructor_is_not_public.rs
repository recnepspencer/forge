use forge_store_layout_indexes::maintenance::LayoutMutationPlan;

fn main() {
    let _ = LayoutMutationPlan {
        admitted_strategy: unsafe { std::mem::zeroed() },
        request: unsafe { std::mem::zeroed() },
    };
}
