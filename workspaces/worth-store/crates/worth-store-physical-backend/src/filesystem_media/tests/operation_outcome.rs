use super::super::transfer::classify_media_transfer;
use super::super::*;
use worth_store_physical_format::store_namespace::StoreNamespaceRelativeRole;

fn operation_identity() -> MediaOperationIdentity {
    MediaOperationIdentity::for_test(41)
}

const MEDIA_OUTCOME_ROLES: [MediaOperationRole; 14] = [
    MediaOperationRole::OpenExisting,
    MediaOperationRole::CreateNew,
    MediaOperationRole::PositionedRead,
    MediaOperationRole::PositionedWrite,
    MediaOperationRole::Append,
    MediaOperationRole::Truncate,
    MediaOperationRole::Allocate,
    MediaOperationRole::ReadMetadata,
    MediaOperationRole::ListDirectory,
    MediaOperationRole::SynchronizeFileData,
    MediaOperationRole::SynchronizeFileState,
    MediaOperationRole::SynchronizeDirectoryPublication,
    MediaOperationRole::AtomicReplace,
    MediaOperationRole::Delete,
];

fn failure_context(operation: MediaOperationRole) -> MediaFailureContext {
    MediaFailureContext::for_test(
        operation,
        MediaPathRole::Namespace(StoreNamespaceRelativeRole::IdentityRecord),
        Some(MediaHandleIdentity::for_test(7)),
        Some(MediaOsCode::for_test(MediaOsCodeFamily::Other, 99)),
        MediaCausalBoundary::OsCallReturned,
    )
}

#[test]
fn every_operation_family_preserves_zero_complete_and_indeterminate_laws() {
    for (sequence, operation) in MEDIA_OUTCOME_ROLES.into_iter().enumerate() {
        let identity = MediaOperationIdentity::for_test(sequence as u64 + 1);
        let context = failure_context(operation);
        let denied = MediaOperationFailure::for_test(
            identity,
            MediaOperationFailureKind::DeniedBeforeEffect,
            context,
        );
        assert_eq!(
            denied.effect_status(),
            MediaEffectStatus::DeniedBeforeEffect
        );
        assert_eq!(denied.retry_posture(), MediaRetryPosture::SafeFromStart);

        let completed =
            MediaOperationOutcome::completed_for_test(identity, completed_effect(operation));
        assert_eq!(
            completed.effect_status(),
            MediaEffectStatus::CompletedEffect
        );

        if operation_can_leave_indeterminate_effect(operation) {
            let indeterminate = MediaOperationFailure::for_test(
                identity,
                MediaOperationFailureKind::IndeterminateEffect {
                    attempted: attempted_effect(operation),
                    last_established: established_boundary(operation),
                },
                context,
            );
            assert_eq!(
                indeterminate.retry_posture(),
                MediaRetryPosture::InspectionRequired
            );
        }
    }
}

#[test]
fn every_byte_transfer_preserves_a_short_prefix_instead_of_claiming_completion() {
    let cases = [
        (
            MediaOperationRole::PositionedRead,
            MediaTransferPosition::PositionedOffset(100),
            MediaRetryPosture::SafeFromContinuationPosition(103),
        ),
        (
            MediaOperationRole::PositionedWrite,
            MediaTransferPosition::PositionedOffset(100),
            MediaRetryPosture::SafeFromContinuationPosition(103),
        ),
        (
            MediaOperationRole::Append,
            MediaTransferPosition::KnownAppendPosition(100),
            MediaRetryPosture::InspectionRequired,
        ),
    ];

    for (operation, position, expected_retry) in cases {
        let progress = classify_media_transfer(8, 3, position).expect("valid short transfer");
        let MediaTransferProgress::Partial(partial) = progress else {
            panic!("{operation:?} must preserve its short prefix");
        };
        let failure = MediaOperationFailure::for_test(
            operation_identity(),
            MediaOperationFailureKind::PartialTransfer(partial),
            failure_context(operation),
        );
        assert_eq!(failure.effect_status(), MediaEffectStatus::PartialTransfer);
        assert_eq!(failure.retry_posture(), expected_retry);
    }
}

#[test]
fn short_positioned_transfer_preserves_exact_prefix_and_continuation() {
    let progress =
        classify_media_transfer(4096, 1024, MediaTransferPosition::PositionedOffset(8192))
            .expect("valid primitive result");
    let MediaTransferProgress::Partial(partial) = progress else {
        panic!("a positive prefix smaller than the request must remain partial");
    };
    assert_eq!(partial.requested_bytes(), 4096);
    assert_eq!(partial.completed_bytes(), 1024);
    assert_eq!(partial.continuation_position(), Some(9216));

    let failure = MediaOperationFailure::for_test(
        operation_identity(),
        MediaOperationFailureKind::PartialTransfer(partial),
        failure_context(MediaOperationRole::PositionedWrite),
    );
    assert_eq!(failure.effect_status(), MediaEffectStatus::PartialTransfer);
    assert_eq!(
        failure.retry_posture(),
        MediaRetryPosture::SafeFromContinuationPosition(9216)
    );
}

#[test]
fn append_prefix_requires_inspection_even_when_its_start_is_known() {
    for start in [
        MediaTransferPosition::KnownAppendPosition(100),
        MediaTransferPosition::UnknownAppendPosition,
    ] {
        let progress = classify_media_transfer(1024, 512, start).expect("valid append prefix");
        let MediaTransferProgress::Partial(transfer) = progress else {
            panic!("short append must remain partial");
        };
        let failure = MediaOperationFailure::for_test(
            operation_identity(),
            MediaOperationFailureKind::PartialTransfer(transfer),
            failure_context(MediaOperationRole::Append),
        );
        assert_eq!(
            failure.retry_posture(),
            MediaRetryPosture::InspectionRequired
        );
    }
}

#[test]
fn impossible_transfer_shapes_never_become_completion_facts() {
    assert_eq!(
        classify_media_transfer(0, 0, MediaTransferPosition::PositionedOffset(0)),
        Err(MediaTransferShapeError::EmptyRequest)
    );
    assert_eq!(
        classify_media_transfer(4, 5, MediaTransferPosition::PositionedOffset(0)),
        Err(MediaTransferShapeError::CompletedBeyondRequest)
    );
    assert_eq!(
        classify_media_transfer(4, 4, MediaTransferPosition::UnknownAppendPosition),
        Err(MediaTransferShapeError::UnestablishedCompletedAppendPosition)
    );
    assert_eq!(
        classify_media_transfer(4, 2, MediaTransferPosition::PositionedOffset(u64::MAX - 1)),
        Err(MediaTransferShapeError::ContinuationPositionOverflow)
    );
}

#[test]
fn positioned_read_eof_is_a_sealed_correlated_observation() {
    let outcome = PositionedReadOutcome::for_test(
        operation_identity(),
        PositionedReadResult::EndOfFile {
            requested_offset: 8192,
        },
    );
    assert_eq!(outcome.operation(), operation_identity());
    assert!(matches!(
        outcome.result(),
        PositionedReadResult::EndOfFile {
            requested_offset: 8192
        }
    ));
}

#[test]
fn failure_context_retains_machine_readable_causal_facts() {
    let context = failure_context(MediaOperationRole::SynchronizeFileState);
    assert_eq!(
        context.operation(),
        MediaOperationRole::SynchronizeFileState
    );
    assert_eq!(
        context.path_role(),
        MediaPathRole::Namespace(StoreNamespaceRelativeRole::IdentityRecord)
    );
    assert_eq!(
        context.handle().map(MediaHandleIdentity::generation),
        Some(7)
    );
    assert_eq!(
        context.os_code().map(MediaOsCode::family),
        Some(MediaOsCodeFamily::Other)
    );
    assert_eq!(context.os_code().map(MediaOsCode::value), Some(99));
    assert_eq!(
        context.causal_boundary(),
        MediaCausalBoundary::OsCallReturned
    );
}

fn completed_effect(operation: MediaOperationRole) -> CompletedMediaEffect {
    match operation {
        MediaOperationRole::OpenRootParent
        | MediaOperationRole::InspectNamespaceEntry
        | MediaOperationRole::CreateDirectory
        | MediaOperationRole::OpenDirectory
        | MediaOperationRole::ValidateRootIdentity
        | MediaOperationRole::ObserveRootProfile
        | MediaOperationRole::OpenMutationLease
        | MediaOperationRole::CreateMutationLease
        | MediaOperationRole::AcquireMutationLease
        | MediaOperationRole::PublishMutationLeaseObservation
        | MediaOperationRole::ReleaseMutationLease => {
            panic!("admission roles do not produce MediaOperationOutcome")
        }
        MediaOperationRole::OpenExisting => CompletedMediaEffect::ExistingHandleOpened {
            handle: MediaHandleIdentity::for_test(17),
        },
        MediaOperationRole::CreateNew => CompletedMediaEffect::NewFileCreated {
            handle: MediaHandleIdentity::for_test(18),
        },
        MediaOperationRole::PositionedRead => CompletedMediaEffect::PositionedReadCompleted(
            CompletedMediaTransfer::new(8, MediaTransferPosition::PositionedOffset(0)),
        ),
        MediaOperationRole::PositionedWrite => CompletedMediaEffect::PositionedWriteCompleted(
            CompletedMediaTransfer::new(8, MediaTransferPosition::PositionedOffset(0)),
        ),
        MediaOperationRole::Append => CompletedMediaEffect::AppendCompleted(
            CompletedMediaTransfer::new(8, MediaTransferPosition::KnownAppendPosition(0)),
        ),
        MediaOperationRole::Truncate => CompletedMediaEffect::LogicalLengthChanged,
        MediaOperationRole::Allocate => CompletedMediaEffect::AllocationCompleted,
        MediaOperationRole::ReadMetadata => CompletedMediaEffect::MetadataObserved,
        MediaOperationRole::ListDirectory => CompletedMediaEffect::DirectoryBatchObserved,
        MediaOperationRole::SynchronizeFileData => CompletedMediaEffect::FileDataSynchronized,
        MediaOperationRole::SynchronizeFileState => CompletedMediaEffect::FileStateSynchronized,
        MediaOperationRole::SynchronizeDirectoryPublication
        | MediaOperationRole::SynchronizeStoreRootPublication
        | MediaOperationRole::SynchronizeRootParentPublication => {
            CompletedMediaEffect::DirectoryPublicationSynchronized
        }
        MediaOperationRole::AtomicReplace => CompletedMediaEffect::AtomicReplacementCompleted,
        MediaOperationRole::Delete => CompletedMediaEffect::NamespaceEntryDeleted,
    }
}

fn attempted_effect(operation: MediaOperationRole) -> MediaAttemptedEffect {
    match operation {
        MediaOperationRole::OpenRootParent
        | MediaOperationRole::InspectNamespaceEntry
        | MediaOperationRole::CreateDirectory
        | MediaOperationRole::OpenDirectory
        | MediaOperationRole::ValidateRootIdentity
        | MediaOperationRole::ObserveRootProfile
        | MediaOperationRole::OpenMutationLease
        | MediaOperationRole::CreateMutationLease
        | MediaOperationRole::AcquireMutationLease
        | MediaOperationRole::PublishMutationLeaseObservation
        | MediaOperationRole::ReleaseMutationLease => {
            panic!("admission roles use admission-local outcomes")
        }
        MediaOperationRole::OpenExisting => MediaAttemptedEffect::ExistingHandleAcquisition,
        MediaOperationRole::CreateNew => MediaAttemptedEffect::NewFileCreation,
        MediaOperationRole::PositionedRead => {
            MediaAttemptedEffect::PositionedRead { requested_bytes: 8 }
        }
        MediaOperationRole::PositionedWrite => {
            MediaAttemptedEffect::PositionedWrite { requested_bytes: 8 }
        }
        MediaOperationRole::Append => MediaAttemptedEffect::Append { requested_bytes: 8 },
        MediaOperationRole::Truncate => MediaAttemptedEffect::LogicalLengthChange,
        MediaOperationRole::Allocate => MediaAttemptedEffect::Allocation,
        MediaOperationRole::ReadMetadata => MediaAttemptedEffect::MetadataObservation,
        MediaOperationRole::ListDirectory => MediaAttemptedEffect::DirectoryObservation,
        MediaOperationRole::SynchronizeFileData => MediaAttemptedEffect::FileDataSynchronization,
        MediaOperationRole::SynchronizeFileState => MediaAttemptedEffect::FileStateSynchronization,
        MediaOperationRole::SynchronizeDirectoryPublication
        | MediaOperationRole::SynchronizeStoreRootPublication
        | MediaOperationRole::SynchronizeRootParentPublication => {
            MediaAttemptedEffect::DirectoryPublicationSynchronization
        }
        MediaOperationRole::AtomicReplace => MediaAttemptedEffect::AtomicReplacement,
        MediaOperationRole::Delete => MediaAttemptedEffect::NamespaceEntryDeletion,
    }
}

fn established_boundary(operation: MediaOperationRole) -> MediaEstablishedBoundary {
    match operation {
        MediaOperationRole::OpenRootParent
        | MediaOperationRole::InspectNamespaceEntry
        | MediaOperationRole::CreateDirectory
        | MediaOperationRole::OpenDirectory
        | MediaOperationRole::ValidateRootIdentity
        | MediaOperationRole::ObserveRootProfile
        | MediaOperationRole::OpenMutationLease
        | MediaOperationRole::CreateMutationLease
        | MediaOperationRole::AcquireMutationLease
        | MediaOperationRole::PublishMutationLeaseObservation
        | MediaOperationRole::ReleaseMutationLease => {
            panic!("admission roles use admission-local outcomes")
        }
        MediaOperationRole::CreateNew => MediaEstablishedBoundary::NamespaceEntryCreationIssued,
        MediaOperationRole::Truncate => MediaEstablishedBoundary::LogicalLengthChangeIssued,
        MediaOperationRole::Allocate => MediaEstablishedBoundary::AllocationIssued,
        MediaOperationRole::SynchronizeFileData => {
            MediaEstablishedBoundary::FileDataSynchronizationIssued
        }
        MediaOperationRole::SynchronizeFileState => {
            MediaEstablishedBoundary::FileStateSynchronizationIssued
        }
        MediaOperationRole::SynchronizeDirectoryPublication
        | MediaOperationRole::SynchronizeStoreRootPublication
        | MediaOperationRole::SynchronizeRootParentPublication => {
            MediaEstablishedBoundary::DirectoryPublicationSynchronizationIssued
        }
        MediaOperationRole::AtomicReplace => MediaEstablishedBoundary::AtomicReplacementIssued,
        MediaOperationRole::Delete => MediaEstablishedBoundary::NamespaceEntryDeletionIssued,
        MediaOperationRole::OpenExisting
        | MediaOperationRole::PositionedRead
        | MediaOperationRole::PositionedWrite
        | MediaOperationRole::Append
        | MediaOperationRole::ReadMetadata
        | MediaOperationRole::ListDirectory => MediaEstablishedBoundary::None,
    }
}

const fn operation_can_leave_indeterminate_effect(operation: MediaOperationRole) -> bool {
    matches!(
        operation,
        MediaOperationRole::CreateNew
            | MediaOperationRole::PositionedWrite
            | MediaOperationRole::Append
            | MediaOperationRole::Truncate
            | MediaOperationRole::Allocate
            | MediaOperationRole::SynchronizeFileData
            | MediaOperationRole::SynchronizeFileState
            | MediaOperationRole::SynchronizeDirectoryPublication
            | MediaOperationRole::SynchronizeStoreRootPublication
            | MediaOperationRole::SynchronizeRootParentPublication
            | MediaOperationRole::AtomicReplace
            | MediaOperationRole::Delete
    )
}
