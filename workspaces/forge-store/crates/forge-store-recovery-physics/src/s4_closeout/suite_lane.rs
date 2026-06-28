#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecoveryPhysicsCloseoutSuiteLane {
    RecoveryEntryAuthority,
    WalLsnTopology,
    ValidWalPrefixClassification,
    DurabilityBarrierAndAck,
    BackendProfileCertification,
    WalBeforeDataPageLsn,
    NoUndoPublication,
    CheckpointManifestPublication,
    CheckpointLocatorCaptureMode,
    WalRetentionTruncation,
    RecoverySourcePrecedence,
    SourceRoleCompactionVisibility,
    IdempotentRedoReplay,
    RedoRecordGrammar,
    PartialPublicationClassification,
    BoundedRecoveryBudget,
    CrashMatrixFaultScheduler,
    FreshRuntimeCrashIsolation,
    OfflineVerifierIndependence,
    RecoveryDeterminism,
    FoundationalRecoveryEvidence,
    FoundationalAspecBoundaryPayload,
    FoundationalDiagnosticRecoveryBundle,
    FoundationalRecoveryPerformanceClaim,
    FoundationalRecoveryAdoption,
    FoundationalNonApplicableSurfaceDenial,
    ProofProgressionRecoveryState,
    ProofRecoveryAdoption,
    SyntheticRecoveryTestRejection,
    RecoveryMutationValidation,
    S5RecoveryReadinessHandoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryPhysicsCloseoutSuiteRequirement {
    lane: RecoveryPhysicsCloseoutSuiteLane,
    positive_control: bool,
    hostile_lane: bool,
    reopen_lane: bool,
    forbidden_shortcut_lane: bool,
    counter_expectations: bool,
}

impl RecoveryPhysicsCloseoutSuiteRequirement {
    pub const fn complete(lane: RecoveryPhysicsCloseoutSuiteLane) -> Self {
        Self {
            lane,
            positive_control: true,
            hostile_lane: true,
            reopen_lane: true,
            forbidden_shortcut_lane: true,
            counter_expectations: true,
        }
    }

    pub const fn lane(self) -> RecoveryPhysicsCloseoutSuiteLane {
        self.lane
    }

    pub const fn is_complete(self) -> bool {
        self.positive_control
            && self.hostile_lane
            && self.reopen_lane
            && self.forbidden_shortcut_lane
            && self.counter_expectations
    }
}

pub(crate) const REQUIRED_S4_CLOSEOUT_LANES: [RecoveryPhysicsCloseoutSuiteLane; 31] = [
    RecoveryPhysicsCloseoutSuiteLane::RecoveryEntryAuthority,
    RecoveryPhysicsCloseoutSuiteLane::WalLsnTopology,
    RecoveryPhysicsCloseoutSuiteLane::ValidWalPrefixClassification,
    RecoveryPhysicsCloseoutSuiteLane::DurabilityBarrierAndAck,
    RecoveryPhysicsCloseoutSuiteLane::BackendProfileCertification,
    RecoveryPhysicsCloseoutSuiteLane::WalBeforeDataPageLsn,
    RecoveryPhysicsCloseoutSuiteLane::NoUndoPublication,
    RecoveryPhysicsCloseoutSuiteLane::CheckpointManifestPublication,
    RecoveryPhysicsCloseoutSuiteLane::CheckpointLocatorCaptureMode,
    RecoveryPhysicsCloseoutSuiteLane::WalRetentionTruncation,
    RecoveryPhysicsCloseoutSuiteLane::RecoverySourcePrecedence,
    RecoveryPhysicsCloseoutSuiteLane::SourceRoleCompactionVisibility,
    RecoveryPhysicsCloseoutSuiteLane::IdempotentRedoReplay,
    RecoveryPhysicsCloseoutSuiteLane::RedoRecordGrammar,
    RecoveryPhysicsCloseoutSuiteLane::PartialPublicationClassification,
    RecoveryPhysicsCloseoutSuiteLane::BoundedRecoveryBudget,
    RecoveryPhysicsCloseoutSuiteLane::CrashMatrixFaultScheduler,
    RecoveryPhysicsCloseoutSuiteLane::FreshRuntimeCrashIsolation,
    RecoveryPhysicsCloseoutSuiteLane::OfflineVerifierIndependence,
    RecoveryPhysicsCloseoutSuiteLane::RecoveryDeterminism,
    RecoveryPhysicsCloseoutSuiteLane::FoundationalRecoveryEvidence,
    RecoveryPhysicsCloseoutSuiteLane::FoundationalAspecBoundaryPayload,
    RecoveryPhysicsCloseoutSuiteLane::FoundationalDiagnosticRecoveryBundle,
    RecoveryPhysicsCloseoutSuiteLane::FoundationalRecoveryPerformanceClaim,
    RecoveryPhysicsCloseoutSuiteLane::FoundationalRecoveryAdoption,
    RecoveryPhysicsCloseoutSuiteLane::FoundationalNonApplicableSurfaceDenial,
    RecoveryPhysicsCloseoutSuiteLane::ProofProgressionRecoveryState,
    RecoveryPhysicsCloseoutSuiteLane::ProofRecoveryAdoption,
    RecoveryPhysicsCloseoutSuiteLane::SyntheticRecoveryTestRejection,
    RecoveryPhysicsCloseoutSuiteLane::RecoveryMutationValidation,
    RecoveryPhysicsCloseoutSuiteLane::S5RecoveryReadinessHandoff,
];
