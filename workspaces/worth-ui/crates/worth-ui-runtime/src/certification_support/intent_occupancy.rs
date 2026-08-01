pub use crate::runtime::intent::{
    UiIntentOccupancyReleasePosture, UiIntentOccupancyReservation,
    UiIntentOccupancyReservationDenial,
};

/// SUPPORT AUTHORITY for holding one real production occupancy reservation
/// while certification probes peer-route isolation.
pub trait WorthUiIntentOccupancyCertificationExt {
    fn reserve_intent_occupancy_for_certification(
        &mut self,
        proof: crate::facade::intent::UiIntentOperabilityProof,
    ) -> Result<UiIntentOccupancyReservation, UiIntentOccupancyReservationDenial>;

    fn release_intent_occupancy_for_certification(
        &mut self,
        reservation: UiIntentOccupancyReservation,
    ) -> UiIntentOccupancyReleasePosture;

    fn active_intent_occupancy_count_for_certification(&self) -> usize;
}

impl WorthUiIntentOccupancyCertificationExt for crate::facade::WorthUiActiveApplicationSession {
    fn reserve_intent_occupancy_for_certification(
        &mut self,
        proof: crate::facade::intent::UiIntentOperabilityProof,
    ) -> Result<UiIntentOccupancyReservation, UiIntentOccupancyReservationDenial> {
        self.reserve_intent_occupancy_for_certification(proof)
    }

    fn release_intent_occupancy_for_certification(
        &mut self,
        reservation: UiIntentOccupancyReservation,
    ) -> UiIntentOccupancyReleasePosture {
        self.release_intent_occupancy_for_certification(reservation)
    }

    fn active_intent_occupancy_count_for_certification(&self) -> usize {
        self.active_intent_occupancy_count_for_certification()
    }
}
