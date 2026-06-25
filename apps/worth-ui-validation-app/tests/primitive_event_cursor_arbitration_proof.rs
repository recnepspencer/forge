use worth_ui::facade::{
    SurfaceId, WorthUiPrimitiveEventDispatchCandidateReceipt, WorthUiPrimitiveEventDispatchOutcome,
    WorthUiPrimitiveEventDispatchPlan, WorthUiPrimitiveEventHitTestPoint,
    WorthUiPrimitiveEventRegionOrder, WorthUiPrimitiveEventRegionReceipt,
    WorthUiPrimitiveProofReceipt, WorthUiPrimitiveResolvedCursorPosture,
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
            edit(INNER_SURFACE, "primitive_disabled", "false"),
            edit(INNER_SURFACE, "interaction_readiness", "enabled"),
        ],
    );
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let inner_point = center_of(inner_region.hit_frame());
    let target = workbench
        .runtime()
        .bind_primitive_event_dispatch_target(&event_plan, inner_point)
        .expect("inner cursor target binds");
    let receipt = event_plan.cursor_receipt_for_target(&target, inner_point);

    assert_eq!(receipt.primary_surface_id(), Some(INNER_SURFACE));
    assert_eq!(
        receipt.cursor(),
        WorthUiPrimitiveResolvedCursorPosture::Grab
    );
    assert!(matches!(
        receipt.outcome(),
        WorthUiPrimitiveEventDispatchOutcome::Denied(_)
    ));
    assert_eq!(
        receipt.query_graph_execution().selected_obligation_count(),
        6
    );
    assert_eq!(receipt.counters().region_count(), 2);
    assert_eq!(receipt.counters().cursor_candidate_count(), 2);
    assert!(receipt
        .candidates()
        .iter()
        .any(|candidate| candidate.surface_id() == INNER_SURFACE && candidate.selected()));
}

#[test]
fn disabled_hover_is_diagnostic_disabled_hit_not_enabled_cursor_target() {
    let workbench = launch_workbench();
    let event_plan = nested_event_plan(&workbench);
    let inner_region = event_plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == INNER_SURFACE)
        .expect("inner event region exists");
    let inner_point = center_of(inner_region.hit_frame());
    let target = workbench
        .runtime()
        .bind_primitive_event_dispatch_target(&event_plan, inner_point)
        .expect("inner cursor target binds");
    let receipt = event_plan.cursor_receipt_for_target(&target, inner_point);

    assert_eq!(
        receipt.cursor(),
        WorthUiPrimitiveResolvedCursorPosture::NotAllowed
    );
    assert!(matches!(
        receipt.outcome(),
        WorthUiPrimitiveEventDispatchOutcome::Denied(_)
    ));
    assert!(receipt.candidates().iter().any(|candidate| matches!(
        candidate,
        WorthUiPrimitiveEventDispatchCandidateReceipt::DisabledHit(_)
            if candidate.surface_id() == INNER_SURFACE
    )));
    assert!(receipt.emitted_surface_ids().is_empty());
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
    let outer = workbench.primitive_proof_for_authored_surface(OUTER_SURFACE);
    let inner = workbench.primitive_proof_for_authored_surface(INNER_SURFACE);
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

trait PrimitiveCursorProofSupport {
    fn primitive_proof_for_authored_surface(
        &self,
        surface_id: &str,
    ) -> WorthUiPrimitiveProofReceipt;
}

impl PrimitiveCursorProofSupport for ValidationRuntimeWorkbench {
    fn primitive_proof_for_authored_surface(
        &self,
        surface_id_text: &str,
    ) -> WorthUiPrimitiveProofReceipt {
        let surface_id = surface_id(surface_id_text);
        let target = self
            .runtime()
            .bind_authored_primitive_proof_target(&surface_id)
            .expect("primitive target binds");
        self.runtime()
            .resolve_primitive_proof_for_target(&target)
            .expect("primitive proof resolves")
    }
}

fn center_of(frame: worth_ui::facade::WorthUiPrimitiveFrame) -> WorthUiPrimitiveEventHitTestPoint {
    WorthUiPrimitiveEventHitTestPoint::new(
        frame.x() + frame.width() * 0.5,
        frame.y() + frame.height() * 0.5,
    )
}
