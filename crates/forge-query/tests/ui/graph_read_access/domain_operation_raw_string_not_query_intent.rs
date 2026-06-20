use forge_query::facade::runtime::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};
use forge_query::facade::{AspectFieldSelector, AuthoredResultShapeField};

fn main() {
    let _ = forge_query::facade::runtime::ForgeQueryReadBuilder::standalone()
        .local_collection(
            "user",
            QuerySchemaView::new(
                "raw-domain-op",
                [SchemaFieldView::new("identity", "id", SchemaFieldKind::String)],
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
