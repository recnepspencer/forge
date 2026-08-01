#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObservationResourceRetirementCause {
    RuntimeWithoutApplication,
    ApplicationReplacement,
    ApplicationShutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObservationResourceRetirementReport {
    cause: UiObservationResourceRetirementCause,
    disposed_sets: usize,
    disposed_observations: usize,
    disposed_bytes: usize,
    active_after: usize,
}

impl UiObservationResourceRetirementReport {
    pub(crate) const fn new(
        cause: UiObservationResourceRetirementCause,
        snapshot: super::UiObservationResourceSnapshot,
    ) -> Self {
        Self {
            cause,
            disposed_sets: snapshot.retained_sets(),
            disposed_observations: snapshot.retained_observations(),
            disposed_bytes: snapshot.retained_bytes(),
            active_after: 0,
        }
    }

    pub const fn cause(self) -> UiObservationResourceRetirementCause {
        self.cause
    }

    pub const fn disposed_sets(self) -> usize {
        self.disposed_sets
    }

    pub const fn disposed_observations(self) -> usize {
        self.disposed_observations
    }

    pub const fn disposed_bytes(self) -> usize {
        self.disposed_bytes
    }

    pub const fn active_after(self) -> usize {
        self.active_after
    }
}

impl Default for UiObservationResourceRetirementReport {
    fn default() -> Self {
        Self {
            cause: UiObservationResourceRetirementCause::RuntimeWithoutApplication,
            disposed_sets: 0,
            disposed_observations: 0,
            disposed_bytes: 0,
            active_after: 0,
        }
    }
}
