use worth_query::facade::runtime::{QuerySchemaView, ScalarAspectType, SchemaFieldView, SchemaRelationView};

fn main() {
    let schema = QuerySchemaView::new(
        "schema",
        [SchemaFieldView::new(worth_query::facade::foundation::AspectName::new("identity").expect("schema aspect literal must be valid"), worth_query::facade::foundation::FieldName::new("id").expect("schema field literal must be valid"), ScalarAspectType::String)],
        [SchemaRelationView::new(worth_query::facade::foundation::RelationName::new("manager").expect("schema relation literal must be valid"), 1)],
    );
    let field = SchemaFieldView::new(worth_query::facade::foundation::AspectName::new("identity").expect("schema aspect literal must be valid"), worth_query::facade::foundation::FieldName::new("id").expect("schema field literal must be valid"), ScalarAspectType::String);
    let relation = SchemaRelationView::new(worth_query::facade::foundation::RelationName::new("manager").expect("schema relation literal must be valid"), 1);

    let _ = schema.field("identity", "id");
    let _ = schema.has_aspect("identity");
    let _ = schema.relation("manager");
    let _ = field.aspect();
    let _ = field.field();
    let _ = relation.relation();
}
