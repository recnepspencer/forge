use worth_ui::facade::{
    WorthUiReplacementCandidate, WorthUiRuntimeHost, WorthUiRuntimeImpactNarrowing,
};

fn cannot_match_unadmitted_candidate(
    host: &WorthUiRuntimeHost,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    candidate: &WorthUiReplacementCandidate,
) {
    let _ = host.build_identity_match_graph(narrowing, candidate);
}

fn main() {}
