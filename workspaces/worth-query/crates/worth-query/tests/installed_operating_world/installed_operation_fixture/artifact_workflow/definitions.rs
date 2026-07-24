use worth_query::facade::domain;

#[derive(Clone, Copy)]
pub enum ArtifactWorkflowKind {
    Move,
    Lease,
}

pub fn workflow_definition(
    contract: &domain::WorthQueryPortableArtifactContract,
    kind: ArtifactWorkflowKind,
) -> domain::WorthQueryPortableWorkflowDefinition {
    let artifact =
        || domain::WorthQueryWorkflowValueContract::installed_artifact(contract.reference());
    let stages = match kind {
        ArtifactWorkflowKind::Move => vec![
            stage(
                "produce",
                [],
                false,
                domain::WorthQueryWorkflowValueContract::Text,
                artifact(),
            ),
            stage(
                "consume",
                ["produce"],
                true,
                artifact(),
                domain::WorthQueryWorkflowValueContract::Text,
            ),
        ],
        ArtifactWorkflowKind::Lease => vec![
            stage(
                "produce",
                [],
                false,
                domain::WorthQueryWorkflowValueContract::Text,
                artifact(),
            ),
            stage(
                "observe-a",
                ["produce"],
                true,
                artifact(),
                domain::WorthQueryWorkflowValueContract::Text,
            ),
            stage(
                "observe-b",
                ["produce"],
                true,
                artifact(),
                domain::WorthQueryWorkflowValueContract::Text,
            ),
        ],
    };
    domain::WorthQueryPortableWorkflowDefinition::new("produce", stages)
}

fn stage(
    identity: &str,
    predecessors: impl IntoIterator<Item = &'static str>,
    terminal: bool,
    input: domain::WorthQueryWorkflowValueContract,
    output: domain::WorthQueryWorkflowValueContract,
) -> domain::WorthQueryPortableWorkflowStage {
    domain::WorthQueryPortableWorkflowStage::new(
        identity,
        predecessors,
        terminal,
        false,
        std::iter::empty::<domain::WorthQueryOperationCapabilityRequirement>(),
    )
    .with_semantics(domain::WorthQueryWorkflowStageSemantics {
        input,
        output,
        cost_roles: vec![
            domain::WorthQueryWorkflowCostRole::Admission,
            domain::WorthQueryWorkflowCostRole::Execution,
            domain::WorthQueryWorkflowCostRole::ResultValidation,
        ],
        terminal_result_states: terminal
            .then_some(domain::WorthQueryOperationResultState::Ready)
            .into_iter()
            .collect(),
        failure_classes: vec![domain::WorthQueryOperationFailureClass::Dependency],
        ..Default::default()
    })
}
