use worth_store_authority::{
    PrimaryServeLease, StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness,
};

use crate::authorization::{consume_authorization_through, recover_authorization_consumption};
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

pub struct ExecutionReadyReplicaPromotion<'control> {
    pub(super) operation_id: OperationalOperationId,
    pub(super) authorization: AuthorizationConsumptionReceipt,
    pub(super) authority_identity: StoreCurrentAuthorityIdentity,
    pub(super) replication: worth_store_replication::LoweredReplicaPromotionPlan,
    pub(super) old_primary_lease: PrimaryServeLease,
    pub(super) control: &'control dyn OperationalControlStorePort,
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
        self.ready_with_control_port(
            control,
            control,
            transition_id,
            current_authority,
            observed_at,
            revocation,
        )
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn ready_with_certification_control_store<'control>(
        self,
        control: &'control OperationalControlStore,
        append: &'control dyn OperationalControlStorePort,
        transition_id: OperationalTransitionId,
        current_authority: &StoreCurrentAuthorityWitness,
        observed_at: u64,
        revocation: AuthorizationRevocationObservation,
    ) -> Result<ExecutionReadyReplicaPromotion<'control>, ReplicaPromotionReadinessDenial> {
        self.ready_with_control_port(
            control,
            append,
            transition_id,
            current_authority,
            observed_at,
            revocation,
        )
    }

    fn ready_with_control_port<'control>(
        self,
        control: &'control OperationalControlStore,
        append: &'control dyn OperationalControlStorePort,
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
        let consumed = consume_authorization_through(
            control,
            append,
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
            authority_identity: current_authority.authority_identity(),
            replication: self.replication,
            old_primary_lease: self.old_primary_lease,
            control: append,
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
        self.recover_ready_with_control_port(handle, control, control, current_authority)
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn recover_ready_with_certification_control_store<'control>(
        self,
        handle: &ReplicaPromotionRecoveryHandle,
        control: &'control OperationalControlStore,
        append: &'control dyn OperationalControlStorePort,
        current_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<ExecutionReadyReplicaPromotion<'control>, ReplicaPromotionReadinessDenial> {
        self.recover_ready_with_control_port(handle, control, append, current_authority)
    }

    fn recover_ready_with_control_port<'control>(
        self,
        handle: &ReplicaPromotionRecoveryHandle,
        control: &'control OperationalControlStore,
        append: &'control dyn OperationalControlStorePort,
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
            control: append,
        })
    }
}
