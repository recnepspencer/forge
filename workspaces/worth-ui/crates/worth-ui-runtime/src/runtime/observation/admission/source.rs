use super::super::progress::UiObservationProgress;
use super::super::turn::{
    UiAdmittedObservation, UiAdmittedObservationPayload, UiAdmittedObservationSeal,
    UiObservationAdmissionDenial, UiObservationAdmissionReceipt, UiObservationTurn,
};
use super::super::UiObservationFamily;

pub struct UiAdmittedSourceObservation<'candidate> {
    candidate: &'candidate crate::runtime::WorthUiWatchedCandidateSubmission,
}

impl<'candidate> UiAdmittedSourceObservation<'candidate> {
    pub(in crate::runtime::observation) const fn new(
        candidate: &'candidate crate::runtime::WorthUiWatchedCandidateSubmission,
    ) -> Self {
        Self { candidate }
    }

    pub fn revision(&self) -> &crate::runtime::WorthUiSourcePackageRevision {
        self.candidate.source_revision()
    }

    pub fn ordering_receipt(&self) -> &crate::runtime::WorthUiCandidateOrderingReceipt {
        self.candidate.ordering_receipt()
    }

    pub fn counters(&self) -> crate::runtime::WorthUiSourceIngressCounters {
        self.candidate.counters()
    }

    pub fn composition_basis(&self) -> &crate::runtime::WorthUiCandidateCompositionBasis {
        self.candidate.composition_basis()
    }
}

impl UiObservationTurn<'_> {
    pub fn admit_source(
        &mut self,
        candidate: crate::runtime::WorthUiWatchedCandidateSubmission,
    ) -> Result<UiObservationAdmissionReceipt, UiObservationAdmissionDenial> {
        if candidate.snapshot_digest() != self.source_basis {
            return Err(self.reject(UiObservationAdmissionDenial::ForeignSourceBasis));
        }
        let owner_order = candidate.ordering_receipt().sequence();
        let retained_bytes = candidate.retained_observation_bytes();
        let progress = UiObservationProgress::authored_source(
            candidate.ordering_receipt().provider_id(),
            owner_order,
        );
        self.admit(UiAdmittedObservation::seal(UiAdmittedObservationSeal {
            family: UiObservationFamily::AuthoredSource,
            owner_order,
            retained_bytes,
            session: self.session,
            source_basis: self.source_basis,
            progress: Some(progress),
            payload: UiAdmittedObservationPayload::Source(candidate),
        }))
    }
}
