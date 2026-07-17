use std::path::Path;

use worth_store_authority::{
    PrimaryServeAdmissionDenial, PrimaryServeLease, PrimaryServingAuthority,
};
use worth_store_offline_verifier::{
    verify_replica_promotion_target, IndependentlyVerifiedReplicaTarget,
    ReplicaTargetVerificationBudget, ReplicaTargetVerificationDenial,
};
use worth_store_replication::{
    DivergentReplicaHistoryReport, OldPrimaryDivergenceDisposition, OldPrimaryRejoinDenial,
    OldPrimaryRejoinExecutionDenial, OldPrimaryRejoinExecutionPort, OldPrimaryRejoinPlan,
    OldPrimaryRejoinReceipt, ReplicaPromotionReceipt, ReplicationPeerId, ReplicationRejoinOwner,
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
    executed: ExecutedReplicaPromotion,
    publication: ReplicaPromotionPublicationReceipt,
    serve_lease: PrimaryServeLease,
}

#[derive(Debug)]
pub struct GovernedOldPrimaryRejoinPlan {
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_id: crate::OperationalOperationId,
    promotion_receipt_identity: [u8; 32],
    plan: OldPrimaryRejoinPlan,
}

#[derive(Debug)]
pub struct ResolvedOldPrimaryRejoin {
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_id: crate::OperationalOperationId,
    promotion_receipt_identity: [u8; 32],
    receipt: OldPrimaryRejoinReceipt,
}

#[derive(Debug)]
pub struct CompletedOldPrimaryRejoin {
    promotion_receipt_identity: [u8; 32],
    receipt: OldPrimaryRejoinReceipt,
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
        let request = publication_request(&self.executed.receipt, &self.verification);
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
            self.executed.authority_identity,
            self.executed.operation_id.clone(),
            transition,
            &self.executed.receipt,
            self.verification.verification_identity(),
            publication.publication_identity,
        );
        control
            .append(&record)
            .map_err(ReplicaPromotionFinalizationDenial::Control)?;
        Ok(PublishedReplicaPromotion {
            executed: self.executed,
            publication,
        })
    }
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
        let minimum_epoch = self
            .executed
            .receipt
            .promoted_epoch()
            .get()
            .saturating_sub(1);
        let serve_lease = serving
            .acquire(minimum_epoch, now_tick, requested_until_tick)
            .map_err(ReplicaPromotionFinalizationDenial::ServeLease)?;
        if serve_lease.epoch() < self.executed.receipt.promoted_epoch().get() {
            return Err(ReplicaPromotionFinalizationDenial::Publication(
                ReplicaPromotionPublicationDenial::BindingMismatch,
            ));
        }
        let record = crate::OperationalControlRecord::replica_promotion_readmitted(
            self.executed.authority_identity,
            self.executed.operation_id.clone(),
            transition,
            self.publication.publication_identity,
            serve_lease,
        );
        control
            .append(&record)
            .map_err(ReplicaPromotionFinalizationDenial::Control)?;
        Ok(CurrentReplicaPromotion {
            executed: self.executed,
            publication: self.publication,
            serve_lease,
        })
    }
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

    #[allow(clippy::too_many_arguments)]
    pub fn plan_old_primary_rejoin(
        &self,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        old_primary: ReplicationPeerId,
        divergence: DivergentReplicaHistoryReport,
        disposition: OldPrimaryDivergenceDisposition,
        disposition_authorization: Option<[u8; 32]>,
    ) -> Result<GovernedOldPrimaryRejoinPlan, ReplicaPromotionFinalizationDenial> {
        let plan = ReplicationRejoinOwner::plan(
            old_primary,
            self.executed.receipt.promoted_peer().clone(),
            divergence,
            disposition,
            disposition_authorization,
        )
        .map_err(ReplicaPromotionFinalizationDenial::Rejoin)?;
        let record = crate::OperationalControlRecord::old_primary_rejoin_planned(
            self.executed.authority_identity,
            self.executed.operation_id.clone(),
            transition,
            self.executed.receipt.receipt_identity(),
            &plan,
        );
        control
            .append(&record)
            .map_err(ReplicaPromotionFinalizationDenial::Control)?;
        Ok(GovernedOldPrimaryRejoinPlan {
            authority_identity: self.executed.authority_identity,
            operation_id: self.executed.operation_id.clone(),
            promotion_receipt_identity: self.executed.receipt.receipt_identity(),
            plan,
        })
    }
}

impl GovernedOldPrimaryRejoinPlan {
    pub const fn promotion_receipt_identity(&self) -> [u8; 32] {
        self.promotion_receipt_identity
    }
    pub const fn plan(&self) -> &OldPrimaryRejoinPlan {
        &self.plan
    }

    pub fn execute(
        self,
        port: &mut impl OldPrimaryRejoinExecutionPort,
    ) -> Result<ResolvedOldPrimaryRejoin, ReplicaPromotionFinalizationDenial> {
        let receipt = self
            .plan
            .execute(port)
            .map_err(ReplicaPromotionFinalizationDenial::RejoinExecution)?;
        Ok(ResolvedOldPrimaryRejoin {
            authority_identity: self.authority_identity,
            operation_id: self.operation_id,
            promotion_receipt_identity: self.promotion_receipt_identity,
            receipt,
        })
    }
}

impl ResolvedOldPrimaryRejoin {
    pub fn complete(
        self,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
    ) -> Result<CompletedOldPrimaryRejoin, ReplicaPromotionFinalizationDenial> {
        let record = crate::OperationalControlRecord::old_primary_rejoin_completed(
            self.authority_identity,
            self.operation_id,
            transition,
            &self.receipt,
        );
        control
            .append(&record)
            .map_err(ReplicaPromotionFinalizationDenial::Control)?;
        Ok(CompletedOldPrimaryRejoin {
            promotion_receipt_identity: self.promotion_receipt_identity,
            receipt: self.receipt,
        })
    }

    pub const fn receipt(&self) -> &OldPrimaryRejoinReceipt {
        &self.receipt
    }
}

impl CompletedOldPrimaryRejoin {
    pub const fn promotion_receipt_identity(&self) -> [u8; 32] {
        self.promotion_receipt_identity
    }
    pub const fn receipt(&self) -> &OldPrimaryRejoinReceipt {
        &self.receipt
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
