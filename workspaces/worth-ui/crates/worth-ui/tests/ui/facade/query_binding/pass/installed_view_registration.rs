use worth_ui::facade::app::WorthUi;
use worth_ui::facade::query_binding::WorthUiInstalledQueryView;

fn register(view: WorthUiInstalledQueryView) {
    let _app = WorthUi::app().with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(view)
        .expect("installed view registration")
        .freeze().expect("application preparation should succeed");
}

fn main() {}
