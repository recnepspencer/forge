use std::path::Path;

use worth_store_authority::{
    PrimaryServeAdmissionDenial, PrimaryServeLease, PrimaryServingAuthority,
};
use worth_store_offline_verifier::{
    verify_replica_promotion_target, IndependentlyVerifiedReplicaTarget,
    ReplicaTargetVerificationBudget, ReplicaTargetVerificationDenial,
};
use worth_store_replication::{
    OldPrimaryRejoinDenial, OldPrimaryRejoinExecutionDenial, ReplicaPromotionReceipt,
};

use crate::{OperationalControlStore, OperationalControlStorePort, OperationalTransitionId};

use super::{ExecutedReplicaPromotion, RecoveredReplicaPromotion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicaPromotionPublicationRequest {
    receipt_identity: [u8; 32],
    target_identity: [u8; 32],
    verification_identity: [u8; 32],
    fence_identity: [u8; 32],
    promoted_epoch: u64,
}

impl ReplicaPromotionPublicationRequest {
    pub const fn receipt_identity(self) -> [u8; 32] {
        self.receipt_identity
    }
    pub const fn target_identity(self) -> [u8; 32] {
        self.target_identity
    }
    pub const fn verification_identity(self) -> [u8; 32] {
        self.verification_identity
    }
    pub const fn fence_identity(self) -> [u8; 32] {
        self.fence_identity
    }
    pub const fn promoted_epoch(self) -> u64 {
        self.promoted_epoch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicaPromotionPublicationReceipt {
    publication_identity: [u8; 32],
    target_identity: [u8; 32],
    promoted_epoch: u64,
}

impl ReplicaPromotionPublicationReceipt {
    pub const fn from_publication_owner(
        publication_identity: [u8; 32],
        target_identity: [u8; 32],
        promoted_epoch: u64,
    ) -> Self {
        Self {
            publication_identity,
            target_identity,
            promoted_epoch,
        }
    }
    pub const fn publication_identity(self) -> [u8; 32] {
        self.publication_identity
    }
}

pub trait ReplicaPromotionPublicationPort {
    fn publish_promoted_replica(
        &mut self,
        request: ReplicaPromotionPublicationRequest,
    ) -> Result<ReplicaPromotionPublicationReceipt, ReplicaPromotionPublicationDenial>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaPromotionPublicationDenial {
    OwnerRejected,
    BindingMismatch,
}

#[derive(Debug)]
pub enum ReplicaPromotionFinalizationDenial {
    Verification(ReplicaTargetVerificationDenial),
    AlreadyPublished,
    Publication(ReplicaPromotionPublicationDenial),
    Control(crate::OperationalControlAppendDenial),
    ServeLease(PrimaryServeAdmissionDenial),
    Rejoin(OldPrimaryRejoinDenial),
    RejoinExecution(OldPrimaryRejoinExecutionDenial),
}

#[derive(Debug)]
pub struct PostVerifiedReplicaPromotion {
    executed: ExecutedReplicaPromotion,
    verification: IndependentlyVerifiedReplicaTarget,
}

#[derive(Debug)]
pub struct PublishedReplicaPromotion {
    executed: ExecutedReplicaPromotion,
    publication: ReplicaPromotionPublicationReceipt,
}

#[derive(Debug)]
pub struct CurrentReplicaPromotion {
    pub(super) executed: ExecutedReplicaPromotion,
    pub(super) publication: ReplicaPromotionPublicationReceipt,
    pub(super) serve_lease: PrimaryServeLease,
}

impl ExecutedReplicaPromotion {
    pub fn post_verify(
        self,
        target_root: &Path,
        budget: ReplicaTargetVerificationBudget,
    ) -> Result<PostVerifiedReplicaPromotion, ReplicaPromotionFinalizationDenial> {
        post_verify(self, target_root, budget)
    }
}

impl RecoveredReplicaPromotion {
    pub fn post_verify(
        self,
        target_root: &Path,
        budget: ReplicaTargetVerificationBudget,
    ) -> Result<PostVerifiedReplicaPromotion, ReplicaPromotionFinalizationDenial> {
        if self.publication.is_some() {
            return Err(ReplicaPromotionFinalizationDenial::AlreadyPublished);
        }
        post_verify(self.into_executed(), target_root, budget)
    }

    fn into_executed(self) -> ExecutedReplicaPromotion {
        ExecutedReplicaPromotion {
            operation_id: self.operation_id,
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            receipt: self.receipt,
        }
    }
}

impl PostVerifiedReplicaPromotion {
    pub fn publish(
        self,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        port: &mut impl ReplicaPromotionPublicationPort,
    ) -> Result<PublishedReplicaPromotion, ReplicaPromotionFinalizationDenial> {
        publish(self, control, transition, port)
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn publish_with_certification_control_store(
        self,
        control: &dyn OperationalControlStorePort,
        transition: OperationalTransitionId,
        port: &mut impl ReplicaPromotionPublicationPort,
    ) -> Result<PublishedReplicaPromotion, ReplicaPromotionFinalizationDenial> {
        publish(self, control, transition, port)
    }
}

fn publish(
    verified: PostVerifiedReplicaPromotion,
    control: &dyn OperationalControlStorePort,
    transition: OperationalTransitionId,
    port: &mut impl ReplicaPromotionPublicationPort,
) -> Result<PublishedReplicaPromotion, ReplicaPromotionFinalizationDenial> {
    let request = publication_request(&verified.executed.receipt, &verified.verification);
    let publication = port
        .publish_promoted_replica(request)
        .map_err(ReplicaPromotionFinalizationDenial::Publication)?;
    if publication.publication_identity == [0; 32]
        || publication.target_identity != request.target_identity
        || publication.promoted_epoch != request.promoted_epoch
    {
        return Err(ReplicaPromotionFinalizationDenial::Publication(
            ReplicaPromotionPublicationDenial::BindingMismatch,
        ));
    }
    let record = crate::OperationalControlRecord::replica_promotion_published(
        verified.executed.authority_identity,
        verified.executed.operation_id.clone(),
        transition,
        &verified.executed.receipt,
        verified.verification.verification_identity(),
        publication.publication_identity,
    );
    control
        .append(&record)
        .map_err(ReplicaPromotionFinalizationDenial::Control)?;
    Ok(PublishedReplicaPromotion {
        executed: verified.executed,
        publication,
    })
}

impl PublishedReplicaPromotion {
    pub fn readmit(
        self,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        serving: &PrimaryServingAuthority<'_>,
        now_tick: u64,
        requested_until_tick: u64,
    ) -> Result<CurrentReplicaPromotion, ReplicaPromotionFinalizationDenial> {
        readmit(
            self,
            control,
            transition,
            serving,
            now_tick,
            requested_until_tick,
        )
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn readmit_with_certification_control_store(
        self,
        control: &dyn OperationalControlStorePort,
        transition: OperationalTransitionId,
        serving: &PrimaryServingAuthority<'_>,
        now_tick: u64,
        requested_until_tick: u64,
    ) -> Result<CurrentReplicaPromotion, ReplicaPromotionFinalizationDenial> {
        readmit(
            self,
            control,
            transition,
            serving,
            now_tick,
            requested_until_tick,
        )
    }
}

fn readmit(
    published: PublishedReplicaPromotion,
    control: &dyn OperationalControlStorePort,
    transition: OperationalTransitionId,
    serving: &PrimaryServingAuthority<'_>,
    now_tick: u64,
    requested_until_tick: u64,
) -> Result<CurrentReplicaPromotion, ReplicaPromotionFinalizationDenial> {
    let minimum_epoch = published
        .executed
        .receipt
        .promoted_epoch()
        .get()
        .saturating_sub(1);
    let serve_lease = serving
        .acquire(minimum_epoch, now_tick, requested_until_tick)
        .map_err(ReplicaPromotionFinalizationDenial::ServeLease)?;
    if serve_lease.epoch() < published.executed.receipt.promoted_epoch().get() {
        return Err(ReplicaPromotionFinalizationDenial::Publication(
            ReplicaPromotionPublicationDenial::BindingMismatch,
        ));
    }
    let record = crate::OperationalControlRecord::replica_promotion_readmitted(
        published.executed.authority_identity,
        published.executed.operation_id.clone(),
        transition,
        published.publication.publication_identity,
        serve_lease,
    );
    control
        .append(&record)
        .map_err(ReplicaPromotionFinalizationDenial::Control)?;
    Ok(CurrentReplicaPromotion {
        executed: published.executed,
        publication: published.publication,
        serve_lease,
    })
}

impl CurrentReplicaPromotion {
    pub const fn serve_lease(&self) -> PrimaryServeLease {
        self.serve_lease
    }
    pub const fn publication(&self) -> ReplicaPromotionPublicationReceipt {
        self.publication
    }
    pub const fn promotion_receipt(&self) -> &ReplicaPromotionReceipt {
        &self.executed.receipt
    }
}

fn post_verify(
    executed: ExecutedReplicaPromotion,
    target_root: &Path,
    budget: ReplicaTargetVerificationBudget,
) -> Result<PostVerifiedReplicaPromotion, ReplicaPromotionFinalizationDenial> {
    let verification = verify_replica_promotion_target(&executed.receipt, target_root, budget)
        .map_err(ReplicaPromotionFinalizationDenial::Verification)?;
    Ok(PostVerifiedReplicaPromotion {
        executed,
        verification,
    })
}

fn publication_request(
    receipt: &ReplicaPromotionReceipt,
    verification: &IndependentlyVerifiedReplicaTarget,
) -> ReplicaPromotionPublicationRequest {
    ReplicaPromotionPublicationRequest {
        receipt_identity: receipt.receipt_identity(),
        target_identity: receipt.durable_target_identity(),
        verification_identity: verification.verification_identity(),
        fence_identity: receipt.fence_identity(),
        promoted_epoch: receipt.promoted_epoch().get(),
    }
}
