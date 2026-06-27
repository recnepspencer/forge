use worth_ui::facade::{WorthUiReplacementCandidate, WorthUiRuntimeHost};

fn main() {
    let host = runtime_host();
    let candidate = replacement_candidate();

    let _ = host.compare_admitted_replacement(&candidate);
}

fn runtime_host() -> WorthUiRuntimeHost {
    unimplemented!()
}

fn replacement_candidate() -> WorthUiReplacementCandidate {
    unimplemented!()
}
