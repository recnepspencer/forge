use sha2::{Digest, Sha256};
use worth_store_authority::StoreCurrentAuthorityIdentity;
use worth_store_offline_verifier::IndependentlyVerifiedDisasterRecoveryBundle;
use worth_store_physical_isolation::BootstrapReachabilityLease;
use worth_store_replication::{ReplicaBootstrapOwner, ReplicationPeerId};

use crate::authorization::{
    authorize_lowered_plan, AuthorizationReplayPolicy, AuthorizedOperationalPlan,
    LoweredOperationalPlan,
};
use crate::owner_plan_dag::{
    CanonicalOwnerPlanDag, DestructiveOperationKind, OperationalPlanBinding, OwnerPlanNode,
    OwnerPlanPrerequisite,
};
use crate::{
    AuthorizationDenial, AuthorizationRevocationObservation, ExternalOperatorAssertion,
    OperationalAuthorizationPort, OperationalOperationId, OperationalSecurityScope,
    OwnerPlanEffect, OwnerPlanExecutionStage, OwnerPlanFootprint, StoreOwnerKind,
};

#[derive(Debug)]
pub struct ReplicaBootstrapIntent {
    operation_id: OperationalOperationId,
    target_peer: ReplicationPeerId,
    target_identity: [u8; 32],
    authority_identity: StoreCurrentAuthorityIdentity,
    security_scope: OperationalSecurityScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaBootstrapResolutionDenial {
    InvalidTargetIdentity,
    SourceLeaseMismatch,
    SecurityScopeMismatch,
}

#[derive(Debug)]
pub struct EvidenceBoundReplicaBootstrapPlan {
    intent: ReplicaBootstrapIntent,
    source: IndependentlyVerifiedDisasterRecoveryBundle,
    source_lease: BootstrapReachabilityLease,
}

impl ReplicaBootstrapIntent {
    pub fn new(
        operation_id: OperationalOperationId,
        target_peer: ReplicationPeerId,
        target_identity: [u8; 32],
        authority_identity: StoreCurrentAuthorityIdentity,
        security_scope: OperationalSecurityScope,
    ) -> Result<Self, ReplicaBootstrapResolutionDenial> {
        if target_identity == [0; 32] {
            return Err(ReplicaBootstrapResolutionDenial::InvalidTargetIdentity);
        }
        Ok(Self {
            operation_id,
            target_peer,
            target_identity,
            authority_identity,
            security_scope,
        })
    }

    pub fn resolve(
        self,
        source: IndependentlyVerifiedDisasterRecoveryBundle,
        source_lease: BootstrapReachabilityLease,
    ) -> Result<EvidenceBoundReplicaBootstrapPlan, ReplicaBootstrapResolutionDenial> {
        if source_lease.source_identity() != source.materialized().manifest_identity() {
            return Err(ReplicaBootstrapResolutionDenial::SourceLeaseMismatch);
        }
        if self.security_scope.fingerprint() != source.materialized().security().scope_fingerprint()
        {
            return Err(ReplicaBootstrapResolutionDenial::SecurityScopeMismatch);
        }
        Ok(EvidenceBoundReplicaBootstrapPlan {
            intent: self,
            source,
            source_lease,
        })
    }
}

#[derive(Debug)]
pub enum ReplicaBootstrapLoweringDenial {
    EmptyFootprint,
    Replication(worth_store_replication::ReplicaBootstrapDenial),
    OwnerDag(crate::OwnerPlanDagDenial),
}

#[derive(Debug)]
pub struct LoweredReplicaBootstrapOwnerPlanDag {
    pub(super) operation_id: OperationalOperationId,
    pub(super) authorization: LoweredOperationalPlan<ReplicaBootstrapOperation>,
    pub(super) replication: worth_store_replication::LoweredReplicaBootstrapPlan,
    explanation: crate::CanonicalOwnerPlanDagExplanation,
}

#[derive(Debug)]
pub(super) struct ReplicaBootstrapOperation;

impl EvidenceBoundReplicaBootstrapPlan {
    pub fn lower(
        self,
    ) -> Result<LoweredReplicaBootstrapOwnerPlanDag, ReplicaBootstrapLoweringDenial> {
        let replication_intent = ReplicaBootstrapOwner::intent(
            self.intent.target_peer,
            self.source.materialized().frontier(),
        )
        .map_err(ReplicaBootstrapLoweringDenial::Replication)?;
        let source_identity = self.source.materialized().manifest_identity();
        let lease_identity = self.source_lease.binding_fingerprint();
        let replication = ReplicaBootstrapOwner::lower(replication_intent, self.source_lease)
            .map_err(ReplicaBootstrapLoweringDenial::Replication)?;
        let total_bytes = self
            .source
            .materialized()
            .components()
            .iter()
            .try_fold(0_u64, |total, component| {
                total.checked_add(component.byte_length())
            })
            .ok_or(ReplicaBootstrapLoweringDenial::EmptyFootprint)?;
        let footprint = OwnerPlanFootprint::bounded(0, total_bytes.max(1))
            .ok_or(ReplicaBootstrapLoweringDenial::EmptyFootprint)?;
        let (dag, explanation) =
            bootstrap_owner_dag(lease_identity, replication.fingerprint(), footprint)?;
        let binding = OperationalPlanBinding::bind(
            DestructiveOperationKind::ReplicaBootstrap,
            dag,
            self.intent.authority_identity,
            self.intent.security_scope,
            source_identity,
            self.intent.target_identity,
            frontier_identity(self.source.materialized().frontier()),
        );
        Ok(LoweredReplicaBootstrapOwnerPlanDag {
            operation_id: self.intent.operation_id,
            authorization: LoweredOperationalPlan::from_binding(binding),
            replication,
            explanation,
        })
    }
}

impl LoweredReplicaBootstrapOwnerPlanDag {
    pub const fn explanation(&self) -> &crate::CanonicalOwnerPlanDagExplanation {
        &self.explanation
    }

    #[allow(clippy::too_many_arguments)]
    pub fn authorize(
        self,
        port: &impl OperationalAuthorizationPort,
        assertion: &ExternalOperatorAssertion,
        requested_at: u64,
        expires_at: u64,
        replay_policy: AuthorizationReplayPolicy,
        revocation: AuthorizationRevocationObservation,
    ) -> Result<AuthorizedReplicaBootstrapPlan, AuthorizationDenial> {
        Ok(AuthorizedReplicaBootstrapPlan {
            operation_id: self.operation_id,
            authorization: authorize_lowered_plan(
                self.authorization,
                port,
                assertion,
                requested_at,
                expires_at,
                replay_policy,
                revocation,
            )?,
            replication: self.replication,
        })
    }
}

#[derive(Debug)]
pub struct AuthorizedReplicaBootstrapPlan {
    pub(super) operation_id: OperationalOperationId,
    pub(super) authorization: AuthorizedOperationalPlan<ReplicaBootstrapOperation>,
    pub(super) replication: worth_store_replication::LoweredReplicaBootstrapPlan,
}

fn bootstrap_owner_dag(
    lease_identity: [u8; 32],
    replication_plan: [u8; 32],
    footprint: OwnerPlanFootprint,
) -> Result<
    (
        CanonicalOwnerPlanDag,
        crate::CanonicalOwnerPlanDagExplanation,
    ),
    ReplicaBootstrapLoweringDenial,
> {
    let lease = OwnerPlanNode::from_owner_observation_binding(
        StoreOwnerKind::PhysicalIsolation,
        OwnerPlanEffect::HoldBootstrapSourceLease,
        OwnerPlanExecutionStage::Staging,
        footprint,
        1,
        lease_identity,
        lease_identity,
    );
    let bootstrap = OwnerPlanNode::from_owner_binding(
        StoreOwnerKind::Replication,
        OwnerPlanEffect::BootstrapReplica,
        footprint,
        footprint.end_exclusive(),
        true,
        replication_plan,
        receipt_fingerprint(replication_plan),
    );
    let edge = OwnerPlanPrerequisite::new(lease.identity(), bootstrap.identity(), true);
    let dag = CanonicalOwnerPlanDag::admit(vec![lease, bootstrap], vec![edge])
        .map_err(ReplicaBootstrapLoweringDenial::OwnerDag)?;
    let explanation = dag.explanation().clone();
    Ok((dag, explanation))
}

fn frontier_identity(frontier: worth_store_replication::ReplicaRecoveryFrontier) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-replica-bootstrap-frontier-v1");
    digest.update(frontier.observed_lsn().to_be_bytes());
    digest.update(frontier.durable_lsn().to_be_bytes());
    digest.update(frontier.client_acknowledged_lsn().to_be_bytes());
    digest.update(frontier.replication_acknowledged_lsn().to_be_bytes());
    digest.update(frontier.authority_epoch().to_be_bytes());
    digest.finalize().into()
}

fn receipt_fingerprint(plan: [u8; 32]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-replica-bootstrap-expected-receipt-v1");
    digest.update(plan);
    digest.finalize().into()
}
