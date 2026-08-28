use worth_ui::facade::declaration::{CommandCategory, CommandDescriptor, CommandId};

fn main() {
    let _ = CommandDescriptor {
        id: CommandId::new("workspace.open").expect("valid command id"),
        label: String::from("Open Workspace"),
        description: None,
        icon: None,
        default_shortcut: None,
        route: None,
        category: CommandCategory::Workspace,
        projection_eligibility: None,
    };
}
