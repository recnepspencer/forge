use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiApplicationBuilder};

fn build_app() -> WorthUiApp {
    WorthUi::app().freeze().expect("application preparation should succeed")
}

fn accepts_builder(_builder: WorthUiApplicationBuilder) {}

fn main() {
    let app = build_app();
    let builder = WorthUi::app();

    accepts_builder(builder);

    let _ = app;
}
