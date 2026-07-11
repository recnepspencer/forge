use forge_store_layout_indexes::layout_strategy_admission::{
    S8LsmCompactionPlan, S8LsmRunGeneration,
};

fn main() {
    let _forged = S8LsmCompactionPlan::new(
        S8LsmRunGeneration::new(1),
        S8LsmRunGeneration::new(2),
        S8LsmRunGeneration::new(3),
    );
}
