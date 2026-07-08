use crate::placement::movement::{
    denial::BlobPlacementMovementDenial,
    receipt_construction::executed_receipt::construct_executed_receipt,
    types::{
        execution_receipt::{
            ExecutedBlobPlacementMovementReceipt, StoreOwnedPlacementMovementExecutionReceipt,
        },
        plan::AdmittedBlobPlacementMovementPlan,
    },
    verification::physical_execution_match::verify_store_owned_execution_receipt_matches_plan,
};

pub(crate) fn transition_execute_movement_receipt(
    receipt: StoreOwnedPlacementMovementExecutionReceipt,
    plan: AdmittedBlobPlacementMovementPlan,
) -> Result<ExecutedBlobPlacementMovementReceipt, BlobPlacementMovementDenial> {
    verify_store_owned_execution_receipt_matches_plan(&receipt, &plan, plan.counters())?;
    let counters = plan.counters().record_execution_receipt();
    Ok(construct_executed_receipt(
        receipt.basis,
        receipt.source_class,
        receipt.target_class,
        counters,
    ))
}
