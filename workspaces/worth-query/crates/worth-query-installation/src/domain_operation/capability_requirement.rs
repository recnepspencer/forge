#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryOperationCapabilityRequirement {
    QueryRead,
    QueryComposition,
    QueryContext,
    IdentityEvolution,
    LiveQuery,
    PreviewSession,
    WorkflowOrchestration,
    HistoricalEvaluation,
    DurableArtifacts,
}

impl WorthQueryOperationCapabilityRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryRead => "query_read",
            Self::QueryComposition => "query_composition",
            Self::QueryContext => "query_context",
            Self::IdentityEvolution => "identity_evolution",
            Self::LiveQuery => "live_query",
            Self::PreviewSession => "preview_session",
            Self::WorkflowOrchestration => "workflow_orchestration",
            Self::HistoricalEvaluation => "historical_evaluation",
            Self::DurableArtifacts => "durable_artifacts",
        }
    }
}
