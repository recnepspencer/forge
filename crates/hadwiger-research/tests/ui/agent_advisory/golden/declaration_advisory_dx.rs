use hadwiger_research::facade::{
    materialize_agent_declaration_advisory_checked, AdvisoryNoteDeclaration,
    AgentAdmissionAdvisory, AgentAdvisoryContributionRecord, AgentAdvisoryError,
    HadwigerResearchHandle,
};

fn materialize(
    handle: &HadwigerResearchHandle,
) -> Result<AgentAdvisoryContributionRecord, AgentAdvisoryError> {
    let advisory = AgentAdmissionAdvisory::caution(
        "candidate-a",
        "geometry evidence is suggestive but not checker-admitted",
    )?;

    materialize_agent_declaration_advisory_checked(
        handle,
        AdvisoryNoteDeclaration::new("candidate-a", "agent-caution"),
        advisory,
    )
}

fn main() {}
