#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryFilesystemQualificationError {
    RootUnavailable,
    ExistingStoreRequired,
    RootIdentityChanged,
    BackendProfileUnsupported,
    OwnershipContended,
    OwnershipUnavailable,
    IdentityUnavailable,
    PersistedIdentityUnavailable,
    BackendCapabilityUnavailable,
    InvalidDiscoveryLimit,
}
