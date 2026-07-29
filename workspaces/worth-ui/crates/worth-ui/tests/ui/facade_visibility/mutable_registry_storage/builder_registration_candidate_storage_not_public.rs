use worth_ui::facade::app::WorthUi;

fn main() {
    let builder = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse());
    let _ = builder.registration_candidates;
}
