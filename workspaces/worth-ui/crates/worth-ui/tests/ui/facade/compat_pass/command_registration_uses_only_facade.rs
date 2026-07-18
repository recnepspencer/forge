use worth_ui::facade::{
    CommandCategory, CommandDescriptor, CommandId, CommandReadinessBinding, WorthUi,
};

fn main() {
    let app = WorthUi::app()
        .register_command(
            CommandDescriptor::new(
                CommandId::new("workspace.open").expect("valid command id"),
                "Open Workspace",
            )
            .with_category(CommandCategory::Workspace)
            .with_readiness(CommandReadinessBinding::always_admitted()),
        )
        .freeze().expect("application preparation should succeed");

    let _ = app.capabilities().commands().len();
}
