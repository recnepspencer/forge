use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiVisualPixelCaptureGrant, UiVisualSnapshotRequest,
};

fn wrong_grant(
    shell: &mut WorthUiNativeApplicationShell,
    pixel_grant: &UiVisualPixelCaptureGrant,
    target: UiCurrentPresentedSurfaceTarget,
) {
    let request = UiVisualSnapshotRequest::for_local_development_unredacted_frame(target);
    let _ = shell.begin_visual_geometry_snapshot(pixel_grant, request);
}

fn main() {}
