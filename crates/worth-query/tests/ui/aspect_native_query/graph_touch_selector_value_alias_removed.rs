use worth_query::facade::runtime::WorthQueryGraphTouchSelector;

fn main() {
    let selector = WorthQueryGraphTouchSelector::any_graph_touch();
    let _ = selector.selector_value();
}
