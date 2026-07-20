use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RootEntityKey,
    TraversalSelector,
};
use crate::validation::{validate_canonical_bundle, QueryValidationError};

#[test]
fn illegal_traversal_depth_rejects() {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new("identity", "id").expect("projection should build"))
        .traverse(TraversalSelector::bounded("manager", 2).expect("traversal should build"))
        .build()
        .expect("query should build");
    let shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("shape field should build"),
        )
        .build()
        .expect("shape should build");
    let bundle =
        GuidedAuthoringPath::canonicalize_detail(query, shape).expect("bundle should canonicalize");

    let error = validate_canonical_bundle(
        bundle,
        crate::harness::fixtures::schema_view::detail_schema_view(),
    )
    .expect_err("illegal traversal depth should reject");

    assert_eq!(
        error,
        QueryValidationError::IllegalTraversalDepth {
            relation: "manager".to_string(),
            requested_depth: 2,
            max_depth: 1,
        }
    );
}
