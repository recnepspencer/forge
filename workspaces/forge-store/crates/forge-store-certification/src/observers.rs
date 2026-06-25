#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioObserverKind {
    CounterBundle,
    DenialBoundary,
    RuntimeLayout,
    OfflineVerifier,
    StorageBoundary,
    EvidenceExport,
    MaterializationShortcut,
}

impl PhysicalScenarioObserverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CounterBundle => "counter_bundle",
            Self::DenialBoundary => "denial_boundary",
            Self::RuntimeLayout => "runtime_layout",
            Self::OfflineVerifier => "offline_verifier",
            Self::StorageBoundary => "storage_boundary",
            Self::EvidenceExport => "evidence_export",
            Self::MaterializationShortcut => "materialization_shortcut",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalScenarioObserverRequirement {
    kind: PhysicalScenarioObserverKind,
}

impl PhysicalScenarioObserverRequirement {
    pub const fn new(kind: PhysicalScenarioObserverKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> PhysicalScenarioObserverKind {
        self.kind
    }
}
