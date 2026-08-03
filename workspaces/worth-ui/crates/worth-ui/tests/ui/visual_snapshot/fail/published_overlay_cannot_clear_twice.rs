use worth_ui::facade::app::WorthUiNativeApplicationShell;
use worth_ui::facade::inspection::UiPublishedVisualOverlay;

fn clear_twice(
    shell: &mut WorthUiNativeApplicationShell,
    published: UiPublishedVisualOverlay,
) {
    let _ = shell.clear_visual_overlay(published, 2, 1);
    let _ = shell.clear_visual_overlay(published, 3, 2);
}

fn main() {}
