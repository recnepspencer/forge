#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Keep the protocol's `Artifact*` vocabulary explicit so later non-artifact
// work families cannot make existing variants semantically ambiguous.
#[allow(clippy::enum_variant_names)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyWorkFamily {
    ArtifactMetadataRead,
    ArtifactRangeRead,
    ArtifactRangeWrite,
    ArtifactPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Completion is part of each wire-level fate, not redundant decoration: later
// terminal failure fates must remain distinguishable from successful effects.
#[allow(clippy::enum_variant_names)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyWorkEffectFate {
    ReadCompleted,
    WriteCompleted,
    PublicationCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyWorkRecovery {
    NoEffect,
    ContinueSettlement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyWorkTerminalFate {
    Settled,
    ContinuedAfterConsumerCancellation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencyMediaRole {
    CreateNew,
    PositionedRead,
    PositionedWrite,
    ReadMetadata,
    SynchronizeFileState,
    SynchronizeDirectoryPublication,
    AtomicReplace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencySignalFamily {
    ReadFault,
    ExactWriteback,
    Publication,
    Lifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencySignalAspectRole {
    Dependency,
    Output,
    DependencyAndOutput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencySignalFamilySet {
    pub(in crate::courtroom_campaign::bounded_residency_siege) read_fault: bool,
    pub(in crate::courtroom_campaign::bounded_residency_siege) exact_writeback: bool,
    pub(in crate::courtroom_campaign::bounded_residency_siege) publication: bool,
    pub(in crate::courtroom_campaign::bounded_residency_siege) lifecycle: bool,
}

impl BoundedResidencySignalFamilySet {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn serves(
        self,
        family: BoundedResidencySignalFamily,
    ) -> bool {
        match family {
            BoundedResidencySignalFamily::ReadFault => self.read_fault,
            BoundedResidencySignalFamily::ExactWriteback => self.exact_writeback,
            BoundedResidencySignalFamily::Publication => self.publication,
            BoundedResidencySignalFamily::Lifecycle => self.lifecycle,
        }
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn is_empty(self) -> bool {
        !self.read_fault && !self.exact_writeback && !self.publication && !self.lifecycle
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencySignalBindingObservation
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) digest: [u8; 32],
    pub(in crate::courtroom_campaign::bounded_residency_siege) aspect_key: String,
    pub(in crate::courtroom_campaign::bounded_residency_siege) role:
        BoundedResidencySignalAspectRole,
    pub(in crate::courtroom_campaign::bounded_residency_siege) families:
        BoundedResidencySignalFamilySet,
    pub(in crate::courtroom_campaign::bounded_residency_siege) partition: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencySchedulerProfile {
    SimulatedStrictDurable,
    PosixFileFsyncDirSync,
    WindowsFlushFileBuffers,
    MmapFlushNotDurabilityCertified,
    AdversarialLostFlush,
    AdversarialReorderedFlush,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencySchedulerEvidenceClass
{
    DeclaredByConfig,
    ObservedByProbe,
    EstablishedByFilesystemAdmission,
    ExternallyGuaranteed,
    UnverifiableAssumption,
    CertifiedBackendProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) enum BoundedResidencySignalSettlement {
    Committed,
    ReconciledFromPhysicalTruth,
    DerivedStateUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencySignalLineageObservation
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) request: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) generation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) branch: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) restore_epoch: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyWorkRouteObservation
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) signal:
        BoundedResidencySignalLineageObservation,
    pub(in crate::courtroom_campaign::bounded_residency_siege) predecessor:
        Option<BoundedResidencySignalLineageObservation>,
    pub(in crate::courtroom_campaign::bounded_residency_siege) signal_attempt: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) signal_family:
        BoundedResidencySignalFamily,
    pub(in crate::courtroom_campaign::bounded_residency_siege) signal_binding: [u8; 32],
    pub(in crate::courtroom_campaign::bounded_residency_siege) scheduler_profile:
        BoundedResidencySchedulerProfile,
    pub(in crate::courtroom_campaign::bounded_residency_siege) scheduler_evidence_class:
        BoundedResidencySchedulerEvidenceClass,
    pub(in crate::courtroom_campaign::bounded_residency_siege) scheduler_grouped_writes: u32,
    pub(in crate::courtroom_campaign::bounded_residency_siege) scheduler_primary_requirement: u8,
    pub(in crate::courtroom_campaign::bounded_residency_siege) scheduler_secondary_present: bool,
    pub(in crate::courtroom_campaign::bounded_residency_siege) signal_settlement:
        BoundedResidencySignalSettlement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyWorkRecordObservation
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) store: [u8; 16],
    pub(in crate::courtroom_campaign::bounded_residency_siege) runtime: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) generation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) operation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) family: BoundedResidencyWorkFamily,
    pub(in crate::courtroom_campaign::bounded_residency_siege) backend_operation: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) backend_role:
        BoundedResidencyMediaRole,
    pub(in crate::courtroom_campaign::bounded_residency_siege) effect_fate:
        BoundedResidencyWorkEffectFate,
    pub(in crate::courtroom_campaign::bounded_residency_siege) recovery:
        BoundedResidencyWorkRecovery,
    pub(in crate::courtroom_campaign::bounded_residency_siege) route:
        BoundedResidencyWorkRouteObservation,
    pub(in crate::courtroom_campaign::bounded_residency_siege) terminal:
        BoundedResidencyWorkTerminalFate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct BoundedResidencyWorkReconciliationObservation
{
    pub(in crate::courtroom_campaign::bounded_residency_siege) causal_overflow: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) terminal_overflow: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) safe_evidence_elided: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) faults: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) source_loads: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) exact_writebacks: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) identified_metadata_reads: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) identified_positioned_reads: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) identified_positioned_writes: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) settled_terminal_fates: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) continued_terminal_fates: u64,
    pub(in crate::courtroom_campaign::bounded_residency_siege) signal_bindings:
        Box<[BoundedResidencySignalBindingObservation]>,
    pub(in crate::courtroom_campaign::bounded_residency_siege) records:
        Box<[BoundedResidencyWorkRecordObservation]>,
}

#[cfg(test)]
pub(in crate::courtroom_campaign::bounded_residency_siege) fn exact_route_fixture(
    operation: u64,
    family: BoundedResidencyWorkFamily,
    signal_binding: [u8; 32],
) -> BoundedResidencyWorkRouteObservation {
    let signal_family = match family {
        BoundedResidencyWorkFamily::ArtifactMetadataRead
        | BoundedResidencyWorkFamily::ArtifactRangeRead => BoundedResidencySignalFamily::ReadFault,
        BoundedResidencyWorkFamily::ArtifactRangeWrite => {
            BoundedResidencySignalFamily::ExactWriteback
        }
        BoundedResidencyWorkFamily::ArtifactPublication => {
            BoundedResidencySignalFamily::Publication
        }
    };
    BoundedResidencyWorkRouteObservation {
        signal: BoundedResidencySignalLineageObservation {
            request: operation + 100,
            generation: 3,
            branch: 1,
            restore_epoch: 0,
        },
        predecessor: None,
        signal_attempt: operation + 200,
        signal_family,
        signal_binding,
        scheduler_profile: BoundedResidencySchedulerProfile::PosixFileFsyncDirSync,
        scheduler_evidence_class:
            BoundedResidencySchedulerEvidenceClass::EstablishedByFilesystemAdmission,
        scheduler_grouped_writes: 1,
        scheduler_primary_requirement: 1,
        scheduler_secondary_present: false,
        signal_settlement: BoundedResidencySignalSettlement::Committed,
    }
}
