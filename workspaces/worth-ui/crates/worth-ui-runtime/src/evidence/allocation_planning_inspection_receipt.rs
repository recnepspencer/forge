use crate::evidence::{
    evidence_authority_binding, evidence_family_summary, evidence_handle, evidence_identity,
    evidence_ref, evidence_slice, preflight_evidence_expansion, UiAllocationPlanningCostReceipt,
    UiAllocationPlanningEvidenceDetail, UiAllocationPlanningEvidenceFamily, UiAllocationSolveTrace,
    UiEvidenceAuthorityKind, UiEvidenceExpansion, UiEvidenceFamily,
    UiEvidenceMaterializationPosture, UiEvidenceMaterializedDetail, UiEvidenceRef,
    UiEvidenceRetentionPosture, UiEvidenceSlice,
};
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiAllocationPlanningDenial, WorthUiAllocationPlanningInspection,
};
use worth_ui_inspection::{UiEvidenceExpansionOutcome, UiEvidenceRichness};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationPlanningInspectionReceipt {
    family: UiAllocationPlanningEvidenceFamily,
    planning_identity_digest: u64,
    measurement_basis_identity_digest: u64,
    neighborhood_identity_digest: u64,
    denial: Option<WorthUiAllocationPlanningDenial>,
    evidence_slice: UiEvidenceSlice,
    cost: UiAllocationPlanningCostReceipt,
    solve_trace: UiAllocationSolveTrace,
}

impl UiAllocationPlanningInspectionReceipt {
    pub fn family(&self) -> UiAllocationPlanningEvidenceFamily {
        self.family
    }

    pub fn planning_identity_digest(&self) -> u64 {
        self.planning_identity_digest
    }

    pub fn measurement_basis_identity_digest(&self) -> u64 {
        self.measurement_basis_identity_digest
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub fn denial(&self) -> Option<&WorthUiAllocationPlanningDenial> {
        self.denial.as_ref()
    }

    pub fn evidence_slice(&self) -> &UiEvidenceSlice {
        &self.evidence_slice
    }

    pub fn cost(&self) -> UiAllocationPlanningCostReceipt {
        self.cost
    }

    pub fn solve_trace(&self) -> &UiAllocationSolveTrace {
        &self.solve_trace
    }

    pub(crate) fn expand_evidence_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        requested_richness: UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        let current_generation = self.evidence_slice.authority_generation();
        if let Some(preflight) =
            preflight_evidence_expansion(current_generation, evidence_ref, requested_richness)
        {
            return preflight;
        }

        let retained_ref = self
            .evidence_slice
            .refs()
            .iter()
            .find(|retained_ref| {
                retained_ref.family() == UiEvidenceFamily::Planning
                    && retained_ref.identity() == evidence_ref.identity()
                    && retained_ref.handle() == evidence_ref.handle()
            })
            .cloned();
        let materialized_detail = if requested_richness == UiEvidenceRichness::materialized_detail()
        {
            self.evidence_slice.materialized_detail().cloned()
        } else {
            None
        };

        match retained_ref {
            Some(retained_ref) => UiEvidenceExpansion::new(
                retained_ref,
                requested_richness,
                UiEvidenceExpansionOutcome::Available,
                materialized_detail,
                Box::new([]),
                None,
            ),
            None => UiEvidenceExpansion::new(
                evidence_ref,
                requested_richness,
                UiEvidenceExpansionOutcome::Unsupported,
                None,
                Box::new([]),
                None,
            ),
        }
    }
}

pub(crate) fn project_allocation_planning_inspection_receipt(
    planning: &WorthUiAllocationPlanning,
) -> UiAllocationPlanningInspectionReceipt {
    let inspection = WorthUiAllocationPlanningInspection::from_planning(planning);
    let evidence_family = UiEvidenceFamily::Planning;
    let authority_generation = planning
        .measurement_basis()
        .declaration_support_authority_generation();
    let identity = evidence_identity(evidence_family, inspection.planning_identity_digest());
    let authority_binding = evidence_authority_binding(
        UiEvidenceAuthorityKind::AllocationPlanning,
        inspection.planning_identity_digest(),
        authority_generation,
        None,
    );
    let evidence_ref = evidence_ref(
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
    );
    let evidence_slice = evidence_slice(
        authority_generation,
        Box::new([evidence_ref]),
        Box::new([evidence_family_summary(evidence_family, 1)]),
        Some(UiEvidenceMaterializedDetail::AllocationPlanning(
            UiAllocationPlanningEvidenceDetail::from_inspection(&inspection),
        )),
        None,
    );
    UiAllocationPlanningInspectionReceipt {
        family: UiAllocationPlanningEvidenceFamily::Planning,
        planning_identity_digest: inspection.planning_identity_digest(),
        measurement_basis_identity_digest: inspection.measurement_basis_identity_digest(),
        neighborhood_identity_digest: inspection.neighborhood_identity_digest(),
        denial: inspection.denial().cloned(),
        evidence_slice,
        cost: UiAllocationPlanningCostReceipt::new(planning, &inspection),
        solve_trace: inspection.solve_trace().clone(),
    }
}
