use worth_ui::facade::WorthUiCanvasSpatialFrameTarget;

fn main() {
    let renderer_pointer = 0usize as *const ();
    let _target = WorthUiCanvasSpatialFrameTarget::from_renderer_pointer(renderer_pointer);
}
