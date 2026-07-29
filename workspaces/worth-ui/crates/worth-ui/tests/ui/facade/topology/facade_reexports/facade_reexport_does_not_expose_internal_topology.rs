use worth_ui::facade::entry::WorthUi;

fn main() {
    let _ = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse());
}
