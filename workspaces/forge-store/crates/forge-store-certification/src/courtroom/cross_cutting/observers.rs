#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalScenarioObserverKind {
    CounterBundle,
    DenialBoundary,
    AllocationEnvelope,
    Materialization,
    ResidentBudget,
    RuntimeLayout,
    OfflineVerifier,
    StorageBoundary,
    EvidenceExport,
    MaterializationShortcut,
    S3PreDecodeAdmission,
    S3SemanticDecoderInvocation,
    S3PhysicalLocality,
    S3DamageClassification,
    S3Quarantine,
    S3RecoveryHandoff,
    S3LineCapComposition,
}

impl PhysicalScenarioObserverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CounterBundle => "counter_bundle",
            Self::DenialBoundary => "denial_boundary",
            Self::AllocationEnvelope => "allocation_envelope",
            Self::Materialization => "materialization",
            Self::ResidentBudget => "resident_budget",
            Self::RuntimeLayout => "runtime_layout",
            Self::OfflineVerifier => "offline_verifier",
            Self::StorageBoundary => "storage_boundary",
            Self::EvidenceExport => "evidence_export",
            Self::MaterializationShortcut => "materialization_shortcut",
            Self::S3PreDecodeAdmission => "s3_pre_decode_admission",
            Self::S3SemanticDecoderInvocation => "s3_semantic_decoder_invocation",
            Self::S3PhysicalLocality => "s3_physical_locality",
            Self::S3DamageClassification => "s3_damage_classification",
            Self::S3Quarantine => "s3_quarantine",
            Self::S3RecoveryHandoff => "s3_recovery_handoff",
            Self::S3LineCapComposition => "s3_line_cap_composition",
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
