use worth_store_offline_verifier::{
    post_verify_closed_staged_recovery, BackupVerificationBudget,
    ClosedStagedRecoveryVerificationRequest, PostVerifiedStagedRecovery,
    StagedRecoveryExpectedFrontier,
};
use worth_store_physical_backend::ClosedNonCurrentStagingMedia;

use crate::workflow::{
    ExecutedAuthorityAffectingRepair, ExecutedBackupRestore, ExecutedPointInTimeRecovery,
    ExecutedRollback,
};
use crate::{OperationalOperationId, OperationalSecurityScope};

use super::{
    CurrentRecoveryAuthoritySnapshot, RecoveryAuthorityDelta, RecoveryAuthorityFrontier,
    RecoveryCutoverDenial,
};

struct PostVerifiedRecoveryCore {
    operation_id: OperationalOperationId,
    verified: PostVerifiedStagedRecovery,
    media: ClosedNonCurrentStagingMedia,
    candidate_frontier: RecoveryAuthorityFrontier,
    security_scope: OperationalSecurityScope,
    source_lease: Option<RecoveryCutoverSourceLease>,
    authority_posture: worth_store_authority::RecoveryAuthorityAdmissionPosture,
}

struct RecoveryPostVerificationBasis {
    operation_id: OperationalOperationId,
    media: ClosedNonCurrentStagingMedia,
    expected_frontier: StagedRecoveryExpectedFrontier,
    candidate_frontier: RecoveryAuthorityFrontier,
    security_scope: OperationalSecurityScope,
    source_lease: Option<RecoveryCutoverSourceLease>,
    expected_owner_verification:
        Option<worth_store_offline_verifier::StagedRecoveryOwnerVerificationSet>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum RecoveryCutoverSourceLease {
    PointInTime(worth_store_physical_isolation::PitrReachabilityLease),
    Rollback(worth_store_physical_isolation::RollbackReachabilityLease),
}

impl RecoveryCutoverSourceLease {
    pub(super) fn release(
        self,
    ) -> Result<
        worth_store_physical_isolation::RecoverySourceLeaseReleaseReceipt,
        worth_store_physical_isolation::RecoverySourceLeaseDenial,
    > {
        match self {
            Self::PointInTime(lease) => lease.release(),
            Self::Rollback(lease) => lease.release(),
        }
    }
}

pub(super) struct ResolvedRecoveryCutoverCore {
    post_verified: PostVerifiedRecoveryCore,
    current: CurrentRecoveryAuthoritySnapshot,
    delta: RecoveryAuthorityDelta,
    admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
}

pub struct PostVerifiedBackupRestore(PostVerifiedRecoveryCore);
pub struct PostVerifiedPointInTimeRecovery(PostVerifiedRecoveryCore);
pub struct PostVerifiedRollback(PostVerifiedRecoveryCore);
pub struct PostVerifiedAuthorityAffectingRepair(PostVerifiedRecoveryCore);

pub struct ResolvedBackupRestoreCutoverCandidate(pub(super) ResolvedRecoveryCutoverCore);
pub struct ResolvedPointInTimeRecoveryCutoverCandidate(pub(super) ResolvedRecoveryCutoverCore);
pub struct ResolvedRollbackCutoverCandidate(pub(super) ResolvedRecoveryCutoverCore);
pub struct ResolvedAuthorityAffectingRepairCutoverCandidate(pub(super) ResolvedRecoveryCutoverCore);

impl ExecutedBackupRestore {
    pub fn post_verify(
        self,
        budget: BackupVerificationBudget,
    ) -> Result<PostVerifiedBackupRestore, RecoveryCutoverDenial> {
        let receipt = self.receipt().recovered_frontier();
        let expected = StagedRecoveryExpectedFrontier::exact(
            receipt.durable_checkpoint_lsn(),
            receipt.wal_end_exclusive_lsn(),
            receipt.acknowledged_frontier(),
        )
        .expect("restore owner receipt carries an ordered frontier");
        let core = post_verify(
            RecoveryPostVerificationBasis {
                operation_id: self.operation_id().clone(),
                media: self.staged_media().clone(),
                expected_frontier: expected,
                candidate_frontier: RecoveryAuthorityFrontier::from_staged(
                    receipt.durable_checkpoint_lsn(),
                    receipt.wal_end_exclusive_lsn(),
                    receipt.acknowledged_frontier(),
                    self.staging_authority(),
                    self.staged_media().content_fingerprint(),
                ),
                security_scope: self.security_scope(),
                source_lease: None,
                expected_owner_verification: Some(self.owner_verification()),
            },
            budget,
        )?;
        Ok(PostVerifiedBackupRestore(core))
    }
}

impl ExecutedPointInTimeRecovery {
    pub fn post_verify(
        self,
        budget: BackupVerificationBudget,
    ) -> Result<PostVerifiedPointInTimeRecovery, RecoveryCutoverDenial> {
        let frontier = self.receipt().recovery().exact_frontier();
        let expected = StagedRecoveryExpectedFrontier::exact(
            frontier.checkpoint_durability(),
            frontier.wal_structural(),
            frontier.client_acknowledged(),
        )
        .expect("PITR owner receipt carries an ordered frontier");
        let core = post_verify(
            RecoveryPostVerificationBasis {
                operation_id: self.operation_id().clone(),
                media: self.staged_media().clone(),
                expected_frontier: expected,
                candidate_frontier: RecoveryAuthorityFrontier::from_exact(frontier),
                security_scope: self.security_scope(),
                source_lease: Some(RecoveryCutoverSourceLease::PointInTime(
                    self.source_lease().clone(),
                )),
                expected_owner_verification: Some(self.owner_verification()),
            },
            budget,
        )?;
        Ok(PostVerifiedPointInTimeRecovery(core))
    }
}

impl ExecutedRollback {
    pub fn post_verify(
        self,
        budget: BackupVerificationBudget,
    ) -> Result<PostVerifiedRollback, RecoveryCutoverDenial> {
        let frontier = self.receipt().recovery().frontier();
        let expected = StagedRecoveryExpectedFrontier::exact(
            frontier.checkpoint_durability(),
            frontier.wal_structural(),
            frontier.client_acknowledged(),
        )
        .expect("rollback owner receipt carries an ordered frontier");
        let core = post_verify(
            RecoveryPostVerificationBasis {
                operation_id: self.operation_id().clone(),
                media: self.staged_media().clone(),
                expected_frontier: expected,
                candidate_frontier: RecoveryAuthorityFrontier::from_exact(frontier),
                security_scope: self.security_scope(),
                source_lease: Some(RecoveryCutoverSourceLease::Rollback(
                    self.source_lease().clone(),
                )),
                expected_owner_verification: Some(self.owner_verification()),
            },
            budget,
        )?;
        Ok(PostVerifiedRollback(core))
    }
}

impl ExecutedAuthorityAffectingRepair {
    pub fn post_verify(
        self,
        budget: BackupVerificationBudget,
    ) -> Result<PostVerifiedAuthorityAffectingRepair, RecoveryCutoverDenial> {
        let frontier = self.recovered_frontier();
        let expected = StagedRecoveryExpectedFrontier::exact(
            frontier.durable_checkpoint_lsn(),
            frontier.wal_end_exclusive_lsn(),
            frontier.acknowledged_frontier(),
        )
        .expect("authority repair owner receipt carries an ordered frontier");
        let core = post_verify(
            RecoveryPostVerificationBasis {
                operation_id: self.operation_id().clone(),
                media: self.staged_media().clone(),
                expected_frontier: expected,
                candidate_frontier: RecoveryAuthorityFrontier::from_staged(
                    frontier.durable_checkpoint_lsn(),
                    frontier.wal_end_exclusive_lsn(),
                    frontier.acknowledged_frontier(),
                    self.staging_authority(),
                    self.staged_media().content_fingerprint(),
                ),
                security_scope: self.security_scope(),
                source_lease: None,
                expected_owner_verification: None,
            },
            budget,
        )?;
        Ok(PostVerifiedAuthorityAffectingRepair(core))
    }
}

fn post_verify(
    basis: RecoveryPostVerificationBasis,
    budget: BackupVerificationBudget,
) -> Result<PostVerifiedRecoveryCore, RecoveryCutoverDenial> {
    let request = ClosedStagedRecoveryVerificationRequest::from_closed_media(
        &basis.media,
        basis.expected_frontier,
    );
    let verified = post_verify_closed_staged_recovery(request, budget)
        .map_err(RecoveryCutoverDenial::PostVerification)?;
    if verified.staging_plan_fingerprint() != basis.media.plan_fingerprint()
        || verified.staged_content_fingerprint() != basis.media.content_fingerprint()
    {
        return Err(RecoveryCutoverDenial::PostVerifiedMediaMismatch);
    }
    if basis
        .expected_owner_verification
        .is_some_and(|expected| expected != verified.owner_verification())
    {
        return Err(RecoveryCutoverDenial::OwnerVerificationMismatch);
    }
    let authority_posture = authority_posture(&verified)?;
    Ok(PostVerifiedRecoveryCore {
        operation_id: basis.operation_id,
        verified,
        media: basis.media,
        candidate_frontier: basis.candidate_frontier,
        security_scope: basis.security_scope,
        source_lease: basis.source_lease,
        authority_posture,
    })
}

fn authority_posture(
    verified: &PostVerifiedStagedRecovery,
) -> Result<worth_store_authority::RecoveryAuthorityAdmissionPosture, RecoveryCutoverDenial> {
    let observed = verified.authority_posture();
    let region = |value: worth_store_offline_verifier::StagedRecoveryRegionPosture| {
        worth_store_authority::RecoveryAuthorityRegionPosture::observed(
            value.identity(),
            value.count(),
        )
        .ok_or(RecoveryCutoverDenial::InvalidAuthorityPosture)
    };
    worth_store_authority::RecoveryAuthorityAdmissionPosture::from_independent_post_verification(
        verified.verification_identity(),
        [
            region(observed.trusted())?,
            region(observed.degraded())?,
            region(observed.rebuildable())?,
            region(observed.quarantined())?,
            region(observed.unavailable())?,
        ],
    )
    .ok_or(RecoveryCutoverDenial::InvalidAuthorityPosture)
}

macro_rules! resolve_cutover {
    ($post:ident, $resolved:ident) => {
        impl $post {
            pub const fn authority_posture(
                &self,
            ) -> worth_store_authority::RecoveryAuthorityAdmissionPosture {
                self.0.authority_posture
            }
            pub fn resolve_cutover(
                self,
                current: CurrentRecoveryAuthoritySnapshot,
                admission_policy: worth_store_authority::RecoveryAuthorityAdmissionPolicy,
            ) -> Result<$resolved, RecoveryCutoverDenial> {
                admission_policy
                    .validate(self.0.authority_posture)
                    .map_err(RecoveryCutoverDenial::AuthorityAdmissionPolicy)?;
                let delta =
                    RecoveryAuthorityDelta::between(current.frontier, self.0.candidate_frontier);
                Ok($resolved(ResolvedRecoveryCutoverCore {
                    post_verified: self.0,
                    current,
                    delta,
                    admission_policy,
                }))
            }
        }
        impl $resolved {
            pub const fn authority_delta(&self) -> RecoveryAuthorityDelta {
                self.0.delta
            }
        }
    };
}

resolve_cutover!(
    PostVerifiedBackupRestore,
    ResolvedBackupRestoreCutoverCandidate
);
resolve_cutover!(
    PostVerifiedPointInTimeRecovery,
    ResolvedPointInTimeRecoveryCutoverCandidate
);
resolve_cutover!(PostVerifiedRollback, ResolvedRollbackCutoverCandidate);
resolve_cutover!(
    PostVerifiedAuthorityAffectingRepair,
    ResolvedAuthorityAffectingRepairCutoverCandidate
);

impl ResolvedRecoveryCutoverCore {
    pub(super) fn operation_id(&self) -> &OperationalOperationId {
        &self.post_verified.operation_id
    }
    pub(super) const fn verified(&self) -> &PostVerifiedStagedRecovery {
        &self.post_verified.verified
    }
    pub(super) const fn media(&self) -> &ClosedNonCurrentStagingMedia {
        &self.post_verified.media
    }
    pub(super) const fn security_scope(&self) -> OperationalSecurityScope {
        self.post_verified.security_scope
    }
    pub(super) const fn current(&self) -> &CurrentRecoveryAuthoritySnapshot {
        &self.current
    }
    pub(super) const fn delta(&self) -> RecoveryAuthorityDelta {
        self.delta
    }
    pub(super) fn take_source_lease(&mut self) -> Option<RecoveryCutoverSourceLease> {
        self.post_verified.source_lease.take()
    }
    pub(super) const fn authority_posture(
        &self,
    ) -> worth_store_authority::RecoveryAuthorityAdmissionPosture {
        self.post_verified.authority_posture
    }
    pub(super) const fn admission_policy(
        &self,
    ) -> worth_store_authority::RecoveryAuthorityAdmissionPolicy {
        self.admission_policy
    }
}
