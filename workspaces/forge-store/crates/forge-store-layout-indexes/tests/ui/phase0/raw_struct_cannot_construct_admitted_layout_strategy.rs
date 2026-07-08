use forge_store_layout_indexes::S8AdmittedLayoutStrategy;
use forge_store_layout_indexes::S8LayoutStrategyFamily;

fn main() {
    let _ = S8AdmittedLayoutStrategy::new(S8LayoutStrategyFamily::BTree);
}
