use worth_store_layout_indexes::{LayoutStrategy, StrategyInvariantEvidence};

fn main() {
    let _ = StrategyInvariantEvidence {
        strategy: LayoutStrategy::BaselineBTree,
    };
}
