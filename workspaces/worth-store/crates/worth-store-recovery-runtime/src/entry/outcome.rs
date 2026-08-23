use worth_store::physical_runtime::{
    RecoveredPhysicalRuntimeConstructionDenial, RecoveryDiscoveryFailure,
    RecoveryFilesystemQualificationError, StoreRecoveryBindingSampleDenial,
};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordArtifactFile};
use worth_store_recovery_physics::{
    OperationReconciliationDenial, PhysicalRedoPlanningDenial, PhysicalRedoTargetIdentity,
    RecoveryPlanCostDenial, RecoveryPlanningCounters,
};

use super::PhysicalRecoveryEntryBindingDrift;
use crate::progression::PhysicalRecoveryDiscoveryCounters;

#[derive(Debug)]
pub enum PhysicalRecoveryOutcome {
    Recovered(crate::handoff::RecoveredPhysicalRuntimeHandoff),
    Refused(PhysicalRecoveryRefusal),
    Blocked(PhysicalRecoveryBlock),
    PublicationIndeterminate(PhysicalRecoveryPublicationIndeterminate),
}

#[derive(Debug)]
pub struct PhysicalRecoveryPublicationIndeterminate {
    store: StableStoreIdentity,
    session: super::PhysicalRecoverySessionIdentity,
    counters: super::PhysicalRecoveryPublicationCounters,
    settlement: super::PhysicalRecoveryPublicationSettlementLedger,
    reopen: Option<super::PhysicalRecoveryReopenFailure>,
    handoff: Option<RecoveredPhysicalRuntimeConstructionDenial>,
    recovery_effects: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRecoveryRefusal {
    pub kind: PhysicalRecoveryRefusalKind,
    recovery_effects: u64,
}

impl PhysicalRecoveryRefusal {
    pub(crate) const fn new(kind: PhysicalRecoveryRefusalKind, recovery_effects: u64) -> Self {
        Self {
            kind,
            recovery_effects,
        }
    }

    pub const fn recovery_effects(&self) -> u64 {
        self.recovery_effects
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoveryRefusalKind {
    CancelledBeforeDiscovery,
    CancelledBeforeReconstruction,
    CancelledBeforeExecution,
    EntryBindingDrift(PhysicalRecoveryEntryBindingDrift),
    PersistedStoreAdmission(RecoveryFilesystemQualificationError),
    CoordinationUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoveryBlockKind {
    DiscoveryLimit,
    MediaObservation,
    RootProtocol,
    Checkpoint,
    WalInventory,
    SourceSelection,
    BindingFreshness,
    PageAdmission,
    OperationReconciliation,
    RedoPlanning,
    Staging,
    Publication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoveryLimitDimension {
    SelectorCandidates,
    ManifestBytes,
    ManifestEntries,
    WalSegments,
    WalFrames,
    WalBytes,
    DistinctPagesAndExtents,
    ObservationBytes,
    OperationBindings,
    RedoTargets,
    RedoBytes,
    StagingBytes,
    RecoveryMemoryBytes,
    DirtyFrames,
    PublicationEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecoveryLimitFailure {
    pub dimension: PhysicalRecoveryLimitDimension,
    pub observed: u64,
    pub admitted: u64,
}

#[derive(Debug, Default)]
pub struct PhysicalRecoveryBlockEvidence {
    pub counters: PhysicalRecoveryDiscoveryCounters,
    pub planning_counters: Option<RecoveryPlanningCounters>,
    pub limit: Option<PhysicalRecoveryLimitFailure>,
    pub artifact: Option<String>,
    pub source_generation: Option<u64>,
    pub lsn: Option<u64>,
    pub source_denials: Vec<super::PhysicalRecoverySourceDenial>,
    pub planning_denial: Option<PhysicalRecoveryPlanningDenial>,
    pub staging_counters: Option<super::PhysicalRecoveryStagingCounters>,
    pub staging_denial: Option<super::PhysicalRecoveryStagingDenial>,
    pub staging_settlements: Option<super::PhysicalRecoveryStagingSettlementLedger>,
    pub publication_counters: Option<super::PhysicalRecoveryPublicationCounters>,
    pub publication_denial: Option<super::PhysicalRecoveryPublicationDenial>,
    pub publication_settlements: Option<super::PhysicalRecoveryPublicationSettlementLedger>,
}

impl PhysicalRecoveryPublicationIndeterminate {
    pub(crate) const fn new(
        store: StableStoreIdentity,
        session: super::PhysicalRecoverySessionIdentity,
        counters: super::PhysicalRecoveryPublicationCounters,
        settlement: super::PhysicalRecoveryPublicationSettlementLedger,
        recovery_effects: u64,
    ) -> Self {
        Self {
            store,
            session,
            counters,
            settlement,
            reopen: None,
            handoff: None,
            recovery_effects,
        }
    }
    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }
    pub const fn session_identity(&self) -> super::PhysicalRecoverySessionIdentity {
        self.session
    }
    pub const fn counters(&self) -> super::PhysicalRecoveryPublicationCounters {
        self.counters
    }
    pub const fn settlement(&self) -> &super::PhysicalRecoveryPublicationSettlementLedger {
        &self.settlement
    }
    pub const fn recovery_effects(&self) -> u64 {
        self.recovery_effects
    }

    pub(crate) fn with_reopen_failure(
        mut self,
        failure: super::PhysicalRecoveryReopenFailure,
    ) -> Self {
        self.reopen = Some(failure);
        self
    }

    pub const fn reopen_failure(&self) -> Option<&super::PhysicalRecoveryReopenFailure> {
        self.reopen.as_ref()
    }

    pub(crate) fn with_handoff_failure(
        mut self,
        failure: RecoveredPhysicalRuntimeConstructionDenial,
    ) -> Self {
        self.handoff = Some(failure);
        self
    }

    pub const fn handoff_failure(&self) -> Option<RecoveredPhysicalRuntimeConstructionDenial> {
        self.handoff
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalRecoveryPlanningDenial {
    BindingFreshness(StoreRecoveryBindingSampleDenial),
    OperationReconciliation(OperationReconciliationDenial),
    Redo(PhysicalRedoPlanningDenial),
    Page(PhysicalRecoveryPageAdmissionDenial),
    Cost(RecoveryPlanCostDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalRecoveryPageAdmissionDenial {
    Media {
        target: Option<PhysicalRedoTargetIdentity>,
        failure: RecoveryDiscoveryFailure,
    },
    MissingArtifact {
        target: Option<PhysicalRedoTargetIdentity>,
        artifact: RecordArtifactFile,
    },
    InvalidManifest {
        target: Option<PhysicalRedoTargetIdentity>,
        artifact: RecordArtifactFile,
    },
    InvalidTarget(PhysicalRedoTargetIdentity),
    InvalidPage(PhysicalRedoTargetIdentity),
    ManifestEntryLimit,
    ObservationByteLimit,
}

#[derive(Debug)]
pub struct PhysicalRecoveryBlock {
    pub kind: PhysicalRecoveryBlockKind,
    store: StableStoreIdentity,
    session: super::PhysicalRecoverySessionIdentity,
    evidence: PhysicalRecoveryBlockEvidence,
    recovery_effects: u64,
}

impl PhysicalRecoveryBlock {
    pub(crate) const fn new(
        kind: PhysicalRecoveryBlockKind,
        store: StableStoreIdentity,
        session: super::PhysicalRecoverySessionIdentity,
        evidence: PhysicalRecoveryBlockEvidence,
        recovery_effects: u64,
    ) -> Self {
        Self {
            kind,
            store,
            session,
            evidence,
            recovery_effects,
        }
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn session_identity(&self) -> super::PhysicalRecoverySessionIdentity {
        self.session
    }

    pub const fn evidence(&self) -> &PhysicalRecoveryBlockEvidence {
        &self.evidence
    }

    pub const fn recovery_effects(&self) -> u64 {
        self.recovery_effects
    }
}
