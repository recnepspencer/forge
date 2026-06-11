use worth_ui::facade::WorthUi;

fn main() {
    let app = WorthUi::app().freeze();
    let index = app.capabilities().index();
    let _ = index.commands;
}
