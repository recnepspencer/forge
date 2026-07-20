use super::{
    MediaCallAudience, MediaCapabilityRequirement, MediaCounterClass, MediaFaultControlAudience,
    MediaHandleRequirement, MediaObservationAudience, MediaOperationContract, MediaOperationRole,
    MediaPartialEffect, MediaRetryRule, MediaSynchronizationMeaning, MediaTransferCardinality,
};

pub(super) const fn contract(role: MediaOperationRole) -> Option<MediaOperationContract> {
    use MediaCounterClass as Counter;
    use MediaOperationRole as Role;
    use MediaPartialEffect as Partial;
    use MediaRetryRule as Retry;
    use MediaSynchronizationMeaning as Sync;
    use MediaTransferCardinality as Transfer;

    let row = match role {
        Role::OpenRootParent => row(
            Transfer::None,
            Partial::Impossible,
            Sync::None,
            Counter::DirectoryAcquisition,
            Retry::RetryOnlyAfterDeniedBeforeEffect,
        ),
        Role::InspectNamespaceEntry | Role::ValidateRootIdentity | Role::ObserveRootProfile => row(
            Transfer::SingleObservation,
            Partial::Impossible,
            Sync::None,
            Counter::AdmissionObservation,
            Retry::RestartReadObservation,
        ),
        Role::CreateDirectory | Role::CreateMutationLease => row(
            Transfer::None,
            Partial::NamespaceMayChange,
            Sync::None,
            Counter::DirectoryAcquisition,
            Retry::InspectAfterPossibleEffect,
        ),
        Role::OpenDirectory => row(
            Transfer::None,
            Partial::Impossible,
            Sync::None,
            Counter::DirectoryAcquisition,
            Retry::RetryOnlyAfterDeniedBeforeEffect,
        ),
        Role::OpenMutationLease => row(
            Transfer::None,
            Partial::Impossible,
            Sync::None,
            Counter::OwnershipAcquisition,
            Retry::RetryOnlyAfterDeniedBeforeEffect,
        ),
        Role::AcquireMutationLease | Role::ReleaseMutationLease => row(
            Transfer::None,
            Partial::Impossible,
            Sync::None,
            Counter::OwnershipAcquisition,
            Retry::RetryOnlyAfterDeniedBeforeEffect,
        ),
        Role::PublishMutationLeaseObservation => row(
            Transfer::BoundedByteTransfer,
            Partial::BytePrefixOrBarrierIndeterminate,
            Sync::FileData,
            Counter::OwnershipPublication,
            Retry::InspectAfterPossibleEffect,
        ),
        _ => return None,
    };
    Some(row)
}

const fn row(
    transfer: MediaTransferCardinality,
    partial: MediaPartialEffect,
    synchronization: MediaSynchronizationMeaning,
    counter: MediaCounterClass,
    retry: MediaRetryRule,
) -> MediaOperationContract {
    MediaOperationContract {
        transfer,
        partial_effect: partial,
        synchronization,
        handle: MediaHandleRequirement::NamespaceOwner,
        capability: MediaCapabilityRequirement::BaseFilesystem,
        counter,
        audience: MediaCallAudience::MediaOwnerInternal,
        retry,
        observation: MediaObservationAudience::RuntimeDiagnostics,
        fault_control: MediaFaultControlAudience::CertificationOnly,
    }
}
