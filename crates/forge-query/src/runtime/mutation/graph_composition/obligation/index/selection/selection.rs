use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use crate::runtime::{
    ForgeQueryGraphObligationExecutionBudget, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphTouchDescriptor,
};

use super::operating_world_descriptor::ForgeQueryGraphObligationOperatingWorldDescriptor;
use super::selection_counters::ForgeQueryGraphObligationSelectionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationSelection {
    index_digest: String,
    touch_descriptor_digest: String,
    operating_world_digest: String,
    matched_registrations: Vec<ForgeQueryGraphObligationRegistration>,
    counters: ForgeQueryGraphObligationSelectionCounters,
    selection_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationSelection {
    pub(super) fn new(
        index_digest: &str,
        touch_descriptor: &ForgeQueryGraphTouchDescriptor,
        operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
        matched_registrations: Vec<ForgeQueryGraphObligationRegistration>,
        counters: ForgeQueryGraphObligationSelectionCounters,
    ) -> Self {
        let registration_digests = matched_registrations
            .iter()
            .map(ForgeQueryGraphObligationRegistration::registration_evidence_digest)
            .collect::<Vec<_>>();
        let selection_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationSelection)
                .field_value(ForgeQueryEvidenceTag::new("index"), index_digest)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("touch_descriptor"),
                    touch_descriptor.descriptor_evidence_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("operating_world"),
                    operating_world.descriptor_evidence_digest(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("counters"),
                    counters.counters_evidence_digest(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("matched_registration"),
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

    pub fn matched_registrations(&self) -> &[ForgeQueryGraphObligationRegistration] {
        &self.matched_registrations
    }

    pub fn matched_support_postures(
        &self,
    ) -> impl Iterator<Item = &ForgeQueryGraphObligationSupportPosture> {
        self.matched_registrations
            .iter()
            .map(ForgeQueryGraphObligationRegistration::support_posture)
    }

    pub fn matched_execution_budgets(
        &self,
    ) -> impl Iterator<Item = &ForgeQueryGraphObligationExecutionBudget> {
        self.matched_registrations
            .iter()
            .map(ForgeQueryGraphObligationRegistration::execution_budget)
    }

    pub fn matched_obligation_count(&self) -> usize {
        self.matched_registrations.len()
    }

    pub fn counters(&self) -> &ForgeQueryGraphObligationSelectionCounters {
        &self.counters
    }

    pub fn selection_digest(&self) -> &str {
        self.selection_digest.as_str()
    }
}
