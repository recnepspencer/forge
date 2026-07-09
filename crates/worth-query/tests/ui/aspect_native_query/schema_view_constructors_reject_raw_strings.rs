use worth_query::facade::{SchemaFieldKind, SchemaFieldView, SchemaRelationView};

fn main() {
    let _field = SchemaFieldView::new("identity", "id", SchemaFieldKind::String);
    let _relation = SchemaRelationView::new("manager", 1);
}
