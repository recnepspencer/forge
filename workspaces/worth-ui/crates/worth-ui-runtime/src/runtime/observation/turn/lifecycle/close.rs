use super::{
    UiAdmittedObservation, UiAdmittedObservationSet, UiObservationAdmissionDenial,
    UiObservationTurn, UiObservationTurnCloseAuthority, UiPreparedObservationProgressCommit,
};

impl UiObservationTurn<'_> {
    pub fn seal(mut self) -> Result<UiAdmittedObservationSet, UiObservationAdmissionDenial> {
        self.validate_close()?;
        self.order_observations();
        self.runtime.observation.finish_committed(
            self.observations
                .iter()
                .filter_map(UiAdmittedObservation::progress),
        );
        Ok(self.seal_set())
    }

    pub(crate) fn prepare_seal(
        mut self,
    ) -> Result<
        (
            UiAdmittedObservationSet,
            UiPreparedObservationProgressCommit,
        ),
        UiObservationAdmissionDenial,
    > {
        self.validate_close()?;
        self.order_observations();
        let progress = self
            .observations
            .iter()
            .filter_map(UiAdmittedObservation::progress)
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok((
            self.seal_set(),
            UiPreparedObservationProgressCommit { progress },
        ))
    }

    fn validate_close(&self) -> Result<(), UiObservationAdmissionDenial> {
        if self.poisoned {
            return Err(UiObservationAdmissionDenial::PoisonedTurn);
        }
        if self.observations.is_empty() {
            return Err(UiObservationAdmissionDenial::EmptyTurn);
        }
        Ok(())
    }

    fn order_observations(&mut self) {
        self.observations.sort_by_key(|observation| {
            (
                observation.family().definition().framework_rank(),
                observation.owner_order(),
            )
        });
    }

    fn seal_set(&mut self) -> UiAdmittedObservationSet {
        let lease = self
            .runtime
            .observation
            .retain_set(self.observations.len(), self.retained_bytes);
        let observations = std::mem::take(&mut self.observations).into_boxed_slice();
        let appearance_owner_snapshot = self.seal_appearance_owner_snapshot();
        UiAdmittedObservationSet::seal(
            self.identity,
            self.session,
            self.source_basis,
            observations,
            self.retained_bytes,
            appearance_owner_snapshot,
            lease,
        )
    }

    fn seal_appearance_owner_snapshot(
        &mut self,
    ) -> Option<crate::runtime::appearance::UiAppearanceOwnerSnapshot> {
        let authority = UiObservationTurnCloseAuthority { _private: () };
        self.appearance_close
            .take()
            .map(|input| input.seal(&authority, self.identity, self.session, self.source_basis))
    }
}
