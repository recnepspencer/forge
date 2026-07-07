use crate::BlobPlacementClass;

use crate::placement::movement::{
    counters::BlobPlacementMovementCounterSnapshot,
    types::{
        basis::BlobPlacementMovementBasis, execution_receipt::ExecutedBlobPlacementMovementReceipt,
    },
};

pub(crate) fn construct_executed_receipt(
    basis: BlobPlacementMovementBasis,
    source_class: BlobPlacementClass,
    target_class: BlobPlacementClass,
    counters: BlobPlacementMovementCounterSnapshot,
) -> ExecutedBlobPlacementMovementReceipt {
    ExecutedBlobPlacementMovementReceipt {
        basis,
        source_class,
        target_class,
        counters,
    }
}
