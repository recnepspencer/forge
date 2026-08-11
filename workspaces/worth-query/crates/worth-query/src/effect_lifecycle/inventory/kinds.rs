#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectLifecycleFamilyKey {
    Mutation,
    Merge,
    Writeback,
    OrderedBatch,
}

impl EffectLifecycleFamilyKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::Merge => "merge",
            Self::Writeback => "writeback",
            Self::OrderedBatch => "ordered_batch",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectLoweredArtifactKind {
    LoweredMutationIntentDeclaration,
    LoweredMergeWorkflowDeclaration,
    QueryWritebackDeclaration,
    LoweredEffectBatchExecutionPlan,
}

impl EffectLoweredArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LoweredMutationIntentDeclaration => "lowered_mutation_intent_declaration",
            Self::LoweredMergeWorkflowDeclaration => "lowered_merge_workflow_declaration",
            Self::QueryWritebackDeclaration => "query_writeback_declaration",
            Self::LoweredEffectBatchExecutionPlan => "lowered_effect_batch_execution_plan",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectReceiptArtifactKind {
    WorthQueryIntentExecution,
    WorthQueryWriteReceipt,
    WorthQueryBatchWriteReceipt,
    SelfDescribingEffectEnvelope,
}

impl EffectReceiptArtifactKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorthQueryIntentExecution => "worth_query_intent_execution",
            Self::WorthQueryWriteReceipt => "worth_query_write_receipt",
            Self::WorthQueryBatchWriteReceipt => "worth_query_batch_write_receipt",
            Self::SelfDescribingEffectEnvelope => "self_describing_effect_envelope",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectPublicSurfaceKind {
    CommonPathIntentAuthoring,
    WritebackCommonPath,
    InspectableLoweredPlan,
    SupportDiscovery,
    DenialOrRebind,
    BatchExecution,
    DiagnosticsEnvelope,
    ProductionCertification,
    HiddenLowerRuntimeTypes,
}

impl EffectPublicSurfaceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CommonPathIntentAuthoring => "common_path_intent_authoring",
            Self::WritebackCommonPath => "writeback_common_path",
            Self::InspectableLoweredPlan => "inspectable_lowered_plan",
            Self::SupportDiscovery => "support_discovery",
            Self::DenialOrRebind => "denial_or_rebind",
            Self::BatchExecution => "batch_execution",
            Self::DiagnosticsEnvelope => "diagnostics_envelope",
            Self::ProductionCertification => "production_certification",
            Self::HiddenLowerRuntimeTypes => "hidden_lower_runtime_types",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectPublicSurfaceAvailability {
    Implemented,
    DeferredToPhase5,
}

impl EffectPublicSurfaceAvailability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::DeferredToPhase5 => "deferred_to_phase5",
        }
    }
}
