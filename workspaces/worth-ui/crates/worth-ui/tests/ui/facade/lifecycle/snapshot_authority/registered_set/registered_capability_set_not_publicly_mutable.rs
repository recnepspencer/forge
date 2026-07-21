use worth_ui::facade::app::WorthUi;

fn main() {
    let app = WorthUi::app().freeze().expect("application preparation should succeed");
    app.capabilities().registered_capabilities().total_width = 1;
}
