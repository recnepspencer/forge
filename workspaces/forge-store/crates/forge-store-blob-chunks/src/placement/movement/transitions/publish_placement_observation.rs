use crate::placement::movement::{
    receipt_construction::published_observation::construct_published_observation,
    types::execution_receipt::{
        ExecutedBlobPlacementMovementReceipt, PublishedBlobPlacementObservation,
        StoreOwnedPlacementMovementPublication,
    },
};

pub(crate) fn transition_publish_placement_observation(
    receipt: ExecutedBlobPlacementMovementReceipt,
    _: StoreOwnedPlacementMovementPublication,
) -> PublishedBlobPlacementObservation {
    construct_published_observation(
        receipt.basis,
        receipt.target_class,
        receipt.counters.record_published_observation(),
    )
}
