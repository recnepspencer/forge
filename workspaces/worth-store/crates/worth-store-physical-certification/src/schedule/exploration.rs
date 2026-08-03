use crate::{PhysicalScenarioActor, PhysicalSimulationPlan};

use super::{
    PhysicalInterleavingSchedule, SchedulePerturbationSeed, ScheduleReplayDenial, StateSpaceBudget,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleExplorationCompletion {
    Complete,
    BoundExhausted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScheduleExploration {
    schedules: Vec<PhysicalInterleavingSchedule>,
    explored_transitions: u32,
    total_schedules: u64,
    completion: ScheduleExplorationCompletion,
}

pub fn explore_physical_interleavings(
    plan: &PhysicalSimulationPlan,
    seed: SchedulePerturbationSeed,
    budget: StateSpaceBudget,
) -> Result<PhysicalScheduleExploration, ScheduleReplayDenial> {
    let mut actors = plan.actors().iter().cloned().collect::<Vec<_>>();
    actors.sort();
    let transitions_per_schedule = u32::try_from(actors.len()).map_err(|_| {
        ScheduleReplayDenial::StateSpaceBudgetExceeded {
            required_steps: u32::MAX,
            max_steps: budget.max_steps(),
        }
    })?;
    if transitions_per_schedule == 0 {
        return Err(ScheduleReplayDenial::EmptyActorStepSchedule);
    }
    if transitions_per_schedule > budget.max_steps() {
        return Err(ScheduleReplayDenial::StateSpaceBudgetExceeded {
            required_steps: transitions_per_schedule,
            max_steps: budget.max_steps(),
        });
    }

    let total_schedules = saturated_factorial(actors.len());
    let schedule_capacity = budget.max_steps() / transitions_per_schedule;
    let mut schedules = Vec::new();
    let mut ordinal = 0_u64;
    loop {
        if schedules.len() as u32 >= schedule_capacity {
            break;
        }
        schedules.push(PhysicalInterleavingSchedule::from_ordered_actors(
            plan,
            SchedulePerturbationSeed::from_u64(seed.value().wrapping_add(ordinal)),
            budget,
            &actors,
        )?);
        ordinal = ordinal.saturating_add(1);
        if !advance_lexicographic_permutation(&mut actors) {
            break;
        }
    }
    let explored_transitions = transitions_per_schedule.saturating_mul(schedules.len() as u32);
    let completion = if schedules.len() as u64 == total_schedules {
        ScheduleExplorationCompletion::Complete
    } else {
        ScheduleExplorationCompletion::BoundExhausted
    };
    Ok(PhysicalScheduleExploration {
        schedules,
        explored_transitions,
        total_schedules,
        completion,
    })
}

impl PhysicalScheduleExploration {
    pub fn schedules(&self) -> &[PhysicalInterleavingSchedule] {
        &self.schedules
    }

    pub const fn explored_transitions(&self) -> u32 {
        self.explored_transitions
    }

    pub const fn total_schedules(&self) -> u64 {
        self.total_schedules
    }

    pub const fn completion(&self) -> ScheduleExplorationCompletion {
        self.completion
    }
}

fn advance_lexicographic_permutation(actors: &mut [PhysicalScenarioActor]) -> bool {
    let Some(pivot) = (0..actors.len().saturating_sub(1))
        .rev()
        .find(|&index| actors[index] < actors[index + 1])
    else {
        return false;
    };
    let successor = (pivot + 1..actors.len())
        .rev()
        .find(|&index| actors[pivot] < actors[index])
        .expect("permutation pivot has a successor");
    actors.swap(pivot, successor);
    actors[pivot + 1..].reverse();
    true
}

fn saturated_factorial(value: usize) -> u64 {
    (2..=value).fold(1_u64, |product, factor| {
        product.saturating_mul(factor as u64)
    })
}
