use worth_ui::WorthUi;

fn main() {
    let _ = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse());
}
