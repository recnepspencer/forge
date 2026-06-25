use worth_ui::facade::{
    SurfaceId, WorthUiMountedInteractionPlanRequest, WorthUiRuntimeHost,
};

fn main() {}

fn raw_surface_cannot_resolve_mounted_interaction_plan(
    runtime: &WorthUiRuntimeHost,
    surface_id: SurfaceId,
) {
    let request = WorthUiMountedInteractionPlanRequest::primary_click(surface_id);
    let _ = runtime.resolve_mounted_interaction_plan(request);
}
