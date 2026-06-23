use worth_ui::facade::{
    SurfaceId, WorthUiPrimitiveEventDispatchOutcome, WorthUiPrimitiveEventDispatchPlan,
    WorthUiPrimitiveEventHitTestPoint, WorthUiPrimitiveEventRegionOrder,
    WorthUiPrimitiveEventRegionReceipt, WorthUiPrimitiveResolvedCursorPosture,
};
use worth_ui_validation_app::reload::{ValidationAuthoredReloadEdit, ValidationReloadRequest};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

const OUTER_SURFACE: &str = "worth.surface.preview.primitive.proof";
const INNER_SURFACE: &str = "worth.surface.preview.primitive.inner";

#[test]
fn cursor_arbitration_explains_nested_region_winner_and_counters() {
    let mut workbench = launch_workbench();
    activate_edits(
        &mut workbench,
        &[
            edit(OUTER_SURFACE, "event_cursor", "text"),
            edit(INNER_SURFACE, "event_cursor", "grab"),
        ],
    );
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let receipt = event_plan.cursor_receipt_at(center_of(inner_region.hit_frame()));

    assert_eq!(receipt.primary_surface_id(), Some(INNER_SURFACE));
    assert_eq!(
        receipt.cursor(),
        WorthUiPrimitiveResolvedCursorPosture::Grab
    );
    assert_eq!(
        receipt.outcome(),
        WorthUiPrimitiveEventDispatchOutcome::HitNoActivation
    );
    assert_eq!(receipt.counters().region_count(), 2);
    assert_eq!(receipt.counters().cursor_candidate_count(), 2);
    assert!(receipt
        .candidates()
        .iter()
        .any(|candidate| candidate.surface_id() == INNER_SURFACE && candidate.selected()));
}

fn launch_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::sample())
        .expect("validation workbench should prepare")
        .into_runtime_workbench()
}

fn activate_edits(
    workbench: &mut ValidationRuntimeWorkbench,
    edits: &[ValidationAuthoredReloadEdit],
) {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let mut source_text = inputs.source().source_text().to_owned();
    for edit in edits {
        source_text = edit
            .apply_to_source_text(&source_text)
            .expect("event geometry edit applies to source");
    }
    let prepared = workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(inputs.source().module_path(), source_text),
    );
    workbench
        .activate_reload(prepared)
        .expect("event geometry reload activates");
}

fn nested_event_plan(workbench: &ValidationRuntimeWorkbench) -> WorthUiPrimitiveEventDispatchPlan {
    let outer = workbench
        .runtime()
        .resolve_primitive_proof(&surface_id(OUTER_SURFACE))
        .expect("outer primitive resolves");
    let inner = workbench
        .runtime()
        .resolve_primitive_proof(&surface_id(INNER_SURFACE))
        .expect("inner primitive resolves");
    let outer_plan = outer.draw_plan(900.0, 600.0);
    let outer_frame = outer_plan.frame();
    let inner_plan = inner.draw_plan(outer_frame.width(), outer_frame.height());
    let outer_region = WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan(
        &outer,
        &outer_plan,
        WorthUiPrimitiveEventRegionOrder::new(0, 0),
    )
    .expect("outer hit region exists");
    let inner_region = WorthUiPrimitiveEventRegionReceipt::from_child_primitive_draw_plan_at(
        &inner,
        &inner_plan,
        WorthUiPrimitiveEventRegionOrder::new(1, 0),
        outer.surface_id(),
        outer_frame.x(),
        outer_frame.y(),
    )
    .expect("inner hit region exists");

    WorthUiPrimitiveEventDispatchPlan::from_regions([outer_region, inner_region])
}

fn edit(surface_id: &str, prop_key: &str, value: &str) -> ValidationAuthoredReloadEdit {
    ValidationAuthoredReloadEdit::set_surface_prop(surface_id, prop_key, value)
}

fn surface_id(surface_id: &str) -> SurfaceId {
    SurfaceId::new(surface_id).expect("valid surface id")
}

fn center_of(frame: worth_ui::facade::WorthUiPrimitiveFrame) -> WorthUiPrimitiveEventHitTestPoint {
    WorthUiPrimitiveEventHitTestPoint::new(
        frame.x() + frame.width() * 0.5,
        frame.y() + frame.height() * 0.5,
    )
}
