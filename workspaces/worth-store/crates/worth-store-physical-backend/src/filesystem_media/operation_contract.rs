macro_rules! define_media_operation_roles {
    ($($role:ident),+ $(,)?) => {
        #[repr(usize)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum MediaOperationRole {
            $($role),+
        }

        impl MediaOperationRole {
            pub const ALL: [Self; define_media_operation_roles!(@count $($role),+)] = [
                $(Self::$role),+
            ];

            pub(super) const fn index(self) -> usize {
                self as usize
            }
        }
    };
    (@count $($role:ident),+) => {
        <[()]>::len(&[$(define_media_operation_roles!(@unit $role)),+])
    };
    (@unit $role:ident) => { () };
}

define_media_operation_roles!(
    OpenRootParent,
    InspectNamespaceEntry,
    CreateDirectory,
    OpenDirectory,
    ValidateRootIdentity,
    ObserveRootProfile,
    OpenMutationLease,
    CreateMutationLease,
    AcquireMutationLease,
    PublishMutationLeaseObservation,
    ReleaseMutationLease,
    OpenExisting,
    CreateNew,
    PositionedRead,
    PositionedWrite,
    Append,
    Truncate,
    Allocate,
    ReadMetadata,
    ListDirectory,
    SynchronizeFileData,
    SynchronizeFileState,
    SynchronizeDirectoryPublication,
    SynchronizeStoreRootPublication,
    SynchronizeRootParentPublication,
    AtomicReplace,
    Delete,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaTransferCardinality {
    None,
    SingleObservation,
    BoundedByteTransfer,
    DirectorySequence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaPartialEffect {
    Impossible,
    BytePrefix,
    BytePrefixOrBarrierIndeterminate,
    LogicalLengthMayChange,
    AllocationMayChange,
    NamespaceMayChange,
    BarrierCompletionMayBeIndeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaSynchronizationMeaning {
    None,
    FileData,
    FileDataAndMetadata,
    ParentNamespacePublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaHandleRequirement {
    NamespaceOwner,
    OpenFile,
    OpenDirectory,
    OwnerIssuedNamespaceEntry,
    SourceAndDestinationDirectory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCapabilityRequirement {
    BaseFilesystem,
    PositionedTransfer,
    Append,
    QualifiedAllocationMode,
    QualifiedDataOnlySynchronization,
    FileStateSynchronization,
    DirectorySynchronization,
    AtomicSameNamespaceReplacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCounterClass {
    AdmissionObservation,
    DirectoryAcquisition,
    OwnershipAcquisition,
    OwnershipPublication,
    HandleAcquisition,
    ReadTransfer,
    WriteTransfer,
    LogicalLengthMutation,
    AllocationMutation,
    MetadataObservation,
    DirectoryObservation,
    SynchronizationBarrier,
    NamespaceMutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaRetryRule {
    RestartReadObservation,
    ContinueFromEstablishedPosition,
    InspectAfterPossibleEffect,
    RetryOnlyAfterDeniedBeforeEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaObservationAudience {
    RuntimeDiagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFaultControlAudience {
    CertificationOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCallAudience {
    MediaOwnerInternal,
    ArtifactOwnerThroughMediaOwner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaOperationContract {
    pub(super) transfer: MediaTransferCardinality,
    pub(super) partial_effect: MediaPartialEffect,
    pub(super) synchronization: MediaSynchronizationMeaning,
    pub(super) handle: MediaHandleRequirement,
    pub(super) capability: MediaCapabilityRequirement,
    pub(super) counter: MediaCounterClass,
    pub(super) audience: MediaCallAudience,
    pub(super) retry: MediaRetryRule,
    pub(super) observation: MediaObservationAudience,
    pub(super) fault_control: MediaFaultControlAudience,
}

impl MediaOperationContract {
    pub const fn transfer(self) -> MediaTransferCardinality {
        self.transfer
    }
    pub const fn partial_effect(self) -> MediaPartialEffect {
        self.partial_effect
    }
    pub const fn synchronization(self) -> MediaSynchronizationMeaning {
        self.synchronization
    }
    pub const fn handle(self) -> MediaHandleRequirement {
        self.handle
    }
    pub const fn capability(self) -> MediaCapabilityRequirement {
        self.capability
    }
    pub const fn counter(self) -> MediaCounterClass {
        self.counter
    }
    pub const fn audience(self) -> MediaCallAudience {
        self.audience
    }
    pub const fn retry(self) -> MediaRetryRule {
        self.retry
    }
    pub const fn observation(self) -> MediaObservationAudience {
        self.observation
    }
    pub const fn fault_control(self) -> MediaFaultControlAudience {
        self.fault_control
    }
}

macro_rules! contract_row {
    (
        transfer: $transfer:expr,
        partial_effect: $partial_effect:expr,
        synchronization: $synchronization:expr,
        handle: $handle:expr,
        capability: $capability:expr,
        counter: $counter:expr,
        audience: $audience:expr,
        retry: $retry:expr $(,)?
    ) => {
        MediaOperationContract {
            transfer: $transfer,
            partial_effect: $partial_effect,
            synchronization: $synchronization,
            handle: $handle,
            capability: $capability,
            counter: $counter,
            audience: $audience,
            retry: $retry,
            observation: MediaObservationAudience::RuntimeDiagnostics,
            fault_control: MediaFaultControlAudience::CertificationOnly,
        }
    };
}

pub(super) const fn operation_contract(operation: MediaOperationRole) -> MediaOperationContract {
    if let Some(contract) = super::admission_operation_contracts::contract(operation) {
        return contract;
    }
    use MediaCallAudience::{
        ArtifactOwnerThroughMediaOwner as Artifact, MediaOwnerInternal as Owner,
    };
    use MediaCapabilityRequirement as Capability;
    use MediaCounterClass as Counter;
    use MediaHandleRequirement as Handle;
    use MediaOperationRole as Operation;
    use MediaPartialEffect as Partial;
    use MediaRetryRule as Retry;
    use MediaSynchronizationMeaning as Sync;
    use MediaTransferCardinality as Transfer;

    match operation {
        Operation::OpenRootParent
        | Operation::InspectNamespaceEntry
        | Operation::CreateDirectory
        | Operation::OpenDirectory
        | Operation::ValidateRootIdentity
        | Operation::ObserveRootProfile
        | Operation::OpenMutationLease
        | Operation::CreateMutationLease
        | Operation::AcquireMutationLease
        | Operation::PublishMutationLeaseObservation
        | Operation::ReleaseMutationLease => {
            panic!("admission operation contract must be routed above")
        }
        Operation::OpenExisting => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::Impossible,
            synchronization: Sync::None,
            handle: Handle::NamespaceOwner,
            capability: Capability::BaseFilesystem,
            counter: Counter::HandleAcquisition,
            audience: Owner,
            retry: Retry::RetryOnlyAfterDeniedBeforeEffect,
        },
        Operation::CreateNew => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::NamespaceMayChange,
            synchronization: Sync::None,
            handle: Handle::NamespaceOwner,
            capability: Capability::BaseFilesystem,
            counter: Counter::HandleAcquisition,
            audience: Owner,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::PositionedRead => contract_row! {
            transfer: Transfer::BoundedByteTransfer,
            partial_effect: Partial::BytePrefix,
            synchronization: Sync::None,
            handle: Handle::OpenFile,
            capability: Capability::PositionedTransfer,
            counter: Counter::ReadTransfer,
            audience: Artifact,
            retry: Retry::ContinueFromEstablishedPosition,
        },
        Operation::PositionedWrite => contract_row! {
            transfer: Transfer::BoundedByteTransfer,
            partial_effect: Partial::BytePrefix,
            synchronization: Sync::None,
            handle: Handle::OpenFile,
            capability: Capability::PositionedTransfer,
            counter: Counter::WriteTransfer,
            audience: Artifact,
            retry: Retry::ContinueFromEstablishedPosition,
        },
        Operation::Append => contract_row! {
            transfer: Transfer::BoundedByteTransfer,
            partial_effect: Partial::BytePrefix,
            synchronization: Sync::None,
            handle: Handle::OpenFile,
            capability: Capability::Append,
            counter: Counter::WriteTransfer,
            audience: Artifact,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::Truncate => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::LogicalLengthMayChange,
            synchronization: Sync::None,
            handle: Handle::OpenFile,
            capability: Capability::BaseFilesystem,
            counter: Counter::LogicalLengthMutation,
            audience: Artifact,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::Allocate => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::AllocationMayChange,
            synchronization: Sync::None,
            handle: Handle::OpenFile,
            capability: Capability::QualifiedAllocationMode,
            counter: Counter::AllocationMutation,
            audience: Artifact,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::ReadMetadata => contract_row! {
            transfer: Transfer::SingleObservation,
            partial_effect: Partial::Impossible,
            synchronization: Sync::None,
            handle: Handle::OwnerIssuedNamespaceEntry,
            capability: Capability::BaseFilesystem,
            counter: Counter::MetadataObservation,
            audience: Artifact,
            retry: Retry::RestartReadObservation,
        },
        Operation::ListDirectory => contract_row! {
            transfer: Transfer::DirectorySequence,
            partial_effect: Partial::Impossible,
            synchronization: Sync::None,
            handle: Handle::OpenDirectory,
            capability: Capability::BaseFilesystem,
            counter: Counter::DirectoryObservation,
            audience: Owner,
            retry: Retry::RestartReadObservation,
        },
        Operation::SynchronizeFileData => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::BarrierCompletionMayBeIndeterminate,
            synchronization: Sync::FileData,
            handle: Handle::OpenFile,
            capability: Capability::QualifiedDataOnlySynchronization,
            counter: Counter::SynchronizationBarrier,
            audience: Artifact,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::SynchronizeFileState => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::BarrierCompletionMayBeIndeterminate,
            synchronization: Sync::FileDataAndMetadata,
            handle: Handle::OpenFile,
            capability: Capability::FileStateSynchronization,
            counter: Counter::SynchronizationBarrier,
            audience: Artifact,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::SynchronizeDirectoryPublication => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::BarrierCompletionMayBeIndeterminate,
            synchronization: Sync::ParentNamespacePublication,
            handle: Handle::OpenDirectory,
            capability: Capability::DirectorySynchronization,
            counter: Counter::SynchronizationBarrier,
            audience: Owner,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::SynchronizeStoreRootPublication
        | Operation::SynchronizeRootParentPublication => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::BarrierCompletionMayBeIndeterminate,
            synchronization: Sync::ParentNamespacePublication,
            handle: Handle::OpenDirectory,
            capability: Capability::DirectorySynchronization,
            counter: Counter::SynchronizationBarrier,
            audience: Owner,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::AtomicReplace => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::NamespaceMayChange,
            synchronization: Sync::None,
            handle: Handle::SourceAndDestinationDirectory,
            capability: Capability::AtomicSameNamespaceReplacement,
            counter: Counter::NamespaceMutation,
            audience: Owner,
            retry: Retry::InspectAfterPossibleEffect,
        },
        Operation::Delete => contract_row! {
            transfer: Transfer::None,
            partial_effect: Partial::NamespaceMayChange,
            synchronization: Sync::None,
            handle: Handle::OwnerIssuedNamespaceEntry,
            capability: Capability::BaseFilesystem,
            counter: Counter::NamespaceMutation,
            audience: Owner,
            retry: Retry::InspectAfterPossibleEffect,
        },
    }
}
