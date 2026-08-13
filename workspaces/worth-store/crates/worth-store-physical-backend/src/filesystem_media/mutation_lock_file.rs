use cap_fs_ext::{FollowSymlinks, OpenOptionsFollowExt};
use cap_std::fs::OpenOptions;
use worth_store_physical_format::store_namespace::StoreNamespaceRelativeRole;

use super::{
    MediaOperationRole, MutationOwnershipDenial, NamespaceConfinementDenial,
    NamespaceConfinementDenialKind, NamespaceDirectoryHandle, NamespaceRelativePath,
};

pub(super) fn open_or_create(
    owner: super::MediaOwnerIdentity,
    namespace: &NamespaceDirectoryHandle,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<OpenedMutationLock, super::owner_admission_effect::MutationOwnershipAcquisitionFailure>
{
    open_with_policy(owner, namespace, boundary, MissingLockPolicy::Create)
}

#[cfg(feature = "recovery-runtime-owner")]
pub(super) fn open_existing(
    owner: super::MediaOwnerIdentity,
    namespace: &NamespaceDirectoryHandle,
    boundary: &super::fault_interposition::MediaFaultInterposer,
) -> Result<OpenedMutationLock, super::owner_admission_effect::MutationOwnershipAcquisitionFailure>
{
    open_with_policy(owner, namespace, boundary, MissingLockPolicy::Deny)
}

#[derive(Clone, Copy)]
enum MissingLockPolicy {
    Create,
    #[cfg(feature = "recovery-runtime-owner")]
    Deny,
}

fn open_with_policy(
    owner: super::MediaOwnerIdentity,
    namespace: &NamespaceDirectoryHandle,
    boundary: &super::fault_interposition::MediaFaultInterposer,
    missing: MissingLockPolicy,
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
        Ok(_) => open_with_mode(
            namespace,
            path.file_name(),
            boundary,
            MutationLockOpenMode::Existing,
        ),
        Err(error)
            if error.kind() == std::io::ErrorKind::NotFound
                && matches!(missing, MissingLockPolicy::Create) =>
        {
            match open_with_mode(
                namespace,
                path.file_name(),
                boundary,
                MutationLockOpenMode::CreateNew,
            ) {
                Err(failure)
                    if matches!(
                        failure.denial(),
                        MutationOwnershipDenial::LockOperationFailed {
                            kind: std::io::ErrorKind::AlreadyExists,
                            ..
                        }
                    ) =>
                {
                    open_with_mode(
                        namespace,
                        path.file_name(),
                        boundary,
                        MutationLockOpenMode::Existing,
                    )
                }
                result => result,
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err(before_effect(super::mutation_ownership::lock_error(error)))
        }
        Err(error) => Err(before_effect(super::mutation_ownership::lock_error(error))),
    }
}

pub(super) struct OpenedMutationLock {
    pub(super) file: std::fs::File,
    pub(super) effect_fate: super::owner_admission_effect::MediaOwnerAdmissionEffectFate,
}

#[derive(Clone, Copy)]
enum MutationLockOpenMode {
    Existing,
    CreateNew,
}

impl MutationLockOpenMode {
    const fn role(self) -> MediaOperationRole {
        match self {
            Self::Existing => MediaOperationRole::OpenMutationLease,
            Self::CreateNew => MediaOperationRole::CreateMutationLease,
        }
    }

    const fn effect_fate(self) -> super::owner_admission_effect::MediaOwnerAdmissionEffectFate {
        match self {
            Self::Existing => {
                super::owner_admission_effect::MediaOwnerAdmissionEffectFate::DeniedBeforeEffect
            }
            Self::CreateNew => {
                super::owner_admission_effect::MediaOwnerAdmissionEffectFate::EffectPossible
            }
        }
    }
}

fn open_with_mode(
    namespace: &NamespaceDirectoryHandle,
    name: &str,
    boundary: &super::fault_interposition::MediaFaultInterposer,
    mode: MutationLockOpenMode,
) -> Result<OpenedMutationLock, super::owner_admission_effect::MutationOwnershipAcquisitionFailure>
{
    use super::owner_admission_effect::MutationOwnershipAcquisitionFailure;

    let attempt = boundary.begin(mode.role(), 0);
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
        .create_new(matches!(mode, MutationLockOpenMode::CreateNew))
        .follow(FollowSymlinks::No);
    match namespace.directory().open_with(name, &options) {
        Ok(file) => classify_open_success(file, attempt, boundary, mode),
        Err(error) => {
            attempt.denied();
            Err(MutationOwnershipAcquisitionFailure::before_effect(
                super::mutation_ownership::lock_error(error),
            ))
        }
    }
}

fn classify_open_success(
    file: cap_std::fs::File,
    attempt: super::fault_interposition::MediaBoundaryAttempt<'_>,
    boundary: &super::fault_interposition::MediaFaultInterposer,
    mode: MutationLockOpenMode,
) -> Result<OpenedMutationLock, super::owner_admission_effect::MutationOwnershipAcquisitionFailure>
{
    use super::owner_admission_effect::MutationOwnershipAcquisitionFailure;

    let effect_fate = mode.effect_fate();
    if attempt.effect_observation_is_indeterminate() {
        attempt.indeterminate(0);
        return Err(MutationOwnershipAcquisitionFailure::new(
            super::mutation_ownership::lock_error(std::io::Error::other(
                "certification interrupted lease-file observation",
            )),
            effect_fate,
            None,
        ));
    }
    attempt.completed(0);
    require_single_link(&file, boundary)
        .map_err(|denial| MutationOwnershipAcquisitionFailure::new(denial, effect_fate, None))?;
    Ok(OpenedMutationLock {
        file: file.into_std(),
        effect_fate,
    })
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
