use worth_ui::facade::registry::CommandId;

fn main() {
    accepts_command("app.command.save");
}

fn accepts_command(_command_id: CommandId) {}
