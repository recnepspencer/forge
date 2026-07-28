use super::{UiAdmittedObservation, UiObservationSetSummary, UiObservationTurnIdentity};

pub struct UiAdmittedObservationSet {
    turn: UiObservationTurnIdentity,
    observations: Box<[UiAdmittedObservation]>,
    summary: UiObservationSetSummary,
}

impl UiAdmittedObservationSet {
    pub(super) fn seal(
        turn: UiObservationTurnIdentity,
        observations: Box<[UiAdmittedObservation]>,
        retained_bytes: usize,
    ) -> Self {
        debug_assert!(!observations.is_empty());
        let families = observations
            .iter()
            .map(UiAdmittedObservation::family)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let summary = UiObservationSetSummary::new(observations.len(), retained_bytes, families);
        Self {
            turn,
            observations,
            summary,
        }
    }

    pub const fn turn(&self) -> UiObservationTurnIdentity {
        self.turn
    }

    pub fn observations(&self) -> &[UiAdmittedObservation] {
        &self.observations
    }

    pub const fn retained_bytes(&self) -> usize {
        self.summary.retained_bytes()
    }

    pub const fn summary(&self) -> &UiObservationSetSummary {
        &self.summary
    }
}
