use hadwiger_research::facade::{AgentAdvisoryArtifact, TilingIterationPacketRequest};

fn forbidden(advisory: AgentAdvisoryArtifact) {
    let _ = TilingIterationPacketRequest::from_agent_advisory_unchecked(advisory);
}

fn main() {}
