use super::{MediaFaultDirective, MediaOperationRole};

pub(super) fn directive_matches_role(
    role: MediaOperationRole,
    directive: &MediaFaultDirective,
) -> bool {
    match directive {
        MediaFaultDirective::AllowPrefix { .. }
        | MediaFaultDirective::AllowPrefixThenPause { .. } => matches!(
            role,
            MediaOperationRole::PositionedRead
                | MediaOperationRole::PositionedWrite
                | MediaOperationRole::Append
                | MediaOperationRole::PublishMutationLeaseObservation
        ),
        MediaFaultDirective::FailBarrier { .. } => matches!(
            role,
            MediaOperationRole::SynchronizeFileData
                | MediaOperationRole::SynchronizeFileState
                | MediaOperationRole::SynchronizeDirectoryPublication
                | MediaOperationRole::SynchronizeStoreRootPublication
                | MediaOperationRole::SynchronizeRootParentPublication
                | MediaOperationRole::PublishMutationLeaseObservation
        ),
        MediaFaultDirective::InterruptReplacementObservation => {
            role == MediaOperationRole::AtomicReplace
        }
        MediaFaultDirective::IndeterminateAfterEffect => matches!(
            role,
            MediaOperationRole::CreateDirectory
                | MediaOperationRole::CreateMutationLease
                | MediaOperationRole::PublishMutationLeaseObservation
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
                | MediaOperationRole::ReleaseMutationLease
        ),
        MediaFaultDirective::FailBefore { .. }
        | MediaFaultDirective::PauseBefore(_)
        | MediaFaultDirective::PauseAfter(_) => true,
    }
}
