use crate::facade::{validate_canonical_bundle, ValidatedQueryBundle};

pub fn runtime_detail_bundle() -> ValidatedQueryBundle {
    validate_canonical_bundle(
        super::canonical_bundles::runtime_detail_bundle(),
        super::schema_view::detail_schema_view(),
    )
    .unwrap()
}

pub fn runtime_bound_detail_bundle() -> ValidatedQueryBundle {
    validate_canonical_bundle(
        super::canonical_bundles::runtime_bound_detail_bundle(),
        super::schema_view::detail_schema_view(),
    )
    .unwrap()
}

pub fn legal_detail_bundle() -> ValidatedQueryBundle {
    validate_canonical_bundle(
        super::canonical_bundles::legal_detail_bundle(),
        super::schema_view::detail_schema_view(),
    )
    .unwrap()
}

pub fn structured_content_bundle() -> ValidatedQueryBundle {
    validate_canonical_bundle(
        super::canonical_bundles::legal_structured_content_bundle(),
        super::schema_view::structured_content_queryable_schema_view(),
    )
    .unwrap()
}

pub fn workflow_bundle() -> ValidatedQueryBundle {
    validate_canonical_bundle(
        super::canonical_bundles::legal_workflow_predicate_bundle(),
        super::schema_view::workflow_queryable_schema_view(),
    )
    .unwrap()
}

pub fn ordered_collection_bundle() -> ValidatedQueryBundle {
    let query = crate::authoring::RawAuthoredQuery::collection_builder(
        crate::facade::RootEntityKey::new("user").unwrap(),
    )
    .project(crate::facade::AspectFieldSelector::new("identity", "id").unwrap())
    .project(crate::facade::AspectFieldSelector::new("profile", "display_name").unwrap())
    .order_by(crate::facade::OrderingSelector::ascending("profile", "display_name").unwrap())
    .traverse(crate::facade::TraversalSelector::bounded("manager", 1).unwrap())
    .build()
    .unwrap();
    let shape = crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(crate::facade::AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(
            crate::facade::AuthoredResultShapeField::new(
                "profile",
                "display_name",
                "display_name",
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    let request = crate::facade::GuidedAuthoringPath::pair_collection(query, shape).unwrap();
    let canonical = crate::facade::canonicalize_request(request).unwrap();
    validate_canonical_bundle(canonical, super::schema_view::detail_schema_view()).unwrap()
}

pub fn descending_collection_bundle() -> ValidatedQueryBundle {
    let query = crate::authoring::RawAuthoredQuery::collection_builder(
        crate::facade::RootEntityKey::new("user").unwrap(),
    )
    .project(crate::facade::AspectFieldSelector::new("identity", "id").unwrap())
    .project(crate::facade::AspectFieldSelector::new("profile", "display_name").unwrap())
    .order_by(crate::facade::OrderingSelector::descending("profile", "display_name").unwrap())
    .traverse(crate::facade::TraversalSelector::bounded("manager", 1).unwrap())
    .build()
    .unwrap();
    let shape = crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(crate::facade::AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(
            crate::facade::AuthoredResultShapeField::new(
                "profile",
                "display_name",
                "display_name",
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    let request = crate::facade::GuidedAuthoringPath::pair_collection(query, shape).unwrap();
    let canonical = crate::facade::canonicalize_request(request).unwrap();
    validate_canonical_bundle(canonical, super::schema_view::detail_schema_view()).unwrap()
}

pub fn unordered_collection_bundle() -> ValidatedQueryBundle {
    let query = crate::authoring::RawAuthoredQuery::collection_builder(
        crate::facade::RootEntityKey::new("user").unwrap(),
    )
    .project(crate::facade::AspectFieldSelector::new("identity", "id").unwrap())
    .project(crate::facade::AspectFieldSelector::new("profile", "display_name").unwrap())
    .traverse(crate::facade::TraversalSelector::bounded("manager", 1).unwrap())
    .build()
    .unwrap();
    let shape = crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(crate::facade::AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(
            crate::facade::AuthoredResultShapeField::new(
                "profile",
                "display_name",
                "display_name",
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    let request = crate::facade::GuidedAuthoringPath::pair_collection(query, shape).unwrap();
    let canonical = crate::facade::canonicalize_request(request).unwrap();
    validate_canonical_bundle(canonical, super::schema_view::detail_schema_view()).unwrap()
}

pub fn multi_order_collection_bundle() -> ValidatedQueryBundle {
    let query = crate::authoring::RawAuthoredQuery::collection_builder(
        crate::facade::RootEntityKey::new("user").unwrap(),
    )
    .project(crate::facade::AspectFieldSelector::new("identity", "id").unwrap())
    .project(crate::facade::AspectFieldSelector::new("profile", "display_name").unwrap())
    .order_by(crate::facade::OrderingSelector::ascending("profile", "display_name").unwrap())
    .order_by(crate::facade::OrderingSelector::ascending("identity", "id").unwrap())
    .traverse(crate::facade::TraversalSelector::bounded("manager", 1).unwrap())
    .build()
    .unwrap();
    let shape = crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(crate::facade::AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(
            crate::facade::AuthoredResultShapeField::new(
                "profile",
                "display_name",
                "display_name",
            )
            .unwrap(),
        )
        .build()
        .unwrap();
    let request = crate::facade::GuidedAuthoringPath::pair_collection(query, shape).unwrap();
    let canonical = crate::facade::canonicalize_request(request).unwrap();
    validate_canonical_bundle(canonical, super::schema_view::detail_schema_view()).unwrap()
}
