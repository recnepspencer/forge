use worth_ui::facade::{
    WorthUiReplacementCandidate,
    WorthUiRuntimeImpactNarrowing,
    runtime::WorthUiRuntime,
};

fn cannot_match_unadmitted_candidate(
    host: &WorthUiRuntime,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    candidate: &WorthUiReplacementCandidate,
) {
    let _ = host.build_identity_match_graph(narrowing, candidate);
}

fn main() {}
