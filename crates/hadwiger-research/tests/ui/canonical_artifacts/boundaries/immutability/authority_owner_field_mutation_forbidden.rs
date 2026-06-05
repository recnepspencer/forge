use hadwiger_research::facade::{GraphIdentity, HadwigerArtifactAuthorityOwner};

fn mutate_authority(graph: &mut GraphIdentity) {
    graph.authority_owner = HadwigerArtifactAuthorityOwner::ProofCandidate;
}

fn main() {
    let _ = mutate_authority as fn(&mut GraphIdentity);
}
