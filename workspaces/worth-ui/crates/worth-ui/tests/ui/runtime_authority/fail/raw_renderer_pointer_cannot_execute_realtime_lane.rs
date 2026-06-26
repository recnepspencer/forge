use worth_ui::facade::WorthUiRealtimeFrameTarget;

fn main() {
    let renderer_pointer = 0usize as *const ();
    let _target = WorthUiRealtimeFrameTarget::from_renderer_pointer(renderer_pointer);
}
