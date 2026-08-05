use super::*;

pub(super) fn detail_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "milestone-eight-detail",
        [
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("identity")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("id").expect("schema field literal must be valid"),
                crate::schema_view::ScalarAspectType::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("profile")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("display_name")
                    .expect("schema field literal must be valid"),
                crate::schema_view::ScalarAspectType::String,
            )
            .text_predicate_queryable(),
            crate::schema_view::SchemaFieldView::new(
                crate::authoring::AspectName::new("status")
                    .expect("schema aspect literal must be valid"),
                crate::authoring::FieldName::new("lane")
                    .expect("schema field literal must be valid"),
                crate::schema_view::ScalarAspectType::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

pub(super) fn collection_schema_view() -> crate::schema_view::QuerySchemaView {
    detail_schema_view()
}

pub(super) fn basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

pub(super) fn runtime_basis(
    schema_basis: crate::identity::SchemaBasisDigest,
) -> crate::basis::ResolvedSnapshotBasis {
    resolve_snapshot_basis(
        basis_intent(),
        ResolvedSnapshotIdentity::new(
            BasisAuthorityFamily::Runtime,
            None,
            WorthQuerySnapshotIdentity::from_relational_snapshot(milestone_eight_snapshot_parts())
                .evidence_identity(),
            schema_basis,
            SnapshotLineageClass::CurrentHead,
        ),
        BasisResolutionMode::RuntimeDirect,
    )
    .unwrap()
}

pub(super) fn detail_query_with_name_filter(name: &str) -> crate::authoring::DetailAuthoredQuery {
    crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .where_equal(
            EqualityPredicate::new(
                "profile",
                "display_name",
                WorthQueryPredicateOperand::string(name.to_string()),
            )
            .unwrap(),
        )
        .build()
        .unwrap()
}

pub(super) fn detail_shape() -> crate::authoring::DetailAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap()
}

pub(super) fn collection_query() -> crate::authoring::CollectionAuthoredQuery {
    crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("status", "lane").unwrap())
        .order_by(OrderingSelector::ascending("profile", "display_name").unwrap())
        .build()
        .unwrap()
}

pub(super) fn collection_shape() -> crate::authoring::CollectionAuthoredResultShape {
    crate::authoring::RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
        .build()
        .unwrap()
}

pub(super) fn direct_detail_canonical(name: &str) -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_detail(detail_query_with_name_filter(name), detail_shape())
        .unwrap()
}

pub(super) fn direct_collection_canonical() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(collection_query(), collection_shape()).unwrap()
}

pub(super) fn view_plan(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
    schema_view: crate::schema_view::QuerySchemaView,
    descriptor: ViewShapeDescriptor,
) -> crate::view_shape::ViewShapePlanArtifact {
    let admitted = admit_view_shape(canonical, descriptor).unwrap();
    let validated =
        validate_canonical_bundle_for_admitted_view_shape(canonical, schema_view, admitted)
            .unwrap();
    plan_admitted_view_shape(validated, basis_intent()).unwrap()
}
