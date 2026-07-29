use super::super::{UiObservationFamily, UiObservationProfile};
use super::{
    UiAdmittedObservation, UiAdmittedObservationSet, UiObservationAdmissionDenial,
    UiObservationAdmissionReceipt, UiObservationTurnDenial, UiObservationTurnIdentity,
};

pub struct UiObservationTurn<'state> {
    pub(in crate::runtime::observation) runtime: &'state mut crate::runtime::WorthUiRuntime,
    identity: UiObservationTurnIdentity,
    pub(in crate::runtime::observation) session:
        crate::facade::WorthUiActiveApplicationSessionIdentity,
    pub(in crate::runtime::observation) source_basis: u64,
    profile: UiObservationProfile,
    observations: Vec<UiAdmittedObservation>,
    retained_bytes: usize,
    poisoned: bool,
}

impl<'state> UiObservationTurn<'state> {
    pub(super) fn new(
        runtime: &'state mut crate::runtime::WorthUiRuntime,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        source_basis: u64,
        identity: u64,
        profile: UiObservationProfile,
    ) -> Self {
        Self {
            runtime,
            identity: UiObservationTurnIdentity::issued_by_runtime(identity),
            session,
            source_basis,
            profile,
            observations: Vec::new(),
            retained_bytes: 0,
            poisoned: false,
        }
    }

    pub const fn identity(&self) -> UiObservationTurnIdentity {
        self.identity
    }

    pub(in crate::runtime::observation) fn admit(
        &mut self,
        observation: UiAdmittedObservation,
    ) -> Result<UiObservationAdmissionReceipt, UiObservationAdmissionDenial> {
        if self.poisoned {
            return Err(UiObservationAdmissionDenial::PoisonedTurn);
        }
        let result = self.admit_unpoisoned(observation);
        if result.is_err() {
            self.poisoned = true;
        }
        result
    }

    pub(in crate::runtime::observation) fn admit_batch(
        &mut self,
        observations: Vec<UiAdmittedObservation>,
    ) -> Result<Box<[UiObservationAdmissionReceipt]>, UiObservationAdmissionDenial> {
        if self.poisoned {
            return Err(UiObservationAdmissionDenial::PoisonedTurn);
        }
        let result = self.admit_batch_unpoisoned(observations);
        if result.is_err() {
            self.poisoned = true;
        }
        result
    }

    pub(in crate::runtime::observation) fn reject(
        &mut self,
        denial: UiObservationAdmissionDenial,
    ) -> UiObservationAdmissionDenial {
        self.poisoned = true;
        denial
    }

    pub(in crate::runtime::observation) fn poison(&mut self) {
        self.poisoned = true;
    }

    fn admit_unpoisoned(
        &mut self,
        observation: UiAdmittedObservation,
    ) -> Result<UiObservationAdmissionReceipt, UiObservationAdmissionDenial> {
        if observation.session() != self.session {
            return Err(UiObservationAdmissionDenial::ForeignSession);
        }
        if observation.source_basis() != self.source_basis {
            return Err(UiObservationAdmissionDenial::ForeignSourceBasis);
        }
        if observation
            .progress()
            .is_some_and(|progress| self.runtime.observation.is_historical(progress))
        {
            return Err(UiObservationAdmissionDenial::HistoricalOwnerOrder);
        }
        self.can_admit(observation.family(), observation.retained_bytes())?;
        let receipt = UiObservationAdmissionReceipt::new(
            observation.family(),
            observation.owner_order(),
            observation.retained_bytes(),
        );
        self.push_admitted(observation);
        Ok(receipt)
    }

    fn admit_batch_unpoisoned(
        &mut self,
        observations: Vec<UiAdmittedObservation>,
    ) -> Result<Box<[UiObservationAdmissionReceipt]>, UiObservationAdmissionDenial> {
        let mut families = std::collections::BTreeSet::new();
        let mut retained_bytes = 0usize;
        for observation in &observations {
            if observation.session() != self.session {
                return Err(UiObservationAdmissionDenial::ForeignSession);
            }
            if observation.source_basis() != self.source_basis {
                return Err(UiObservationAdmissionDenial::ForeignSourceBasis);
            }
            if observation
                .progress()
                .is_some_and(|progress| self.runtime.observation.is_historical(progress))
            {
                return Err(UiObservationAdmissionDenial::HistoricalOwnerOrder);
            }
            if !families.insert(observation.family())
                || self
                    .observations
                    .iter()
                    .any(|item| item.family() == observation.family())
            {
                return Err(UiObservationAdmissionDenial::DuplicateFamily);
            }
            retained_bytes = retained_bytes
                .checked_add(observation.retained_bytes())
                .ok_or(UiObservationAdmissionDenial::ByteCapacityExceeded)?;
        }
        if self
            .observations
            .len()
            .checked_add(observations.len())
            .is_none_or(|count| count > self.profile.admitted_per_turn())
        {
            return Err(UiObservationAdmissionDenial::TurnCapacityExceeded);
        }
        if self
            .retained_bytes
            .checked_add(retained_bytes)
            .is_none_or(|bytes| bytes > self.profile.retained_bytes_per_turn())
        {
            return Err(UiObservationAdmissionDenial::ByteCapacityExceeded);
        }
        let receipts = observations
            .iter()
            .map(|observation| {
                UiObservationAdmissionReceipt::new(
                    observation.family(),
                    observation.owner_order(),
                    observation.retained_bytes(),
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        for observation in observations {
            self.push_admitted(observation);
        }
        Ok(receipts)
    }

    pub(in crate::runtime::observation) fn can_admit(
        &self,
        family: UiObservationFamily,
        retained_bytes: usize,
    ) -> Result<(), UiObservationAdmissionDenial> {
        if self.poisoned {
            return Err(UiObservationAdmissionDenial::PoisonedTurn);
        }
        if self.observations.iter().any(|item| item.family() == family) {
            return Err(UiObservationAdmissionDenial::DuplicateFamily);
        }
        if self.observations.len() == self.profile.admitted_per_turn() {
            return Err(UiObservationAdmissionDenial::TurnCapacityExceeded);
        }
        let total = self
            .retained_bytes
            .checked_add(retained_bytes)
            .ok_or(UiObservationAdmissionDenial::ByteCapacityExceeded)?;
        if total > self.profile.retained_bytes_per_turn() {
            return Err(UiObservationAdmissionDenial::ByteCapacityExceeded);
        }
        Ok(())
    }

    pub(in crate::runtime::observation) fn push_admitted(
        &mut self,
        observation: UiAdmittedObservation,
    ) {
        self.retained_bytes += observation.retained_bytes();
        self.observations.push(observation);
    }

    pub fn seal(mut self) -> Result<UiAdmittedObservationSet, UiObservationAdmissionDenial> {
        if self.poisoned {
            return Err(UiObservationAdmissionDenial::PoisonedTurn);
        }
        if self.observations.is_empty() {
            return Err(UiObservationAdmissionDenial::EmptyTurn);
        }
        self.observations.sort_by_key(|observation| {
            (
                observation.family().definition().framework_rank(),
                observation.owner_order(),
            )
        });
        self.runtime.observation.finish_committed(
            self.observations
                .iter()
                .filter_map(UiAdmittedObservation::progress),
        );
        let observations = std::mem::take(&mut self.observations).into_boxed_slice();
        Ok(UiAdmittedObservationSet::seal(
            self.identity,
            self.session,
            self.source_basis,
            observations,
            self.retained_bytes,
        ))
    }
}

impl Drop for UiObservationTurn<'_> {
    fn drop(&mut self) {
        self.runtime.observation.abandon();
    }
}

impl crate::runtime::WorthUiRuntime {
    pub(crate) fn begin_observation_turn(
        &mut self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
        source_basis: u64,
    ) -> Result<UiObservationTurn<'_>, UiObservationTurnDenial> {
        let profile = self.change_profile.observation();
        let (identity, profile) = self.observation.begin(profile)?;
        Ok(UiObservationTurn::new(
            self,
            session,
            source_basis,
            identity,
            profile,
        ))
    }
}
