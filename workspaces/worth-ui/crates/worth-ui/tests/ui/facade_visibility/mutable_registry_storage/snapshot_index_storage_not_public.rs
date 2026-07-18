use worth_ui::facade::WorthUi;

fn main() {
    let app = WorthUi::app().freeze().expect("application preparation should succeed");
    let index = app.capabilities().index();
    let _ = index.commands;
}
