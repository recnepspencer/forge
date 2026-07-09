use worth_query::facade::WorthQueryWriteCommand;

fn assert_no_terminal_touched_path_projection(command: &WorthQueryWriteCommand) {
    let _ = command.terminal_touched_aspect_paths_projection();
}

fn main() {}
