#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeConditionalDenialKind {
    ForeignSignalGraph,
    CorrespondenceAdmission,
    EmptyCorrespondenceSet,
    MixedSignalNodes,
    DeclarationLocationMismatch,
    DeclarationCorrespondenceMismatch,
    SignalNodeAlreadyBound,
    MissingConditionProvider,
    ExtraConditionProvider,
    MissingDependencyComparator,
    ExtraDependencyComparator,
    MissingOutputComparator,
    ExtraOutputComparator,
    MissingReuseComparator,
    ExtraReuseComparator,
    MissingTriggerProvider,
    ExtraTriggerProvider,
    MissingWakeProvider,
    ExtraWakeProvider,
    MissingComputeProvider,
    ExtraComputeProvider,
    UnsupportedMaintenancePosture,
    UnsupportedArtifactPosture,
    SignalContractInstallation,
    StaleLowering,
    OperationAuthorityMismatch,
    GraphAuthorityMismatch,
    SignalContractMismatch,
    DependencyOrdinalMismatch,
    SnapshotMismatch,
    SnapshotAdmission,
    AttemptMismatch,
    SignalExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeConditionalDenial {
    kind: BridgeConditionalDenialKind,
    detail: String,
    signal_counters: worth_signal::facade::SignalConditionalDecisionCounters,
    semantic_observation_reads: usize,
}

impl BridgeConditionalDenial {
    pub(crate) fn new(kind: BridgeConditionalDenialKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
            signal_counters: worth_signal::facade::SignalConditionalDecisionCounters::default(),
            semantic_observation_reads: 0,
        }
    }
    pub(crate) fn with_execution_counters(
        mut self,
        signal_counters: worth_signal::facade::SignalConditionalDecisionCounters,
        semantic_observation_reads: usize,
    ) -> Self {
        self.signal_counters = signal_counters;
        self.semantic_observation_reads = semantic_observation_reads;
        self
    }
    pub const fn kind(&self) -> BridgeConditionalDenialKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
    pub const fn signal_counters(&self) -> worth_signal::facade::SignalConditionalDecisionCounters {
        self.signal_counters
    }
    pub const fn semantic_observation_reads(&self) -> usize {
        self.semantic_observation_reads
    }
}
