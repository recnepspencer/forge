#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiObservationResourceSnapshot {
    active_turns: usize,
    retained_sets: usize,
    retained_observations: usize,
    retained_bytes: usize,
}

impl UiObservationResourceSnapshot {
    pub(super) const fn from_active_turn(
        active: bool,
        retained_observations: usize,
        retained_bytes: usize,
    ) -> Self {
        Self {
            active_turns: active as usize,
            retained_sets: 0,
            retained_observations,
            retained_bytes,
        }
    }

    pub(super) const fn from_retained_sets(
        retained_sets: usize,
        retained_observations: usize,
        retained_bytes: usize,
    ) -> Self {
        Self {
            active_turns: 0,
            retained_sets,
            retained_observations,
            retained_bytes,
        }
    }

    pub(super) fn combine(self, retained: Self) -> Self {
        Self {
            active_turns: self.active_turns,
            retained_sets: retained.retained_sets,
            retained_observations: self
                .retained_observations
                .checked_add(retained.retained_observations)
                .expect("bounded retained observation count fits"),
            retained_bytes: self
                .retained_bytes
                .checked_add(retained.retained_bytes)
                .expect("bounded retained observation bytes fit"),
        }
    }

    pub const fn active_turns(self) -> usize {
        self.active_turns
    }

    pub const fn retained_sets(self) -> usize {
        self.retained_sets
    }

    pub const fn retained_observations(self) -> usize {
        self.retained_observations
    }

    pub const fn retained_bytes(self) -> usize {
        self.retained_bytes
    }
}
