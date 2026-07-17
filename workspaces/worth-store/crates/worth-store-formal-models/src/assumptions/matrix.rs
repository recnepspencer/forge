use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionDenial,
    BackendCapabilityClaimWitness, BackendCapabilityKind, BackendDurabilityProfile,
    BackendDurabilityProfileId, BackendDurabilitySupport, BackendTargetProfile,
    CapabilityEvidenceClass,
};

use crate::ProtocolFamily;

use super::{
    ChecksumCoverageAssumption, ClockOrderingAssumption, IoBufferingAssumption,
    ModeledBackendDurabilityAssumption, PublicationAtomicityAssumption, TornWriteAssumption,
    WriteCompletionAssumption,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolBackendAssumptionRow {
    protocol: ProtocolFamily,
    write_completion: WriteCompletionAssumption,
    publication_atomicity: PublicationAtomicityAssumption,
    torn_write: TornWriteAssumption,
    buffering: IoBufferingAssumption,
    checksum_coverage: ChecksumCoverageAssumption,
    clock_ordering: ClockOrderingAssumption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedProtocolBackendProfile {
    row: ProtocolBackendAssumptionRow,
    durability: ModeledBackendDurabilityAssumption,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedProtocolBackendAssumptions {
    profile: SupportedProtocolBackendProfile,
    capabilities: Vec<BackendCapabilityClaimWitness>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedProtocolBackendProfile {
    protocol: ProtocolFamily,
    profile: BackendDurabilityProfileId,
    support: BackendDurabilitySupport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolBackendCapabilityDenial {
    UnsupportedProfile(UnsupportedProtocolBackendProfile),
    RuntimeProfileMismatch {
        expected: BackendTargetProfile,
        actual: BackendTargetProfile,
    },
    Capability(BackendCapabilityAdmissionDenial),
}

pub fn admit_protocol_backend_profile<P: BackendDurabilityProfile>(
    protocol: ProtocolFamily,
) -> Result<SupportedProtocolBackendProfile, UnsupportedProtocolBackendProfile> {
    let durability = ModeledBackendDurabilityAssumption::from_runtime_profile::<P>();
    if durability.support() != BackendDurabilitySupport::Certified {
        return Err(UnsupportedProtocolBackendProfile {
            protocol,
            profile: durability.runtime_profile(),
            support: durability.support(),
        });
    }
    Ok(SupportedProtocolBackendProfile {
        row: protocol_backend_assumption_row(protocol),
        durability,
    })
}

pub fn admit_protocol_backend_capabilities<P: BackendDurabilityProfile>(
    protocol: ProtocolFamily,
    runtime: &AdmittedBackendCapabilityWitness,
) -> Result<AdmittedProtocolBackendAssumptions, ProtocolBackendCapabilityDenial> {
    let profile = admit_protocol_backend_profile::<P>(protocol)
        .map_err(ProtocolBackendCapabilityDenial::UnsupportedProfile)?;
    if runtime.profile() != P::TARGET {
        return Err(ProtocolBackendCapabilityDenial::RuntimeProfileMismatch {
            expected: P::TARGET,
            actual: runtime.profile(),
        });
    }
    let mut capabilities = Vec::new();
    for kind in required_capabilities(profile.row) {
        capabilities.push(
            runtime
                .require(kind, CapabilityEvidenceClass::CertifiedBackendProfile)
                .map_err(ProtocolBackendCapabilityDenial::Capability)?,
        );
    }
    Ok(AdmittedProtocolBackendAssumptions {
        profile,
        capabilities,
    })
}

pub fn current_protocol_backend_assumption_matrix() -> [ProtocolBackendAssumptionRow; 8] {
    ProtocolFamily::all().map(protocol_backend_assumption_row)
}

pub const fn protocol_backend_assumption_row(
    protocol: ProtocolFamily,
) -> ProtocolBackendAssumptionRow {
    use ProtocolFamily::{
        CompactionVisibility, DurabilityRecovery, ImportPublication, LeaseReclaim,
        QuarantineReadmission, RecoverySourcePrecedence, ReplicationAdmission, SharedFrontiers,
    };

    match protocol {
        LeaseReclaim => ProtocolBackendAssumptionRow::new(
            protocol,
            WriteCompletionAssumption::BackendAccepted,
            PublicationAtomicityAssumption::NoAtomicReplacementClaim,
            TornWriteAssumption::TornPagePossible,
            IoBufferingAssumption::BufferedWriteback,
            ChecksumCoverageAssumption::FrameHeaderAndPayload,
            ClockOrderingAssumption::PersistedLeaseEpochOrdering,
        ),
        QuarantineReadmission => ProtocolBackendAssumptionRow::new(
            protocol,
            WriteCompletionAssumption::BackendAccepted,
            PublicationAtomicityAssumption::NoAtomicReplacementClaim,
            TornWriteAssumption::TornPagePossible,
            IoBufferingAssumption::BufferedWriteback,
            ChecksumCoverageAssumption::PageAndPublicationEnvelope,
            ClockOrderingAssumption::NoClockDependency,
        ),
        DurabilityRecovery
        | RecoverySourcePrecedence
        | CompactionVisibility
        | ImportPublication
        | ReplicationAdmission
        | SharedFrontiers => ProtocolBackendAssumptionRow::new(
            protocol,
            WriteCompletionAssumption::DurabilityFenceCompleted,
            PublicationAtomicityAssumption::RenameNotDurableWithoutDirectoryFence,
            TornWriteAssumption::TornPagePossible,
            IoBufferingAssumption::BufferedWriteback,
            ChecksumCoverageAssumption::PageAndPublicationEnvelope,
            ClockOrderingAssumption::NoClockDependency,
        ),
    }
}

impl ProtocolBackendAssumptionRow {
    const fn new(
        protocol: ProtocolFamily,
        write_completion: WriteCompletionAssumption,
        publication_atomicity: PublicationAtomicityAssumption,
        torn_write: TornWriteAssumption,
        buffering: IoBufferingAssumption,
        checksum_coverage: ChecksumCoverageAssumption,
        clock_ordering: ClockOrderingAssumption,
    ) -> Self {
        Self {
            protocol,
            write_completion,
            publication_atomicity,
            torn_write,
            buffering,
            checksum_coverage,
            clock_ordering,
        }
    }

    pub const fn protocol(self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn write_completion(self) -> WriteCompletionAssumption {
        self.write_completion
    }

    pub const fn publication_atomicity(self) -> PublicationAtomicityAssumption {
        self.publication_atomicity
    }

    pub const fn torn_write(self) -> TornWriteAssumption {
        self.torn_write
    }

    pub const fn buffering(self) -> IoBufferingAssumption {
        self.buffering
    }

    pub const fn checksum_coverage(self) -> ChecksumCoverageAssumption {
        self.checksum_coverage
    }

    pub const fn clock_ordering(self) -> ClockOrderingAssumption {
        self.clock_ordering
    }
}

fn required_capabilities(row: ProtocolBackendAssumptionRow) -> Vec<BackendCapabilityKind> {
    let mut required = vec![BackendCapabilityKind::BufferedFile];
    if row.write_completion == WriteCompletionAssumption::DurabilityFenceCompleted {
        required.push(BackendCapabilityKind::Fsync);
        required.push(BackendCapabilityKind::DirectorySync);
    }
    if row.publication_atomicity
        == PublicationAtomicityAssumption::RenameNotDurableWithoutDirectoryFence
    {
        required.push(BackendCapabilityKind::DurableRename);
    }
    required
}

impl SupportedProtocolBackendProfile {
    pub const fn row(self) -> ProtocolBackendAssumptionRow {
        self.row
    }

    pub const fn durability(self) -> ModeledBackendDurabilityAssumption {
        self.durability
    }
}

impl AdmittedProtocolBackendAssumptions {
    pub const fn profile(&self) -> SupportedProtocolBackendProfile {
        self.profile
    }

    pub fn capabilities(&self) -> &[BackendCapabilityClaimWitness] {
        &self.capabilities
    }
}

impl UnsupportedProtocolBackendProfile {
    pub const fn protocol(self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn profile(self) -> BackendDurabilityProfileId {
        self.profile
    }

    pub const fn support(self) -> BackendDurabilitySupport {
        self.support
    }
}
