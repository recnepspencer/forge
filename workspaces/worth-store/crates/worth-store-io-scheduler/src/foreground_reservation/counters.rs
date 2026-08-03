use super::ForegroundResourceBudget;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForegroundReservationCounterSnapshot {
    requested: ForegroundResourceBudget,
    available: ForegroundResourceBudget,
    admitted: ForegroundResourceBudget,
    denied: ForegroundResourceBudget,
    denied_capacity_events: u64,
}

impl ForegroundReservationCounterSnapshot {
    pub const fn admitted(
        requested: ForegroundResourceBudget,
        available: ForegroundResourceBudget,
        admitted: ForegroundResourceBudget,
    ) -> Self {
        Self {
            requested,
            available,
            admitted,
            denied: ForegroundResourceBudget::new(),
            denied_capacity_events: 0,
        }
    }

    pub const fn denied_capacity(
        requested: ForegroundResourceBudget,
        available: ForegroundResourceBudget,
        denied: ForegroundResourceBudget,
    ) -> Self {
        Self {
            requested,
            available,
            admitted: ForegroundResourceBudget::new(),
            denied,
            denied_capacity_events: 1,
        }
    }

    pub const fn requested(self) -> ForegroundResourceBudget {
        self.requested
    }

    pub const fn available(self) -> ForegroundResourceBudget {
        self.available
    }

    pub const fn admitted_budget(self) -> ForegroundResourceBudget {
        self.admitted
    }

    pub const fn denied_budget(self) -> ForegroundResourceBudget {
        self.denied
    }

    pub const fn denied_capacity_events(self) -> u64 {
        self.denied_capacity_events
    }
}
