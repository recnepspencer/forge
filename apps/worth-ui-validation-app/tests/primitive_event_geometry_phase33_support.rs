use worth_ui::facade::{
    SurfaceId, WorthUiPrimitiveEventDispatchPlan, WorthUiPrimitiveEventHitTestPoint,
    WorthUiPrimitiveEventRegionOrder, WorthUiPrimitiveEventRegionReceipt,
    WorthUiPrimitiveProofReceipt,
};
use worth_ui_validation_app::reload::{
    ValidationAuthoredReloadEdit, ValidationPreparedReload, ValidationReloadRequest,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

pub const OUTER_SURFACE: &str = "worth.surface.preview.primitive.proof";
pub const INNER_SURFACE: &str = "worth.surface.preview.primitive.inner";

pub fn launch_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::sample())
        .expect("validation workbench should prepare")
        .into_runtime_workbench()
}

pub fn activate_edits(
    workbench: &mut ValidationRuntimeWorkbench,
    edits: &[ValidationAuthoredReloadEdit],
) {
    let prepared = prepare_reload(workbench, edits);
    workbench
        .activate_reload(prepared)
        .expect("event geometry reload activates");
}

pub fn prepare_reload(
    workbench: &ValidationRuntimeWorkbench,
    edits: &[ValidationAuthoredReloadEdit],
) -> ValidationPreparedReload {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let mut source_text = inputs.source().source_text().to_owned();
    for edit in edits {
        source_text = edit
            .apply_to_source_text(&source_text)
            .expect("event geometry edit applies to source");
    }
    workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(inputs.source().module_path(), source_text),
    )
}

pub fn nested_event_plan(
    workbench: &ValidationRuntimeWorkbench,
) -> WorthUiPrimitiveEventDispatchPlan {
    nested_event_plan_with_inner_requirement(workbench, true)
}

pub fn nested_event_plan_without_required_inner(
    workbench: &ValidationRuntimeWorkbench,
) -> WorthUiPrimitiveEventDispatchPlan {
    nested_event_plan_with_inner_requirement(workbench, false)
}

fn nested_event_plan_with_inner_requirement(
    workbench: &ValidationRuntimeWorkbench,
    require_inner: bool,
) -> WorthUiPrimitiveEventDispatchPlan {
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
    let maybe_inner_region = WorthUiPrimitiveEventRegionReceipt::from_child_primitive_draw_plan_at(
        &inner,
        &inner_plan,
        WorthUiPrimitiveEventRegionOrder::new(1, 0),
        outer.surface_id(),
        outer_frame.x(),
        outer_frame.y(),
    );
    let Some(inner_region) = maybe_inner_region else {
        assert!(!require_inner, "inner hit region exists");
        return WorthUiPrimitiveEventDispatchPlan::from_regions([outer_region]);
    };

    WorthUiPrimitiveEventDispatchPlan::from_regions([outer_region, inner_region])
}

pub fn outer_event_region(
    workbench: &ValidationRuntimeWorkbench,
) -> WorthUiPrimitiveEventRegionReceipt {
    let outer = primitive_proof_for_authored_surface(workbench, OUTER_SURFACE);
    let outer_plan = outer.draw_plan(900.0, 600.0);
    WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan(
        &outer,
        &outer_plan,
        WorthUiPrimitiveEventRegionOrder::new(0, 0),
    )
    .expect("outer hit region exists")
}

pub fn primitive_proof_for_authored_surface(
    workbench: &ValidationRuntimeWorkbench,
    surface_id: &str,
) -> WorthUiPrimitiveProofReceipt {
    workbench.primitive_proof_for_authored_surface(surface_id)
}

pub fn edit(surface_id: &str, prop_key: &str, value: &str) -> ValidationAuthoredReloadEdit {
    ValidationAuthoredReloadEdit::set_surface_prop(surface_id, prop_key, value)
}

pub fn surface_id(surface_id: &str) -> SurfaceId {
    SurfaceId::new(surface_id).expect("valid surface id")
}

trait PrimitiveEventGeometryProofSupport {
    fn primitive_proof_for_authored_surface(
        &self,
        surface_id: &str,
    ) -> WorthUiPrimitiveProofReceipt;
}

impl PrimitiveEventGeometryProofSupport for ValidationRuntimeWorkbench {
    fn primitive_proof_for_authored_surface(
        &self,
        surface_id: &str,
    ) -> WorthUiPrimitiveProofReceipt {
        let surface_id = SurfaceId::new(surface_id).expect("valid surface id");
        let target = self
            .runtime()
            .bind_authored_primitive_proof_target(&surface_id)
            .expect("primitive target binds");
        self.runtime()
            .resolve_primitive_proof_for_target(&target)
            .expect("primitive proof resolves")
    }
}

pub fn center_of(
    frame: worth_ui::facade::WorthUiPrimitiveFrame,
) -> WorthUiPrimitiveEventHitTestPoint {
    WorthUiPrimitiveEventHitTestPoint::new(
        frame.x() + frame.width() * 0.5,
        frame.y() + frame.height() * 0.5,
    )
}
