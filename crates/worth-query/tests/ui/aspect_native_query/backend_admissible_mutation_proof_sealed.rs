use worth_query::facade::{WorthQueryBackendAdmissibleMutation, WorthQueryWriteCommand};

fn main() {
    let command = command_fixture();
    let _ = WorthQueryBackendAdmissibleMutation::from_admitted_command(command);

    let mutation = mutation_fixture();
    let _ = mutation.into_command();

    let mutation = mutation_fixture();
    let _ = mutation.command();
}

fn command_fixture() -> WorthQueryWriteCommand {
    panic!("fixture only")
}

fn mutation_fixture() -> WorthQueryBackendAdmissibleMutation {
    panic!("fixture only")
}
