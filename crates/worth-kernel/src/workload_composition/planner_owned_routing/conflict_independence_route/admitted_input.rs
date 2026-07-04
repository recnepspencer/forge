use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictIndependencePlannerRouteWitness, ConflictIndependencePlannerRouteWitnessKind,
};

use super::family_catalog::ConflictIndependencePlannerRouteFamilyCatalog;
use crate::workload_composition::{
    planner_owned_routing::{PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind},
    BatchAdmissionExecutionReceipt, BatchAdmissionPlanDenialKind,
};

#[derive(Clone, Debug)]
pub(crate) struct AdmittedConflictIndependencePlannerRouteInput {
    family_catalog: ConflictIndependencePlannerRouteFamilyCatalog,
    receipt: BatchAdmissionExecutionReceipt,
    denial_witness: Option<ConflictIndependencePlannerRouteWitness>,
}

pub(crate) fn admit_conflict_independence_planner_route_input(
    family_catalog: ConflictIndependencePlannerRouteFamilyCatalog,
    receipt: BatchAdmissionExecutionReceipt,
) -> Result<AdmittedConflictIndependencePlannerRouteInput, PlannerOwnedRoutingError> {
    if receipt.selected_batch_plan_digest().is_empty()
        || receipt.execution_receipt_digest().is_empty()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
            "planner-owned conflict/independence route requires selected batch and execution receipt identity",
        ));
    }
    if receipt.selected_conflict_plan_digests().is_empty() && receipt.denial().is_none() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
            "planner-owned conflict/independence route requires selected conflict-plan authority or a typed denial witness",
        ));
    }

    let denial_witness = receipt.denial().map(|denial| {
        let kind = match denial.kind() {
            BatchAdmissionPlanDenialKind::SelectedPlanDenied => {
                ConflictIndependencePlannerRouteWitnessKind::ConflictRouteDenial
            }
            BatchAdmissionPlanDenialKind::MissingExplicitIndependenceProof
            | BatchAdmissionPlanDenialKind::DeclaredDenied => {
                ConflictIndependencePlannerRouteWitnessKind::IndependenceDenial
            }
        };
        ConflictIndependencePlannerRouteWitness::new(
            kind,
            receipt.selected_batch_plan_digest(),
            receipt.execution_receipt_digest(),
        )
    });

    Ok(AdmittedConflictIndependencePlannerRouteInput {
        family_catalog,
        receipt,
        denial_witness,
    })
}

impl AdmittedConflictIndependencePlannerRouteInput {
    pub(crate) const fn family_catalog(&self) -> ConflictIndependencePlannerRouteFamilyCatalog {
        self.family_catalog
    }

    pub(crate) fn receipt(&self) -> &BatchAdmissionExecutionReceipt {
        &self.receipt
    }

    pub(crate) fn denial_witness(&self) -> Option<&ConflictIndependencePlannerRouteWitness> {
        self.denial_witness.as_ref()
    }
}
