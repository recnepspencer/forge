use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, OrderingSelector,
    QueryFamily, RootEntityKey, TraversalSelector,
};
use crate::composition::{GuidedCompositionPath, QueryCompositionFamily, QueryScopeDescriptor};

fn named_scope_expansion_base_collection_query() -> crate::authoring::CollectionAuthoredQuery {
    crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap()
}

fn named_scope_expansion_collection_shape() -> crate::authoring::CollectionAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap()
}

fn named_scope_expansion_identity_only_collection_shape(
) -> crate::authoring::CollectionAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap()
}

fn assert_collection_scope_parity(
    base_query: crate::authoring::CollectionAuthoredQuery,
    direct_query: crate::authoring::CollectionAuthoredQuery,
    result_shape: crate::authoring::CollectionAuthoredResultShape,
    scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    expected_scope_count: usize,
    expected_scope_width: usize,
) {
    let direct =
        GuidedAuthoringPath::canonicalize_collection(direct_query, result_shape.clone()).unwrap();
    let (artifact, expanded) =
        GuidedCompositionPath::expand_collection_scopes(base_query, result_shape, scopes).unwrap();
    let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();

    assert_eq!(artifact.query_family(), QueryFamily::Collection);
    assert_eq!(
        direct.query().digest(),
        composed.canonical().query().digest()
    );
    assert_eq!(
        direct.result_shape().digest(),
        composed.canonical().result_shape().digest()
    );
    assert_eq!(
        composed.composition().family(),
        QueryCompositionFamily::NamedScopeExpansion
    );
    assert_eq!(
        composed.composition().scope_lineage_digest(),
        Some(artifact.scope_lineage_digest())
    );
    assert_ne!(artifact.scope_lineage_digest().as_str(), "");
    assert_ne!(
        composed.composition().composition_digest().as_str(),
        artifact.scope_lineage_digest().as_str(),
        "composition digest should not collapse to raw lineage alone"
    );
    assert_eq!(
        composed.composition().counters().scope_expansion_count(),
        expected_scope_count
    );
    assert_eq!(
        composed.composition().counters().scope_expansion_width(),
        expected_scope_width
    );
    assert_eq!(
        composed.composition().counters().scope_rediscovery_count(),
        0
    );
}

#[test]
fn named_scope_expansion_preserves_projection_scope_parity() {
    let direct_query =
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap();

    assert_collection_scope_parity(
        named_scope_expansion_base_collection_query(),
        direct_query,
        named_scope_expansion_collection_shape(),
        [QueryScopeDescriptor::projection(
            "display_name_projection",
            [AspectFieldSelector::new("profile", "display_name").unwrap()],
        )],
        1,
        1,
    );
}

#[test]
fn named_scope_expansion_preserves_ordering_scope_parity() {
    let direct_query =
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
            .build()
            .unwrap();

    assert_collection_scope_parity(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .build()
            .unwrap(),
        direct_query,
        named_scope_expansion_identity_only_collection_shape(),
        [QueryScopeDescriptor::ordering(
            "name_first",
            [OrderingSelector::ascending("profile", "display_name").unwrap()],
        )],
        1,
        1,
    );
}

#[test]
fn named_scope_expansion_preserves_traversal_scope_parity() {
    let direct_query =
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .traverse(TraversalSelector::bounded("manager", 1).unwrap())
            .build()
            .unwrap();

    assert_collection_scope_parity(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .build()
            .unwrap(),
        direct_query,
        named_scope_expansion_identity_only_collection_shape(),
        [QueryScopeDescriptor::traversal_bound(
            "manager_hop",
            1,
            [TraversalSelector::bounded("manager", 1).unwrap()],
        )],
        1,
        1,
    );
}

#[test]
fn named_scope_expansion_preserves_projection_ordering_and_traversal_parity() {
    let direct_query =
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
            .traverse(TraversalSelector::bounded("manager", 1).unwrap())
            .build()
            .unwrap();

    assert_collection_scope_parity(
        named_scope_expansion_base_collection_query(),
        direct_query,
        named_scope_expansion_collection_shape(),
        [
            QueryScopeDescriptor::projection(
                "display_name_projection",
                [AspectFieldSelector::new("profile", "display_name").unwrap()],
            ),
            QueryScopeDescriptor::ordering(
                "name_first",
                [OrderingSelector::ascending("profile", "display_name").unwrap()],
            ),
            QueryScopeDescriptor::traversal_bound(
                "manager_hop",
                1,
                [TraversalSelector::bounded("manager", 1).unwrap()],
            ),
        ],
        3,
        3,
    );
}
