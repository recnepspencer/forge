impl super::MediaOperationRole {
    pub const fn contract(self) -> super::MediaOperationContract {
        super::operation_contract::operation_contract(self)
    }

    pub const fn metric_name(self) -> &'static str {
        match self {
            Self::OpenRootParent => "open_root_parent",
            Self::InspectNamespaceEntry => "inspect_namespace_entry",
            Self::CreateDirectory => "create_directory",
            Self::OpenDirectory => "open_directory",
            Self::ValidateRootIdentity => "validate_root_identity",
            Self::ObserveRootProfile => "observe_root_profile",
            Self::OpenMutationLease => "open_mutation_lease",
            Self::CreateMutationLease => "create_mutation_lease",
            Self::AcquireMutationLease => "acquire_mutation_lease",
            Self::PublishMutationLeaseObservation => "publish_mutation_lease_observation",
            Self::ReleaseMutationLease => "release_mutation_lease",
            Self::OpenExisting => "open_existing",
            Self::CreateNew => "create_new",
            Self::PositionedRead => "positioned_read",
            Self::PositionedWrite => "positioned_write",
            Self::Append => "append",
            Self::Truncate => "truncate",
            Self::Allocate => "allocate",
            Self::ReadMetadata => "read_metadata",
            Self::ListDirectory => "list_directory",
            Self::SynchronizeFileData => "synchronize_file_data",
            Self::SynchronizeFileState => "synchronize_file_state",
            Self::SynchronizeDirectoryPublication => "synchronize_directory_publication",
            Self::SynchronizeStoreRootPublication => "synchronize_store_root_publication",
            Self::SynchronizeRootParentPublication => "synchronize_root_parent_publication",
            Self::AtomicReplace => "atomic_replace",
            Self::Delete => "delete",
        }
    }
}
