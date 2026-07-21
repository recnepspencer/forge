use worth_ui::facade::app::WorthUi;

fn main() {
    let builder = WorthUi::app();
    let _ = builder.command_registry;
}
