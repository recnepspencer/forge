use worth_query::facade::runtime::{ScalarAspectType, SchemaFieldView, SchemaRelationView};

fn main() {
    let _field = SchemaFieldView::new("identity", "id", ScalarAspectType::String);
    let _relation = SchemaRelationView::new("manager", 1);
}
