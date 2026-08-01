use super::UiIntentExecutionState;

impl UiIntentExecutionState {
    pub(crate) fn install_capacity_for_certification(
        &mut self,
        capacity: super::super::UiIntentExecutionCapacity,
    ) -> bool {
        if self.active_count() != 0 {
            return false;
        }
        self.capacity = capacity;
        true
    }

    pub(crate) fn exhaust_reservation_identities_for_certification(&mut self) -> usize {
        let mut exhausted = 0;
        for slot in &mut self.slots {
            if slot.phase.is_none() {
                slot.generation = u64::MAX;
                exhausted += 1;
            }
        }
        exhausted
    }

    pub(crate) fn reserve_occupancy_for_certification(
        &mut self,
        proof: crate::runtime::intent::UiIntentOperabilityProof,
    ) -> Result<
        crate::runtime::intent::UiIntentOccupancyReservation,
        crate::runtime::intent::UiIntentOccupancyReservationDenial,
    > {
        self.occupancy.reserve(proof)
    }

    pub(crate) fn release_occupancy_for_certification(
        &mut self,
        reservation: crate::runtime::intent::UiIntentOccupancyReservation,
    ) -> crate::runtime::intent::UiIntentOccupancyReleasePosture {
        self.occupancy.release(reservation)
    }
}
