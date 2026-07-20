#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryConditionalNodeRole {
    Computed,
    WorkflowStage,
    OperationGate,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryConditionalNodeContext {
    Basis,
    Snapshot,
    QueryContext,
    OperationInput,
    WorkflowRun,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryMaintenancePosture {
    EagerOnEligibleInvalidation,
    LazyUntilObserved,
    OnDemandOnly,
    Temporal,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryArtifactPosture {
    Ephemeral,
    ReusableWhenEquivalent,
    Durable,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryOutputRelationship {
    IntermediateOnly,
    ContributesToOperationOutput,
    IsOperationOutput,
    IsWorkflowStageOutput,
}

pub(crate) fn context_name(context: WorthQueryConditionalNodeContext) -> &'static str {
    match context {
        WorthQueryConditionalNodeContext::Basis => "basis",
        WorthQueryConditionalNodeContext::Snapshot => "snapshot",
        WorthQueryConditionalNodeContext::QueryContext => "query-context",
        WorthQueryConditionalNodeContext::OperationInput => "operation-input",
        WorthQueryConditionalNodeContext::WorkflowRun => "workflow-run",
    }
}
