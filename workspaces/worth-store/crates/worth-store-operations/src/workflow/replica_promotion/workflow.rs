use sha2::{Digest, Sha256};
use worth_store_authority::{
    PrimaryServeLease, PrimaryServingAuthority, PromotionFenceRequest, StoreCurrentAuthorityIdentity,
    StoreCurrentAuthorityWitness,
};
use worth_store_offline_verifier::IndependentlyVerifiedDisasterRecoveryBundle;
use worth_store_replication::{
    DivergentReplicaHistoryReport, ReplicaPromotionOwner, ReplicaPromotionReceipt,
    ReplicaRecoveryFrontier, ReplicationPeerId,
};

use crate::authorization::{
    authorize_lowered_plan, consume_authorization, AuthorizationReplayPolicy,
    AuthorizedOperationalPlan, LoweredOperationalPlan,
};
use crate::control_store::OperationalControlStorePort;
use crate::owner_plan_dag::{
    CanonicalOwnerPlanDag, DestructiveOperationKind, OperationalPlanBinding, OwnerPlanNode,
    OwnerPlanPrerequisite,
};
use crate::{
    AuthorizationConsumptionDenial, AuthorizationConsumptionReceipt, AuthorizationDenial,
    AuthorizationRevocationObservation, ExternalOperatorAssertion, OperationalAuthorizationPort,
    OperationalControlStore, OperationalOperationId, OperationalSecurityScope,
    OperationalTransitionId, OwnerPlanEffect, OwnerPlanFootprint, StoreOwnerKind,
};

#[derive(Debug)]
pub struct ReplicaPromotionIntent {
    operation_id: OperationalOperationId,
    candidate_peer: ReplicationPeerId,
    target_identity: [u8; 32],
    current_frontier: ReplicaRecoveryFrontier,
    authority_identity: StoreCurrentAuthorityIdentity,
    security_scope: OperationalSecurityScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaPromotionResolutionDenial {
    InvalidTargetIdentity,
    SecurityScopeMismatch,
    SourceLineageMismatch,
    SourceFrontierMismatch,
    Replication(worth_store_replication::ReplicaPromotionDenial),
}

#[derive(Debug)]
pub struct EvidenceBoundReplicaPromotionPlan {
    intent: ReplicaPromotionIntent,
    source: IndependentlyVerifiedDisasterRecoveryBundle,
    candidate: worth_store_replication::ReplicaPromotionCandidate,
    old_primary_lease: PrimaryServeLease,
}

impl ReplicaPromotionIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationalOperationId,
        candidate_peer: ReplicationPeerId,
        target_identity: [u8; 32],
        current_frontier: ReplicaRecoveryFrontier,
        authority_identity: StoreCurrentAuthorityIdentity,
        security_scope: OperationalSecurityScope,
    ) -> Result<Self, ReplicaPromotionResolutionDenial> {
        if target_identity == [0; 32] {
            return Err(ReplicaPromotionResolutionDenial::InvalidTargetIdentity);
        }
        Ok(Self {
            operation_id,
            candidate_peer,
            target_identity,
            current_frontier,
            authority_identity,
            security_scope,
        })
    }

    pub fn resolve(
        self,
        source: IndependentlyVerifiedDisasterRecoveryBundle,
        history: DivergentReplicaHistoryReport,
        old_primary_lease: PrimaryServeLease,
    ) -> Result<EvidenceBoundReplicaPromotionPlan, ReplicaPromotionResolutionDenial> {
        if self.security_scope.identity() != source.materialized().security_scope() {
            return Err(ReplicaPromotionResolutionDenial::SecurityScopeMismatch);
        }
        if history.observation().lineage() != source.materialized().source_lineage() {
            return Err(ReplicaPromotionResolutionDenial::SourceLineageMismatch);
        }
        if history.observation().frontier() != source.materialized().frontier() {
            return Err(ReplicaPromotionResolutionDenial::SourceFrontierMismatch);
        }
        let replication_intent =
            ReplicaPromotionOwner::intent(self.candidate_peer.clone(), self.current_frontier);
        let candidate = ReplicaPromotionOwner::resolve_candidate(replication_intent, history)
            .map_err(ReplicaPromotionResolutionDenial::Replication)?;
        Ok(EvidenceBoundReplicaPromotionPlan {
            intent: self,
            source,
            candidate,
            old_primary_lease,
        })
    }
}

#[derive(Debug)]
pub enum ReplicaPromotionLoweringDenial {
    EmptyFootprint,
    OwnerDag(crate::OwnerPlanDagDenial),
}

#[derive(Debug)]
pub struct LoweredReplicaPromotionOwnerPlanDag {
    operation_id: OperationalOperationId,
    authorization: LoweredOperationalPlan<ReplicaPromotionOperation>,
    replication: worth_store_replication::LoweredReplicaPromotionPlan,
    old_primary_lease: PrimaryServeLease,
    explanation: crate::CanonicalOwnerPlanDagExplanation,
}

#[derive(Debug)]
struct ReplicaPromotionOperation;

impl EvidenceBoundReplicaPromotionPlan {
    pub fn lower(self) -> Result<LoweredReplicaPromotionOwnerPlanDag, ReplicaPromotionLoweringDenial> {
        let source_identity = self.source.materialized().manifest_identity();
        let total_bytes = self
            .source
            .materialized()
            .components()
            .iter()
            .try_fold(0_u64, |total, component| total.checked_add(component.byte_length()))
            .ok_or(ReplicaPromotionLoweringDenial::EmptyFootprint)?;
        let footprint = OwnerPlanFootprint::bounded(0, total_bytes.max(1))
            .ok_or(ReplicaPromotionLoweringDenial::EmptyFootprint)?;
        let replication = ReplicaPromotionOwner::lower(self.candidate);
        let (dag, explanation) = promotion_owner_dag(replication.fingerprint(), footprint)?;
        let frontier = replication.candidate().frontier();
        let binding = OperationalPlanBinding::bind(
            DestructiveOperationKind::ReplicaPromotion,
            dag,
            self.intent.authority_identity,
            self.intent.security_scope,
            source_identity,
            self.intent.target_identity,
            frontier_identity(frontier),
        );
        Ok(LoweredReplicaPromotionOwnerPlanDag {
            operation_id: self.intent.operation_id,
            authorization: LoweredOperationalPlan::from_binding(binding),
            replication,
            old_primary_lease: self.old_primary_lease,
            explanation,
        })
    }
}

impl LoweredReplicaPromotionOwnerPlanDag {
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
    ) -> Result<AuthorizedReplicaPromotionPlan, AuthorizationDenial> {
        Ok(AuthorizedReplicaPromotionPlan {
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
            old_primary_lease: self.old_primary_lease,
        })
    }
}

#[derive(Debug)]
pub struct AuthorizedReplicaPromotionPlan {
    operation_id: OperationalOperationId,
    authorization: AuthorizedOperationalPlan<ReplicaPromotionOperation>,
    replication: worth_store_replication::LoweredReplicaPromotionPlan,
    old_primary_lease: PrimaryServeLease,
}

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

#[derive(Debug)]
pub enum ReplicaPromotionExecutionDenial {
    Fence(worth_store_authority::PromotionFenceDenial),
    Replication(worth_store_replication::ReplicaPromotionDenial),
    Control(crate::OperationalControlAppendDenial),
}

#[derive(Debug)]
pub struct ExecutedReplicaPromotion {
    operation_id: OperationalOperationId,
    authorization: AuthorizationConsumptionReceipt,
    receipt: ReplicaPromotionReceipt,
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

impl ExecutionReadyReplicaPromotion<'_> {
    pub fn execute(
        self,
        serving_authority: &PrimaryServingAuthority<'_>,
        receipt_transition: OperationalTransitionId,
    ) -> Result<ExecutedReplicaPromotion, ReplicaPromotionExecutionDenial> {
        let plan_fingerprint = self.replication.fingerprint();
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
            ))
            .map_err(ReplicaPromotionExecutionDenial::Fence)?;
        let receipt = ReplicaPromotionOwner::record_fenced_promotion(self.replication, fence)
            .map_err(ReplicaPromotionExecutionDenial::Replication)?;
        let control_record = crate::OperationalControlRecord::operational_owner_receipt_persisted(
            self.authority_identity,
            self.operation_id.clone(),
            receipt_transition,
            crate::OperationalWorkflowKind::ReplicaPromotion,
            plan_fingerprint,
            receipt.receipt_identity(),
            8,
        );
        self.control
            .append(&control_record)
            .map_err(ReplicaPromotionExecutionDenial::Control)?;
        Ok(ExecutedReplicaPromotion {
            operation_id: self.operation_id,
            authorization: self.authorization,
            receipt,
        })
    }
}

fn promotion_owner_dag(
    replication_plan: [u8; 32],
    footprint: OwnerPlanFootprint,
) -> Result<(CanonicalOwnerPlanDag, crate::CanonicalOwnerPlanDagExplanation), ReplicaPromotionLoweringDenial> {
    let fence = OwnerPlanNode::from_owner_binding(
        StoreOwnerKind::Authority,
        OwnerPlanEffect::FenceOldPrimary,
        footprint,
        1,
        true,
        replication_plan,
        expected_receipt(b"fence", replication_plan),
    );
    let promotion = OwnerPlanNode::from_owner_binding(
        StoreOwnerKind::Replication,
        OwnerPlanEffect::PromoteReplica,
        footprint,
        footprint.end_exclusive(),
        true,
        replication_plan,
        expected_receipt(b"promotion", replication_plan),
    );
    let edge = OwnerPlanPrerequisite::new(fence.identity(), promotion.identity(), true);
    let dag = CanonicalOwnerPlanDag::admit(vec![fence, promotion], vec![edge])
        .map_err(ReplicaPromotionLoweringDenial::OwnerDag)?;
    let explanation = dag.explanation().clone();
    Ok((dag, explanation))
}

fn frontier_identity(frontier: ReplicaRecoveryFrontier) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-replica-promotion-frontier-v1");
    digest.update(frontier.observed_lsn().to_be_bytes());
    digest.update(frontier.durable_lsn().to_be_bytes());
    digest.update(frontier.client_acknowledged_lsn().to_be_bytes());
    digest.update(frontier.replication_acknowledged_lsn().to_be_bytes());
    digest.update(frontier.authority_epoch().to_be_bytes());
    digest.finalize().into()
}

fn expected_receipt(domain: &[u8], plan: [u8; 32]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-replica-promotion-expected-receipt-v1");
    digest.update(domain);
    digest.update(plan);
    digest.finalize().into()
}
