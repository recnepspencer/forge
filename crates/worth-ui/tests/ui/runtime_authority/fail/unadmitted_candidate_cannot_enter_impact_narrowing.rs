use worth_ui::facade::{
    WorthUiReplacementCandidate, WorthUiReplacementImpactClassification, WorthUiRuntimeHost,
};

fn main() {
    let host = runtime_host();
    let classification = impact_classification();
    let candidate = replacement_candidate();

    let _ = host.narrow_replacement_impact(&classification, &candidate);
}

fn runtime_host() -> WorthUiRuntimeHost {
    unimplemented!()
}

fn impact_classification() -> WorthUiReplacementImpactClassification {
    unimplemented!()
}

fn replacement_candidate() -> WorthUiReplacementCandidate {
    unimplemented!()
}
