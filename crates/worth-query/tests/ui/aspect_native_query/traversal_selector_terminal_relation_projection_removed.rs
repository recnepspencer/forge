use worth_query::facade::foundation::TraversalSelector;

fn main() {
    let traversal = TraversalSelector::bounded("manager", 2).unwrap();
    let _ = traversal.terminal_relation_projection();
}
