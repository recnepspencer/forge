use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RootEntityKey,
};
use crate::canonicalization::CanonicalQueryBundle;
use crate::harness::fixtures::schema_view::detail_schema_view;
use crate::validation::{validate_canonical_bundle, QueryValidationError};

pub(super) fn canonical_bundle_with_projection(aspect: &str, field: &str) -> CanonicalQueryBundle {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = crate::authoring::DetailQueryBuilder::new(root)
        .project(AspectFieldSelector::new(aspect, field).expect("projection should build"))
        .build()
        .expect("query should build");
    let shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new(aspect, field, "value")
                .expect("shape field should build"),
        )
        .build()
        .expect("shape should build");

    GuidedAuthoringPath::canonicalize_detail(query, shape).expect("bundle should canonicalize")
}

pub(super) fn assert_rejects_with(
    bundle: CanonicalQueryBundle,
    expected: QueryValidationError,
    message: &str,
) {
    let error = validate_canonical_bundle(bundle, detail_schema_view()).expect_err(message);
    assert_eq!(error, expected);
}
