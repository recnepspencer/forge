use worth_ui::facade::WorthUi;

fn main() {
    let builder = WorthUi::app();
    let app = builder.freeze();

    let _second_app = builder.freeze();
    let _ = app;
}
