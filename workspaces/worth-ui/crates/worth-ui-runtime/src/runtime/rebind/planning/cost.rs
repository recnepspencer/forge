#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindPlanCost {
    selected_decisions: usize,
    graph_and_mounted_entries: usize,
    measurement_and_allocation_entries: usize,
    binding_transitions: usize,
    effects: usize,
}

impl UiRebindPlanCost {
    pub(crate) const fn new(
        selected_decisions: usize,
        graph_and_mounted_entries: usize,
        measurement_and_allocation_entries: usize,
        binding_transitions: usize,
        effects: usize,
    ) -> Self {
        Self {
            selected_decisions,
            graph_and_mounted_entries,
            measurement_and_allocation_entries,
            binding_transitions,
            effects,
        }
    }

    pub const fn selected_decisions(self) -> usize {
        self.selected_decisions
    }

    pub const fn graph_and_mounted_entries(self) -> usize {
        self.graph_and_mounted_entries
    }

    pub const fn measurement_and_allocation_entries(self) -> usize {
        self.measurement_and_allocation_entries
    }

    pub const fn binding_transitions(self) -> usize {
        self.binding_transitions
    }

    pub const fn effects(self) -> usize {
        self.effects
    }
}
