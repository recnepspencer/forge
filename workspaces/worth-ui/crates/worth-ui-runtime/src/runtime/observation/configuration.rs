#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObservationProfile {
    admitted_per_turn: usize,
    retained_bytes_per_turn: usize,
    queued_during_effecting_rebind: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObservationProfileInput {
    pub admitted_per_turn: usize,
    pub retained_bytes_per_turn: usize,
    pub queued_during_effecting_rebind: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObservationProfileConstructionDenial {
    EmptyTurnCapacity,
    EmptyByteCapacity,
    EmptyQueueCapacity,
}

impl UiObservationProfile {
    pub fn bounded(
        input: UiObservationProfileInput,
    ) -> Result<Self, UiObservationProfileConstructionDenial> {
        if input.admitted_per_turn == 0 {
            return Err(UiObservationProfileConstructionDenial::EmptyTurnCapacity);
        }
        if input.retained_bytes_per_turn == 0 {
            return Err(UiObservationProfileConstructionDenial::EmptyByteCapacity);
        }
        if input.queued_during_effecting_rebind == 0 {
            return Err(UiObservationProfileConstructionDenial::EmptyQueueCapacity);
        }
        Ok(Self {
            admitted_per_turn: input.admitted_per_turn,
            retained_bytes_per_turn: input.retained_bytes_per_turn,
            queued_during_effecting_rebind: input.queued_during_effecting_rebind,
        })
    }

    pub fn platform_pulse() -> Self {
        Self {
            admitted_per_turn: 8,
            retained_bytes_per_turn: 65_536,
            queued_during_effecting_rebind: 16,
        }
    }

    pub const fn admitted_per_turn(self) -> usize {
        self.admitted_per_turn
    }

    pub const fn retained_bytes_per_turn(self) -> usize {
        self.retained_bytes_per_turn
    }

    pub const fn queued_during_effecting_rebind(self) -> usize {
        self.queued_during_effecting_rebind
    }
}
