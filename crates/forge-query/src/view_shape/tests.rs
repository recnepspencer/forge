use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RootEntityKey,
};
use crate::identity_evolution::InspectorIdentityClassification;
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor, ViewShapeFailureClass, ViewShapeInvalidationPosture,
    ViewShapePatchPosture,
};

fn detail_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "view-shape-detail",
        [
            crate::schema_view::SchemaFieldView::new(
                "identity",
                "id",
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                "profile",
                "display_name",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn collection_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "view-shape-collection",
        [
            crate::schema_view::SchemaFieldView::new(
                "identity",
                "id",
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                "profile",
                "display_name",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
            crate::schema_view::SchemaFieldView::new(
                "status",
                "lane",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn wide_collection_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "view-shape-wide-collection",
        [
            crate::schema_view::SchemaFieldView::new(
                "identity",
                "id",
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                "profile",
                "display_name",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
            crate::schema_view::SchemaFieldView::new(
                "status",
                "lane",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
            crate::schema_view::SchemaFieldView::new(
                "meta",
                "priority",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn direct_detail() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_detail(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::detail_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn direct_collection() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .project(AspectFieldSelector::new("status", "lane").unwrap())
            .order_by(
                crate::authoring::OrderingSelector::ascending("profile", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::collection_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn wide_collection() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .project(AspectFieldSelector::new("status", "lane").unwrap())
            .project(AspectFieldSelector::new("meta", "priority").unwrap())
            .order_by(
                crate::authoring::OrderingSelector::ascending("profile", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::collection_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .field(AuthoredResultShapeField::new("status", "lane", "lane").unwrap())
            .field(AuthoredResultShapeField::new("meta", "priority", "priority").unwrap())
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn basis_intent() -> crate::basis::ExecutionBasisIntent {
    crate::basis::ExecutionBasisIntent::new(
        crate::basis::BasisAuthorityFamily::Runtime,
        crate::basis::SnapshotLineageClass::CurrentHead,
        false,
    )
}

#[test]
fn table_denies_detail_queries() {
    let error = admit_view_shape(&direct_detail(), ViewShapeDescriptor::table()).unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::IncompatibleCanonicalFamily
    );
}

#[test]
fn inspector_denies_collection_queries() {
    let error = admit_view_shape(
        &direct_collection(),
        ViewShapeDescriptor::inspector_detail_observed(),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::IncompatibleCanonicalFamily
    );
}

#[test]
fn focused_inspector_requires_focus_contract() {
    let error = crate::view_shape::admit_view_shape(
        &direct_detail(),
        ViewShapeDescriptor::inspector_detail_focused_missing_for_test(),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::FocusAspectRequired
    );
}

#[test]
fn kanban_grouped_requires_grouping_contract() {
    let error = crate::view_shape::admit_view_shape(
        &direct_collection(),
        ViewShapeDescriptor::kanban_grouped_missing_for_test(),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::GroupingAspectRequired
    );
}

#[test]
fn observed_and_focused_inspector_produce_distinct_plan_metadata() {
    let canonical = direct_detail();
    let observed = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(&canonical, ViewShapeDescriptor::inspector_detail_observed()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let focused = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(
                &canonical,
                ViewShapeDescriptor::inspector_detail_focused("profile"),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    assert_ne!(observed.view_shape_digest(), focused.view_shape_digest());
    assert_ne!(
        observed.view_plan_digest().as_str(),
        focused.view_plan_digest().as_str()
    );
    assert_eq!(
        observed.invalidation_posture(),
        &ViewShapeInvalidationPosture::InspectorObservedNarrowDetail
    );
    assert_eq!(
        focused.invalidation_posture(),
        &ViewShapeInvalidationPosture::InspectorFocusedAspect
    );
    assert_eq!(
        observed.patch_posture(),
        &ViewShapePatchPosture::ObservedInspectorPatch
    );
    assert_eq!(
        focused.patch_posture(),
        &ViewShapePatchPosture::FocusedInspectorAspectPatch
    );
}

#[test]
fn identity_aware_focused_inspector_mints_distinct_digest_and_binding() {
    let canonical = direct_detail();
    let ordinary = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(
                &canonical,
                ViewShapeDescriptor::inspector_detail_focused("profile"),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let identity_aware = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(
                &canonical,
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    "profile",
                    InspectorIdentityClassification::AuthoritativeContinuity,
                ),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    assert_ne!(
        ordinary.view_shape_digest(),
        identity_aware.view_shape_digest()
    );
    assert_ne!(
        ordinary.delivery_metadata().identity_consumption().digest(),
        identity_aware
            .delivery_metadata()
            .identity_consumption()
            .digest()
    );
    assert_eq!(
        identity_aware
            .delivery_metadata()
            .identity_consumption()
            .classification(),
        Some(InspectorIdentityClassification::AuthoritativeContinuity)
    );
}

#[test]
fn identity_consumption_is_rejected_for_non_inspector_views() {
    let error = admit_view_shape(
        &direct_collection(),
        ViewShapeDescriptor::identity_aware_inspector_detail_observed(),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::IncompatibleCanonicalFamily
    );
}

#[test]
fn kanban_grouped_is_admitted_with_explicit_grouping_aspect() {
    let canonical = direct_collection();
    let admitted = admit_view_shape(&canonical, ViewShapeDescriptor::kanban_grouped("status"))
        .expect("grouped collection view should admit with grouping aspect");
    let planned = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            collection_schema_view(),
            admitted,
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    assert_eq!(
        planned.family(),
        crate::view_shape::ViewShapeFamily::KanbanGrouped
    );
    assert_eq!(
        planned.delivery_metadata().grouping_aspect(),
        Some("status")
    );
    assert!(planned.delivery_metadata().grouped_delivery());
    let grouped_policy = planned
        .grouped_delta_policy()
        .expect("grouped plans must carry planner-issued grouped delta policy");
    let grouped_evidence = planned
        .grouped_planning_artifact()
        .expect("grouped plans must carry planner-issued grouped evidence");
    assert_eq!(
        grouped_policy.contract(),
        &crate::view_shape::KanbanGroupedLiveContract::DeltaBound
    );
    assert_eq!(grouped_evidence.grouping_aspect(), "status");
    assert_eq!(grouped_evidence.identity_binding_index(), 0);
    assert_eq!(grouped_evidence.grouping_binding_index(), 2);
    assert_eq!(
        grouped_evidence.identity_binding().field_key(),
        "identity.id"
    );
    assert_eq!(
        grouped_evidence.grouping_binding().field_key(),
        "status.lane"
    );
    assert_eq!(grouped_evidence.grouped_binding_width(), 3);
    assert_eq!(grouped_evidence.grouped_projection_width(), 3);
    assert_eq!(grouped_evidence.ordering_count(), 1);
    assert_eq!(
        planned.patch_posture(),
        &ViewShapePatchPosture::KanbanGroupMembershipPatch
    );
}

#[test]
fn kanban_grouped_wide_surface_degrades_to_refresh_debt() {
    let canonical = wide_collection();
    let admitted = admit_view_shape(&canonical, ViewShapeDescriptor::kanban_grouped("status"))
        .expect("wide grouped collection view should still admit");
    let planned = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            wide_collection_schema_view(),
            admitted,
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    let grouped_policy = planned
        .grouped_delta_policy()
        .expect("grouped plans must carry planner-issued grouped delta policy");
    let grouped_evidence = planned
        .grouped_planning_artifact()
        .expect("grouped plans must carry planner-issued grouped evidence");

    assert_eq!(grouped_evidence.grouped_binding_width(), 4);
    assert_eq!(grouped_evidence.grouped_projection_width(), 4);
    assert_eq!(
        grouped_policy.contract(),
        &crate::view_shape::KanbanGroupedLiveContract::RefreshDeferredDebt
    );
}
