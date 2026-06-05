use hadwiger_research::facade::AgentSourceRecord;

fn main() {
    let mut source = AgentSourceRecord::new(
        "codex",
        "local-agent-session",
        "transcript:digest",
        "tool:digest",
    )
    .unwrap();
    source.agent_identity = "other".to_string();
}
