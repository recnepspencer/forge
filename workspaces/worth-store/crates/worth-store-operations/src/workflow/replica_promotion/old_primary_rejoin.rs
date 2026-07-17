use worth_store_replication::{
    DivergentReplicaHistoryReport, OldPrimaryDivergenceDisposition, OldPrimaryRejoinExecutionPort,
    OldPrimaryRejoinPlan, OldPrimaryRejoinReceipt, ReplicationPeerId, ReplicationRejoinOwner,
};

use crate::{OperationalControlStore, OperationalControlStorePort, OperationalTransitionId};

use super::{CurrentReplicaPromotion, ReplicaPromotionFinalizationDenial};

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

impl CurrentReplicaPromotion {
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
        self.plan_old_primary_rejoin_through(
            control,
            transition,
            old_primary,
            divergence,
            disposition,
            disposition_authorization,
        )
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    #[allow(clippy::too_many_arguments)]
    pub fn plan_old_primary_rejoin_with_certification_control_store(
        &self,
        control: &dyn OperationalControlStorePort,
        transition: OperationalTransitionId,
        old_primary: ReplicationPeerId,
        divergence: DivergentReplicaHistoryReport,
        disposition: OldPrimaryDivergenceDisposition,
        disposition_authorization: Option<[u8; 32]>,
    ) -> Result<GovernedOldPrimaryRejoinPlan, ReplicaPromotionFinalizationDenial> {
        self.plan_old_primary_rejoin_through(
            control,
            transition,
            old_primary,
            divergence,
            disposition,
            disposition_authorization,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn plan_old_primary_rejoin_through(
        &self,
        control: &dyn OperationalControlStorePort,
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
        self.complete_through(control, transition)
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn complete_with_certification_control_store(
        self,
        control: &dyn OperationalControlStorePort,
        transition: OperationalTransitionId,
    ) -> Result<CompletedOldPrimaryRejoin, ReplicaPromotionFinalizationDenial> {
        self.complete_through(control, transition)
    }

    fn complete_through(
        self,
        control: &dyn OperationalControlStorePort,
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
