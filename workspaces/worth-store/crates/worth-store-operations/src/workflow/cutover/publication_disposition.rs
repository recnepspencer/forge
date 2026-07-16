use sha2::{Digest, Sha256};
use worth_store_authority::{
    RecoveryAuthorityReadmissionDenial, RecoveryCutoverAuthorityOwner,
    RecoveryWriteFenceDisposition, RecoveryWriteFencePort, RecoveryWriteFenceReleaseReceipt,
    StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness,
};
use worth_store_physical_isolation::AtomicRecoveryPublicationReceipt;

use crate::{
    OperationalControlRecord, OperationalControlStore, OperationalControlStorePort,
    OperationalOperationId, OperationalTransitionId,
};

use super::protocol::{PublishedCutoverCore, ReadmittedCutoverCore};
use super::RecoveryCutoverExecutionDenial;

pub(super) enum CoreReadmissionOutcome {
    Readmitted(Box<ReadmittedCutoverCore>),
    Rejected(Box<PublishedRejectedCore>),
}

pub(super) struct PublishedRejectedCore {
    pub(super) operation_id: OperationalOperationId,
    pub(super) publication: AtomicRecoveryPublicationReceipt,
    pub(super) denial: RecoveryAuthorityReadmissionDenial,
    pub(super) observed_authority: StoreCurrentAuthorityIdentity,
    pub(super) fence_release: RecoveryWriteFenceReleaseReceipt,
    pub(super) source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
}

pub(super) struct PublishedAbandonedCore {
    pub(super) publication: AtomicRecoveryPublicationReceipt,
    pub(super) reason_identity: [u8; 32],
    pub(super) fence_release: RecoveryWriteFenceReleaseReceipt,
    pub(super) source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
}

pub(super) struct PublishedRetainedForForensicsCore {
    pub(super) publication: AtomicRecoveryPublicationReceipt,
    pub(super) retention_plan_identity: [u8; 32],
    pub(super) fence_release: RecoveryWriteFenceReleaseReceipt,
    pub(super) source_lease: Option<super::post_verification::RecoveryCutoverSourceLease>,
}

pub(super) fn release_terminal_source_lease(
    source_lease: &mut Option<super::post_verification::RecoveryCutoverSourceLease>,
) -> Result<
    worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
    super::operation_cutover::RecoverySourceLeaseFinalizationDenial,
> {
    source_lease
        .take()
        .ok_or(
            super::operation_cutover::RecoverySourceLeaseFinalizationDenial::MissingOrWrongLease,
        )?
        .release()
        .map_err(super::operation_cutover::RecoverySourceLeaseFinalizationDenial::Isolation)
}

pub(super) fn attempt_readmission<K>(
    published: PublishedCutoverCore<K>,
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    current: &StoreCurrentAuthorityWitness,
    fence_port: &impl RecoveryWriteFencePort,
) -> Result<CoreReadmissionOutcome, RecoveryCutoverExecutionDenial> {
    match RecoveryCutoverAuthorityOwner::readmit_published_recovery(
        current,
        published.fence,
        published.publication.publication_identity(),
        published.publication.candidate_media_identity(),
        published.authority_posture,
        published.admission_policy,
    ) {
        Ok(readmission) => {
            let basis = readmitted_disposition_basis(readmission.publication_identity());
            persist_disposition(
                control,
                published.fence.fenced_authority(),
                current.authority_identity(),
                published.operation_id.clone(),
                transition,
                published.publication.publication_identity(),
                1,
                basis,
            )?;
            let fence_release = release_and_record(
                control,
                published.fence.fenced_authority(),
                published.operation_id.clone(),
                published.publication.publication_identity(),
                published.fence,
                RecoveryWriteFenceDisposition::Readmitted,
                fence_port,
            )?;
            let _authorization_receipt = published.consumed.receipt();
            Ok(CoreReadmissionOutcome::Readmitted(Box::new(
                ReadmittedCutoverCore {
                    publication: published.publication,
                    readmission,
                    fence_release,
                    source_lease: published.source_lease,
                },
            )))
        }
        Err(denial) => {
            let basis = rejection_disposition_basis(denial, current.authority_identity());
            persist_disposition(
                control,
                published.fence.fenced_authority(),
                current.authority_identity(),
                published.operation_id.clone(),
                transition,
                published.publication.publication_identity(),
                2,
                basis,
            )?;
            let fence_release = release_and_record(
                control,
                published.fence.fenced_authority(),
                published.operation_id.clone(),
                published.publication.publication_identity(),
                published.fence,
                RecoveryWriteFenceDisposition::RejectedByAuthority,
                fence_port,
            )?;
            Ok(CoreReadmissionOutcome::Rejected(Box::new(
                PublishedRejectedCore {
                    operation_id: published.operation_id,
                    publication: published.publication,
                    denial,
                    observed_authority: current.authority_identity(),
                    fence_release,
                    source_lease: published.source_lease,
                },
            )))
        }
    }
}

pub(super) fn abandon<K>(
    published: PublishedCutoverCore<K>,
    reason_identity: [u8; 32],
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    fence_port: &impl RecoveryWriteFencePort,
) -> Result<PublishedAbandonedCore, RecoveryCutoverExecutionDenial> {
    if reason_identity == [0; 32] {
        return Err(RecoveryCutoverExecutionDenial::InvalidDispositionBasis);
    }
    persist_disposition(
        control,
        published.fence.fenced_authority(),
        published.fence.fenced_authority(),
        published.operation_id.clone(),
        transition,
        published.publication.publication_identity(),
        3,
        reason_identity,
    )?;
    let fence_release = release_and_record(
        control,
        published.fence.fenced_authority(),
        published.operation_id,
        published.publication.publication_identity(),
        published.fence,
        RecoveryWriteFenceDisposition::Abandoned,
        fence_port,
    )?;
    Ok(PublishedAbandonedCore {
        publication: published.publication,
        reason_identity,
        fence_release,
        source_lease: published.source_lease,
    })
}

pub(super) fn retain_for_forensics<K>(
    published: PublishedCutoverCore<K>,
    retention_plan_identity: [u8; 32],
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    fence_port: &impl RecoveryWriteFencePort,
) -> Result<PublishedRetainedForForensicsCore, RecoveryCutoverExecutionDenial> {
    if retention_plan_identity == [0; 32] {
        return Err(RecoveryCutoverExecutionDenial::InvalidDispositionBasis);
    }
    persist_disposition(
        control,
        published.fence.fenced_authority(),
        published.fence.fenced_authority(),
        published.operation_id.clone(),
        transition,
        published.publication.publication_identity(),
        4,
        retention_plan_identity,
    )?;
    let fence_release = release_and_record(
        control,
        published.fence.fenced_authority(),
        published.operation_id,
        published.publication.publication_identity(),
        published.fence,
        RecoveryWriteFenceDisposition::RetainedForForensics,
        fence_port,
    )?;
    Ok(PublishedRetainedForForensicsCore {
        publication: published.publication,
        retention_plan_identity,
        fence_release,
        source_lease: published.source_lease,
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn release_and_record(
    control: &OperationalControlStore,
    authority: StoreCurrentAuthorityIdentity,
    operation: OperationalOperationId,
    publication_identity: [u8; 32],
    fence: worth_store_authority::RecoveryWriteFenceReceipt,
    disposition: RecoveryWriteFenceDisposition,
    port: &impl RecoveryWriteFencePort,
) -> Result<RecoveryWriteFenceReleaseReceipt, RecoveryCutoverExecutionDenial> {
    let receipt = RecoveryCutoverAuthorityOwner::release_write_fence(fence, disposition, port)
        .map_err(RecoveryCutoverExecutionDenial::Fence)?;
    control
        .append(
            &OperationalControlRecord::recovery_publication_fence_released(
                authority,
                operation,
                publication_identity,
                fence.fence_identity(),
                fence.plan_fingerprint(),
                match disposition {
                    RecoveryWriteFenceDisposition::Readmitted => 1,
                    RecoveryWriteFenceDisposition::RejectedByAuthority => 2,
                    RecoveryWriteFenceDisposition::Abandoned => 3,
                    RecoveryWriteFenceDisposition::RetainedForForensics => 4,
                },
            ),
        )
        .map_err(RecoveryCutoverExecutionDenial::Control)?;
    Ok(receipt)
}

#[allow(clippy::too_many_arguments)]
fn persist_disposition(
    control: &OperationalControlStore,
    authority: StoreCurrentAuthorityIdentity,
    observed_authority: StoreCurrentAuthorityIdentity,
    operation: OperationalOperationId,
    transition: OperationalTransitionId,
    publication_identity: [u8; 32],
    disposition_tag: u8,
    disposition_basis: [u8; 32],
) -> Result<(), RecoveryCutoverExecutionDenial> {
    control
        .append(&OperationalControlRecord::recovery_publication_disposition(
            authority,
            operation,
            transition,
            publication_identity,
            disposition_tag,
            disposition_basis,
            observed_authority,
        ))
        .map_err(RecoveryCutoverExecutionDenial::Control)?;
    Ok(())
}

pub(super) fn readmitted_disposition_basis(publication_identity: [u8; 32]) -> [u8; 32] {
    Sha256::digest(
        [
            b"worth-store-readmitted-publication-v1".as_slice(),
            &publication_identity,
        ]
        .concat(),
    )
    .into()
}

pub(super) fn rejection_disposition_basis(
    denial: RecoveryAuthorityReadmissionDenial,
    current: StoreCurrentAuthorityIdentity,
) -> [u8; 32] {
    let denial_tag = match denial {
        RecoveryAuthorityReadmissionDenial::StaleCurrentAuthority => 1,
        RecoveryAuthorityReadmissionDenial::PublicationMismatch => 2,
        RecoveryAuthorityReadmissionDenial::AdmissionPolicy(_) => 3,
    };
    let mut digest = Sha256::new();
    digest.update(b"worth-store-publication-rejection-v1");
    digest.update([denial_tag]);
    digest.update(current.fingerprint());
    digest.finalize().into()
}
