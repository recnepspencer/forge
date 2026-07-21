use worth_ui::facade::app::WorthUi;

fn main() {
    let builder = WorthUi::app();
    let app = builder.freeze().expect("application preparation should succeed");

    let _second_app = builder.freeze().expect("application preparation should succeed");
    let _ = app;
}
