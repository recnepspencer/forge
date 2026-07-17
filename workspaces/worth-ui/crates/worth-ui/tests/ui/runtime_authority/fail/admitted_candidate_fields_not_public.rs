use worth_ui::facade::WorthUiAdmittedReplacementCandidate;

fn main() {
    let _candidate = WorthUiAdmittedReplacementCandidate {
        candidate: uninitialized_field(),
        active_basis: uninitialized_field(),
        report: uninitialized_field(),
        admitted_query_contract_identity: uninitialized_field(),
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
