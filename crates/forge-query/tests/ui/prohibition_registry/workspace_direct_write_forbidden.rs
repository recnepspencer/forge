use forge_query::facade::runtime::{ForgeQueryWorkspace, ForgeQueryWriteCommand};

fn forbidden(mut workspace: ForgeQueryWorkspace, command: ForgeQueryWriteCommand) {
    let _ = workspace.write(command);
}

fn main() {}
