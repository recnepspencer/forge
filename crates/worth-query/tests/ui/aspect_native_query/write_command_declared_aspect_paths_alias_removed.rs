use worth_query::facade::runtime::WorthQueryWriteCommand;

fn forbidden(command: WorthQueryWriteCommand) {
    let _ = command.declared_aspect_paths();
}

fn main() {}
