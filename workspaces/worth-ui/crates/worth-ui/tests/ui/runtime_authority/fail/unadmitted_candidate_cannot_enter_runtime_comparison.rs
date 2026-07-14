use worth_ui::facade::{WorthUiReplacementCandidate, WorthUiRuntime};

fn main() {
    let host = runtime_host();
    let candidate = replacement_candidate();

    let _ = host.compare_admitted_replacement(&candidate);
}

fn runtime_host() -> WorthUiRuntime {
    unimplemented!()
}

fn replacement_candidate() -> WorthUiReplacementCandidate {
    unimplemented!()
}
