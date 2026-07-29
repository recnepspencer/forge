use worth_ui::facade::app::WorthUi;

fn main() {
    let app = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse()).freeze().expect("application preparation should succeed");
    let index = app.capabilities().index();
    let _ = index.commands;
}
