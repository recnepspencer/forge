use worth_store_formal_models::{
    map_compaction_observation, map_lsm_maintenance_observation, map_lsm_membership_observation,
    CompactionVisibilityMappedOwnerCase, CompactionVisibilityOwnerCase,
};
use worth_store_physical_isolation::{
    CompactionCutoverDelta, CompactionOwnerCaseObservation, CompactionReadInterlockPlan,
    CompactionRewritePublication,
};
use worth_store_test_support::harness::{
    observe_lsm_owner_cases,
    physical_isolation::{
        compaction::{admitted_compaction_plan, admitted_compaction_plan_for_seed},
        publication,
    },
};

#[derive(Debug, Clone, Copy)]
enum ExecutedOwnerObservation {
    LsmMembership(worth_store_lsm_authority::LsmMembershipOwnerCaseObservation),
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
        .collect::<Vec<_>>();
    observations.extend(
        crate::courtroom::layout::owner_scenarios::maintenance::lsm::execute_observations()
            .into_iter()
            .map(ExecutedOwnerObservation::LsmMaintenance),
    );
    observations.extend(
        lower_compaction_observations(admitted_compaction_plan())
            .into_iter()
            .map(ExecutedOwnerObservation::PhysicalCompaction),
    );
    OrdinaryCompactionVisibilityExecutionReceipt { observations }
}

pub(in crate::courtroom::protocol_models) fn replay_compaction_publication_guard(
    seed: u64,
) -> Vec<worth_store_formal_models::CompactionVisibilityAction> {
    let seed = seed.max(1);
    let plan = admitted_compaction_plan_for_seed(seed);
    let publication_inputs = publication::publication_inputs_with_root_generation(seed);
    let delta = CompactionCutoverDelta::lower_to_manifest(
        plan,
        publication_inputs.new_root.manifest_epoch().get(),
    )
    .expect("admitted compaction plan lowers to the publication manifest");
    let lowered = map_compaction_observation(delta.owner_case_observation()).action();
    let publication_receipt =
        publication::admitted_copy_on_write_plan(&publication_inputs).complete();
    let published = CompactionRewritePublication::publish_rewrite(delta, publication_receipt)
        .expect("lowered compaction rewrite binds to the executed publication");
    let published = map_compaction_observation(published.owner_case_observation()).action();
    vec![lowered, published]
}

pub(in crate::courtroom::protocol_models) fn execute_compaction_visibility_legal_traces(
) -> Vec<Vec<worth_store_formal_models::CompactionVisibilityAction>> {
    lower_compaction_observations(admitted_compaction_plan())
        .into_iter()
        .map(map_compaction_observation)
        .map(|mapped| vec![mapped.action()])
        .collect()
}

fn lower_compaction_observations(
    plan: CompactionReadInterlockPlan,
) -> Vec<CompactionOwnerCaseObservation> {
    let manifest_epoch = plan.protected().root().manifest_epoch().get() + 1;
    let delta = CompactionCutoverDelta::lower_to_manifest(plan, manifest_epoch)
        .expect("admitted compaction plan lowers a rewrite candidate");
    vec![delta.owner_case_observation()]
}

impl ExecutedOwnerObservation {
    const fn owner_case(self) -> CompactionVisibilityOwnerCase {
        match self {
            Self::LsmMembership(observation) => {
                CompactionVisibilityOwnerCase::LsmMembership(observation.id())
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
