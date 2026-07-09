use worth_query::facade::runtime::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use worth_query::facade::{AspectFieldSelector, AuthoredResultShapeField};

fn main() {
    let _ = worth_query::facade::runtime::WorthQueryReadBuilder::standalone()
        .local_collection(
            "user",
            QuerySchemaView::new(
                "raw-domain-op",
                [SchemaFieldView::new(worth_query::facade::AspectName::new("identity").expect("schema aspect literal must be valid"), worth_query::facade::FieldName::new("id").expect("schema field literal must be valid"), SchemaFieldKind::String)],
                [],
            ),
            |query| {
                query
                    .domain_graph_operation("worth.geometry.visible_face_neighborhood")
                    .project(AspectFieldSelector::new("identity", "id").unwrap())
            },
            |shape| {
                shape.field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            },
        )
        .unwrap();
}
