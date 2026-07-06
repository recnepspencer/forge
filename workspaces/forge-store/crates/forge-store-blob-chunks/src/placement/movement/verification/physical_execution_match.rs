use forge_store_physical_isolation::PhysicalPlacementMovementExecutionReceipt;

use crate::placement::movement::{
    counters::BlobPlacementMovementCounterSnapshot, denial::BlobPlacementMovementDenial,
    types::{
        execution_receipt::StoreOwnedPlacementMovementExecutionReceipt,
        plan::{AdmittedBlobPlacementMovementPlan, BlobPlacementMovementPhysicalExecutionIntent},
    },
};

pub(crate) fn verify_physical_execution_receipt_matches_plan(
    plan: &AdmittedBlobPlacementMovementPlan,
    physical_receipt: &PhysicalPlacementMovementExecutionReceipt<
        BlobPlacementMovementPhysicalExecutionIntent,
    >,
    counters: BlobPlacementMovementCounterSnapshot,
) -> Result<(), BlobPlacementMovementDenial> {
    if physical_receipt.movement_interlock() == plan.read_hold().movement_interlock()
        && physical_receipt.intent().basis_digest() == &plan.physical_execution_basis_digest()
    {
        return Ok(());
    }
    Err(BlobPlacementMovementDenial::MovementExecutionReceiptMismatch {
        counters: counters.record_protected_denial(),
    })
}

pub(crate) fn verify_store_owned_execution_receipt_matches_plan(
    receipt: &StoreOwnedPlacementMovementExecutionReceipt,
    plan: &AdmittedBlobPlacementMovementPlan,
    counters: BlobPlacementMovementCounterSnapshot,
) -> Result<(), BlobPlacementMovementDenial> {
    if receipt.movement_interlock == plan.read_hold().movement_interlock()
        && receipt.basis == *plan.basis()
        && receipt.source_class == plan.source_class()
        && receipt.target_class == plan.target_class()
    {
        return Ok(());
    }
    Err(BlobPlacementMovementDenial::MovementExecutionReceiptMismatch {
        counters: counters.record_protected_denial(),
    })
}