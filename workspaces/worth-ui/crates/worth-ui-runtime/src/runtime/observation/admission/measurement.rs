use super::super::progress::UiObservationProgress;
use super::super::turn::{
    UiAdmittedObservation, UiAdmittedObservationPayload, UiAdmittedObservationSeal,
    UiObservationAdmissionDenial, UiObservationAdmissionReceipt, UiObservationTurn,
};
use super::super::UiObservationFamily;

impl UiObservationTurn<'_> {
    pub fn admit_measurement(
        &mut self,
        measurement: crate::host_exchange::measurement_admission::UiSolicitedHostMeasurementResult,
    ) -> Result<UiObservationAdmissionReceipt, UiObservationAdmissionDenial> {
        let owner_order = measurement.source_order();
        let progress =
            UiObservationProgress::measurement(measurement.source_identity(), owner_order);
        self.admit(UiAdmittedObservation::seal(UiAdmittedObservationSeal {
            family: UiObservationFamily::Measurement,
            owner_order,
            retained_bytes: measurement.retained_bytes(),
            session: self.session,
            source_basis: self.source_basis,
            progress: Some(progress),
            payload: UiAdmittedObservationPayload::Measurement(measurement),
        }))
    }
}
