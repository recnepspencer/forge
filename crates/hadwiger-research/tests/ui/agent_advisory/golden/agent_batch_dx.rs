use hadwiger_research::facade::{
    AgentExperimentProposal, AgentExplorationBatch, AgentMotifSuggestion, AgentSourceRecord,
    GraphResidentFailure, GraphVersion, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
};

fn build_batch(
    graph_failure: GraphResidentFailure,
    graph_version: GraphVersion,
) -> Result<AgentExplorationBatch, HadwigerArtifactShapeError> {
    let source = AgentSourceRecord::new(
        "codex",
        "local-agent-session",
        "transcript:digest:phase9-smoke",
        "tool:digest:hadwiger-cli",
    )?;

    AgentExplorationBatch::builder("frontier-agent-pass-a", source)
        .with_motif_suggestion(
            AgentMotifSuggestion::new("motif-a", graph_failure.reference())
                .with_observation("edge-local unit-distance failures recur")?,
        )?
        .with_experiment_proposal(
            AgentExperimentProposal::new("try-local-edge-rewire", graph_version.reference())
                .with_rationale("test whether the failure is tied to one local edge orbit")?,
        )?
        .finish()
}

fn main() {}
