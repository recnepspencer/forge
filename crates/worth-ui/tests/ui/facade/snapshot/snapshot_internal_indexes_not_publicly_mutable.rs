use worth_ui::facade::WorthUi;

fn main() {
    let app = WorthUi::app().freeze();
    let mut index = app.capabilities().index();

    index.commands = index.commands;
}
