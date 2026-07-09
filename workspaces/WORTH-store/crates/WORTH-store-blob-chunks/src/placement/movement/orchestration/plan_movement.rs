use crate::placement::movement::{
    denial::BlobPlacementMovementDenial,
    transitions::admit_movement_plan::transition_admit_movement_plan,
    transitions::execute_movement_receipt::transition_execute_movement_receipt,
    types::{
        AdmittedBlobPlacementMovementPlan, BlobMovementReadPhase, BlobPlacementMovementAuthority,
        BlobPlacementMovementPhysicalExecutionIntent, BlobPlacementMovementRequest,
        BlobReadDuringPlacementMove, ExecutedBlobPlacementMovementReceipt,
        StoreOwnedPlacementMovementExecutionReceipt,
    },
};

impl BlobPlacementMovementAuthority {
    pub fn plan_movement(
        &self,
        request: BlobPlacementMovementRequest,
    ) -> Result<AdmittedBlobPlacementMovementPlan, BlobPlacementMovementDenial> {
        transition_admit_movement_plan(request)
    }
}

impl AdmittedBlobPlacementMovementPlan {
    pub fn execute_with_receipt(
        self,
        receipt: StoreOwnedPlacementMovementExecutionReceipt,
    ) -> Result<ExecutedBlobPlacementMovementReceipt, BlobPlacementMovementDenial> {
        transition_execute_movement_receipt(receipt, self)
    }

    pub fn read_guard(&self, phase: BlobMovementReadPhase) -> BlobReadDuringPlacementMove {
        BlobReadDuringPlacementMove::from_plan(self, phase)
    }

    pub fn physical_execution_intent(&self) -> BlobPlacementMovementPhysicalExecutionIntent {
        BlobPlacementMovementPhysicalExecutionIntent {
            basis_digest: self.physical_execution_basis_digest(),
        }
    }
}
