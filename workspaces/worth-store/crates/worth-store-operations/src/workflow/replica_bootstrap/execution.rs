use worth_store_authority::{StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness};
use worth_store_physical_isolation::{
    BootstrapReachabilityLease, RecoverySourceLeaseReleaseReceipt,
};
use worth_store_replication::{
    ReplicaBootstrapExecutionPort, ReplicaBootstrapOwner, ReplicaBootstrapReceipt,
};

use crate::authorization::{consume_authorization, recover_authorization_consumption};
use crate::control_store::ReplicaBootstrapRecoveryHandle;
use crate::{
    AuthorizationConsumptionDenial, AuthorizationConsumptionReceipt,
    AuthorizationRevocationObservation, OperationalControlStore, OperationalControlStorePort,
    OperationalOperationId, OperationalTransitionId,
};

use super::{AuthorizedReplicaBootstrapPlan, LoweredReplicaBootstrapOwnerPlanDag};

#[derive(Debug)]
pub enum ReplicaBootstrapReadinessDenial {
    StaleAuthority,
    Authorization(AuthorizationConsumptionDenial),
    RecordedTransfer(worth_store_replication::ReplicaBootstrapDenial),
    TerminalSourceLease(worth_store_physical_isolation::RecoverySourceLeaseDenial),
}

#[derive(Debug)]
pub struct ExecutionReadyReplicaBootstrap<'control> {
    operation_id: OperationalOperationId,
    authorization: AuthorizationConsumptionReceipt,
    authority_identity: StoreCurrentAuthorityIdentity,
    replication: worth_store_replication::LoweredReplicaBootstrapPlan,
    control: &'control OperationalControlStore,
}

impl AuthorizedReplicaBootstrapPlan {
    pub fn ready<'control>(
        self,
        control: &'control OperationalControlStore,
        transition_id: OperationalTransitionId,
        current_authority: &StoreCurrentAuthorityWitness,
        observed_at: u64,
        revocation: AuthorizationRevocationObservation,
    ) -> Result<ExecutionReadyReplicaBootstrap<'control>, ReplicaBootstrapReadinessDenial> {
        if self.authorization.binding().authority_identity()
            != current_authority.authority_identity()
        {
            return Err(ReplicaBootstrapReadinessDenial::StaleAuthority);
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
        .map_err(ReplicaBootstrapReadinessDenial::Authorization)?;
        Ok(ExecutionReadyReplicaBootstrap {
            operation_id: self.operation_id,
            authorization: consumed.receipt(),
            authority_identity,
            replication: self.replication,
            control,
        })
    }
}

impl LoweredReplicaBootstrapOwnerPlanDag {
    pub fn recover<'control>(
        self,
        handle: &ReplicaBootstrapRecoveryHandle,
        control: &'control OperationalControlStore,
        current_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<ReplicaBootstrapResume<'control>, ReplicaBootstrapReadinessDenial> {
        let binding = self.authorization.binding();
        if self.operation_id != *handle.operation_id()
            || binding.authority_identity() != current_authority.authority_identity()
            || handle.authority_identity() != current_authority.authority_identity()
            || binding.fingerprint() != handle.authorization_plan_fingerprint()
            || self.replication.fingerprint() != handle.execution_plan_fingerprint()
        {
            return Err(ReplicaBootstrapReadinessDenial::StaleAuthority);
        }
        let authorization = recover_authorization_consumption(
            control,
            handle.operation_id(),
            handle.authorization_identity(),
            handle.authorization_plan_fingerprint(),
        )
        .map_err(ReplicaBootstrapReadinessDenial::Authorization)?;
        if let Some(transfer) = handle.transfer() {
            let (receipt, retained_source_lease) = ReplicaBootstrapOwner::recover_recorded(
                self.replication,
                transfer.receipt_identity(),
                transfer.durable_target_identity(),
                transfer.source_lease_identity(),
                transfer.execution_counters(),
            )
            .map_err(ReplicaBootstrapReadinessDenial::RecordedTransfer)?;
            if let Some(disposition) = handle.disposition() {
                let source_release = retained_source_lease
                    .release()
                    .map_err(ReplicaBootstrapReadinessDenial::TerminalSourceLease)?;
                return Ok(ReplicaBootstrapResume::Terminal(
                    RecoveredTerminalReplicaBootstrap {
                        operation_id: self.operation_id,
                        receipt_identity: receipt.receipt_identity(),
                        disposition,
                        source_release,
                    },
                ));
            }
            return Ok(ReplicaBootstrapResume::Recorded(
                RecoveredReplicaBootstrap {
                    operation_id: self.operation_id,
                    authorization,
                    authority_identity: handle.authority_identity(),
                    receipt,
                    retained_source_lease,
                },
            ));
        }
        Ok(ReplicaBootstrapResume::Ready(
            ExecutionReadyReplicaBootstrap {
                operation_id: self.operation_id,
                authorization,
                authority_identity: current_authority.authority_identity(),
                replication: self.replication,
                control,
            },
        ))
    }
}

#[derive(Debug)]
pub enum ReplicaBootstrapResume<'control> {
    Ready(ExecutionReadyReplicaBootstrap<'control>),
    Recorded(RecoveredReplicaBootstrap),
    Terminal(RecoveredTerminalReplicaBootstrap),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredTerminalReplicaBootstrap {
    operation_id: OperationalOperationId,
    receipt_identity: [u8; 32],
    disposition: crate::RecoveredReplicaBootstrapDisposition,
    source_release: RecoverySourceLeaseReleaseReceipt,
}

impl RecoveredTerminalReplicaBootstrap {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }
    pub const fn receipt_identity(&self) -> [u8; 32] {
        self.receipt_identity
    }
    pub const fn disposition(&self) -> crate::RecoveredReplicaBootstrapDisposition {
        self.disposition
    }
    pub const fn source_release(&self) -> RecoverySourceLeaseReleaseReceipt {
        self.source_release
    }
}

#[derive(Debug)]
pub struct RecoveredReplicaBootstrap {
    pub(super) operation_id: OperationalOperationId,
    pub(super) authorization: AuthorizationConsumptionReceipt,
    pub(super) authority_identity: StoreCurrentAuthorityIdentity,
    pub(super) receipt: ReplicaBootstrapReceipt,
    pub(super) retained_source_lease: BootstrapReachabilityLease,
}

#[derive(Debug)]
pub enum ReplicaBootstrapExecutionDenial {
    Replication(worth_store_replication::ReplicaBootstrapDenial),
}

#[derive(Debug)]
pub struct TransferredReplicaBootstrap<'control> {
    operation_id: OperationalOperationId,
    authorization: AuthorizationConsumptionReceipt,
    authority_identity: StoreCurrentAuthorityIdentity,
    receipt: ReplicaBootstrapReceipt,
    retained_source_lease: BootstrapReachabilityLease,
    control: &'control OperationalControlStore,
}

impl<'control> ExecutionReadyReplicaBootstrap<'control> {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub fn transfer(
        self,
        port: &mut impl ReplicaBootstrapExecutionPort,
    ) -> Result<TransferredReplicaBootstrap<'control>, ReplicaBootstrapExecutionDenial> {
        let (receipt, retained_source_lease) =
            ReplicaBootstrapOwner::execute(self.replication, port)
                .map_err(ReplicaBootstrapExecutionDenial::Replication)?;
        Ok(TransferredReplicaBootstrap {
            operation_id: self.operation_id,
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            receipt,
            retained_source_lease,
            control: self.control,
        })
    }
}

#[derive(Debug)]
pub enum ReplicaBootstrapPersistenceDenial {
    Control(crate::OperationalControlAppendDenial),
}

#[derive(Debug)]
pub struct ExecutedReplicaBootstrap {
    pub(super) operation_id: OperationalOperationId,
    pub(super) authorization: AuthorizationConsumptionReceipt,
    pub(super) authority_identity: StoreCurrentAuthorityIdentity,
    pub(super) receipt: ReplicaBootstrapReceipt,
    pub(super) retained_source_lease: BootstrapReachabilityLease,
}

impl TransferredReplicaBootstrap<'_> {
    pub fn persist(
        &self,
        receipt_transition: OperationalTransitionId,
    ) -> Result<ExecutedReplicaBootstrap, ReplicaBootstrapPersistenceDenial> {
        let record = crate::OperationalControlRecord::replica_bootstrap_transfer_recorded(
            self.authority_identity,
            self.operation_id.clone(),
            receipt_transition,
            self.authorization.plan_fingerprint(),
            &self.receipt,
        );
        self.control
            .append(&record)
            .map_err(ReplicaBootstrapPersistenceDenial::Control)?;
        Ok(ExecutedReplicaBootstrap {
            operation_id: self.operation_id.clone(),
            authorization: self.authorization,
            authority_identity: self.authority_identity,
            receipt: self.receipt.clone(),
            retained_source_lease: self.retained_source_lease.clone(),
        })
    }

    pub const fn receipt(&self) -> &ReplicaBootstrapReceipt {
        &self.receipt
    }

    pub const fn retained_source_lease(&self) -> &BootstrapReachabilityLease {
        &self.retained_source_lease
    }
}

impl ExecutedReplicaBootstrap {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub const fn authorization(&self) -> AuthorizationConsumptionReceipt {
        self.authorization
    }

    pub const fn receipt(&self) -> &ReplicaBootstrapReceipt {
        &self.receipt
    }

    pub const fn retained_source_lease(&self) -> &BootstrapReachabilityLease {
        &self.retained_source_lease
    }
}

impl RecoveredReplicaBootstrap {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub const fn authorization(&self) -> AuthorizationConsumptionReceipt {
        self.authorization
    }

    pub const fn receipt(&self) -> &ReplicaBootstrapReceipt {
        &self.receipt
    }

    pub const fn retained_source_lease(&self) -> &BootstrapReachabilityLease {
        &self.retained_source_lease
    }
}
