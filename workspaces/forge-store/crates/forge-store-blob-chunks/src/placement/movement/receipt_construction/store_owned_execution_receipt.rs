use forge_store_physical_isolation::ChunkMigrationReadInterlockPlan;

use crate::BlobPlacementClass;

use crate::placement::movement::types::{
    basis::BlobPlacementMovementBasis,
    execution_receipt::StoreOwnedPlacementMovementExecutionReceipt,
};

pub(crate) fn construct_store_owned_execution_receipt(
    basis: BlobPlacementMovementBasis,
    source_class: BlobPlacementClass,
    target_class: BlobPlacementClass,
    movement_interlock: ChunkMigrationReadInterlockPlan,
) -> StoreOwnedPlacementMovementExecutionReceipt {
    StoreOwnedPlacementMovementExecutionReceipt {
        basis,
        source_class,
        target_class,
        movement_interlock,
    }
}
