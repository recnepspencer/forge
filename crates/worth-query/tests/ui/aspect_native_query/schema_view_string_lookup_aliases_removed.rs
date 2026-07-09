use worth_query::facade::{QuerySchemaView, SchemaFieldKind, SchemaFieldView, SchemaRelationView};

fn main() {
    let schema = QuerySchemaView::new(
        "schema",
        [SchemaFieldView::new(worth_query::facade::AspectName::new("identity").expect("schema aspect literal must be valid"), worth_query::facade::FieldName::new("id").expect("schema field literal must be valid"), SchemaFieldKind::String)],
        [SchemaRelationView::new(worth_query::facade::RelationName::new("manager").expect("schema relation literal must be valid"), 1)],
    );
    let field = SchemaFieldView::new(worth_query::facade::AspectName::new("identity").expect("schema aspect literal must be valid"), worth_query::facade::FieldName::new("id").expect("schema field literal must be valid"), SchemaFieldKind::String);
    let relation = SchemaRelationView::new(worth_query::facade::RelationName::new("manager").expect("schema relation literal must be valid"), 1);

    let _ = schema.field("identity", "id");
    let _ = schema.has_aspect("identity");
    let _ = schema.relation("manager");
    let _ = field.aspect();
    let _ = field.field();
    let _ = relation.relation();
}
