use worth_query_declaration::facade::authoring::{
    AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField, DetailQueryBuilder,
    DetailResultShapeBuilder, RootEntityKey,
};
use worth_query_declaration::facade::binding::QueryBindingDescriptor;
use worth_query_declaration::facade::canonicalization::{
    canonicalize_request, CanonicalQueryBundle,
};

pub(super) fn canonical_bundle() -> CanonicalQueryBundle {
    let selector = AspectFieldSelector::new("profile", "name").unwrap();
    let query = DetailQueryBuilder::new(RootEntityKey::new("BridgeFixture").unwrap())
        .project(selector)
        .build()
        .unwrap()
        .into_raw();
    let shape = DetailResultShapeBuilder::new()
        .field(AuthoredResultShapeField::new("profile", "name", "name").unwrap())
        .build()
        .unwrap()
        .into_raw();
    canonicalize_request(
        AuthoredQueryBundleRequest::new(query, shape, QueryBindingDescriptor::new()).unwrap(),
    )
    .unwrap()
}
