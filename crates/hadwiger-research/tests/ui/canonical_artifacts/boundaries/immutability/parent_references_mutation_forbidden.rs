use hadwiger_research::facade::{GraphVersion, HadwigerArtifactReference};

fn mutate_parents(version: &mut GraphVersion, parent: HadwigerArtifactReference) {
    version.parent_artifacts.push(parent);
}

fn main() {
    let _ = mutate_parents as fn(&mut GraphVersion, HadwigerArtifactReference);
}
