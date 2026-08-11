use worth_store_physical_format::store_namespace::{
    NamespaceInitializationAttempt, ProposedStoreIdentity, StableStoreIdentity,
    StagedNamespaceName, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
    STORE_NAMESPACE_IDENTITY_RECORD_LENGTH,
};

use super::{
    FilesystemMediaOwner, MediaOwnerIdentity, MediaQualificationPostOwnershipCause,
    NamespaceFileOpenResult, PositionedReadRequest, PositionedReadResult, StagedNamespaceFile,
    StagedNamespaceFileOutcome, StagedNamespaceSynchronizationOutcome, StagedNamespaceWriteOutcome,
};

pub(super) struct AdmittedStoreIdentity {
    stable: StableStoreIdentity,
    owner: MediaOwnerIdentity,
}

impl AdmittedStoreIdentity {
    pub(super) const fn stable_identity(&self) -> StableStoreIdentity {
        self.stable
    }

    pub(super) fn belongs_to(&self, owner: MediaOwnerIdentity) -> bool {
        self.owner == owner
    }
}

pub(super) fn admit_store_identity(
    owner: &FilesystemMediaOwner,
) -> Result<AdmittedStoreIdentity, MediaQualificationPostOwnershipCause> {
    admit_store_identity_with_policy(owner, OrdinaryIdentityAdmission)
}

#[cfg(feature = "recovery-runtime-owner")]
pub(crate) fn admit_existing_store_identity(
    owner: &FilesystemMediaOwner,
) -> Result<AdmittedStoreIdentity, MediaQualificationPostOwnershipCause> {
    admit_store_identity_with_policy(owner, ExistingRecoveryIdentityAdmission)
}

fn admit_store_identity_with_policy<P: IdentityAdmissionPolicy>(
    owner: &FilesystemMediaOwner,
    policy: P,
) -> Result<AdmittedStoreIdentity, MediaQualificationPostOwnershipCause> {
    let path = owner.identity_record_path();
    match owner.open_existing(&path).into_result() {
        NamespaceFileOpenResult::Opened { handle, .. } => {
            let length = match handle.metadata().result() {
                super::MediaMetadataResult::Observed(metadata) => metadata.logical_length(),
                super::MediaMetadataResult::Failed(_) => return Err(identity_read(owner)),
            };
            if length != STORE_NAMESPACE_IDENTITY_RECORD_LENGTH as u64 {
                return Err(identity_read(owner));
            }
            let mut bytes = [0_u8; STORE_NAMESPACE_IDENTITY_RECORD_LENGTH];
            match handle
                .positioned_read(PositionedReadRequest::new(0, &mut bytes))
                .result()
            {
                PositionedReadResult::Transferred(transfer)
                    if transfer.bytes() == STORE_NAMESPACE_IDENTITY_RECORD_LENGTH as u64 => {}
                _ => return Err(identity_read(owner)),
            }
            let identity = StoreNamespaceIdentityRecord::decode(&bytes)
                .map(|record| record.published_identity())
                .map_err(|_| identity_read(owner))?;
            policy.reconcile_existing(owner)?;
            Ok(AdmittedStoreIdentity {
                stable: identity,
                owner: owner.identity(),
            })
        }
        NamespaceFileOpenResult::Failed(failure)
            if failure.context().io_kind() == Some(std::io::ErrorKind::NotFound) =>
        {
            policy.admit_missing(owner)
        }
        NamespaceFileOpenResult::Failed(_) => Err(identity_read(owner)),
    }
}

trait IdentityAdmissionPolicy {
    fn reconcile_existing(
        self,
        owner: &FilesystemMediaOwner,
    ) -> Result<(), MediaQualificationPostOwnershipCause>;

    fn admit_missing(
        self,
        owner: &FilesystemMediaOwner,
    ) -> Result<AdmittedStoreIdentity, MediaQualificationPostOwnershipCause>;
}

#[derive(Clone, Copy)]
struct OrdinaryIdentityAdmission;

impl IdentityAdmissionPolicy for OrdinaryIdentityAdmission {
    fn reconcile_existing(
        self,
        owner: &FilesystemMediaOwner,
    ) -> Result<(), MediaQualificationPostOwnershipCause> {
        reconcile_identity_publication(owner)
    }

    fn admit_missing(
        self,
        owner: &FilesystemMediaOwner,
    ) -> Result<AdmittedStoreIdentity, MediaQualificationPostOwnershipCause> {
        publish_store_identity(owner)
    }
}

#[cfg(feature = "recovery-runtime-owner")]
#[derive(Clone, Copy)]
struct ExistingRecoveryIdentityAdmission;

#[cfg(feature = "recovery-runtime-owner")]
impl IdentityAdmissionPolicy for ExistingRecoveryIdentityAdmission {
    fn reconcile_existing(
        self,
        _owner: &FilesystemMediaOwner,
    ) -> Result<(), MediaQualificationPostOwnershipCause> {
        Ok(())
    }

    fn admit_missing(
        self,
        owner: &FilesystemMediaOwner,
    ) -> Result<AdmittedStoreIdentity, MediaQualificationPostOwnershipCause> {
        Err(identity_read(owner))
    }
}

fn reconcile_identity_publication(
    owner: &FilesystemMediaOwner,
) -> Result<(), MediaQualificationPostOwnershipCause> {
    if !matches!(
        owner.synchronize_directory_publication(owner.namespace_directory()),
        super::DirectoryPublicationSynchronizationOutcome::Synchronized(_)
    ) {
        return Err(identity_publication(owner));
    }
    if matches!(
        owner.synchronize_store_root_publication(),
        super::StoreRootPublicationSynchronizationOutcome::Failed(_)
    ) {
        return Err(identity_publication(owner));
    }
    if matches!(
        owner.synchronize_created_root_parent(),
        super::RootParentPublicationSynchronizationOutcome::Failed(_)
    ) {
        return Err(identity_publication(owner));
    }
    Ok(())
}

fn publish_store_identity(
    owner: &FilesystemMediaOwner,
) -> Result<AdmittedStoreIdentity, MediaQualificationPostOwnershipCause> {
    let proposed = ProposedStoreIdentity::from_nonzero_bytes(random_nonzero(owner)?)
        .ok_or_else(|| identity_publication(owner))?;
    let attempt = NamespaceInitializationAttempt::from_nonzero_bytes(random_nonzero(owner)?)
        .ok_or_else(|| identity_publication(owner))?;
    let record = StoreNamespaceIdentityRecord::new(StoreNamespaceVersion::CURRENT, proposed);
    let name = StagedNamespaceName::for_identity(attempt);
    let staged = match StagedNamespaceFile::create(owner, owner.staged_identity_path(&name)) {
        StagedNamespaceFileOutcome::Created(staged) => staged,
        _ => return Err(identity_publication(owner)),
    };
    let completed = match staged.write_all(&record.encode()) {
        StagedNamespaceWriteOutcome::Completed(completed) => completed,
        _ => return Err(identity_publication(owner)),
    };
    let synchronized = match completed.synchronize() {
        StagedNamespaceSynchronizationOutcome::Synchronized(synchronized) => synchronized,
        _ => return Err(identity_publication(owner)),
    };
    let replaced = match synchronized.replace(owner.identity_publication_target()) {
        super::AtomicReplacementOutcome::Replaced(replaced) => replaced,
        _ => return Err(identity_publication(owner)),
    };
    match replaced.synchronize_publication() {
        super::DurableNamespacePublicationOutcome::Published(_) => Ok(AdmittedStoreIdentity {
            stable: record.published_identity(),
            owner: owner.identity(),
        }),
        _ => Err(identity_publication(owner)),
    }
}

fn random_nonzero(
    owner: &FilesystemMediaOwner,
) -> Result<[u8; 16], MediaQualificationPostOwnershipCause> {
    loop {
        let mut bytes = [0_u8; 16];
        getrandom::fill(&mut bytes).map_err(|_| identity_publication(owner))?;
        if bytes != [0; 16] {
            return Ok(bytes);
        }
    }
}

fn identity_read(_: &FilesystemMediaOwner) -> MediaQualificationPostOwnershipCause {
    MediaQualificationPostOwnershipCause::IdentityRead
}

fn identity_publication(_: &FilesystemMediaOwner) -> MediaQualificationPostOwnershipCause {
    MediaQualificationPostOwnershipCause::IdentityPublication
}
