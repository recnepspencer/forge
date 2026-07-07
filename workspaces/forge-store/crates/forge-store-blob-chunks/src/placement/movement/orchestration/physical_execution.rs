use forge_store_physical_isolation::PhysicalPlacementMovementExecutionReceipt;

use crate::placement::movement::{
    denial::BlobPlacementMovementDenial,
    receipt_construction::store_owned_execution_receipt::construct_store_owned_execution_receipt,
    types::{
        AdmittedBlobPlacementMovementPlan, BlobPlacementMovementPhysicalExecutionIntent,
        StoreOwnedPlacementMovementExecution, StoreOwnedPlacementMovementExecutionReceipt,
    },
    verification::physical_execution_match::verify_physical_execution_receipt_matches_plan,
};

impl StoreOwnedPlacementMovementExecution {
    pub const fn store_owned() -> Self {
        Self { _private: () }
    }

    pub fn execute_physical_movement(
        self,
        plan: &AdmittedBlobPlacementMovementPlan,
        physical_receipt: PhysicalPlacementMovementExecutionReceipt<
            BlobPlacementMovementPhysicalExecutionIntent,
        >,
    ) -> Result<StoreOwnedPlacementMovementExecutionReceipt, BlobPlacementMovementDenial> {
        verify_physical_execution_receipt_matches_plan(plan, &physical_receipt, plan.counters())?;
        Ok(construct_store_owned_execution_receipt(
            plan.basis().clone(),
            plan.source_class(),
            plan.target_class(),
            physical_receipt.movement_interlock(),
        ))
    }
}
