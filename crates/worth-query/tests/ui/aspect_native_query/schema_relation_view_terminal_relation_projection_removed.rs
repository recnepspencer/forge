use worth_query::facade::SchemaRelationView;

fn main() {
    let relation = SchemaRelationView::new(worth_query::facade::RelationName::new("manager").expect("schema relation literal must be valid"), 2);
    let _ = relation.terminal_relation_projection();
}
