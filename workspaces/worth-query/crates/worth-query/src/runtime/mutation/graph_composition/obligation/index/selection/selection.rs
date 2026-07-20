use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use crate::runtime::{
    WorthQueryGraphObligationExecutionBudget, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationSupportPosture, WorthQueryGraphTouchDescriptor,
};

use super::operating_world_descriptor::WorthQueryGraphObligationOperatingWorldDescriptor;
use super::selection_counters::WorthQueryGraphObligationSelectionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSelection {
    index_digest: String,
    touch_descriptor_digest: String,
    operating_world_digest: String,
    matched_registrations: Vec<WorthQueryGraphObligationRegistration>,
    counters: WorthQueryGraphObligationSelectionCounters,
    selection_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationSelection {
    pub(super) fn new(
        index_digest: &str,
        touch_descriptor: &WorthQueryGraphTouchDescriptor,
        operating_world: &WorthQueryGraphObligationOperatingWorldDescriptor,
        matched_registrations: Vec<WorthQueryGraphObligationRegistration>,
        counters: WorthQueryGraphObligationSelectionCounters,
    ) -> Self {
        let registration_digests = matched_registrations
            .iter()
            .map(WorthQueryGraphObligationRegistration::registration_evidence_digest)
            .collect::<Vec<_>>();
        let selection_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationSelection)
                .field_value(WorthQueryEvidenceTag::new("index"), index_digest)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("touch_descriptor"),
                    touch_descriptor.descriptor_evidence_digest(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("operating_world"),
                    operating_world.descriptor_evidence_digest(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("counters"),
                    counters.counters_evidence_digest(),
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("matched_registration"),
                    registration_digests,
                )
                .seal();
        Self {
            index_digest: index_digest.to_string(),
            touch_descriptor_digest: touch_descriptor.descriptor_digest().to_string(),
            operating_world_digest: operating_world.descriptor_digest().to_string(),
            matched_registrations,
            counters,
            selection_digest,
        }
    }

    pub fn index_digest(&self) -> &str {
        &self.index_digest
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        &self.touch_descriptor_digest
    }

    pub fn operating_world_digest(&self) -> &str {
        &self.operating_world_digest
    }

    pub fn matched_registrations(&self) -> &[WorthQueryGraphObligationRegistration] {
        &self.matched_registrations
    }

    pub fn matched_support_postures(
        &self,
    ) -> impl Iterator<Item = &WorthQueryGraphObligationSupportPosture> {
        self.matched_registrations
            .iter()
            .map(WorthQueryGraphObligationRegistration::support_posture)
    }

    pub fn matched_execution_budgets(
        &self,
    ) -> impl Iterator<Item = &WorthQueryGraphObligationExecutionBudget> {
        self.matched_registrations
            .iter()
            .map(WorthQueryGraphObligationRegistration::execution_budget)
    }

    pub fn matched_obligation_count(&self) -> usize {
        self.matched_registrations.len()
    }

    pub fn counters(&self) -> &WorthQueryGraphObligationSelectionCounters {
        &self.counters
    }

    pub fn selection_digest(&self) -> &str {
        self.selection_digest.as_str()
    }
}
