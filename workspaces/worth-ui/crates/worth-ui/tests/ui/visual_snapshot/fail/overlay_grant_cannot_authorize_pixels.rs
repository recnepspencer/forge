use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiPixelsRequired, UiVisualOverlayGrant,
    UiVisualSnapshotRequest,
};

fn wrong_grant(
    shell: &mut WorthUiNativeApplicationShell,
    overlay_grant: &UiVisualOverlayGrant,
    target: UiCurrentPresentedSurfaceTarget,
) {
    let request =
        UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
            .artifacts(UiPixelsRequired::policy());
    let _ = shell.begin_visual_pixel_snapshot(overlay_grant, request);
}

fn main() {}
