use forge_query::facade::TraversalSelector;

fn main() {
    let traversal = TraversalSelector::bounded("manager", 1).unwrap();
    let _ = traversal.relation();
}
