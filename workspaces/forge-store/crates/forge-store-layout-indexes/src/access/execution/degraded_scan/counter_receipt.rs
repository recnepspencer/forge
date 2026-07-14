#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegradedScanCounterReceipt {
    plan_binding: crate::AccessPlanIdentity,
    planned: crate::AccessPathCounterSnapshot,
    observed: crate::AccessPathCounterSnapshot,
    observation: crate::PlannedCounterObservation,
}

impl DegradedScanCounterReceipt {
    pub(super) fn issue(
        plan_binding: &crate::AccessPlanIdentity,
        observed_rows: u16,
        allocation_events: u64,
    ) -> Result<Self, crate::CounterEnvelopeViolation> {
        let observed = crate::AccessPathCounterSnapshot::exact(
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            observed_rows,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        )
        .with_allocation_events(allocation_events)
        .with_selected_plan_authority_allocation();
        let planned = plan_binding.planned_counter_envelope().lookup();
        let observation = planned.validate_observation(observed)?;
        Ok(Self {
            plan_binding: plan_binding.clone(),
            planned,
            observed,
            observation,
        })
    }

    pub const fn plan_binding(&self) -> &crate::AccessPlanIdentity {
        &self.plan_binding
    }

    pub const fn planned(&self) -> crate::AccessPathCounterSnapshot {
        self.planned
    }

    pub const fn observed(&self) -> crate::AccessPathCounterSnapshot {
        self.observed
    }

    pub const fn observation(&self) -> crate::PlannedCounterObservation {
        self.observation
    }
}
