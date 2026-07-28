use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::{UiVisualOverlayTarget, UiVisualPixelCaptureGrant};

fn wrong_grant(
    shell: &mut WorthUiNativeApplicationShell,
    pixel_grant: &UiVisualPixelCaptureGrant,
    target: UiVisualOverlayTarget,
) {
    let _ = shell.show_identity_overlay(pixel_grant, target);
}

fn main() {}
