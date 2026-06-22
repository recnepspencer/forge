use forge_query::facade::{ForgeQueryBackendAdmissibleMutation, ForgeQueryWriteCommand};

fn main() {
    let command = command_fixture();
    let _ = ForgeQueryBackendAdmissibleMutation::from_admitted_command(command);

    let mutation = mutation_fixture();
    let _ = mutation.into_command();

    let mutation = mutation_fixture();
    let _ = mutation.command();
}

fn command_fixture() -> ForgeQueryWriteCommand {
    panic!("fixture only")
}

fn mutation_fixture() -> ForgeQueryBackendAdmissibleMutation {
    panic!("fixture only")
}
