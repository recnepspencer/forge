use schema::facade::platform::authority::touched_graph_conflict::BatchAdmissionPlannerRouteWitness;

use crate::workload_composition::{
    planner_owned_routing::{PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind},
    BatchAdmissionExecutionReceipt,
};

#[derive(Clone, Debug)]
pub(crate) struct AdmittedBatchAdmissionPlannerRouteInput {
    receipt: BatchAdmissionExecutionReceipt,
    denial_witness: Option<BatchAdmissionPlannerRouteWitness>,
}

pub(crate) fn admit_batch_admission_planner_route_input(
    receipt: BatchAdmissionExecutionReceipt,
) -> Result<AdmittedBatchAdmissionPlannerRouteInput, PlannerOwnedRoutingError> {
    if receipt.selected_batch_plan_digest().is_empty()
        || receipt.execution_receipt_digest().is_empty()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
            "planner-owned batch-admission route requires selected batch and execution receipt identity",
        ));
    }
    if receipt.selected_family_rows().is_empty() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
            "planner-owned batch-admission route requires selected family rows",
        ));
    }

    let denial_witness = receipt.denial().map(|_| {
        BatchAdmissionPlannerRouteWitness::new(
            receipt.selected_batch_plan_digest(),
            receipt.execution_receipt_digest(),
        )
    });

    Ok(AdmittedBatchAdmissionPlannerRouteInput {
        receipt,
        denial_witness,
    })
}

impl AdmittedBatchAdmissionPlannerRouteInput {
    pub(crate) fn receipt(&self) -> &BatchAdmissionExecutionReceipt {
        &self.receipt
    }

    pub(crate) fn denial_witness(&self) -> Option<&BatchAdmissionPlannerRouteWitness> {
        self.denial_witness.as_ref()
    }
}
