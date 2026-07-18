#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiFrameworkTransitionPlanningCounters {
    admitted_ingress_width: u16,
    invalidation_breadth: u16,
    selected_neighborhood_breadth: u16,
    policy_family_count: u8,
    policy_classification_count: u8,
}

impl UiFrameworkTransitionPlanningCounters {
    pub(super) fn from_planned_frame(
        plan: &crate::runtime::UiNarrowedAllocationFramePlan,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    ) -> Self {
        Self {
            admitted_ingress_width: plan.resolution_counters().entry_visits(),
            invalidation_breadth: plan.counters().emitted_targets(),
            selected_neighborhood_breadth: selection.counters().set_cardinality(),
            policy_family_count: plan.resolution_counters().policy_family_count(),
            policy_classification_count: 1,
        }
    }

    pub fn admitted_ingress_width(self) -> u16 {
        self.admitted_ingress_width
    }

    pub fn invalidation_breadth(self) -> u16 {
        self.invalidation_breadth
    }

    pub fn selected_neighborhood_breadth(self) -> u16 {
        self.selected_neighborhood_breadth
    }

    pub fn policy_family_count(self) -> u8 {
        self.policy_family_count
    }

    pub fn policy_classification_count(self) -> u8 {
        self.policy_classification_count
    }
}
