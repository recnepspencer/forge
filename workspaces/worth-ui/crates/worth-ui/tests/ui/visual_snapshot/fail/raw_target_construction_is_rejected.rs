use worth_ui::facade::inspection::{
    UiClientRegionVisualTarget, UiCurrentPresentedSurfaceTarget, UiMountedNodeVisualTarget,
    UiVisualOverlayTarget,
};

fn main() {
    let _ = UiCurrentPresentedSurfaceTarget {};
    let _ = UiMountedNodeVisualTarget {};
    let _ = UiClientRegionVisualTarget {};
    let _ = UiVisualOverlayTarget {};
}
