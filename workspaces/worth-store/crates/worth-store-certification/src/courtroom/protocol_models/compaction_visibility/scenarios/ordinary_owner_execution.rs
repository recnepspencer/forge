use worth_store_formal_models::{
    map_compaction_observation, map_lsm_execution_observation, map_lsm_maintenance_observation,
    map_lsm_membership_observation, CompactionVisibilityMappedOwnerCase,
    CompactionVisibilityOwnerCase,
};
use worth_store_physical_isolation::{
    CompactionCutoverStabilityProof, CompactionDeferredReclaimQueue, CompactionOwnerCaseObservation,
};
use worth_store_test_support::harness::{
    observe_lsm_owner_cases,
    physical_isolation::compaction::{
        admitted_compaction_plan, admitted_compaction_plan_for_seed, execute_compaction_cutover,
    },
    recovery::compaction_mutation::complete_compaction_mutation_receipts,
};

#[derive(Debug, Clone, Copy)]
enum ExecutedOwnerObservation {
    LsmMembership(worth_store_lsm_authority::LsmMembershipOwnerCaseObservation),
    LsmExecution(worth_store_layout_indexes::LsmExecutionOwnerCaseObservation),
    LsmMaintenance(worth_store_layout_indexes::LsmMaintenanceOwnerCaseObservation),
    PhysicalCompaction(CompactionOwnerCaseObservation),
}

pub(in crate::courtroom::protocol_models) struct OrdinaryCompactionVisibilityExecutionReceipt {
    observations: Vec<ExecutedOwnerObservation>,
}

pub(in crate::courtroom::protocol_models) fn execute_compaction_visibility_owner_cases(
) -> OrdinaryCompactionVisibilityExecutionReceipt {
    let lsm = observe_lsm_owner_cases();
    let mut observations = lsm
        .membership()
        .map(ExecutedOwnerObservation::LsmMembership)
        .chain(lsm.execution().map(ExecutedOwnerObservation::LsmExecution))
        .collect::<Vec<_>>();
    observations.extend(
        crate::courtroom::layout::owner_scenarios::maintenance::lsm::execute_observations()
            .into_iter()
            .map(ExecutedOwnerObservation::LsmMaintenance),
    );
    observations.extend(
        execute_compaction_observations()
            .into_iter()
            .map(ExecutedOwnerObservation::PhysicalCompaction),
    );
    OrdinaryCompactionVisibilityExecutionReceipt { observations }
}

pub(in crate::courtroom::protocol_models) fn replay_compaction_publication_guard(
    seed: u64,
) -> Vec<worth_store_formal_models::CompactionVisibilityAction> {
    let actions = execute_compaction_observations_for_plan(admitted_compaction_plan_for_seed(seed))
        .into_iter()
        .map(map_compaction_observation)
        .map(|mapped| mapped.action())
        .collect::<Vec<_>>();
    let lower = actions
        .iter()
        .position(|action| {
            *action == worth_store_formal_models::CompactionVisibilityAction::LowerRewrite
        })
        .expect("executed compaction carries its lowering observation");
    let publish = actions
        .iter()
        .position(|action| {
            *action == worth_store_formal_models::CompactionVisibilityAction::PublishRewrite
        })
        .expect("executed compaction carries its publication observation");
    assert!(lower < publish);
    actions
}

pub(in crate::courtroom::protocol_models) fn execute_compaction_visibility_legal_traces(
) -> Vec<Vec<worth_store_formal_models::CompactionVisibilityAction>> {
    let actions = execute_compaction_observations()
        .into_iter()
        .map(map_compaction_observation)
        .map(|mapped| mapped.action())
        .collect::<Vec<_>>();
    let lifecycle = actions[..5].to_vec();
    std::iter::once(lifecycle)
        .chain(actions[5..].iter().copied().map(|action| vec![action]))
        .collect()
}

fn execute_compaction_observations() -> Vec<CompactionOwnerCaseObservation> {
    execute_compaction_observations_for_plan(admitted_compaction_plan())
}

fn execute_compaction_observations_for_plan(
    plan: worth_store_physical_isolation::CompactionReadInterlockPlan,
) -> Vec<CompactionOwnerCaseObservation> {
    let (publication, recovery, pre_cutover_read, _) =
        execute_compaction_cutover(&plan).into_parts();
    let stability = CompactionCutoverStabilityProof::admit(publication.clone(), recovery)
        .expect("ordinary recovery evidence admits compaction stability");
    let reclaim = CompactionDeferredReclaimQueue::admit(publication.clone())
        .expect("ordinary publication admits deferred reclaim");
    let drained = reclaim
        .clone()
        .drain_after_release(pre_cutover_read.read_plan_release())
        .expect("ordinary reader release drains deferred reclaim");
    let mut observations = vec![
        publication.delta().owner_case_observation(),
        publication.owner_case_observation(),
        stability.owner_case_observation(),
        reclaim.owner_case_observation(),
        drained.owner_case_observation(),
    ];
    observations.extend(
        complete_compaction_mutation_receipts()
            .into_iter()
            .map(|receipt| receipt.owner_case_observation()),
    );
    observations
}

impl ExecutedOwnerObservation {
    const fn owner_case(self) -> CompactionVisibilityOwnerCase {
        match self {
            Self::LsmMembership(observation) => {
                CompactionVisibilityOwnerCase::LsmMembership(observation.id())
            }
            Self::LsmExecution(observation) => {
                CompactionVisibilityOwnerCase::LsmExecution(observation.id())
            }
            Self::LsmMaintenance(observation) => {
                CompactionVisibilityOwnerCase::LsmMaintenance(observation.id())
            }
            Self::PhysicalCompaction(observation) => {
                CompactionVisibilityOwnerCase::PhysicalCompaction(observation.id())
            }
        }
    }

    fn map(self) -> CompactionVisibilityMappedOwnerCase {
        match self {
            Self::LsmMembership(observation) => map_lsm_membership_observation(observation),
            Self::LsmExecution(observation) => map_lsm_execution_observation(observation),
            Self::LsmMaintenance(observation) => map_lsm_maintenance_observation(observation),
            Self::PhysicalCompaction(observation) => map_compaction_observation(observation),
        }
    }
}

impl OrdinaryCompactionVisibilityExecutionReceipt {
    pub(in crate::courtroom::protocol_models) fn owner_cases(
        &self,
    ) -> impl Iterator<Item = CompactionVisibilityOwnerCase> + '_ {
        self.observations
            .iter()
            .copied()
            .map(ExecutedOwnerObservation::owner_case)
    }

    pub(in crate::courtroom::protocol_models) fn mapped_cases(
        &self,
    ) -> impl Iterator<Item = CompactionVisibilityMappedOwnerCase> + '_ {
        self.observations
            .iter()
            .copied()
            .map(ExecutedOwnerObservation::map)
    }

    pub(in crate::courtroom::protocol_models) fn retained_owner_observation_count(&self) -> usize {
        self.observations.len()
    }
}
