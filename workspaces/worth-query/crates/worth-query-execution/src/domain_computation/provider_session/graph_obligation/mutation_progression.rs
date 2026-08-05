use std::sync::Arc;

use worth_query_admission::facade::graph_obligation::WorthQueryGraphWorkPlanIdentity;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryInstalledGraphObligationSetIdentity,
};
use worth_relational::facade::history::BranchId;
use worth_runtime_bridge::facade::TruthBranchIdentity;

use super::{
    WorthQueryGraphWorkManagedRunIdentity, WorthQueryGraphWorkSessionIdentity,
    WorthQueryManagedGraphWorkSession,
};

/// Private proof that the legacy managed-run engine is a worker of one exact
/// graph-work session rather than a second application-mutation authority.
pub(in crate::domain_computation) struct WorthQueryMutationRunBinding {
    session: WorthQueryGraphWorkSessionIdentity,
    managed_run: WorthQueryGraphWorkManagedRunIdentity,
    plan: WorthQueryGraphWorkPlanIdentity,
    binding: ApplicationSchemaBindingIdentity,
    obligation: WorthQueryInstalledGraphObligationSetIdentity,
    relational_branch: BranchId,
    truth_branch: TruthBranchIdentity,
    worker: Arc<str>,
    resource_plan: Arc<str>,
    reservation_count: usize,
}

/// Non-authoritative terminal evidence for one exact mutation graph-work run.
///
/// Construction is private to successful managed-run cleanup. Consumers may
/// inspect this value, but no execution transition accepts it as authority.
#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryMutationGraphWorkCompletion {
    session: WorthQueryGraphWorkSessionIdentity,
    managed_run: WorthQueryGraphWorkManagedRunIdentity,
    plan: WorthQueryGraphWorkPlanIdentity,
    binding: ApplicationSchemaBindingIdentity,
    obligation: WorthQueryInstalledGraphObligationSetIdentity,
    relational_branch: BranchId,
    truth_branch: TruthBranchIdentity,
    snapshot_released: bool,
    cleanup: crate::domain_computation::WorthQueryDirectRunCleanupReceipt,
}

impl Eq for WorthQueryMutationGraphWorkCompletion {}

impl WorthQueryManagedGraphWorkSession {
    pub(in crate::domain_computation) fn bind_mutation_run(
        &self,
        run: &crate::domain_computation::WorthQueryRunningDirectRun,
    ) -> Option<WorthQueryMutationRunBinding> {
        let affinity = run.graph_work_affinity()?;
        let identity = self.identity();
        let managed_run = self.managed_run_identity();
        let (resource_plan, reservation_count) = run.mutation_resource_release_expectation();
        (affinity.session == identity && affinity.managed_run == managed_run).then(|| {
            WorthQueryMutationRunBinding {
                session: identity,
                managed_run,
                plan: self.plan_identity(),
                binding: self.binding().clone(),
                obligation: self.obligation().clone(),
                relational_branch: self.branch().relational().clone(),
                truth_branch: self.branch().truth().clone(),
                worker: run.identity().into(),
                resource_plan: resource_plan.into(),
                reservation_count,
            }
        })
    }
}

impl WorthQueryMutationRunBinding {
    pub(in crate::domain_computation) fn admits(
        &self,
        plan: &crate::domain_computation::WorthQueryProviderExecutionPlanContract,
    ) -> bool {
        plan.managed_run_identity() == self.worker.as_ref()
            && plan.graph_work_session_identity() == Some(self.session.as_u64())
            && plan.graph_work_managed_run_identity() == Some(self.managed_run.as_u64())
    }

    pub(in crate::domain_computation) fn finish(
        self,
        running: crate::domain_computation::WorthQueryRunningDirectRun,
        terminal: crate::domain_computation::WorthQueryManagedRunTerminalKind,
        snapshot_released: bool,
    ) -> Result<WorthQueryMutationGraphWorkCompletion, ()> {
        let affinity = running.graph_work_affinity().ok_or(())?;
        if running.identity() != self.worker.as_ref()
            || affinity.session != self.session
            || affinity.managed_run != self.managed_run
        {
            return Err(());
        }
        let cleanup = running
            .terminate_for_convergence(terminal)
            .cleanup()
            .map_err(|_| ())?;
        let capacity = cleanup.attempt().capacity();
        if !(snapshot_released
            && cleanup.run_identity() == self.worker.as_ref()
            && cleanup.terminal() == terminal
            && cleanup.relational().released()
            && cleanup.bridge().reservation_released()
            && cleanup.provider_work().provider_retained_bytes() == 0
            && capacity.resource_plan_identity() == self.resource_plan.as_ref()
            && capacity.released_reservation_count() == self.reservation_count)
        {
            return Err(());
        }
        Ok(WorthQueryMutationGraphWorkCompletion {
            session: self.session,
            managed_run: self.managed_run,
            plan: self.plan,
            binding: self.binding,
            obligation: self.obligation,
            relational_branch: self.relational_branch,
            truth_branch: self.truth_branch,
            snapshot_released,
            cleanup,
        })
    }
}

impl WorthQueryMutationGraphWorkCompletion {
    pub const fn session_identity(&self) -> WorthQueryGraphWorkSessionIdentity {
        self.session
    }

    pub const fn managed_run_identity(&self) -> WorthQueryGraphWorkManagedRunIdentity {
        self.managed_run
    }

    pub const fn plan_identity(&self) -> WorthQueryGraphWorkPlanIdentity {
        self.plan
    }

    pub const fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding
    }

    pub const fn obligation_identity(&self) -> &WorthQueryInstalledGraphObligationSetIdentity {
        &self.obligation
    }

    pub const fn relational_branch(&self) -> &BranchId {
        &self.relational_branch
    }

    pub const fn truth_branch(&self) -> &TruthBranchIdentity {
        &self.truth_branch
    }

    pub const fn snapshot_released(&self) -> bool {
        self.snapshot_released
    }

    pub const fn cleanup(&self) -> &crate::domain_computation::WorthQueryDirectRunCleanupReceipt {
        &self.cleanup
    }
}
