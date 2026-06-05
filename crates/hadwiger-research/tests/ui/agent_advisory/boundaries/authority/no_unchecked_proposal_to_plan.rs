use hadwiger_research::facade::{
    AgentExperimentProposal, GraphVersion, HadwigerCanonicalArtifact,
};

fn unchecked(graph_version: GraphVersion) {
    let proposal = AgentExperimentProposal::new("try-local-edge-rewire", graph_version.reference());
    let _ = proposal.into_experiment_plan_unchecked();
}

fn main() {}
