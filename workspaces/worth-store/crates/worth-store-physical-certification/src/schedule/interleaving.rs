use crate::{PhysicalSimulationPlan, PhysicalSimulationProfile};

use super::actor_sequence::PhysicalActorStepSequence;
use super::actor_step::PhysicalActorStep;
use super::authority::{AdmittedScheduleOrderingAuthority, ScheduleOrderingAuthorityAttempt};
use super::budget::{PartialOrderReductionPosture, ScheduleExplorationCost, StateSpaceBudget};
use super::identity::{ScheduleReplayIdentity, ScheduleReplayIdentityParts};
use super::{SchedulePerturbationSeed, ScheduleReplayDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalInterleavingSchedule {
    seed: SchedulePerturbationSeed,
    profile: PhysicalSimulationProfile,
    ordering_authority: AdmittedScheduleOrderingAuthority,
    actor_steps: PhysicalActorStepSequence,
    exploration_cost: ScheduleExplorationCost,
    identity: ScheduleReplayIdentity,
}

impl PhysicalInterleavingSchedule {
    pub fn from_lowered_plan(
        plan: &PhysicalSimulationPlan,
        seed: SchedulePerturbationSeed,
        budget: StateSpaceBudget,
    ) -> Result<Self, ScheduleReplayDenial> {
        let ordering_authority =
            ScheduleOrderingAuthorityAttempt::deterministic_actor_steps().admit()?;
        Self::from_lowered_plan_with_ordering_authority(plan, seed, budget, ordering_authority)
    }

    pub fn from_lowered_plan_with_ordering_authority(
        plan: &PhysicalSimulationPlan,
        seed: SchedulePerturbationSeed,
        budget: StateSpaceBudget,
        ordering_authority: AdmittedScheduleOrderingAuthority,
    ) -> Result<Self, ScheduleReplayDenial> {
        let actor_steps = actor_steps_from_plan(plan, seed)?;
        Self::from_ordered_actor_steps(plan, seed, budget, ordering_authority, actor_steps)
    }

    pub(crate) fn from_ordered_actors(
        plan: &PhysicalSimulationPlan,
        seed: SchedulePerturbationSeed,
        budget: StateSpaceBudget,
        actors: &[crate::PhysicalScenarioActor],
    ) -> Result<Self, ScheduleReplayDenial> {
        let ordering_authority =
            ScheduleOrderingAuthorityAttempt::deterministic_actor_steps().admit()?;
        let yieldpoint = plan.yieldpoint_binding().declared_yieldpoint().name();
        let actor_steps = actors
            .iter()
            .enumerate()
            .map(|(index, actor)| PhysicalActorStep::from_actor(index as u32, actor, yieldpoint))
            .collect::<Result<Vec<_>, _>>()?;
        Self::from_ordered_actor_steps(plan, seed, budget, ordering_authority, actor_steps)
    }

    fn from_ordered_actor_steps(
        plan: &PhysicalSimulationPlan,
        seed: SchedulePerturbationSeed,
        budget: StateSpaceBudget,
        ordering_authority: AdmittedScheduleOrderingAuthority,
        actor_steps: Vec<PhysicalActorStep>,
    ) -> Result<Self, ScheduleReplayDenial> {
        let actor_steps = PhysicalActorStepSequence::from_steps(actor_steps)?;
        let required_steps = actor_steps.len() as u32;
        if required_steps > budget.max_steps() {
            return Err(ScheduleReplayDenial::StateSpaceBudgetExceeded {
                required_steps,
                max_steps: budget.max_steps(),
            });
        }
        let exploration_cost = ScheduleExplorationCost::new(
            budget,
            required_steps,
            0,
            PartialOrderReductionPosture::NotApplied,
        );
        let identity = ScheduleReplayIdentity::from_parts(ScheduleReplayIdentityParts {
            plan,
            seed,
            ordering_authority,
            actor_steps: &actor_steps,
            exploration_cost,
        })?;
        Ok(Self {
            seed,
            profile: plan.profile(),
            ordering_authority,
            actor_steps,
            exploration_cost,
            identity,
        })
    }

    pub fn from_optional_seed(
        plan: &PhysicalSimulationPlan,
        seed: Option<SchedulePerturbationSeed>,
        budget: StateSpaceBudget,
    ) -> Result<Self, ScheduleReplayDenial> {
        let seed = seed.ok_or(ScheduleReplayDenial::MissingSeed)?;
        Self::from_lowered_plan(plan, seed, budget)
    }

    pub fn require_replayable(self) -> Result<Self, ScheduleReplayDenial> {
        Ok(self)
    }

    pub const fn seed(&self) -> SchedulePerturbationSeed {
        self.seed
    }

    pub const fn profile(&self) -> PhysicalSimulationProfile {
        self.profile
    }

    pub const fn ordering_authority(&self) -> AdmittedScheduleOrderingAuthority {
        self.ordering_authority
    }

    pub fn actor_steps(&self) -> &[PhysicalActorStep] {
        self.actor_steps.as_slice()
    }

    pub const fn actor_step_sequence(&self) -> &PhysicalActorStepSequence {
        &self.actor_steps
    }

    pub const fn exploration_cost(&self) -> ScheduleExplorationCost {
        self.exploration_cost
    }

    pub const fn identity(&self) -> &ScheduleReplayIdentity {
        &self.identity
    }

    pub fn replay_identity_matches_plan(&self, plan: &PhysicalSimulationPlan) -> bool {
        let Ok(expected) = ScheduleReplayIdentity::from_parts(ScheduleReplayIdentityParts {
            plan,
            seed: self.seed,
            ordering_authority: self.ordering_authority,
            actor_steps: &self.actor_steps,
            exploration_cost: self.exploration_cost,
        }) else {
            return false;
        };
        expected == self.identity
    }
}

fn actor_steps_from_plan(
    plan: &PhysicalSimulationPlan,
    seed: SchedulePerturbationSeed,
) -> Result<Vec<PhysicalActorStep>, ScheduleReplayDenial> {
    let yieldpoint = plan.yieldpoint_binding().declared_yieldpoint().name();
    let mut actors = plan.actors().iter().cloned().collect::<Vec<_>>();
    deterministically_permute_actors(&mut actors, seed);
    actors
        .iter()
        .enumerate()
        .map(|(index, actor)| PhysicalActorStep::from_actor(index as u32, actor, yieldpoint))
        .collect()
}

fn deterministically_permute_actors(
    actors: &mut [crate::PhysicalScenarioActor],
    seed: SchedulePerturbationSeed,
) {
    let mut state = seed.value();
    for upper in (1..actors.len()).rev() {
        state = next_replay_word(state);
        actors.swap(upper, (state as usize) % (upper + 1));
    }
}

const fn next_replay_word(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}
