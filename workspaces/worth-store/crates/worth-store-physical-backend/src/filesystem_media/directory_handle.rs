use cap_std::fs::Dir;

use super::{MediaHandleIdentity, MediaOwnerIdentity, MediaPathRole};

/// A directory capability bound to one live media-owner incarnation.
///
/// The held directory is intentionally not cloneable or extractable through
/// the public facade.
#[derive(Debug)]
pub struct NamespaceDirectoryHandle {
    identity: MediaHandleIdentity,
    role: MediaPathRole,
    directory: Dir,
    _accounting: super::handle_accounting::MediaDirectoryHandleAccounting,
}

impl NamespaceDirectoryHandle {
    pub const fn identity(&self) -> MediaHandleIdentity {
        self.identity
    }

    pub const fn role(&self) -> MediaPathRole {
        self.role
    }

    pub(super) fn new(
        owner: MediaOwnerIdentity,
        generation: u64,
        role: MediaPathRole,
        directory: Dir,
        accounting: super::handle_accounting::MediaDirectoryHandleAccounting,
    ) -> Self {
        Self {
            identity: MediaHandleIdentity::new(owner, generation),
            role,
            directory,
            _accounting: accounting,
        }
    }

    pub(super) fn belongs_to(&self, owner: MediaOwnerIdentity) -> bool {
        self.identity.owner() == owner
    }

    pub(super) const fn directory(&self) -> &Dir {
        &self.directory
    }
}

#[derive(Debug)]
pub struct ArtifactFamilyDirectory(NamespaceDirectoryHandle);

impl ArtifactFamilyDirectory {
    pub const fn identity(&self) -> MediaHandleIdentity {
        self.0.identity()
    }

    pub(super) const fn new(handle: NamespaceDirectoryHandle) -> Self {
        Self(handle)
    }

    pub(super) const fn handle(&self) -> &NamespaceDirectoryHandle {
        &self.0
    }
}

#[derive(Debug)]
pub struct StagingDirectory(NamespaceDirectoryHandle);

impl StagingDirectory {
    pub const fn identity(&self) -> MediaHandleIdentity {
        self.0.identity()
    }

    pub(super) const fn new(handle: NamespaceDirectoryHandle) -> Self {
        Self(handle)
    }

    pub(super) const fn handle(&self) -> &NamespaceDirectoryHandle {
        &self.0
    }
}
