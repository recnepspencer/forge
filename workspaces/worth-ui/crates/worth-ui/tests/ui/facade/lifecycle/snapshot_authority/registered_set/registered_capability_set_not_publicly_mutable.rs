use worth_ui::facade::app::WorthUi;

fn main() {
    let app = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse()).freeze().expect("application preparation should succeed");
    app.capabilities().registered_capabilities().total_width = 1;
}
