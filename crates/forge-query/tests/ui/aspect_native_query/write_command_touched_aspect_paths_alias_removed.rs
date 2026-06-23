use forge_query::facade::ForgeQueryWriteCommand;

fn assert_no_neutral_touched_path_alias(command: &ForgeQueryWriteCommand) {
    let _ = command.touched_aspect_paths();
}

fn main() {}
