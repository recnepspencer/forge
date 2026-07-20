use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{
    MediaCapabilityRequirement, MediaHandleIdentity, MediaOperationIdentity, MediaOperationRole,
    MediaOwnerIdentity, MediaPathRole, NamespacePublicationStage,
};

/// Immutable semantic coordinates captured before one OS primitive boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaOperationContext {
    owner: Option<MediaOwnerIdentity>,
    runtime_incarnation: Option<u64>,
    store: Option<StableStoreIdentity>,
    operation: Option<MediaOperationIdentity>,
    path_role: Option<MediaPathRole>,
    handle: Option<MediaHandleIdentity>,
    role: MediaOperationRole,
    requested_bytes: u64,
    requested_offset: Option<u64>,
    publication_stage: Option<NamespacePublicationStage>,
    required_capability: MediaCapabilityRequirement,
    role_ordinal: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MediaOperationIdentityBinding {
    pub(super) owner: Option<MediaOwnerIdentity>,
    pub(super) runtime_incarnation: Option<u64>,
    pub(super) store: Option<StableStoreIdentity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MediaOperationCoordinates {
    operation: Option<MediaOperationIdentity>,
    path_role: Option<MediaPathRole>,
    handle: Option<MediaHandleIdentity>,
    requested_offset: Option<u64>,
    publication_stage: Option<NamespacePublicationStage>,
}

impl MediaOperationCoordinates {
    pub(super) const fn unbound() -> Self {
        Self {
            operation: None,
            path_role: None,
            handle: None,
            requested_offset: None,
            publication_stage: None,
        }
    }

    pub(super) const fn for_path(
        operation: MediaOperationIdentity,
        path_role: MediaPathRole,
        handle: Option<MediaHandleIdentity>,
    ) -> Self {
        Self {
            operation: Some(operation),
            path_role: Some(path_role),
            handle,
            requested_offset: None,
            publication_stage: None,
        }
    }

    pub(super) const fn at_offset(mut self, requested_offset: u64) -> Self {
        self.requested_offset = Some(requested_offset);
        self
    }

    pub(super) const fn at_publication_stage(
        mut self,
        publication_stage: NamespacePublicationStage,
    ) -> Self {
        self.publication_stage = Some(publication_stage);
        self
    }
}

impl MediaOperationContext {
    pub(super) const fn new(
        binding: MediaOperationIdentityBinding,
        role: MediaOperationRole,
        requested_bytes: u64,
        coordinates: MediaOperationCoordinates,
        role_ordinal: u64,
    ) -> Self {
        Self {
            owner: binding.owner,
            runtime_incarnation: binding.runtime_incarnation,
            store: binding.store,
            operation: coordinates.operation,
            path_role: coordinates.path_role,
            handle: coordinates.handle,
            role,
            requested_bytes,
            requested_offset: coordinates.requested_offset,
            publication_stage: coordinates.publication_stage,
            required_capability: role.contract().capability(),
            role_ordinal,
        }
    }

    pub const fn runtime_incarnation(self) -> Option<u64> {
        self.runtime_incarnation
    }

    pub const fn owner(self) -> Option<MediaOwnerIdentity> {
        self.owner
    }

    pub const fn store(self) -> Option<StableStoreIdentity> {
        self.store
    }

    pub const fn role(self) -> MediaOperationRole {
        self.role
    }

    pub const fn operation(self) -> Option<MediaOperationIdentity> {
        self.operation
    }

    pub const fn path_role(self) -> Option<MediaPathRole> {
        self.path_role
    }

    pub const fn handle(self) -> Option<MediaHandleIdentity> {
        self.handle
    }

    pub const fn requested_bytes(self) -> u64 {
        self.requested_bytes
    }

    pub const fn requested_offset(self) -> Option<u64> {
        self.requested_offset
    }

    pub const fn publication_stage(self) -> Option<NamespacePublicationStage> {
        self.publication_stage
    }

    pub const fn required_capability(self) -> MediaCapabilityRequirement {
        self.required_capability
    }

    pub const fn role_ordinal(self) -> u64 {
        self.role_ordinal
    }
}
