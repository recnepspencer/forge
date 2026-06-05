use hadwiger_research::facade::{AspectDependencyGraph, HadwigerArtifactReference};

fn pass_raw_reference(reference: HadwigerArtifactReference) {
    let _ = AspectDependencyGraph::builder("closure-a").with_aspect(reference);
}

fn main() {
    let _ = pass_raw_reference as fn(HadwigerArtifactReference);
}
