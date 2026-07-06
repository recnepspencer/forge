use crate::BlobPlacementClass;

use crate::placement::movement::{
    counters::BlobPlacementMovementCounterSnapshot,
    types::{
        basis::BlobPlacementMovementBasis, execution_receipt::PublishedBlobPlacementObservation,
    },
};

pub(crate) fn construct_published_observation(
    basis: BlobPlacementMovementBasis,
    placement_class: BlobPlacementClass,
    counters: BlobPlacementMovementCounterSnapshot,
) -> PublishedBlobPlacementObservation {
    PublishedBlobPlacementObservation {
        basis,
        placement_class,
        counters,
    }
}