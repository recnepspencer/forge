mod admitted;
mod normalization;
mod rejection;

use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RootEntityKey,
};

fn canonical_predicate_bundle(
    configure: impl FnOnce(crate::authoring::DetailQueryBuilder) -> crate::authoring::DetailQueryBuilder,
) -> crate::facade::CanonicalQueryBundle {
    let root = RootEntityKey::new("user").expect("root should build");
    let query = configure(
        crate::authoring::DetailQueryBuilder::new(root)
            .project(AspectFieldSelector::new("identity", "id").expect("projection should build")),
    )
    .build()
    .expect("query should build");
    let shape = crate::authoring::DetailResultShapeBuilder::new()
        .field(
            AuthoredResultShapeField::new("identity", "id", "id")
                .expect("shape field should build"),
        )
        .build()
        .expect("shape should build");
    GuidedAuthoringPath::canonicalize_detail(query, shape).expect("bundle should canonicalize")
}
