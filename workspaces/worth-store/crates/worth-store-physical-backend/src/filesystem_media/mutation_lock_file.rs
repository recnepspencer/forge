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
) -> Result<OpenedMutationLock, super::owner_admission_effect::MutationOwnershipAcquisitionFailure>
{
    let path = NamespaceRelativePath::bind_role(owner, StoreNamespaceRelativeRole::MutationLock);
    match super::admitted_namespace::boundary_io_call(
        boundary,
        MediaOperationRole::InspectNamespaceEntry,
        || namespace.directory().symlink_metadata(path.file_name()),
    ) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(before_effect(confinement(
            NamespaceConfinementDenialKind::LinkLikeEntry,
        ))),
        Ok(metadata) if !metadata.is_file() => Err(before_effect(confinement(
            NamespaceConfinementDenialKind::EntryTypeMismatch,
        ))),
        Ok(_) => open_with_mode(namespace, path.file_name(), boundary, false),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match open_with_mode(namespace, path.file_name(), boundary, true) {
                Err(failure)
                    if matches!(
                        failure.denial(),
                        MutationOwnershipDenial::LockOperationFailed {
                            kind: std::io::ErrorKind::AlreadyExists,
                            ..
                        }
                    ) =>
                {
                    open_with_mode(namespace, path.file_name(), boundary, false)
                }
                result => result,
            }
        }
        Err(error) => Err(before_effect(super::mutation_ownership::lock_error(error))),
    }
}

pub(super) struct OpenedMutationLock {
    pub(super) file: std::fs::File,
    pub(super) effect_fate: super::owner_admission_effect::MediaOwnerAdmissionEffectFate,
}

fn open_with_mode(
    namespace: &NamespaceDirectoryHandle,
    name: &str,
    boundary: &super::fault_interposition::MediaFaultInterposer,
    create: bool,
) -> Result<OpenedMutationLock, super::owner_admission_effect::MutationOwnershipAcquisitionFailure>
{
    use super::owner_admission_effect::{
        MediaOwnerAdmissionEffectFate, MutationOwnershipAcquisitionFailure,
    };

    let role = if create {
        MediaOperationRole::CreateMutationLease
    } else {
        MediaOperationRole::OpenMutationLease
    };
    let attempt = boundary.begin(role, 0);
    if let Some(error) = attempt.fail_before_error() {
        attempt.denied();
        return Err(MutationOwnershipAcquisitionFailure::before_effect(
            super::mutation_ownership::lock_error(error),
        ));
    }
    let mut options = OpenOptions::new();
    options
        .read(true)
        .write(true)
        .create_new(create)
        .follow(FollowSymlinks::No);
    match namespace.directory().open_with(name, &options) {
        Ok(_) if attempt.effect_observation_is_indeterminate() => {
            attempt.indeterminate(0);
            Err(MutationOwnershipAcquisitionFailure::new(
                super::mutation_ownership::lock_error(std::io::Error::other(
                    "certification interrupted lease-file observation",
                )),
                if create {
                    MediaOwnerAdmissionEffectFate::EffectPossible
                } else {
                    MediaOwnerAdmissionEffectFate::DeniedBeforeEffect
                },
                None,
            ))
        }
        Ok(file) => {
            attempt.completed(0);
            let effect_fate = if create {
                MediaOwnerAdmissionEffectFate::EffectPossible
            } else {
                MediaOwnerAdmissionEffectFate::DeniedBeforeEffect
            };
            require_single_link(&file, boundary).map_err(|denial| {
                MutationOwnershipAcquisitionFailure::new(denial, effect_fate, None)
            })?;
            Ok(OpenedMutationLock {
                file: file.into_std(),
                effect_fate,
            })
        }
        Err(error) => {
            attempt.denied();
            Err(MutationOwnershipAcquisitionFailure::before_effect(
                super::mutation_ownership::lock_error(error),
            ))
        }
    }
}

fn require_single_link(
    file: &cap_std::fs::File,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<(), MutationOwnershipDenial> {
    let metadata = super::admitted_namespace::boundary_io_call(
        boundary,
        MediaOperationRole::InspectNamespaceEntry,
        || file.metadata(),
    )
    .map_err(super::mutation_ownership::lock_error)?;
    if opened_handle_link_count(&metadata) == Some(1) {
        Ok(())
    } else {
        Err(confinement(NamespaceConfinementDenialKind::MultipleLinks))
    }
}

#[cfg(any(unix, windows))]
fn opened_handle_link_count(metadata: &cap_std::fs::Metadata) -> Option<u64> {
    use cap_fs_ext::MetadataExt;

    Some(metadata.nlink())
}

#[cfg(not(any(unix, windows)))]
fn opened_handle_link_count(_metadata: &cap_std::fs::Metadata) -> Option<u64> {
    None
}

const fn before_effect(
    denial: MutationOwnershipDenial,
) -> super::owner_admission_effect::MutationOwnershipAcquisitionFailure {
    super::owner_admission_effect::MutationOwnershipAcquisitionFailure::before_effect(denial)
}

fn confinement(kind: NamespaceConfinementDenialKind) -> MutationOwnershipDenial {
    MutationOwnershipDenial::Confinement(NamespaceConfinementDenial::structural(kind))
}
