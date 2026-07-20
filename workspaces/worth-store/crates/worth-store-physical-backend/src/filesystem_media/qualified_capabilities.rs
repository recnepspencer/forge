use super::{MediaHandleIdentity, MediaQualificationIdentity};
use core::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCapabilityScope {
    AdmittedNamespace,
    OpenFile(MediaHandleIdentity),
}

#[derive(Debug)]
pub struct QualifiedBaseMediaCapabilities {
    qualification: MediaQualificationIdentity,
    scope: MediaCapabilityScope,
    claims: BaseCapabilityClaims,
}

#[derive(Debug)]
struct BaseCapabilityClaims {
    buffered_file: crate::BackendCapabilityClaimWitness,
    file_sync: crate::BackendCapabilityClaimWitness,
    directory_sync: crate::BackendCapabilityClaimWitness,
    durable_rename: crate::BackendCapabilityClaimWitness,
}

impl QualifiedBaseMediaCapabilities {
    pub const fn scope(&self) -> MediaCapabilityScope {
        self.scope
    }

    pub const fn qualification(&self) -> MediaQualificationIdentity {
        self.qualification
    }

    pub const fn claim_kinds(&self) -> [crate::BackendCapabilityKind; 4] {
        [
            self.claims.buffered_file.kind(),
            self.claims.file_sync.kind(),
            self.claims.directory_sync.kind(),
            self.claims.durable_rename.kind(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSyncMetadataPosture {
    MetadataDurabilityNotEstablished,
}

#[derive(Debug)]
pub struct QualifiedDataSyncCapability {
    qualification: MediaQualificationIdentity,
    scope: MediaCapabilityScope,
    metadata_posture: DataSyncMetadataPosture,
    claim: crate::BackendCapabilityClaimWitness,
}

impl QualifiedDataSyncCapability {
    pub const fn scope(&self) -> MediaCapabilityScope {
        self.scope
    }

    pub const fn qualification(&self) -> MediaQualificationIdentity {
        self.qualification
    }

    pub const fn metadata_posture(&self) -> DataSyncMetadataPosture {
        self.metadata_posture
    }

    pub const fn claim_kind(&self) -> crate::BackendCapabilityKind {
        self.claim.kind()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationLengthPosture {
    PreservesLogicalLength,
    ExtendsToRequestedEnd,
}

#[derive(Debug)]
pub struct QualifiedSparseAllocationCapability {
    qualification: MediaQualificationIdentity,
    scope: MediaCapabilityScope,
    granularity: NonZeroU64,
    length_posture: AllocationLengthPosture,
}

impl QualifiedSparseAllocationCapability {
    pub const fn scope(&self) -> MediaCapabilityScope {
        self.scope
    }

    pub const fn qualification(&self) -> MediaQualificationIdentity {
        self.qualification
    }

    pub const fn granularity(&self) -> NonZeroU64 {
        self.granularity
    }

    pub const fn length_posture(&self) -> AllocationLengthPosture {
        self.length_posture
    }
}

#[derive(Debug)]
pub struct QualifiedPreallocationCapability {
    qualification: MediaQualificationIdentity,
    scope: MediaCapabilityScope,
    granularity: NonZeroU64,
    length_posture: AllocationLengthPosture,
}

impl QualifiedPreallocationCapability {
    pub const fn scope(&self) -> MediaCapabilityScope {
        self.scope
    }

    pub const fn qualification(&self) -> MediaQualificationIdentity {
        self.qualification
    }

    pub const fn granularity(&self) -> NonZeroU64 {
        self.granularity
    }

    pub const fn length_posture(&self) -> AllocationLengthPosture {
        self.length_posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappedTruncationPosture {
    RequiresUnmap,
    PermittedWhileMapped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappedDurabilityPosture {
    RequiresFileStateSynchronization,
    DataOnlySynchronizationSufficient,
}

#[derive(Debug)]
pub struct QualifiedMmapCapability {
    qualification: MediaQualificationIdentity,
    scope: MediaCapabilityScope,
    mapping_granularity: NonZeroU64,
    truncation_posture: MappedTruncationPosture,
    durability_posture: MappedDurabilityPosture,
}

impl QualifiedMmapCapability {
    pub const fn scope(&self) -> MediaCapabilityScope {
        self.scope
    }

    pub const fn qualification(&self) -> MediaQualificationIdentity {
        self.qualification
    }

    pub const fn mapping_granularity(&self) -> NonZeroU64 {
        self.mapping_granularity
    }

    pub const fn truncation_posture(&self) -> MappedTruncationPosture {
        self.truncation_posture
    }

    pub const fn durability_posture(&self) -> MappedDurabilityPosture {
        self.durability_posture
    }
}

#[derive(Debug)]
pub struct QualifiedDirectIoCapability {
    qualification: MediaQualificationIdentity,
    scope: MediaCapabilityScope,
    memory_alignment: NonZeroU64,
    transfer_granularity: NonZeroU64,
    offset_granularity: NonZeroU64,
}

impl QualifiedDirectIoCapability {
    pub const fn scope(&self) -> MediaCapabilityScope {
        self.scope
    }

    pub const fn qualification(&self) -> MediaQualificationIdentity {
        self.qualification
    }

    pub const fn memory_alignment(&self) -> NonZeroU64 {
        self.memory_alignment
    }

    pub const fn transfer_granularity(&self) -> NonZeroU64 {
        self.transfer_granularity
    }

    pub const fn offset_granularity(&self) -> NonZeroU64 {
        self.offset_granularity
    }
}

/// Concrete handles produced only by the later root qualification owner.
#[derive(Debug)]
pub struct QualifiedMediaCapabilities {
    base: QualifiedBaseMediaCapabilities,
    data_sync: Option<QualifiedDataSyncCapability>,
    sparse_allocation: Option<QualifiedSparseAllocationCapability>,
    preallocation: Option<QualifiedPreallocationCapability>,
    memory_map: Option<QualifiedMmapCapability>,
    direct_io: Option<QualifiedDirectIoCapability>,
}

impl QualifiedMediaCapabilities {
    pub(super) fn for_observed_profile(
        qualification: MediaQualificationIdentity,
        profile: &super::FilesystemBackendProfile,
        buffered_file: crate::BackendCapabilityClaimWitness,
        file_sync: crate::BackendCapabilityClaimWitness,
        directory_sync: crate::BackendCapabilityClaimWitness,
        durable_rename: crate::BackendCapabilityClaimWitness,
    ) -> Self {
        let scope = MediaCapabilityScope::AdmittedNamespace;
        let base = QualifiedBaseMediaCapabilities {
            qualification,
            scope,
            claims: BaseCapabilityClaims {
                buffered_file,
                file_sync,
                directory_sync,
                durable_rename,
            },
        };
        let data_sync = (profile.support(super::MediaCapability::DataOnlySynchronization)
            == super::CapabilitySupport::Supported)
            .then_some(QualifiedDataSyncCapability {
                qualification,
                scope,
                metadata_posture: DataSyncMetadataPosture::MetadataDurabilityNotEstablished,
                claim: file_sync,
            });
        let sparse_allocation = (profile.support(super::MediaCapability::SparseAllocation)
            == super::CapabilitySupport::Supported)
            .then_some(QualifiedSparseAllocationCapability {
                qualification,
                scope,
                granularity: profile.allocation_granularity(),
                length_posture: AllocationLengthPosture::PreservesLogicalLength,
            });
        let preallocation = (profile.support(super::MediaCapability::EagerAllocation)
            == super::CapabilitySupport::Supported)
            .then_some(QualifiedPreallocationCapability {
                qualification,
                scope,
                granularity: profile.allocation_granularity(),
                length_posture: AllocationLengthPosture::PreservesLogicalLength,
            });
        Self {
            base,
            data_sync,
            sparse_allocation,
            preallocation,
            memory_map: None,
            direct_io: None,
        }
    }

    pub const fn base(&self) -> &QualifiedBaseMediaCapabilities {
        &self.base
    }

    pub const fn data_sync(&self) -> Option<&QualifiedDataSyncCapability> {
        self.data_sync.as_ref()
    }

    pub const fn sparse_allocation(&self) -> Option<&QualifiedSparseAllocationCapability> {
        self.sparse_allocation.as_ref()
    }

    pub const fn preallocation(&self) -> Option<&QualifiedPreallocationCapability> {
        self.preallocation.as_ref()
    }

    pub const fn memory_map(&self) -> Option<&QualifiedMmapCapability> {
        self.memory_map.as_ref()
    }

    pub const fn direct_io(&self) -> Option<&QualifiedDirectIoCapability> {
        self.direct_io.as_ref()
    }
}
