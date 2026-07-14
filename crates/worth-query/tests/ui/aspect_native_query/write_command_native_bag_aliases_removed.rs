use worth_query::facade::runtime::WorthQueryWriteCommand;

fn main() {}

fn removed_write_command_native_bag_aliases(command: &WorthQueryWriteCommand) {
    let _ = command.aspect_values();
    let _ = command.asserted_aspect_values();
    let _ = command.touched_aspects();
}
