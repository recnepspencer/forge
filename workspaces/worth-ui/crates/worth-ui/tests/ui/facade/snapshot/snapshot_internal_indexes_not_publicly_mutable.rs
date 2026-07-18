use worth_ui::facade::WorthUi;

fn main() {
    let app = WorthUi::app().freeze().expect("application preparation should succeed");
    let mut index = app.capabilities().index();

    index.commands = index.commands;
}
