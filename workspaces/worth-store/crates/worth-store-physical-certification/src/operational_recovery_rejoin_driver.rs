use worth_store_operations::{
    CompletedOldPrimaryRejoin, CurrentReplicaPromotion, GovernedOldPrimaryRejoinPlan,
    OperationalControlStore, OperationalTransitionId, ReplicaPromotionFinalizationDenial,
    ResolvedOldPrimaryRejoin,
};
use worth_store_replication::{
    DivergentReplicaHistoryReport, OldPrimaryDivergenceDisposition, OldPrimaryRejoinExecutionPort,
    ReplicationPeerId,
};

use crate::{
    DrivenOperationalTransition, OperationalRecoveryProductionDriver, OperationalRecoveryYieldpoint,
};

impl OperationalRecoveryProductionDriver {
    #[allow(clippy::too_many_arguments)]
    pub fn plan_old_primary_rejoin(
        &mut self,
        current: &CurrentReplicaPromotion,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        old_primary: ReplicationPeerId,
        divergence: DivergentReplicaHistoryReport,
        disposition: OldPrimaryDivergenceDisposition,
        authorization: Option<[u8; 32]>,
    ) -> Result<
        DrivenOperationalTransition<GovernedOldPrimaryRejoinPlan>,
        ReplicaPromotionFinalizationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeOldPrimaryRejoinPlan) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let plan = current.plan_old_primary_rejoin(
            control,
            transition,
            old_primary,
            divergence,
            disposition,
            authorization,
        )?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterOldPrimaryRejoinPlan,
            plan,
        ))
    }

    pub fn execute_old_primary_rejoin(
        &mut self,
        plan: GovernedOldPrimaryRejoinPlan,
        port: &mut impl OldPrimaryRejoinExecutionPort,
    ) -> Result<
        DrivenOperationalTransition<ResolvedOldPrimaryRejoin>,
        ReplicaPromotionFinalizationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeOldPrimaryRejoinExecution) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let resolved = plan.execute(port)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterOldPrimaryRejoinExecution,
            resolved,
        ))
    }

    pub fn complete_old_primary_rejoin(
        &mut self,
        resolved: ResolvedOldPrimaryRejoin,
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
    ) -> Result<
        DrivenOperationalTransition<CompletedOldPrimaryRejoin>,
        ReplicaPromotionFinalizationDenial,
    > {
        if self.before(OperationalRecoveryYieldpoint::BeforeOldPrimaryRejoinCompletion) {
            return Ok(DrivenOperationalTransition::InterruptedBefore);
        }
        let completed = resolved.complete(control, transition)?;
        Ok(self.after(
            OperationalRecoveryYieldpoint::AfterOldPrimaryRejoinCompletion,
            completed,
        ))
    }
}
