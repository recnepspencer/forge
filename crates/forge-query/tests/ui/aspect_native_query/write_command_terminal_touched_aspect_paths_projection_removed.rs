use forge_query::facade::ForgeQueryWriteCommand;

fn assert_no_terminal_touched_path_projection(command: &ForgeQueryWriteCommand) {
    let _ = command.terminal_touched_aspect_paths_projection();
}

fn main() {}
