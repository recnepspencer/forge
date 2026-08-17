use worth_ui::facade::app::{WorthUi, WorthUiHostNeutralApp};

fn main() {
    let builder = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse());
    let _: WorthUiHostNeutralApp = builder.freeze().unwrap();
}
