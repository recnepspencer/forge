use super::ScheduleReplayDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PartialOrderReductionPosture {
    NotApplied,
    AppliedDeterministically,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateSpaceBudget {
    max_steps: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScheduleExplorationCost {
    budget: StateSpaceBudget,
    explored_steps: u32,
    pruned_steps: u32,
    partial_order_reduction: PartialOrderReductionPosture,
}

impl StateSpaceBudget {
    pub fn bounded_steps(max_steps: u32) -> Result<Self, ScheduleReplayDenial> {
        if max_steps == 0 {
            return Err(ScheduleReplayDenial::EmptyStateSpaceBudget);
        }
        Ok(Self { max_steps })
    }

    pub fn unbounded_exploration() -> Result<Self, ScheduleReplayDenial> {
        Err(ScheduleReplayDenial::UnboundedExplorationDenied)
    }

    pub const fn max_steps(self) -> u32 {
        self.max_steps
    }
}

impl ScheduleExplorationCost {
    pub(crate) const fn new(
        budget: StateSpaceBudget,
        explored_steps: u32,
        pruned_steps: u32,
        partial_order_reduction: PartialOrderReductionPosture,
    ) -> Self {
        Self {
            budget,
            explored_steps,
            pruned_steps,
            partial_order_reduction,
        }
    }

    pub const fn budget(self) -> StateSpaceBudget {
        self.budget
    }

    pub const fn explored_steps(self) -> u32 {
        self.explored_steps
    }

    pub const fn pruned_steps(self) -> u32 {
        self.pruned_steps
    }

    pub const fn partial_order_reduction(self) -> PartialOrderReductionPosture {
        self.partial_order_reduction
    }
}
