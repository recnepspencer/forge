use super::receipt::UiAllocationPlanningInspectionReceipt;
use crate::evidence::construction::preflight_evidence_expansion;
use crate::evidence::shared::evidence_expansion::UiEvidenceExpansion;
use crate::evidence::shared::evidence_reference::UiEvidenceRef;
use worth_ui_inspection::{UiEvidenceExpansionOutcome, UiEvidenceFamily, UiEvidenceRichness};

impl UiAllocationPlanningInspectionReceipt {
    pub(crate) fn expand_evidence_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        requested_richness: UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        let current_generation = self.evidence_slice().authority_generation();
        if let Some(preflight) =
            preflight_evidence_expansion(current_generation, evidence_ref, requested_richness)
        {
            return preflight;
        }

        let retained_ref = locate_retained_planning_ref(self, evidence_ref);
        let materialized_detail = materialized_detail_for_richness(self, requested_richness);

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

fn locate_retained_planning_ref(
    receipt: &UiAllocationPlanningInspectionReceipt,
    evidence_ref: UiEvidenceRef,
) -> Option<UiEvidenceRef> {
    receipt
        .evidence_slice()
        .refs()
        .iter()
        .find(|retained_ref| {
            retained_ref.family() == UiEvidenceFamily::Planning
                && retained_ref.identity() == evidence_ref.identity()
                && retained_ref.handle() == evidence_ref.handle()
        })
        .cloned()
}

fn materialized_detail_for_richness(
    receipt: &UiAllocationPlanningInspectionReceipt,
    requested_richness: UiEvidenceRichness,
) -> Option<crate::evidence::shared::evidence_materialized_detail::UiEvidenceMaterializedDetail> {
    if requested_richness == UiEvidenceRichness::materialized_detail() {
        receipt.evidence_slice().materialized_detail().cloned()
    } else {
        None
    }
}
