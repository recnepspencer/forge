use worth_ui::facade::{WorthUiAdmittedReplacementCandidate, WorthUiReplacementCandidate};

fn requires_admitted_candidate(_candidate: WorthUiAdmittedReplacementCandidate) {}

fn main() {
    let raw_candidate: WorthUiReplacementCandidate = uninitialized_field();

    requires_admitted_candidate(raw_candidate);
}

fn uninitialized_field<T>() -> T {
    unimplemented!()
}
