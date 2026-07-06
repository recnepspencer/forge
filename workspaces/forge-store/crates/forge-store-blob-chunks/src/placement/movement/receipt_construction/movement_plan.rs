use crate::placement::movement::{
    counters::BlobPlacementMovementCounterSnapshot,
    types::{
        basis::BlobPlacementMovementBasis,
        plan::AdmittedBlobPlacementMovementPlan, read_hold::BlobPlacementMovementReadHold,
        request::BlobPlacementMovementRequest,
    },
};

pub(crate) fn construct_movement_plan(
    request: BlobPlacementMovementRequest,
    read_hold: BlobPlacementMovementReadHold,
    counters: BlobPlacementMovementCounterSnapshot,
) -> AdmittedBlobPlacementMovementPlan {
    AdmittedBlobPlacementMovementPlan {
        basis: BlobPlacementMovementBasis::from_lifecycle(request.lifecycle()),
        source_class: request.source().class(),
        target_class: request.target().class(),
        read_hold,
        cold_outcome: request.cold_outcome(),
        counters: counters
            .record_read(request.source().class())
            .record_read(request.target().class())
            .record_move(),
    }
}