use crate::live::{BridgeChangeSummary, BridgeFieldDelta};
use crate::view_shape::ViewShapeDescriptor;

use super::super::{
    execute_live_view_shape_change, lower_view_shape_plan_to_live, ViewShapeLiveFailureClass,
    ViewShapePatchFamily,
};
use super::view_plan_world::{collection_canonical, detail_canonical, planned_view, runtime_basis};

#[test]
fn table_live_lowering_emits_table_row_patch() {
    let planned = planned_view(&collection_canonical(), ViewShapeDescriptor::table());
    let live = lower_view_shape_plan_to_live(
        &planned,
        runtime_basis(planned.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(
        &live,
        &BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Ada"),
            Some("Ada Lovelace"),
        )),
    )
    .unwrap();

    assert_eq!(
        execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::TableRowPatch)
    );
    assert_eq!(execution.counters().table_ordering_key_count(), 1);
    assert_eq!(execution.counters().complexity_status_debt_count(), 0);
    assert_eq!(
        execution.counters().view_shape_executor_rediscovery_count(),
        0
    );
}

#[test]
fn detail_live_lowering_emits_detail_field_patch() {
    let planned = planned_view(&detail_canonical(), ViewShapeDescriptor::detail());
    let live = lower_view_shape_plan_to_live(
        &planned,
        runtime_basis(planned.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let execution = execute_live_view_shape_change(
        &live,
        &BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Ada"),
            Some("Ada Lovelace"),
        )),
    )
    .unwrap();

    assert_eq!(
        execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::DetailFieldPatch)
    );
    assert_eq!(execution.counters().complexity_status_debt_count(), 0);
}

#[test]
fn observed_and_focused_inspector_emit_distinct_live_patches() {
    let canonical = detail_canonical();
    let observed_plan = planned_view(&canonical, ViewShapeDescriptor::inspector_detail_observed());
    let observed_live = lower_view_shape_plan_to_live(
        &observed_plan,
        runtime_basis(observed_plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let focused_plan = planned_view(
        &canonical,
        ViewShapeDescriptor::inspector_detail_focused(
            worth_foundational::facade::AspectKey::new("profile").unwrap(),
        ),
    );
    let focused_live = lower_view_shape_plan_to_live(
        &focused_plan,
        runtime_basis(focused_plan.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let change = BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Ada"),
        Some("Ada Lovelace"),
    ));
    let observed_execution = execute_live_view_shape_change(&observed_live, &change).unwrap();
    let focused_execution = execute_live_view_shape_change(&focused_live, &change).unwrap();

    assert_eq!(
        observed_execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::ObservedInspectorPatch)
    );
    assert_eq!(
        observed_execution.counters().complexity_status_debt_count(),
        0
    );
    assert_eq!(
        focused_execution.patch_envelope().patch_family(),
        Some(ViewShapePatchFamily::FocusedInspectorAspectPatch)
    );
    assert_eq!(
        focused_execution.counters().complexity_status_debt_count(),
        0
    );
    assert_ne!(
        observed_execution.patch_envelope().replay_digest(),
        focused_execution.patch_envelope().replay_digest()
    );
}

#[test]
fn focused_inspector_widening_is_denied_and_counted() {
    let planned = planned_view(
        &detail_canonical(),
        ViewShapeDescriptor::inspector_detail_focused(
            worth_foundational::facade::AspectKey::new("profile").unwrap(),
        ),
    );
    let live = lower_view_shape_plan_to_live(
        &planned,
        runtime_basis(planned.validated().query().schema_basis().clone()),
        None,
        None,
    )
    .unwrap();
    let error = execute_live_view_shape_change(
        &live,
        &BridgeChangeSummary::default()
            .with_field_delta(BridgeFieldDelta::new(
                "profile",
                "display_name",
                Some("Ada"),
                Some("Ada Lovelace"),
            ))
            .with_field_delta(BridgeFieldDelta::new(
                "identity",
                "id",
                Some("user-1"),
                Some("user-2"),
            )),
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        &ViewShapeLiveFailureClass::FocusedInspectorWideningDenied
    );
    assert_eq!(
        error.counters().focused_inspector_widening_denial_count(),
        1
    );
}
