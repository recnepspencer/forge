use super::*;
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor, ViewShapeFailureClass, ViewShapePatchPosture,
};
use worth_foundational::facade::AspectKey;

#[test]
fn kanban_grouped_requires_grouping_contract() {
    let error = admit_view_shape(
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
fn kanban_grouped_is_admitted_with_explicit_grouping_aspect() {
    let canonical = direct_collection();
    let admitted = admit_view_shape(
        &canonical,
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    )
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

    assert_eq!(planned.family().as_str(), "kanban_grouped");
    assert_eq!(
        planned.delivery_metadata().native_grouping_aspect_key(),
        Some(&aspect_key("status"))
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
    assert_eq!(
        grouped_evidence.native_grouping_aspect_key(),
        &aspect_key("status")
    );
    assert_eq!(grouped_evidence.identity_binding_index(), 0);
    assert_eq!(grouped_evidence.grouping_binding_index(), 2);
    assert_eq!(
        grouped_evidence
            .identity_binding()
            .native_binding_aspect_key(),
        &aspect_key("identity.id")
    );
    assert_eq!(
        grouped_evidence
            .grouping_binding()
            .native_binding_aspect_key(),
        &aspect_key("status.lane")
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
fn kanban_grouped_wide_surface_still_carries_grouped_delta_contract() {
    let canonical = wide_collection();
    let admitted = admit_view_shape(
        &canonical,
        ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
    )
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
        &crate::view_shape::KanbanGroupedLiveContract::DeltaBound
    );
    assert_eq!(grouped_policy.max_member_transitions(), usize::MAX);
    assert_eq!(grouped_policy.max_lane_reassignments(), usize::MAX);
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("test grouped aspect must be foundational")
}
