use forge_query::facade::ForgeQueryWriteCommand;

fn forbidden(command: ForgeQueryWriteCommand) {
    let _ = command.declared_aspect_paths();
}

fn main() {}
