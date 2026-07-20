use cap_fs_ext::{FollowSymlinks, OpenOptionsFollowExt};
use cap_std::fs::OpenOptions;
use worth_store_physical_format::store_namespace::StoreNamespaceRelativeRole;

use super::{
    MediaOperationRole, MutationOwnershipDenial, NamespaceConfinementDenial,
    NamespaceConfinementDenialKind, NamespaceDirectoryHandle, NamespaceRelativePath,
};

pub(super) fn open(
    owner: super::MediaOwnerIdentity,
    namespace: &NamespaceDirectoryHandle,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<std::fs::File, MutationOwnershipDenial> {
    let path = NamespaceRelativePath::bind_role(owner, StoreNamespaceRelativeRole::MutationLock);
    match super::admitted_namespace::boundary_io_call(
        boundary,
        MediaOperationRole::InspectNamespaceEntry,
        || namespace.directory().symlink_metadata(path.file_name()),
    ) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            Err(confinement(NamespaceConfinementDenialKind::LinkLikeEntry))
        }
        Ok(metadata) if !metadata.is_file() => Err(confinement(
            NamespaceConfinementDenialKind::EntryTypeMismatch,
        )),
        Ok(_) => open_with_mode(namespace, path.file_name(), boundary, false),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match open_with_mode(namespace, path.file_name(), boundary, true) {
                Err(MutationOwnershipDenial::LockOperationFailed {
                    kind: std::io::ErrorKind::AlreadyExists,
                    ..
                }) => open_with_mode(namespace, path.file_name(), boundary, false),
                result => result,
            }
        }
        Err(error) => Err(super::mutation_ownership::lock_error(error)),
    }
}

fn open_with_mode(
    namespace: &NamespaceDirectoryHandle,
    name: &str,
    boundary: &super::fault_interposition::MediaFaultInterposer,
    create: bool,
) -> Result<std::fs::File, MutationOwnershipDenial> {
    let role = if create {
        MediaOperationRole::CreateMutationLease
    } else {
        MediaOperationRole::OpenMutationLease
    };
    let attempt = boundary.begin(role, 0);
    if let Some(error) = attempt.fail_before_error() {
        attempt.denied();
        return Err(super::mutation_ownership::lock_error(error));
    }
    let mut options = OpenOptions::new();
    options
        .read(true)
        .write(true)
        .create_new(create)
        .follow(FollowSymlinks::No);
    match namespace
        .directory()
        .open_with(name, &options)
        .map(cap_std::fs::File::into_std)
    {
        Ok(_) if attempt.effect_observation_is_indeterminate() => {
            attempt.indeterminate(0);
            Err(super::mutation_ownership::lock_error(
                std::io::Error::other("certification interrupted lease-file observation"),
            ))
        }
        Ok(file) => {
            attempt.completed(0);
            Ok(file)
        }
        Err(error) => {
            attempt.denied();
            Err(super::mutation_ownership::lock_error(error))
        }
    }
}

fn confinement(kind: NamespaceConfinementDenialKind) -> MutationOwnershipDenial {
    MutationOwnershipDenial::Confinement(NamespaceConfinementDenial::structural(kind))
}
