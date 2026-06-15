use worth_ui::facade::WorthUiAdmittedReplacementCandidate;

fn main() {
    let _candidate = WorthUiAdmittedReplacementCandidate {
        candidate: uninitialized_field(),
        active_basis: uninitialized_field(),
        report: uninitialized_field(),
        admitted_query_support_receipt_digest: 0,
    };
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
