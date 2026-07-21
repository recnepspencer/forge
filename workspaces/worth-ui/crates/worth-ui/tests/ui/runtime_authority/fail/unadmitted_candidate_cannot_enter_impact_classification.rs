use worth_ui::facade::{
    WorthUiReplacementCandidate,
    WorthUiRuntimeArtifactComparison,
    runtime::WorthUiRuntime,
};

fn main() {
    let host = runtime_host();
    let comparison = runtime_comparison();
    let candidate = replacement_candidate();

    let _ = host.classify_replacement_impact(&comparison, &candidate);
}

fn runtime_host() -> WorthUiRuntime {
    unimplemented!()
}

fn runtime_comparison() -> WorthUiRuntimeArtifactComparison {
    unimplemented!()
}

fn replacement_candidate() -> WorthUiReplacementCandidate {
    unimplemented!()
}
