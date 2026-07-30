use worth_store_physical_backend::MediaOperationRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::bounded_residency) enum PhysicalWorkBackendRoleEvidence {
    CreateNew,
    PositionedRead,
    PositionedWrite,
    ReadMetadata,
    SynchronizeFileState,
    SynchronizeDirectoryPublication,
    AtomicReplace,
}

impl TryFrom<MediaOperationRole> for PhysicalWorkBackendRoleEvidence {
    type Error = String;

    fn try_from(role: MediaOperationRole) -> Result<Self, Self::Error> {
        match role {
            MediaOperationRole::CreateNew => Ok(Self::CreateNew),
            MediaOperationRole::PositionedRead => Ok(Self::PositionedRead),
            MediaOperationRole::PositionedWrite => Ok(Self::PositionedWrite),
            MediaOperationRole::ReadMetadata => Ok(Self::ReadMetadata),
            MediaOperationRole::SynchronizeFileState => Ok(Self::SynchronizeFileState),
            MediaOperationRole::SynchronizeDirectoryPublication => {
                Ok(Self::SynchronizeDirectoryPublication)
            }
            MediaOperationRole::AtomicReplace => Ok(Self::AtomicReplace),
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
            | MediaOperationRole::ReleaseMutationLease
            | MediaOperationRole::OpenExisting
            | MediaOperationRole::Append
            | MediaOperationRole::Truncate
            | MediaOperationRole::Allocate
            | MediaOperationRole::ListDirectory
            | MediaOperationRole::SynchronizeFileData
            | MediaOperationRole::SynchronizeStoreRootPublication
            | MediaOperationRole::SynchronizeRootParentPublication
            | MediaOperationRole::Delete => Err(format!(
                "media-reaching physical work carried unsupported backend role {role:?}"
            )),
        }
    }
}
