use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiGeometryOnly, UiPendingVisualCapture,
};

fn cancel_twice(
    shell: &mut WorthUiNativeApplicationShell,
    pending: UiPendingVisualCapture<UiCurrentPresentedSurfaceTarget, UiGeometryOnly>,
) {
    let _ = shell.cancel_visual_snapshot(pending);
    let _ = shell.cancel_visual_snapshot(pending);
}

fn main() {}
