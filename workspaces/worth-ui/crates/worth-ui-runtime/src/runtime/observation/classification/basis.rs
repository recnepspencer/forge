use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
use crate::facade::WorthUiActiveApplicationSessionIdentity;
use crate::runtime::observation::UiObservationTurnIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiChangeClassificationBasis {
    session: WorthUiActiveApplicationSessionIdentity,
    source_basis: u64,
    turn: UiObservationTurnIdentity,
    observation_count: usize,
    predecessor_generation: WorthUiPreparedApplicationGenerationIdentity,
}

impl UiChangeClassificationBasis {
    pub(crate) const fn new(
        session: WorthUiActiveApplicationSessionIdentity,
        source_basis: u64,
        turn: UiObservationTurnIdentity,
        observation_count: usize,
        predecessor_generation: WorthUiPreparedApplicationGenerationIdentity,
    ) -> Self {
        Self {
            session,
            source_basis,
            turn,
            observation_count,
            predecessor_generation,
        }
    }

    pub const fn session(&self) -> WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub const fn source_basis(&self) -> u64 {
        self.source_basis
    }

    pub const fn turn(&self) -> UiObservationTurnIdentity {
        self.turn
    }

    pub const fn observation_count(&self) -> usize {
        self.observation_count
    }

    pub fn predecessor_generation(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.predecessor_generation
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn replace_session_for_certification(
        &mut self,
        session: WorthUiActiveApplicationSessionIdentity,
    ) {
        self.session = session;
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn replace_predecessor_generation_for_certification(
        &mut self,
        generation: WorthUiPreparedApplicationGenerationIdentity,
    ) {
        self.predecessor_generation = generation;
    }
}
