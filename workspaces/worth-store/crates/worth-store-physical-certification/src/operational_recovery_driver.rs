use std::path::Path;

pub use crate::operational_recovery_yieldpoint::OperationalRecoveryYieldpoint;
use worth_store_authority::PrimaryServingAuthority;
use worth_store_offline_verifier::{
    ForensicAcquisitionCounters, ForensicAcquisitionDenial, ForensicAcquisitionProgress,
    ForensicAcquisitionSession, ForensicBundle, ReplicaTargetVerificationBudget,
};
use worth_store_operations::{
    CompletedReplicaBootstrap, CurrentReplicaPromotion, DurablyFencedReplicaPromotion,
    ExecutedReplicaBootstrap, ExecutedReplicaPromotion, ExecutionReadyReplicaBootstrap,
    ExecutionReadyReplicaPromotion, FencedReplicaPromotion, OperationalControlStore,
    OperationalOperationId, OperationalTransitionId, PostVerifiedReplicaBootstrap,
    PostVerifiedReplicaPromotion, PublishedReplicaPromotion, ReplicaBootstrapExecutionDenial,
    ReplicaBootstrapFinalizationDenial, ReplicaBootstrapPersistenceDenial,
    ReplicaPromotionExecutionDenial, ReplicaPromotionFencePersistenceDenial,
    ReplicaPromotionFencingDenial, ReplicaPromotionFinalizationDenial,
    ReplicaPromotionPublicationPort, TransferredReplicaBootstrap,
};
use worth_store_replication::ReplicaBootstrapExecutionPort;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryDriverTrace {
    reached: Vec<OperationalRecoveryYieldpoint>,
    operation_identities: Vec<String>,
}

impl OperationalRecoveryDriverTrace {
    pub fn reached(&self) -> &[OperationalRecoveryYieldpoint] {
        &self.reached
    }

    pub fn operation_identities(&self) -> &[String] {
        &self.operation_identities
    }
}

#[derive(Debug)]
pub enum DrivenOperationalTransition<T> {
    InterruptedBefore,
    Completed(T),
    InterruptedAfter(T),
}

#[derive(Debug)]
pub struct OperationalRecoveryProductionDriver {
    pause_at: Option<OperationalRecoveryYieldpoint>,
    reached: Vec<OperationalRecoveryYieldpoint>,
    operation_identities: Vec<String>,
}

impl OperationalRecoveryProductionDriver {
    pub const fn uninterrupted() -> Self {
        Self {
            pause_at: None,
            reached: Vec::new(),
            operation_identities: Vec::new(),
        }
    }

    pub const fn pause_once_at(yieldpoint: OperationalRecoveryYieldpoint) -> Self {
        Self {
            pause_at: Some(yieldpoint),
            reached: Vec::new(),
            operation_identities: Vec::new(),
        }
    }

    pub fn trace(&self) -> OperationalRecoveryDriverTrace {
        OperationalRecoveryDriverTrace {
            reached: self.reached.clone(),
            operation_identities: self.operation_identities.clone(),
        }
    }

    pub fn forensic_acquire_next(
        &mut self,
        operation: &OperationalOperationId,
        session: &mut ForensicAcquisitionSession,
    ) -> Result<DrivenOperationalTransition<ForensicAcquisitionProgress>, ForensicAcquisitionDenial>
    {
        self.observe_operation(operation);
        if self.before(OperationalRecoveryYieldpoint::BeforeForensicSourceAcquisition) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let progress = session.acquire_next()?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterForensicSourceRecord,
            progress,
        ))
    }

    pub fn forensic_finish(
        &mut self,
        session: ForensicAcquisitionSession,
        completed_at_tick: u64,
    ) -> Result<
        DrivenOperationalTransition<(ForensicBundle, ForensicAcquisitionCounters)>,
        ForensicAcquisitionDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeForensicFinalization) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let finished = session.finish(completed_at_tick)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterForensicFinalization,
            finished,
        ))
    }

    pub fn bootstrap_transfer<'control>(
        &mut self,
        ready: ExecutionReadyReplicaBootstrap<'control>,
        port: &mut impl ReplicaBootstrapExecutionPort,
    ) -> Result<
        DrivenOperationalTransition<TransferredReplicaBootstrap<'control>>,
        ReplicaBootstrapExecutionDenial,
    > {
        self.observe_operation(ready.operation_id());
        if self.before(OperationalRecoveryYieldpoint::BeforeBootstrapTransfer) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let transferred = ready.transfer(port)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterBootstrapTransfer,
            transferred,
        ))
    }

    pub fn persist_bootstrap_transfer(
        &mut self,
        transferred: &TransferredReplicaBootstrap<'_>,
        transition: OperationalTransitionId,
    ) -> Result<
        DrivenOperationalTransition<ExecutedReplicaBootstrap>,
        ReplicaBootstrapPersistenceDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeBootstrapControlRecord) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let executed = transferred.persist(transition)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterBootstrapControlRecord,
            executed,
        ))
    }

    pub fn post_verify_bootstrap(
        &mut self,
        executed: ExecutedReplicaBootstrap,
        target_root: &Path,
        budget: ReplicaTargetVerificationBudget,
    ) -> Result<
        DrivenOperationalTransition<PostVerifiedReplicaBootstrap>,
        ReplicaBootstrapFinalizationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeBootstrapPostVerification) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let verified = executed.post_verify(target_root, budget)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterBootstrapPostVerification,
            verified,
        ))
    }

    pub fn complete_bootstrap(
        &mut self,
        verified: PostVerifiedReplicaBootstrap,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
    ) -> Result<
        DrivenOperationalTransition<CompletedReplicaBootstrap>,
        ReplicaBootstrapFinalizationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeBootstrapCompletion) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let completed = verified.complete(control, transition)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterBootstrapCompletion,
            completed,
        ))
    }

    pub fn promotion_fence<'control>(
        &mut self,
        ready: ExecutionReadyReplicaPromotion<'control>,
        authority: &PrimaryServingAuthority<'_>,
    ) -> Result<
        DrivenOperationalTransition<FencedReplicaPromotion<'control>>,
        ReplicaPromotionFencingDenial,
    > {
        self.observe_operation(ready.operation_id());
        if self.before(OperationalRecoveryYieldpoint::BeforePromotionExternalFence) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let fenced = ready.fence(authority)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterPromotionExternalFence,
            fenced,
        ))
    }

    pub fn persist_promotion_fence<'control>(
        &mut self,
        fenced: &FencedReplicaPromotion<'control>,
        transition: OperationalTransitionId,
    ) -> Result<
        DrivenOperationalTransition<DurablyFencedReplicaPromotion<'control>>,
        ReplicaPromotionFencePersistenceDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforePromotionFenceRecord) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let durable = fenced.persist_fence(transition)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterPromotionFenceRecord,
            durable,
        ))
    }

    pub fn record_promotion(
        &mut self,
        durable: &DurablyFencedReplicaPromotion<'_>,
        transition: OperationalTransitionId,
    ) -> Result<
        DrivenOperationalTransition<ExecutedReplicaPromotion>,
        ReplicaPromotionExecutionDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforePromotionRecord) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let executed = durable.promote(transition)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterPromotionRecord,
            executed,
        ))
    }

    pub fn post_verify_promotion(
        &mut self,
        executed: ExecutedReplicaPromotion,
        target_root: &Path,
        budget: ReplicaTargetVerificationBudget,
    ) -> Result<
        DrivenOperationalTransition<PostVerifiedReplicaPromotion>,
        ReplicaPromotionFinalizationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforePromotionPostVerification) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let verified = executed.post_verify(target_root, budget)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterPromotionPostVerification,
            verified,
        ))
    }

    pub fn publish_promotion(
        &mut self,
        verified: PostVerifiedReplicaPromotion,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        port: &mut impl ReplicaPromotionPublicationPort,
    ) -> Result<
        DrivenOperationalTransition<PublishedReplicaPromotion>,
        ReplicaPromotionFinalizationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforePromotionPublication) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let published = verified.publish(control, transition, port)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterPromotionPublication,
            published,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn readmit_promotion(
        &mut self,
        published: PublishedReplicaPromotion,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        serving: &PrimaryServingAuthority<'_>,
        now_tick: u64,
        requested_until_tick: u64,
    ) -> Result<
        DrivenOperationalTransition<CurrentReplicaPromotion>,
        ReplicaPromotionFinalizationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforePromotionReadmission) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let current =
            published.readmit(control, transition, serving, now_tick, requested_until_tick)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterPromotionReadmission,
            current,
        ))
    }

    pub(super) fn before(&mut self, point: OperationalRecoveryYieldpoint) -> bool {
        self.reached.push(point);
        self.take_pause(point)
    }

    pub(super) fn after<T>(
        &mut self,
        point: OperationalRecoveryYieldpoint,
        value: T,
    ) -> DrivenOperationalTransition<T> {
        self.reached.push(point);
        if self.take_pause(point) {
            DrivenOperationalTransition::InterruptedAfter(value)
        } else {
            DrivenOperationalTransition::Completed(value)
        }
    }

    fn take_pause(&mut self, point: OperationalRecoveryYieldpoint) -> bool {
        if self.pause_at == Some(point) {
            self.pause_at = None;
            true
        } else {
            false
        }
    }

    fn observe_operation(&mut self, operation: &OperationalOperationId) {
        let identity = operation.as_str();
        if !self
            .operation_identities
            .iter()
            .any(|observed| observed == identity)
        {
            self.operation_identities.push(identity.to_owned());
        }
    }
}
