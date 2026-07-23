#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum WorthQuerySemanticAspectDependencyLocus {
    InstalledOperation,
    OperationNativeProjection,
    CollectionRowIdentity,
    CollectionOrdering {
        field_ordinal: usize,
    },
    CollectionGrouping {
        field_ordinal: usize,
    },
    CollectionWindow,
    ResultShape,
    TouchGraphRole {
        role_ordinal: usize,
    },
    TouchScope {
        scope_ordinal: usize,
    },
    EffectFamily {
        effect_ordinal: usize,
    },
    InstalledInvariant {
        invariant_ordinal: usize,
    },
    ReplayContract,
    LineageContract,
    SupportContract,
    GraphReadNativeProjection {
        graph_read_role: String,
        projection_ordinal: usize,
    },
    WorkflowStage {
        stage_identity: String,
    },
    WorkflowStageRead {
        stage_identity: String,
        graph_read_role: String,
    },
    ConditionalNode {
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    },
    ConditionalTruth {
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
    },
    DirectGraphCall {
        call_ordinal: usize,
    },
    WorkflowGraphCall {
        stage_identity: String,
        call_ordinal: usize,
    },
    WorkflowPrimaryRead {
        stage_identity: String,
        read_ordinal: usize,
    },
    ConditionalOutcome {
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    },
    DirectOutput,
    WorkflowEffect {
        stage_identity: String,
        effect_ordinal: usize,
    },
    WorkflowInvariant {
        stage_identity: String,
        invariant_ordinal: usize,
    },
    WorkflowLineage {
        stage_identity: String,
        lineage_ordinal: usize,
    },
    WorkflowOutput {
        stage_identity: String,
    },
}

impl WorthQuerySemanticAspectDependencyLocus {
    pub(crate) const KIND_COUNT: usize = 28;

    pub(crate) const fn kind_ordinal(&self) -> usize {
        match self {
            Self::InstalledOperation => 0,
            Self::OperationNativeProjection => 1,
            Self::CollectionRowIdentity => 2,
            Self::CollectionOrdering { .. } => 3,
            Self::CollectionGrouping { .. } => 4,
            Self::CollectionWindow => 5,
            Self::ResultShape => 6,
            Self::TouchGraphRole { .. } => 7,
            Self::TouchScope { .. } => 8,
            Self::EffectFamily { .. } => 9,
            Self::InstalledInvariant { .. } => 10,
            Self::ReplayContract => 11,
            Self::LineageContract => 12,
            Self::SupportContract => 13,
            Self::GraphReadNativeProjection { .. } => 14,
            Self::WorkflowStage { .. } => 15,
            Self::WorkflowStageRead { .. } => 16,
            Self::ConditionalNode { .. } => 17,
            Self::ConditionalTruth { .. } => 18,
            Self::DirectGraphCall { .. } => 19,
            Self::WorkflowGraphCall { .. } => 20,
            Self::WorkflowPrimaryRead { .. } => 21,
            Self::ConditionalOutcome { .. } => 22,
            Self::DirectOutput => 23,
            Self::WorkflowEffect { .. } => 24,
            Self::WorkflowInvariant { .. } => 25,
            Self::WorkflowLineage { .. } => 26,
            Self::WorkflowOutput { .. } => 27,
        }
    }
}
