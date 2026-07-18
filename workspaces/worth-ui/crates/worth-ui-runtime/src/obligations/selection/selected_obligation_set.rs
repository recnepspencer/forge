use std::cell::Cell;

use crate::admission::UiSupportSnapshot;
use crate::declaration::stable_text_digest;
use crate::declaration::UiDeclarationIdentity;
use crate::evidence::{
    preflight_evidence_expansion, UiAllocationNeighborhood, UiEvidenceAuthorityGeneration,
    UiEvidenceExpansion, UiEvidenceExpansionOutcome, UiEvidenceMaterializedDetail, UiEvidenceRef,
    UiMeasurementBasis,
};
use crate::graph::{UiAllocationNeighborhoodDenial, UiGraphSnapshot};
use crate::obligations::closeout::UiObligationSelectionHandoff;
use crate::obligations::inspection::{UiObligationEvidenceIndex, UiObligationEvidenceQuery};
use crate::obligations::touch::UiGraphTouchDescriptor;
use worth_ui_inspection::{UiInspectionQuery, UiInspectionRelevanceOutcome};

use crate::facade::foreign_evidence_refs_for_obligation_record;
use crate::facade::inspection_bridge::UiInspectionReceipt;

use super::UiSelectedObligation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObligationInspectionObservation {
    rich_artifact_materialization_count: u64,
}

impl UiObligationInspectionObservation {
    const fn new(rich_artifact_materialization_count: u64) -> Self {
        Self {
            rich_artifact_materialization_count,
        }
    }

    pub fn rich_artifact_materialization_count(self) -> u64 {
        self.rich_artifact_materialization_count
    }
}

#[derive(Clone, Debug)]
struct UiObligationInspectionObservationState {
    rich_artifact_materialization_count: Cell<u64>,
}

impl UiObligationInspectionObservationState {
    const fn new() -> Self {
        Self {
            rich_artifact_materialization_count: Cell::new(0),
        }
    }

    fn snapshot(&self) -> UiObligationInspectionObservation {
        UiObligationInspectionObservation::new(self.rich_artifact_materialization_count.get())
    }

    fn record_rich_artifact_materialization(&self) {
        self.rich_artifact_materialization_count
            .set(self.rich_artifact_materialization_count.get() + 1);
    }
}

#[derive(Clone, Debug)]
pub struct UiSelectedObligationSet {
    identity_digest: u64,
    authority_generation: UiEvidenceAuthorityGeneration,
    touch: UiGraphTouchDescriptor,
    support_snapshot: UiSupportSnapshot,
    requested_target: crate::admission::UiAdmissionTarget,
    selected_declaration_identity: Option<UiDeclarationIdentity>,
    obligations: Box<[UiSelectedObligation]>,
    evidence_index: UiObligationEvidenceIndex,
    inspection_observation: UiObligationInspectionObservationState,
}

impl PartialEq for UiSelectedObligationSet {
    fn eq(&self, other: &Self) -> bool {
        self.identity_digest == other.identity_digest
            && self.authority_generation == other.authority_generation
            && self.touch == other.touch
            && self.support_snapshot == other.support_snapshot
            && self.requested_target == other.requested_target
            && self.selected_declaration_identity == other.selected_declaration_identity
            && self.obligations == other.obligations
            && self.evidence_index == other.evidence_index
    }
}

impl Eq for UiSelectedObligationSet {}

impl UiSelectedObligationSet {
    pub(crate) fn new(
        authority_generation: UiEvidenceAuthorityGeneration,
        touch: UiGraphTouchDescriptor,
        support_snapshot: UiSupportSnapshot,
        requested_target: crate::admission::UiAdmissionTarget,
        selected_declaration_identity: Option<UiDeclarationIdentity>,
        obligations: Box<[UiSelectedObligation]>,
        evidence_index: UiObligationEvidenceIndex,
    ) -> Self {
        let identity_digest = selected_obligation_set_identity_digest(&touch, &support_snapshot);
        Self {
            identity_digest,
            authority_generation,
            touch,
            support_snapshot,
            requested_target,
            selected_declaration_identity,
            obligations,
            evidence_index,
            inspection_observation: UiObligationInspectionObservationState::new(),
        }
    }

    pub(crate) fn identity_digest_for(
        touch: &UiGraphTouchDescriptor,
        support_snapshot: &UiSupportSnapshot,
    ) -> u64 {
        selected_obligation_set_identity_digest(touch, support_snapshot)
    }

    pub(crate) fn with_evidence_index(mut self, evidence_index: UiObligationEvidenceIndex) -> Self {
        self.evidence_index = evidence_index;
        self
    }

    pub fn touch(&self) -> &UiGraphTouchDescriptor {
        &self.touch
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    pub fn authority_generation(&self) -> UiEvidenceAuthorityGeneration {
        self.authority_generation
    }

    pub fn support_snapshot(&self) -> &UiSupportSnapshot {
        &self.support_snapshot
    }

    pub fn requested_target(&self) -> &crate::admission::UiAdmissionTarget {
        &self.requested_target
    }

    pub fn selected_declaration_identity(&self) -> Option<&UiDeclarationIdentity> {
        self.selected_declaration_identity.as_ref()
    }

    pub fn obligations(&self) -> &[UiSelectedObligation] {
        &self.obligations
    }

    pub fn obligation_for_family(
        &self,
        family: crate::obligations::catalog::UiObligationFamily,
    ) -> Option<&UiSelectedObligation> {
        self.obligations
            .iter()
            .find(|obligation| obligation.family() == family)
    }

    pub fn evidence_index(&self) -> &UiObligationEvidenceIndex {
        &self.evidence_index
    }

    pub fn inspection_observation(&self) -> UiObligationInspectionObservation {
        self.inspection_observation.snapshot()
    }

    pub fn selected_obligation_handles(
        &self,
    ) -> Box<[crate::obligations::inspection::UiObligationEvidenceHandle]> {
        self.obligations
            .iter()
            .map(UiSelectedObligation::evidence_handle)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn handoff(&self) -> UiObligationSelectionHandoff<'_> {
        UiObligationSelectionHandoff::new(self)
    }

    pub(crate) fn admit_allocation_neighborhood(
        &self,
        snapshot: &UiGraphSnapshot,
        measurement_basis: &UiMeasurementBasis,
    ) -> Result<UiAllocationNeighborhood, UiAllocationNeighborhoodDenial> {
        measurement_basis.admit_allocation_neighborhood(snapshot, self)
    }

    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        let relevance_admission = query.admit_relevance();
        if !matches!(
            relevance_admission.outcome(),
            UiInspectionRelevanceOutcome::Matched
        ) {
            return UiInspectionReceipt::from_relevance_admission(
                query,
                relevance_admission,
                Some(self.authority_generation),
            );
        }
        UiInspectionReceipt::from_obligation(
            query.clone(),
            relevance_admission,
            self.authority_generation,
            self.evidence_index.inspect(
                &UiObligationEvidenceQuery::from_inspection_query(&query),
                self.authority_generation,
            ),
        )
    }

    pub fn expand_evidence_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        requested_richness: worth_ui_inspection::UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        if let Some(preflight) = preflight_evidence_expansion(
            self.authority_generation,
            evidence_ref,
            requested_richness,
        ) {
            return preflight;
        }

        if evidence_ref.family() != worth_ui_inspection::UiEvidenceFamily::Obligation {
            return UiEvidenceExpansion::new(
                evidence_ref,
                requested_richness,
                UiEvidenceExpansionOutcome::Unsupported,
                None,
                Box::new([]),
                None,
            );
        }

        let Some(record) = self
            .evidence_index
            .records()
            .iter()
            .find(|record| record.evidence_ref(self.authority_generation) == evidence_ref)
        else {
            return UiEvidenceExpansion::new(
                evidence_ref,
                requested_richness,
                UiEvidenceExpansionOutcome::Unsupported,
                None,
                Box::new([]),
                None,
            );
        };

        self.inspection_observation
            .record_rich_artifact_materialization();
        let detail = UiEvidenceMaterializedDetail::Obligation(
            self.evidence_index.inspect(
                &UiObligationEvidenceQuery::new()
                    .for_handle_digest(evidence_ref.handle().handle_digest()),
                self.authority_generation,
            ),
        );
        let foreign_evidence_refs = foreign_evidence_refs_for_obligation_record(record);

        UiEvidenceExpansion::new(
            evidence_ref,
            requested_richness,
            UiEvidenceExpansionOutcome::Available,
            Some(detail),
            foreign_evidence_refs,
            None,
        )
    }
}

fn selected_obligation_set_identity_digest(
    touch: &UiGraphTouchDescriptor,
    support_snapshot: &UiSupportSnapshot,
) -> u64 {
    stable_text_digest("selected-obligation-set")
        ^ touch.identity_digest().rotate_left(7)
        ^ touch
            .target()
            .graph_node_identity()
            .digest()
            .rotate_left(19)
        ^ stable_text_digest(&format!("{:?}", support_snapshot.posture())).rotate_left(31)
}
