use crate::placement::movement::{
    performance::counter_backed_placement_movement_performance_receipt,
    transitions::publish_placement_observation::transition_publish_placement_observation,
    types::{
        BlobMovementReadPhase, BlobReadDuringPlacementMove, ExecutedBlobPlacementMovementReceipt,
        PublishedBlobPlacementObservation, StoreOwnedPlacementMovementPublication,
    },
    BlobPlacementMovementCounterBackedPerformanceReceipt,
};

impl ExecutedBlobPlacementMovementReceipt {
    pub fn publish_observation(
        self,
        publication: StoreOwnedPlacementMovementPublication,
    ) -> PublishedBlobPlacementObservation {
        transition_publish_placement_observation(self, publication)
    }

    pub fn read_guard(&self, phase: BlobMovementReadPhase) -> BlobReadDuringPlacementMove {
        BlobReadDuringPlacementMove::from_executed(self, phase)
    }

    pub fn lower_to_foundational_performance(
        &self,
    ) -> BlobPlacementMovementCounterBackedPerformanceReceipt {
        counter_backed_placement_movement_performance_receipt(self.counters())
    }
}

impl PublishedBlobPlacementObservation {
    pub fn read_guard(&self) -> BlobReadDuringPlacementMove {
        BlobReadDuringPlacementMove::from_published(self)
    }
}