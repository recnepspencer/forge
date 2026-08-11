use crate::identity_evolution::InspectorIdentityClassification;
use crate::live::{BridgeChangeSummary, BridgeFieldDelta};
use crate::view_shape::ViewShapeDescriptor;

use super::super::{
    execute_live_view_shape_change, lower_view_shape_plan_to_live, ViewShapeLiveFailureClass,
};
use super::view_plan_world::{
    detail_canonical, inspector_identity_artifact, planned_view, runtime_basis,
};

#[test]
fn identity_aware_focused_inspector_emits_explicit_identity_artifact() {
    let canonical = detail_canonical();
    let focused_plan = planned_view(
        &canonical,
        ViewShapeDescriptor::identity_aware_inspector_detail_focused(
            worth_foundational::facade::AspectKey::new("profile").unwrap(),
            InspectorIdentityClassification::IdentityBreak,
        ),
    );
    let focused_live = lower_view_shape_plan_to_live(
        &focused_plan,
        runtime_basis(focused_plan.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::IdentityBreak,
        )),
    )
    .unwrap();
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Ada"),
        Some("Ada Lovelace"),
    ));
    let execution = execute_live_view_shape_change(&focused_live, &change).unwrap();
    let crate::view_shape_live::ViewShapePatchPayload::FocusedInspectorAspectPatch(patch) =
        execution.patch_envelope().payload()
    else {
        panic!("expected focused inspector aspect patch");
    };
    let inspector_identity = patch
        .inspector_identity()
        .expect("identity-aware focused inspector should attach identity artifact");

    assert_eq!(
        inspector_identity.classification(),
        InspectorIdentityClassification::IdentityBreak
    );
    assert!(inspector_identity.identity_break());
    assert_eq!(
        focused_plan
            .delivery_metadata()
            .identity_consumption()
            .classification(),
        Some(InspectorIdentityClassification::IdentityBreak)
    );
    assert_eq!(execution.counters().complexity_status_debt_count(), 0);
}

#[test]
fn identity_aware_focused_inspector_requires_matching_identity_binding() {
    let canonical = detail_canonical();
    let focused_plan = planned_view(
        &canonical,
        ViewShapeDescriptor::identity_aware_inspector_detail_focused(
            worth_foundational::facade::AspectKey::new("profile").unwrap(),
            InspectorIdentityClassification::IdentityBreak,
        ),
    );
    let missing_error = lower_view_shape_plan_to_live(
        &focused_plan,
        runtime_basis(focused_plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap_err();
    assert_eq!(
        missing_error.failure_class(),
        &ViewShapeLiveFailureClass::InspectorIdentityBindingRejected
    );

    let mismatch_error = lower_view_shape_plan_to_live(
        &focused_plan,
        runtime_basis(focused_plan.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::AuthoritativeContinuity,
        )),
    )
    .unwrap_err();
    assert_eq!(
        mismatch_error.failure_class(),
        &ViewShapeLiveFailureClass::InspectorIdentityBindingRejected
    );
}

#[test]
fn ordinary_detail_live_lowering_rejects_smuggled_identity_binding() {
    let planned = planned_view(&detail_canonical(), ViewShapeDescriptor::detail());
    let error = lower_view_shape_plan_to_live(
        &planned,
        runtime_basis(planned.validated().query().schema_basis().clone()),
        None,
        Some(inspector_identity_artifact(
            InspectorIdentityClassification::AuthoritativeContinuity,
        )),
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        &ViewShapeLiveFailureClass::InspectorIdentityBindingRejected
    );
}
