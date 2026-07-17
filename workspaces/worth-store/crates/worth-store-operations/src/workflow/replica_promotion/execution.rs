use worth_store_authority::{
    FenceProof, PrimaryServeLease, PrimaryServingAuthority, PromotionFenceDenial,
    PromotionFenceOperationIdentity, PromotionFenceRecoveryRequest, PromotionFenceRequest,
    StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness,
};
use worth_store_replication::{ReplicaPromotionOwner, ReplicaPromotionReceipt};

use crate::authorization::{consume_authorization, recover_authorization_consumption};
use crate::control_store::ReplicaPromotionRecoveryHandle;
use crate::{
    AuthorizationConsumptionDenial, AuthorizationConsumptionReceipt,
    AuthorizationRevocationObservation, OperationalControlStore, OperationalControlStorePort,
    OperationalOperationId, OperationalTransitionId,
};

use super::{AuthorizedReplicaPromotionPlan, LoweredReplicaPromotionOwnerPlanDag};

#[derive(Debug)]
pub enum ReplicaPromotionReadinessDenial {
    StaleAuthority,
    Authorization(AuthorizationConsumptionDenial),
}

#[derive(Debug)]
pub struct ExecutionReadyReplicaPromotion<'control> {
    operation_id: OperationalOperationId,
    authorization: AuthorizationConsumptionReceipt,
    authority_identity: StoreCurrentAuthorityIdentity,
    replication: worth_store_replication::LoweredReplicaPromotionPlan,
    old_primary_lease: PrimaryServeLease,
    control: &'control OperationalControlStore,
}

impl AuthorizedReplicaPromotionPlan {
    pub fn ready<'control>(
        self,
        control: &'control OperationalControlStore,
        transition_id: OperationalTransitionId,
        current_authority: &StoreCurrentAuthorityWitness,
        observed_at: u64,
        revocation: AuthorizationRevocationObservation,
    ) -> Result<ExecutionReadyReplicaPromotion<'control>, ReplicaPromotionReadinessDenial> {
        if self.authorization.binding().authority_identity()
            != current_authority.authority_identity()
        {
            return Err(ReplicaPromotionReadinessDenial::StaleAuthority);
        }
        let authority_identity = current_authority.authority_identity();
        let consumed = consume_authorization(
            control,
            self.operation_id.clone(),
            transition_id,
            self.authorization,
            Some(self.replication.fingerprint()),
            observed_at,
            revocation,
        )
        .map_err(ReplicaPromotionReadinessDenial::Authorization)?;
        Ok(ExecutionReadyReplicaPromotion {
            operation_id: self.operation_id,
            authorization: consumed.receipt(),
            authority_identity,
            replication: self.replication,
            old_primary_lease: self.old_primary_lease,
            control,
        })
    }
}

impl LoweredReplicaPromotionOwnerPlanDag {
    pub fn recover_ready<'control>(
        self,
        handle: &ReplicaPromotionRecoveryHandle,
        control: &'control OperationalControlStore,
        current_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<ExecutionReadyReplicaPromotion<'control>, ReplicaPromotionReadinessDenial> {
        let binding = self.authorization.binding();
        if self.operation_id != *handle.operation_id()
            || binding.authority_identity() != current_authority.authority_identity()
            || handle.authority_identity() != current_authority.authority_identity()
            || binding.fingerprint() != handle.authorization_plan_fingerprint()
            || self.replication.fingerprint() != handle.execution_plan_fingerprint()
        {
            return Err(ReplicaPromotionReadinessDenial::StaleAuthority);
        }
        let authorization = recover_authorization_consumption(
            control,
            handle.operation_id(),
            handle.authorization_identity(),
            handle.authorization_plan_fingerprint(),
        )
        .map_err(ReplicaPromotionReadinessDenial::Authorization)?;
        Ok(ExecutionReadyReplicaPromotion {
            operation_id: self.operation_id,
            authorization,
            authority_identity: current_authority.authority_identity(),
            replication: self.replication,
            old_primary_lease: self.old_primary_lease,
            control,
        })
    }
}

#[derive(Debug)]
pub enum ReplicaPromotionFencingDenial {
    InvalidOperationIdentity,
    RecoveredFenceMismatch,
    RecoveredReceiptWithoutFence,
    RecoveredReceiptMismatch,
    Replication(worth_store_replication::ReplicaPromotionDenial),
    Fence(worth_store_authority::PromotionFenceDenial),
}

#[derive(Debug)]
pub enum ReplicaPromotionResume<'control> {
    Ready(ExecutionReadyReplicaPromotion<'control>),
    FenceNeedsPersistence(FencedReplicaPromotion<'control>),
    DurablyFenced(DurablyFencedReplicaPromotion<'control>),
    Recorded(RecoveredReplicaPromotion),
}

#[derive(Debug)]
pub struct RecoveredReplicaPromotion {
    pub(super) operation_id: OperationalOperationId,
    pub(super) authorization: AuthorizationConsumptionReceipt,
    pub(super) authority_identity: StoreCurrentAuthorityIdentity,
    pub(super) receipt: ReplicaPromotionReceipt,
    pub(super) publication: Option<crate::RecoveredReplicaPromotionPublication>,
    pub(super) readmission: Option<crate::RecoveredReplicaPromotionReadmission>,
    pub(super) rejoin_plan_fingerprint: Option<[u8; 32]>,
}

#[derive(Debug)]
pub struct FencedReplicaPromotion<'control> {
    operation_id: OperationalOperationId,
    authorization: AuthorizationConsumptionReceipt,
    authority_identity: StoreCurrentAuthorityIdentity,
    replication: worth_store_replication::LoweredReplicaPromotionPlan,
    fence: FenceProof,
    control: &'control OperationalControlStore,
}

impl<'control> ExecutionReadyReplicaPromotion<'control> {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub fn fence(
        self,
        serving_authority: &PrimaryServingAuthority<'_>,
    ) -> Result<FencedReplicaPromotion<'control>, ReplicaPromotionFencingDenial> {
        let minimum_epoch = self
            .replication
            .candidate()
            .frontier()
            .authority_epoch()
            .max(self.old_primary_lease.epoch());
        let fence = serving_authority
            .fence_old_primary(PromotionFenceRequest::for_old_primary(
                self.old_primary_lease,
                minimum_epoch,
                PromotionFenceOperationIdentity::admit(self.operation_id.stable_fingerprint())
                    .ok_or(ReplicaPromotionFencingDenial::InvalidOperationIdentity)?,
            ))
            .map_err(ReplicaPromotionFencingDenial::Fence)?;
        Ok(FencedReplicaPromotion {
            operation_id: self.operation_id,
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            replication: self.replication,
            fence,
            control: self.control,
        })
    }

    pub fn resume(
        self,
        handle: &ReplicaPromotionRecoveryHandle,
        serving_authority: &PrimaryServingAuthority<'_>,
    ) -> Result<ReplicaPromotionResume<'control>, ReplicaPromotionFencingDenial> {
        let operation_identity =
            PromotionFenceOperationIdentity::admit(self.operation_id.stable_fingerprint())
                .ok_or(ReplicaPromotionFencingDenial::InvalidOperationIdentity)?;
        let minimum_epoch = self
            .replication
            .candidate()
            .frontier()
            .authority_epoch()
            .max(self.old_primary_lease.epoch());
        let recovered_fence =
            match serving_authority.recover_promotion_fence(PromotionFenceRecoveryRequest::new(
                operation_identity,
                self.old_primary_lease,
                minimum_epoch,
            )) {
                Ok(fence) => Some(fence),
                Err(PromotionFenceDenial::FenceNotFound) => None,
                Err(denial) => return Err(ReplicaPromotionFencingDenial::Fence(denial)),
            };

        let Some(recorded_fence) = handle.fence() else {
            if handle.receipt().is_some() {
                return Err(ReplicaPromotionFencingDenial::RecoveredReceiptWithoutFence);
            }
            return match recovered_fence {
                Some(fence) => Ok(ReplicaPromotionResume::FenceNeedsPersistence(
                    self.with_fence(fence),
                )),
                None => Ok(ReplicaPromotionResume::Ready(self)),
            };
        };
        let fence = recovered_fence
            .filter(|proof| {
                proof.fence_identity() == recorded_fence.fence_identity()
                    && proof.promoted_epoch().get() == recorded_fence.promoted_epoch()
            })
            .ok_or(ReplicaPromotionFencingDenial::RecoveredFenceMismatch)?;
        if let Some(receipt) = handle.receipt() {
            let recovered_receipt =
                ReplicaPromotionOwner::record_fenced_promotion(self.replication.clone(), fence)
                    .map_err(ReplicaPromotionFencingDenial::Replication)?;
            if recovered_receipt.receipt_identity() != receipt.receipt_identity() {
                return Err(ReplicaPromotionFencingDenial::RecoveredReceiptMismatch);
            }
            return Ok(ReplicaPromotionResume::Recorded(
                RecoveredReplicaPromotion {
                    operation_id: self.operation_id,
                    authorization: self.authorization,
                    authority_identity: self.authority_identity,
                    receipt: recovered_receipt,
                    publication: handle.publication(),
                    readmission: handle.readmission(),
                    rejoin_plan_fingerprint: handle.rejoin_plan_fingerprint(),
                },
            ));
        }
        Ok(ReplicaPromotionResume::DurablyFenced(
            self.with_fence(fence).into_durably_fenced(),
        ))
    }

    fn with_fence(self, fence: FenceProof) -> FencedReplicaPromotion<'control> {
        FencedReplicaPromotion {
            operation_id: self.operation_id,
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            replication: self.replication,
            fence,
            control: self.control,
        }
    }
}

#[derive(Debug)]
pub enum ReplicaPromotionFencePersistenceDenial {
    Control(crate::OperationalControlAppendDenial),
}

#[derive(Debug)]
pub struct DurablyFencedReplicaPromotion<'control> {
    operation_id: OperationalOperationId,
    authorization: AuthorizationConsumptionReceipt,
    authority_identity: StoreCurrentAuthorityIdentity,
    replication: worth_store_replication::LoweredReplicaPromotionPlan,
    fence: FenceProof,
    control: &'control OperationalControlStore,
}

impl<'control> FencedReplicaPromotion<'control> {
    pub fn persist_fence(
        &self,
        transition_id: OperationalTransitionId,
    ) -> Result<DurablyFencedReplicaPromotion<'control>, ReplicaPromotionFencePersistenceDenial>
    {
        let record = crate::OperationalControlRecord::replica_promotion_fence_recorded(
            self.authority_identity,
            self.operation_id.clone(),
            transition_id,
            self.authorization.plan_fingerprint(),
            self.replication.fingerprint(),
            self.fence,
        );
        self.control
            .append(&record)
            .map_err(ReplicaPromotionFencePersistenceDenial::Control)?;
        Ok(DurablyFencedReplicaPromotion {
            operation_id: self.operation_id.clone(),
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            replication: self.replication.clone(),
            fence: self.fence,
            control: self.control,
        })
    }

    pub const fn fence_proof(&self) -> FenceProof {
        self.fence
    }

    fn into_durably_fenced(self) -> DurablyFencedReplicaPromotion<'control> {
        DurablyFencedReplicaPromotion {
            operation_id: self.operation_id,
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            replication: self.replication,
            fence: self.fence,
            control: self.control,
        }
    }
}

#[derive(Debug)]
pub enum ReplicaPromotionExecutionDenial {
    Replication(worth_store_replication::ReplicaPromotionDenial),
    Control(crate::OperationalControlAppendDenial),
}

#[derive(Debug)]
pub struct ExecutedReplicaPromotion {
    pub(super) operation_id: OperationalOperationId,
    pub(super) authorization: AuthorizationConsumptionReceipt,
    pub(super) authority_identity: StoreCurrentAuthorityIdentity,
    pub(super) receipt: ReplicaPromotionReceipt,
}

impl DurablyFencedReplicaPromotion<'_> {
    pub fn promote(
        &self,
        transition_id: OperationalTransitionId,
    ) -> Result<ExecutedReplicaPromotion, ReplicaPromotionExecutionDenial> {
        let receipt =
            ReplicaPromotionOwner::record_fenced_promotion(self.replication.clone(), self.fence)
                .map_err(ReplicaPromotionExecutionDenial::Replication)?;
        let record = crate::OperationalControlRecord::replica_promotion_recorded(
            self.authority_identity,
            self.operation_id.clone(),
            transition_id,
            self.authorization.plan_fingerprint(),
            &receipt,
        );
        self.control
            .append(&record)
            .map_err(ReplicaPromotionExecutionDenial::Control)?;
        Ok(ExecutedReplicaPromotion {
            operation_id: self.operation_id.clone(),
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            receipt,
        })
    }

    pub const fn fence_proof(&self) -> FenceProof {
        self.fence
    }
}

impl ExecutedReplicaPromotion {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub const fn authorization(&self) -> AuthorizationConsumptionReceipt {
        self.authorization
    }

    pub const fn receipt(&self) -> &ReplicaPromotionReceipt {
        &self.receipt
    }
}

impl RecoveredReplicaPromotion {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub const fn authorization(&self) -> AuthorizationConsumptionReceipt {
        self.authorization
    }

    pub const fn receipt(&self) -> &ReplicaPromotionReceipt {
        &self.receipt
    }

    pub const fn publication(&self) -> Option<crate::RecoveredReplicaPromotionPublication> {
        self.publication
    }

    pub const fn readmission(&self) -> Option<crate::RecoveredReplicaPromotionReadmission> {
        self.readmission
    }

    pub const fn rejoin_plan_fingerprint(&self) -> Option<[u8; 32]> {
        self.rejoin_plan_fingerprint
    }
}
