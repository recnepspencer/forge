use crate::{PhysicalSimulationPlan, PhysicalSimulationProfile};

use super::actor_sequence::PhysicalActorStepSequence;
use super::actor_step::PhysicalActorStep;
use super::authority::{AdmittedScheduleOrderingAuthority, ScheduleOrderingAuthorityAttempt};
use super::budget::{PartialOrderReductionPosture, ScheduleExplorationCost, StateSpaceBudget};
use super::identity::{ScheduleReplayIdentity, ScheduleReplayIdentityParts};
use super::{ReplaySeed, ScheduleReplayDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalInterleavingSchedule {
    seed: ReplaySeed,
    profile: PhysicalSimulationProfile,
    ordering_authority: AdmittedScheduleOrderingAuthority,
    actor_steps: PhysicalActorStepSequence,
    exploration_cost: ScheduleExplorationCost,
    identity: ScheduleReplayIdentity,
}

impl PhysicalInterleavingSchedule {
    pub fn from_lowered_plan(
        plan: &PhysicalSimulationPlan,
        seed: ReplaySeed,
        budget: StateSpaceBudget,
    ) -> Result<Self, ScheduleReplayDenial> {
        let ordering_authority =
            ScheduleOrderingAuthorityAttempt::deterministic_actor_steps().admit()?;
        Self::from_lowered_plan_with_ordering_authority(plan, seed, budget, ordering_authority)
    }

    pub fn from_lowered_plan_with_ordering_authority(
        plan: &PhysicalSimulationPlan,
        seed: ReplaySeed,
        budget: StateSpaceBudget,
        ordering_authority: AdmittedScheduleOrderingAuthority,
    ) -> Result<Self, ScheduleReplayDenial> {
        let actor_steps = PhysicalActorStepSequence::from_steps(actor_steps_from_plan(plan)?)?;
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
            budget.max_steps() - required_steps,
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
        seed: Option<ReplaySeed>,
        budget: StateSpaceBudget,
    ) -> Result<Self, ScheduleReplayDenial> {
        let seed = seed.ok_or(ScheduleReplayDenial::MissingSeed)?;
        Self::from_lowered_plan(plan, seed, budget)
    }

    pub fn require_replayable(self) -> Result<Self, ScheduleReplayDenial> {
        Ok(self)
    }

    pub const fn seed(&self) -> ReplaySeed {
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
) -> Result<Vec<PhysicalActorStep>, ScheduleReplayDenial> {
    let yieldpoint = plan.yieldpoint_binding().declared_yieldpoint().name();
    plan.actors()
        .iter()
        .enumerate()
        .map(|(index, actor)| PhysicalActorStep::from_actor(index as u32, actor, yieldpoint))
        .collect()
}
