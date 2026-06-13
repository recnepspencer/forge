use worth_ui::facade::{
    WorthUiReplacementCandidate, WorthUiRuntimeArtifactComparison, WorthUiRuntimeHost,
};

fn main() {
    let host = runtime_host();
    let comparison = runtime_comparison();
    let candidate = replacement_candidate();

    let _ = host.classify_replacement_impact(&comparison, &candidate);
}

fn runtime_host() -> WorthUiRuntimeHost {
    unimplemented!()
}

fn runtime_comparison() -> WorthUiRuntimeArtifactComparison {
    unimplemented!()
}

fn replacement_candidate() -> WorthUiReplacementCandidate {
    unimplemented!()
}
