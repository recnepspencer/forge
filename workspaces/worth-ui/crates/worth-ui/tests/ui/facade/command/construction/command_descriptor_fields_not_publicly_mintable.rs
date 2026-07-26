use worth_ui::facade::{
    declaration::{CommandCategory, CommandDescriptor, CommandId, CommandReadinessBinding},
};

fn main() {
    let _ = CommandDescriptor {
        id: CommandId::new("workspace.open").expect("valid command id"),
        label: String::from("Open Workspace"),
        description: None,
        icon: None,
        default_shortcut_reference: None,
        category: CommandCategory::Workspace,
        readiness: CommandReadinessBinding::always_admitted(),
        projection_eligibility: None,
    };
}
