use super::PlannedCounterEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessPlanBudget {
    planned_counter_envelope: PlannedCounterEnvelope,
}

impl AccessPlanBudget {
    pub(crate) const fn from_planned_counter_envelope(
        planned_counter_envelope: PlannedCounterEnvelope,
    ) -> Self {
        Self {
            planned_counter_envelope,
        }
    }

    pub const fn planned_counter_envelope(self) -> PlannedCounterEnvelope {
        self.planned_counter_envelope
    }
}
