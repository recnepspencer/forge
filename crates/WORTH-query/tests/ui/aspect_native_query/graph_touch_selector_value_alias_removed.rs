use worth_query::facade::WorthQueryGraphTouchSelector;

fn main() {
    let selector = WorthQueryGraphTouchSelector::any_graph_touch();
    let _ = selector.selector_value();
}
