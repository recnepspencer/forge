impl super::WorthUiActiveApplicationSession {
    pub fn begin_observation_turn(
        &mut self,
    ) -> Result<
        crate::facade::observation::UiObservationTurn<'_>,
        crate::facade::observation::UiObservationTurnDenial,
    > {
        let session = self.identity;
        self.application.begin_observation_turn(session)
    }
}
