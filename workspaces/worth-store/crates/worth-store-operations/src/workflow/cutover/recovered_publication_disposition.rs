use worth_store_authority::{
    RecoveryCutoverAuthorityOwner, RecoveryWriteFenceDisposition, RecoveryWriteFencePort,
    StoreCurrentAuthorityWitness,
};

use crate::{
    OperationalControlRecord, OperationalControlStore, OperationalControlStorePort,
    OperationalTransitionId,
};

use super::publication_disposition::{
    readmitted_disposition_basis, rejection_disposition_basis, release_and_record,
    PublishedAbandonedCore, PublishedRejectedCore, PublishedRetainedForForensicsCore,
};
use super::recovered_publication::{RecoveredPublishedCore, RecoveredReadmittedCore};
use super::RecoveryCutoverExecutionDenial;

pub(super) enum RecoveredCoreReadmissionOutcome {
    Readmitted(Box<RecoveredReadmittedCore>),
    Rejected(Box<PublishedRejectedCore>),
}

pub(super) fn attempt_recovered_readmission(
    recovered: RecoveredPublishedCore,
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    current: &StoreCurrentAuthorityWitness,
    fence_port: &impl RecoveryWriteFencePort,
) -> Result<RecoveredCoreReadmissionOutcome, RecoveryCutoverExecutionDenial> {
    match RecoveryCutoverAuthorityOwner::readmit_published_recovery(
        current,
        recovered.fence,
        recovered.publication.publication_identity(),
        recovered.publication.candidate_media_identity(),
        recovered.authority_posture,
        recovered.admission_policy,
    ) {
        Ok(readmission) => {
            persist(
                control,
                recovered.fence.fenced_authority(),
                current.authority_identity(),
                recovered.operation_id.clone(),
                transition,
                recovered.publication.publication_identity(),
                1,
                readmitted_disposition_basis(recovered.publication.publication_identity()),
            )?;
            let fence_release = release_and_record(
                control,
                recovered.fence.fenced_authority(),
                recovered.operation_id.clone(),
                recovered.publication.publication_identity(),
                recovered.fence,
                RecoveryWriteFenceDisposition::Readmitted,
                fence_port,
            )?;
            Ok(RecoveredCoreReadmissionOutcome::Readmitted(Box::new(
                RecoveredReadmittedCore {
                    operation_id: recovered.operation_id,
                    publication: recovered.publication,
                    readmission,
                    fence_release,
                    source_lease: recovered.source_lease,
                },
            )))
        }
        Err(denial) => {
            persist(
                control,
                recovered.fence.fenced_authority(),
                current.authority_identity(),
                recovered.operation_id.clone(),
                transition,
                recovered.publication.publication_identity(),
                2,
                rejection_disposition_basis(denial, current.authority_identity()),
            )?;
            let fence_release = release_and_record(
                control,
                recovered.fence.fenced_authority(),
                recovered.operation_id.clone(),
                recovered.publication.publication_identity(),
                recovered.fence,
                RecoveryWriteFenceDisposition::RejectedByAuthority,
                fence_port,
            )?;
            Ok(RecoveredCoreReadmissionOutcome::Rejected(Box::new(
                PublishedRejectedCore {
                    operation_id: recovered.operation_id,
                    publication: recovered.publication,
                    denial,
                    observed_authority: current.authority_identity(),
                    fence_release,
                    source_lease: recovered.source_lease,
                },
            )))
        }
    }
}

pub(super) fn abandon_recovered(
    recovered: RecoveredPublishedCore,
    reason_identity: [u8; 32],
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    fence_port: &impl RecoveryWriteFencePort,
) -> Result<PublishedAbandonedCore, RecoveryCutoverExecutionDenial> {
    let source_lease = recovered.source_lease.clone();
    terminal_recovered(
        recovered,
        reason_identity,
        control,
        transition,
        fence_port,
        RecoveryWriteFenceDisposition::Abandoned,
        3,
    )
    .map(|(publication, fence_release)| PublishedAbandonedCore {
        publication,
        reason_identity,
        fence_release,
        source_lease,
    })
}

pub(super) fn retain_recovered_for_forensics(
    recovered: RecoveredPublishedCore,
    retention_plan_identity: [u8; 32],
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    fence_port: &impl RecoveryWriteFencePort,
) -> Result<PublishedRetainedForForensicsCore, RecoveryCutoverExecutionDenial> {
    let source_lease = recovered.source_lease.clone();
    terminal_recovered(
        recovered,
        retention_plan_identity,
        control,
        transition,
        fence_port,
        RecoveryWriteFenceDisposition::RetainedForForensics,
        4,
    )
    .map(
        |(publication, fence_release)| PublishedRetainedForForensicsCore {
            publication,
            retention_plan_identity,
            fence_release,
            source_lease,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn terminal_recovered(
    recovered: RecoveredPublishedCore,
    basis: [u8; 32],
    control: &OperationalControlStore,
    transition: OperationalTransitionId,
    fence_port: &impl RecoveryWriteFencePort,
    disposition: RecoveryWriteFenceDisposition,
    tag: u8,
) -> Result<
    (
        worth_store_physical_isolation::AtomicRecoveryPublicationReceipt,
        worth_store_authority::RecoveryWriteFenceReleaseReceipt,
    ),
    RecoveryCutoverExecutionDenial,
> {
    if basis == [0; 32] {
        return Err(RecoveryCutoverExecutionDenial::InvalidDispositionBasis);
    }
    persist(
        control,
        recovered.fence.fenced_authority(),
        recovered.fence.fenced_authority(),
        recovered.operation_id.clone(),
        transition,
        recovered.publication.publication_identity(),
        tag,
        basis,
    )?;
    let fence_release = release_and_record(
        control,
        recovered.fence.fenced_authority(),
        recovered.operation_id,
        recovered.publication.publication_identity(),
        recovered.fence,
        disposition,
        fence_port,
    )?;
    Ok((recovered.publication, fence_release))
}

#[allow(clippy::too_many_arguments)]
fn persist(
    control: &OperationalControlStore,
    authority: worth_store_authority::StoreCurrentAuthorityIdentity,
    observed_authority: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation: crate::OperationalOperationId,
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
