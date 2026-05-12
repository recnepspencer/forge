use super::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, ForgeQueryLiveViewBuilder,
    ForgeQueryWorkspaceLiveViewDeclaration, QuerySchemaView,
};
use crate::authoring::TraversalSelector;
use crate::declarative_live::DeclarativeProjectionField;
use crate::schema_view::{SchemaFieldKind, SchemaFieldView};

#[test]
fn traversal_relation_declarations_reject_zero_depth() {
    let error = ForgeQueryLiveViewBuilder::surface("runtime.zero-depth")
        .from("WorthTopologyEntity")
        .allow_traversal_relation("HalfEdgeNext", 0)
        .build()
        .expect_err("zero-depth traversal relations must fail early");

    assert!(error
        .to_string()
        .contains("must declare a non-zero max depth"));
}

#[test]
fn traversal_relation_declarations_reject_duplicates() {
    let error = ForgeQueryLiveViewBuilder::surface("runtime.duplicate-relation")
        .from("WorthTopologyEntity")
        .allow_traversal_relation("HalfEdgeNext", 2)
        .allow_traversal_relation("HalfEdgeNext", 4)
        .build()
        .expect_err("duplicate traversal relations must fail early");

    assert!(error
        .to_string()
        .contains("may only be declared once per live view"));
}

#[test]
fn traversal_relation_declarations_lower_into_request_and_schema_view() {
    let declaration = ForgeQueryLiveViewBuilder::surface("runtime.traversal-lowered")
        .from("WorthTopologyEntity")
        .select(["identity.id"])
        .allow_traversal_relation("HalfEdgeNext", 2)
        .build()
        .expect("declared traversal relations should lower into the request");

    assert_eq!(declaration.request().traversal().len(), 1);
    assert_eq!(
        declaration.request().traversal()[0].relation(),
        "HalfEdgeNext"
    );
    assert_eq!(declaration.request().traversal()[0].depth(), 2);
    let relation = declaration
        .schema_view()
        .relation("HalfEdgeNext")
        .expect("schema view should retain the declared traversal relation");
    assert_eq!(relation.relation(), "HalfEdgeNext");
    assert_eq!(relation.max_depth(), 2);
}

#[test]
fn direct_live_view_declaration_rejects_traversal_schema_mismatch() {
    let request =
        DeclarativeLiveQueryRequest::new("WorthTopologyEntity", DeclarativeLiveViewShape::detail())
            .project(DeclarativeProjectionField::new("identity", "id"))
            .traverse(TraversalSelector::bounded("HalfEdgeNext", 2).unwrap());
    let schema_view = QuerySchemaView::new(
        "runtime.traversal-mismatch",
        [SchemaFieldView::new(
            "identity",
            "id",
            SchemaFieldKind::String,
        )],
        [],
    );

    let error = ForgeQueryWorkspaceLiveViewDeclaration::try_from_request(request, schema_view)
        .expect_err("direct request/schema pairs must reject undeclared traversal relations");

    assert!(error.to_string().contains("TraversalNotDeclaredInSchema"));
}
