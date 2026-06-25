use worth_ui::facade::{SurfaceId, WorthUiRuntimeHost};

fn main() {}

fn raw_surface_cannot_resolve_primitive_projection(
    runtime: &WorthUiRuntimeHost,
    surface_id: &SurfaceId,
) {
    let _ = runtime.resolve_primitive_projection(surface_id, None);
}
