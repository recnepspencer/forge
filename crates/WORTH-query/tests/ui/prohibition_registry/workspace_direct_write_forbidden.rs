use worth_query::facade::runtime::{WorthQueryWorkspace, WorthQueryWriteCommand};

fn forbidden(mut workspace: WorthQueryWorkspace, command: WorthQueryWriteCommand) {
    let _ = workspace.write(command);
}

fn main() {}
