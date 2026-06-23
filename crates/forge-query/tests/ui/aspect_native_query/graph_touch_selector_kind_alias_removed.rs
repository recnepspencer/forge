use forge_query::facade::ForgeQueryGraphTouchSelector;

fn main() {
    let selector = ForgeQueryGraphTouchSelector::any_graph_touch();
    let _ = selector.selector_kind();
}
