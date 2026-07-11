use crate::production_transition::{
    S8LayoutMachineContract, S8LayoutMachineState as State,
    S8LayoutMachineTransition as Transition, S8LayoutProductionOperation as Operation,
    S8LayoutProductionTransition, S8LayoutStateMachine as Machine, S8OwnerTransitionContract,
};
use forge_store_physical_isolation::{
    compaction_cutover_outcome_facts, CompactionCutoverState as OwnerState,
    CompactionCutoverTransition as OwnerTransition,
    CompactionCutoverTransitionKind as OwnerTransitionKind,
};

const fn map_state(state: OwnerState) -> State {
    match state {
        OwnerState::PlanAdmitted => State::CompactionPlanAdmitted,
        OwnerState::RewriteLowered => State::CompactionRewriteLowered,
        OwnerState::LsmTombstoneRetentionAdmitted => State::CompactionTombstoneRetentionAdmitted,
        OwnerState::PublicationCommitted => State::CompactionPublicationCommitted,
        OwnerState::RecoveryVisibilityAdmitted => State::CompactionRecoveryVisibilityAdmitted,
        OwnerState::ReclaimDeferred => State::CompactionReclaimDeferred,
        OwnerState::Reclaimed => State::CompactionReclaimed,
        OwnerState::Denied => State::Denied,
    }
}

const fn map_kind(kind: OwnerTransitionKind) -> Transition {
    match kind {
        OwnerTransitionKind::LowerRewrite => Transition::Lower,
        OwnerTransitionKind::PublishRewrite => Transition::Publish,
        OwnerTransitionKind::AdmitLsmTombstoneRetention => Transition::AdmitTombstoneRetention,
        OwnerTransitionKind::AdmitRecoveryVisibility => Transition::AdmitRecoveryVisibility,
        OwnerTransitionKind::DeferReclaim => Transition::DeferReclaim,
        OwnerTransitionKind::DrainReclaimAfterReadRelease => Transition::Reclaim,
        OwnerTransitionKind::DenyInPlaceOverwrite
        | OwnerTransitionKind::DenyLsmPhysicalTarget
        | OwnerTransitionKind::DenyEarlyReclaim
        | OwnerTransitionKind::DenyStaleEpochReuse
        | OwnerTransitionKind::DenyBackendResidue
        | OwnerTransitionKind::DenyLatchHierarchyInversion
        | OwnerTransitionKind::DenyMixedRootRead => Transition::Deny,
    }
}

macro_rules! compaction_family_contract {
    ($function:ident, $operation:ident) => {
        pub(crate) fn $function() -> S8OwnerTransitionContract {
            static FACTS: std::sync::OnceLock<Box<[S8LayoutProductionTransition]>> =
                std::sync::OnceLock::new();
            let facts = FACTS.get_or_init(|| {
                compaction_cutover_outcome_facts()
                    .map(project_owner_transition)
                    .filter(|fact| fact.production_operation() == Operation::$operation)
                    .collect()
            });
            S8OwnerTransitionContract::from_owner_outcomes(
                Machine::CompactionCutover,
                Operation::$operation,
                facts,
            )
        }
    };
}

compaction_family_contract!(lower_rewrite_contract, LowerCompactionRewrite);
compaction_family_contract!(
    tombstone_retention_contract,
    AdmitCompactionTombstoneRetention
);
compaction_family_contract!(publication_contract, PublishCompactionRewrite);
compaction_family_contract!(
    recovery_visibility_contract,
    AdmitCompactionRecoveryVisibility
);
compaction_family_contract!(reclaim_deferral_contract, DeferCompactionReclaim);
compaction_family_contract!(reclaim_drain_contract, DrainCompactionReclaim);
compaction_family_contract!(mutation_denial_contract, DenyCompactionMutation);

fn project_owner_transition(owner: OwnerTransition) -> S8LayoutProductionTransition {
    S8LayoutProductionTransition::from_compaction_owner(owner)
}

pub(crate) fn owner_contract_is_preserved(contract: &S8LayoutMachineContract) -> bool {
    compaction_cutover_outcome_facts().all(|owner| {
        contract.permits_edge(
            map_state(owner.from()),
            map_kind(owner.kind()),
            map_state(owner.to()),
        )
    })
}
