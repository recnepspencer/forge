use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{
    UiCurrentPresentedSurfaceTarget, UiPixelsRequired, UiVisualGeometryGrant,
    UiVisualSnapshotRequest,
};

fn wrong_grant(
    shell: &mut WorthUiNativeApplicationShell,
    geometry_grant: &UiVisualGeometryGrant,
    target: UiCurrentPresentedSurfaceTarget,
) {
    let request =
        UiVisualSnapshotRequest::for_local_development_unredacted_frame(target)
            .artifacts(UiPixelsRequired::policy());
    let _ = shell.begin_visual_pixel_snapshot(geometry_grant, request);
}

fn main() {}
