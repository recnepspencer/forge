use std::path::Path;

use cap_fs_ext::DirExt;
use cap_std::ambient_authority;
use cap_std::fs::Dir;
use same_file::Handle as FileIdentityHandle;
use worth_store_physical_format::store_namespace::{
    NamespaceEntryType, StoreNamespaceClassification, StoreNamespaceRelativeRole,
};

use super::{
    MediaOwnerIdentity, MediaPathRole, NamespaceConfinementDenial, NamespaceConfinementDenialKind,
    NamespaceDirectoryHandle, NamespaceRelativePath,
};

/// A root directory admitted once from ambient configuration and thereafter
/// accessed only through relative directory capabilities.
#[derive(Debug)]
pub struct AdmittedStoreNamespace {
    owner: MediaOwnerIdentity,
    root: NamespaceDirectoryHandle,
    publication_parent: Option<Dir>,
    store_root_publication_required: bool,
    root_parent_publication_required: bool,
    next_handle_generation: u64,
}

impl AdmittedStoreNamespace {
    pub const fn owner_identity(&self) -> MediaOwnerIdentity {
        self.owner
    }

    pub const fn root_handle(&self) -> &NamespaceDirectoryHandle {
        &self.root
    }

    fn from_opened_directory(
        directory: Dir,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<Self, NamespaceConfinementDenial> {
        let owner = MediaOwnerIdentity::generate().map_err(|_| {
            NamespaceConfinementDenial::structural(
                NamespaceConfinementDenialKind::AuthorityIdentityUnavailable,
            )
        })?;
        boundary.bind_owner(owner);
        Ok(Self {
            owner,
            root: NamespaceDirectoryHandle::new(
                owner,
                1,
                MediaPathRole::ArtifactOwned,
                directory,
                boundary.shared_counters().directory_handle_opened(),
            ),
            publication_parent: None,
            store_root_publication_required: false,
            root_parent_publication_required: false,
            next_handle_generation: 2,
        })
    }

    pub(super) fn create_or_open(
        root: &Path,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<Self, NamespaceConfinementDenial> {
        let name = root.file_name().ok_or_else(|| {
            NamespaceConfinementDenial::structural(
                NamespaceConfinementDenialKind::MissingParentPublicationBoundary,
            )
        })?;
        let parent_path = root
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let publication_parent =
            boundary_io_call(boundary, super::MediaOperationRole::OpenRootParent, || {
                Dir::open_ambient_dir(parent_path, ambient_authority())
            })
            .map_err(|error| NamespaceConfinementDenial::from_io(&error))?;
        let root_was_absent = match boundary_io_call(
            boundary,
            super::MediaOperationRole::InspectNamespaceEntry,
            || publication_parent.symlink_metadata(name),
        ) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(NamespaceConfinementDenial::structural(
                    NamespaceConfinementDenialKind::LinkLikeEntry,
                ));
            }
            Ok(metadata) if !metadata.is_dir() => {
                return Err(NamespaceConfinementDenial::structural(
                    NamespaceConfinementDenialKind::EntryTypeMismatch,
                ));
            }
            Ok(_) => false,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                match boundary_io_call(boundary, super::MediaOperationRole::CreateDirectory, || {
                    publication_parent.create_dir(name)
                }) {
                    Ok(()) => true,
                    Err(create_error)
                        if create_error.kind() == std::io::ErrorKind::AlreadyExists =>
                    {
                        false
                    }
                    Err(create_error) => {
                        return Err(NamespaceConfinementDenial::from_io(&create_error));
                    }
                }
            }
            Err(error) => return Err(NamespaceConfinementDenial::from_io(&error)),
        };
        let directory =
            boundary_io_call(boundary, super::MediaOperationRole::OpenDirectory, || {
                publication_parent.open_dir_nofollow(name)
            })
            .map_err(|error| NamespaceConfinementDenial::from_io(&error))?;
        require_opened_root_identity(root, &directory, boundary)?;
        let classification = if root_was_absent {
            StoreNamespaceClassification::AbsentEligible
        } else {
            super::namespace_root_inventory::classify_opened_root(&directory, boundary)?
        };
        let initialize_scaffold = matches!(
            classification,
            StoreNamespaceClassification::AbsentEligible
                | StoreNamespaceClassification::EmptyEligible
                | StoreNamespaceClassification::IncompleteScaffold { .. }
        );
        if let StoreNamespaceClassification::Initialized { identity, .. } = classification {
            boundary.bind_store(identity);
        }
        if !(initialize_scaffold
            || matches!(
                classification,
                StoreNamespaceClassification::Initialized { .. }
            ))
        {
            return Err(NamespaceConfinementDenial::structural(
                classification_denial_kind(&classification),
            ));
        }
        let mut namespace = Self::from_opened_directory(directory, boundary)?;
        namespace.publication_parent = Some(publication_parent);
        if initialize_scaffold {
            namespace.create_fixed_scaffold(boundary)?;
        }
        namespace.store_root_publication_required = true;
        namespace.root_parent_publication_required = true;
        Ok(namespace)
    }
    pub(super) fn open_directory(
        &mut self,
        path: &NamespaceRelativePath,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<NamespaceDirectoryHandle, NamespaceConfinementDenial> {
        self.open_directory_after_check(path, boundary)
    }

    fn open_directory_after_check(
        &mut self,
        path: &NamespaceRelativePath,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<NamespaceDirectoryHandle, NamespaceConfinementDenial> {
        if path.owner_identity() != self.owner
            || path.parent() != super::namespace_confinement::NamespaceParent::Root
        {
            return Err(NamespaceConfinementDenial::structural(
                NamespaceConfinementDenialKind::AuthorityMismatch,
            ));
        }
        let relative = path.file_name();
        let metadata = boundary_io_call(
            boundary,
            super::MediaOperationRole::InspectNamespaceEntry,
            || self.root.directory().symlink_metadata(relative),
        )
        .map_err(|error| NamespaceConfinementDenial::from_io(&error))?;
        if metadata.file_type().is_symlink() {
            return Err(NamespaceConfinementDenial::structural(
                NamespaceConfinementDenialKind::LinkLikeEntry,
            ));
        }
        if !metadata.is_dir() {
            return Err(NamespaceConfinementDenial::structural(
                NamespaceConfinementDenialKind::EntryTypeMismatch,
            ));
        }

        let directory =
            boundary_io_call(boundary, super::MediaOperationRole::OpenDirectory, || {
                self.root.directory().open_dir_nofollow(relative)
            })
            .map_err(|error| NamespaceConfinementDenial::from_io(&error))?;
        let generation = self.issue_handle_generation()?;
        Ok(NamespaceDirectoryHandle::new(
            self.owner,
            generation,
            path.role(),
            directory,
            boundary.shared_counters().directory_handle_opened(),
        ))
    }

    pub(super) fn open_role_directory(
        &mut self,
        role: StoreNamespaceRelativeRole,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<NamespaceDirectoryHandle, NamespaceConfinementDenial> {
        if role.expected_entry_type() != NamespaceEntryType::Directory {
            return Err(NamespaceConfinementDenial::structural(
                NamespaceConfinementDenialKind::EntryTypeMismatch,
            ));
        }
        self.open_directory(
            &NamespaceRelativePath::bind_role(self.owner, role),
            boundary,
        )
    }

    pub(super) fn require_owned_directory(
        &self,
        handle: &NamespaceDirectoryHandle,
    ) -> Result<(), NamespaceConfinementDenial> {
        if handle.belongs_to(self.owner) {
            Ok(())
        } else {
            Err(NamespaceConfinementDenial::structural(
                NamespaceConfinementDenialKind::AuthorityMismatch,
            ))
        }
    }

    pub(super) const fn publication_parent(&self) -> Option<&Dir> {
        self.publication_parent.as_ref()
    }

    pub(super) const fn store_root_publication_required(&self) -> bool {
        self.store_root_publication_required
    }

    pub(super) const fn root_parent_publication_required(&self) -> bool {
        self.root_parent_publication_required
    }

    fn create_fixed_scaffold(
        &self,
        boundary: &super::fault_interposition::MediaFaultInterposer,
    ) -> Result<(), NamespaceConfinementDenial> {
        for role in [
            StoreNamespaceRelativeRole::NamespaceDirectory,
            StoreNamespaceRelativeRole::FamiliesDirectory,
            StoreNamespaceRelativeRole::StagingDirectory,
        ] {
            let path = role.components()[0];
            match boundary_io_call(
                boundary,
                super::MediaOperationRole::InspectNamespaceEntry,
                || self.root.directory().symlink_metadata(path),
            ) {
                Ok(metadata) => require_directory_metadata(metadata)?,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    match boundary_io_call(
                        boundary,
                        super::MediaOperationRole::CreateDirectory,
                        || self.root.directory().create_dir(path),
                    ) {
                        Ok(()) => {}
                        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                            let metadata = boundary_io_call(
                                boundary,
                                super::MediaOperationRole::InspectNamespaceEntry,
                                || self.root.directory().symlink_metadata(path),
                            )
                            .map_err(|error| NamespaceConfinementDenial::from_io(&error))?;
                            require_directory_metadata(metadata)?;
                        }
                        Err(error) => {
                            return Err(NamespaceConfinementDenial::from_io(&error));
                        }
                    }
                }
                Err(error) => return Err(NamespaceConfinementDenial::from_io(&error)),
            }
        }
        Ok(())
    }

    fn issue_handle_generation(&mut self) -> Result<u64, NamespaceConfinementDenial> {
        let generation = self.next_handle_generation;
        self.next_handle_generation = generation.checked_add(1).ok_or_else(|| {
            NamespaceConfinementDenial::structural(
                NamespaceConfinementDenialKind::AuthorityIdentityUnavailable,
            )
        })?;
        Ok(generation)
    }
}

fn classification_denial_kind(
    classification: &StoreNamespaceClassification,
) -> NamespaceConfinementDenialKind {
    match classification {
        StoreNamespaceClassification::IncompleteScaffold { .. } => {
            NamespaceConfinementDenialKind::NamespaceIncomplete
        }
        StoreNamespaceClassification::UnsupportedVersion(_) => {
            NamespaceConfinementDenialKind::NamespaceVersionUnsupported
        }
        StoreNamespaceClassification::Damaged(_) => {
            NamespaceConfinementDenialKind::NamespaceDamaged
        }
        StoreNamespaceClassification::Ambiguous(_) => {
            NamespaceConfinementDenialKind::NamespaceAmbiguous
        }
        StoreNamespaceClassification::ContendedCompatible { .. }
        | StoreNamespaceClassification::AbsentEligible
        | StoreNamespaceClassification::EmptyEligible
        | StoreNamespaceClassification::Initialized { .. } => {
            NamespaceConfinementDenialKind::NamespaceNotAdmissible
        }
    }
}

fn require_directory_metadata(
    metadata: cap_std::fs::Metadata,
) -> Result<(), NamespaceConfinementDenial> {
    if metadata.file_type().is_symlink() {
        return Err(NamespaceConfinementDenial::structural(
            NamespaceConfinementDenialKind::LinkLikeEntry,
        ));
    }
    if !metadata.is_dir() {
        return Err(NamespaceConfinementDenial::structural(
            NamespaceConfinementDenialKind::EntryTypeMismatch,
        ));
    }
    Ok(())
}

fn require_opened_root_identity(
    root: &Path,
    directory: &Dir,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<(), NamespaceConfinementDenial> {
    let (opened, current) = boundary_io_call(
        boundary,
        super::MediaOperationRole::ValidateRootIdentity,
        || {
            let opened = directory
                .try_clone()
                .map(Dir::into_std_file)
                .and_then(FileIdentityHandle::from_file)?;
            let current = FileIdentityHandle::from_path(root)?;
            Ok((opened, current))
        },
    )
    .map_err(|error| NamespaceConfinementDenial::from_io(&error))?;
    if opened != current {
        return Err(NamespaceConfinementDenial::structural(
            NamespaceConfinementDenialKind::RootIdentityChanged,
        ));
    }
    Ok(())
}

pub(super) fn boundary_io_call<T>(
    boundary: &super::fault_interposition::MediaFaultInterposer,
    role: super::MediaOperationRole,
    call: impl FnOnce() -> std::io::Result<T>,
) -> std::io::Result<T> {
    let attempt = boundary.begin(role, 0);
    if let Some(error) = attempt.fail_before_error() {
        attempt.denied();
        return Err(error);
    }
    match call() {
        Ok(_) if attempt.effect_observation_is_indeterminate() => {
            attempt.indeterminate(0);
            Err(std::io::Error::other(
                "certification interrupted admission-effect observation",
            ))
        }
        Ok(value) => {
            attempt.completed(0);
            Ok(value)
        }
        Err(error) => {
            attempt.denied();
            Err(error)
        }
    }
}
