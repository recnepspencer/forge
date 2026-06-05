use hadwiger_research::facade::{
    AgentAdvisoryArtifact, AgentInvariantHypothesisSuggestion,
};

fn main() {
    let _ = AgentAdvisoryArtifact::admitted_theorem_authority("claim");
    let _ = AgentInvariantHypothesisSuggestion::admitted_query_invariant("invariant");
}
