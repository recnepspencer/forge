use forge_query::facade::{ForgeQueryBackendAdmissibleMutation, ForgeQueryWriteCommand};

fn main() {
    let command = command_fixture();
    let _ = command.declared_collection();
    let _ = command.declared_collection_ref();
    let _ = command.terminal_declared_collection_projection();

    let mutation = backend_admissible_fixture();
    let _ = mutation.declared_collection();
    let _ = mutation.declared_collection_ref();
    let _ = mutation.terminal_declared_collection_projection();
}

fn command_fixture() -> ForgeQueryWriteCommand {
    panic!("fixture only")
}

fn backend_admissible_fixture() -> ForgeQueryBackendAdmissibleMutation {
    panic!("fixture only")
}
