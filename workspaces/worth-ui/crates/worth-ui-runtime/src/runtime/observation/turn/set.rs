use super::{UiAdmittedObservation, UiObservationSetSummary, UiObservationTurnIdentity};

pub struct UiAdmittedObservationSet {
    turn: UiObservationTurnIdentity,
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    source_basis: u64,
    observations: Box<[UiAdmittedObservation]>,
    summary: UiObservationSetSummary,
}

impl UiAdmittedObservationSet {
    pub(super) fn seal(
        turn: UiObservationTurnIdentity,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        source_basis: u64,
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
            session,
            source_basis,
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

    pub(in crate::runtime::observation) const fn session(
        &self,
    ) -> crate::facade::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub(in crate::runtime::observation) const fn source_basis(&self) -> u64 {
        self.source_basis
    }

    pub(in crate::runtime::observation) fn into_observations(self) -> Box<[UiAdmittedObservation]> {
        self.observations
    }
}
