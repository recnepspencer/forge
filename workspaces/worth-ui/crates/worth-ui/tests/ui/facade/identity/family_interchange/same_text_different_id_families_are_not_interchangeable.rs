use worth_ui::facade::{
    declaration::{CommandId, ComponentId},
};

fn main() {
    let command_id = CommandId::new("app.shared.save").expect("valid command id");
    let component_id = ComponentId::new("app.shared.save").expect("valid component id");

    accepts_component(command_id);
    accepts_command(component_id);
}

fn accepts_command(_command_id: CommandId) {}

fn accepts_component(_component_id: ComponentId) {}
