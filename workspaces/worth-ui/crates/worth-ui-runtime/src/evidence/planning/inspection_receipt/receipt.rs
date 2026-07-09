use super::cost_receipt::UiAllocationPlanningCostReceipt;
use super::evidence_detail::UiAllocationPlanningEvidenceDetail;
use super::evidence_family::UiAllocationPlanningEvidenceFamily;
use crate::evidence::construction::{
    evidence_authority_binding, evidence_family_summary, evidence_handle, evidence_identity,
    evidence_ref, evidence_slice,
};
use crate::evidence::planning::allocation_solve::UiAllocationSolveTrace;
use crate::evidence::shared::evidence_materialized_detail::UiEvidenceMaterializedDetail;
use crate::evidence::shared::evidence_reference::UiEvidenceRef;
use crate::evidence::shared::evidence_slice::UiEvidenceSlice;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningDenial, WorthUiAllocationPlanningInspection,
};
use worth_ui_inspection::{
    UiEvidenceAuthorityKind, UiEvidenceFamily, UiEvidenceMaterializationPosture,
    UiEvidenceRetentionPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationPlanningInspectionReceipt {
    family: UiAllocationPlanningEvidenceFamily,
    evidence_slice: UiEvidenceSlice,
    cost: UiAllocationPlanningCostReceipt,
}

impl UiAllocationPlanningInspectionReceipt {
    pub fn family(&self) -> UiAllocationPlanningEvidenceFamily {
        self.family
    }

    pub fn planning_identity_digest(&self) -> u64 {
        self.solve_trace().planning_identity_digest()
    }

    pub fn measurement_basis_identity_digest(&self) -> u64 {
        self.allocation_planning_detail()
            .neighborhood()
            .identity()
            .measurement_basis_identity_digest()
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.allocation_planning_detail()
            .neighborhood()
            .identity()
            .identity_digest()
    }

    pub fn denial(&self) -> Option<&WorthUiAllocationPlanningDenial> {
        self.allocation_planning_detail().denial()
    }

    pub fn evidence_slice(&self) -> &UiEvidenceSlice {
        &self.evidence_slice
    }

    pub fn cost(&self) -> UiAllocationPlanningCostReceipt {
        self.cost.clone()
    }

    pub fn solve_trace(&self) -> &UiAllocationSolveTrace {
        self.allocation_planning_detail().solve_trace()
    }

    pub(crate) fn allocation_planning_detail(&self) -> &UiAllocationPlanningEvidenceDetail {
        match self.evidence_slice.materialized_detail() {
            Some(UiEvidenceMaterializedDetail::AllocationPlanning(detail)) => detail,
            _ => panic!("planning inspection receipt must retain allocation-planning detail"),
        }
    }
}

pub(crate) fn project_allocation_planning_inspection_receipt(
    planning: &WorthUiAllocationPlanning,
) -> UiAllocationPlanningInspectionReceipt {
    let inspection = collect_planning_inspection(planning);
    let authority_generation = planning_authority_generation(planning);
    let evidence_ref = construct_planning_evidence_ref(&inspection, authority_generation);
    let evidence_slice =
        construct_planning_evidence_slice(evidence_ref, &inspection, authority_generation);
    assemble_planning_inspection_receipt(evidence_slice, planning, &inspection)
}

fn collect_planning_inspection(
    planning: &WorthUiAllocationPlanning,
) -> WorthUiAllocationPlanningInspection {
    WorthUiAllocationPlanningInspection::from_planning(planning)
}

fn planning_authority_generation(
    planning: &WorthUiAllocationPlanning,
) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
    planning
        .measurement_basis()
        .declaration_support_authority_generation()
}

fn construct_planning_evidence_ref(
    inspection: &WorthUiAllocationPlanningInspection,
    authority_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
) -> UiEvidenceRef {
    let evidence_family = UiEvidenceFamily::Planning;
    let identity = evidence_identity(evidence_family, inspection.planning_identity_digest());
    let authority_binding = evidence_authority_binding(
        UiEvidenceAuthorityKind::AllocationPlanning,
        inspection.planning_identity_digest(),
        authority_generation,
        None,
    );
    evidence_ref(
        evidence_family,
        identity,
        authority_binding,
        UiEvidenceMaterializationPosture::DetailAvailable,
        UiEvidenceRetentionPosture::CurrentGenerationOnly,
        evidence_handle(
            evidence_family,
            identity,
            inspection.planning_identity_digest(),
        ),
    )
}

fn construct_planning_evidence_slice(
    evidence_ref: UiEvidenceRef,
    inspection: &WorthUiAllocationPlanningInspection,
    authority_generation: worth_ui_inspection::UiEvidenceAuthorityGeneration,
) -> UiEvidenceSlice {
    let evidence_family = UiEvidenceFamily::Planning;
    evidence_slice(
        authority_generation,
        Box::new([evidence_ref]),
        Box::new([evidence_family_summary(evidence_family, 1)]),
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(
            UiAllocationPlanningEvidenceDetail::from_inspection(inspection),
        )),
        None,
    )
}

fn assemble_planning_inspection_receipt(
    evidence_slice: UiEvidenceSlice,
    planning: &WorthUiAllocationPlanning,
    inspection: &WorthUiAllocationPlanningInspection,
) -> UiAllocationPlanningInspectionReceipt {
    UiAllocationPlanningInspectionReceipt {
        family: UiAllocationPlanningEvidenceFamily::Planning,
        evidence_slice,
        cost: UiAllocationPlanningCostReceipt::new(planning, inspection),
    }
}
