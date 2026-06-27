use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiAppBuilder};

fn build_app() -> WorthUiApp {
    WorthUi::app().freeze()
}

fn accepts_builder(_builder: WorthUiAppBuilder) {}

fn main() {
    let app = build_app();
    let builder = WorthUi::app();

    accepts_builder(builder);

    let _ = app;
}
