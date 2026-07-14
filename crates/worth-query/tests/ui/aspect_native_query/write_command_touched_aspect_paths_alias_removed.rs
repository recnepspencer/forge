use worth_query::facade::runtime::WorthQueryWriteCommand;

fn assert_no_neutral_touched_path_alias(command: &WorthQueryWriteCommand) {
    let _ = command.touched_aspect_paths();
}

fn main() {}
