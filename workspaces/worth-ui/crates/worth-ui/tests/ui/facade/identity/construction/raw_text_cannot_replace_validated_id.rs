use worth_ui::facade::declaration::CommandId;

fn main() {
    accepts_command("app.command.save");
}

fn accepts_command(_command_id: CommandId) {}
