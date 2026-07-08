use super::S8PlannedCounterEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessPlanBudget {
    planned_counter_envelope: S8PlannedCounterEnvelope,
}

impl S8AccessPlanBudget {
    pub(crate) const fn from_planned_counter_envelope(
        planned_counter_envelope: S8PlannedCounterEnvelope,
    ) -> Self {
        Self {
            planned_counter_envelope,
        }
    }

    pub const fn planned_counter_envelope(self) -> S8PlannedCounterEnvelope {
        self.planned_counter_envelope
    }
}
