use crate::application::{
    WorthQueryDeclarationEntryOrchestrationStage, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};
use crate::binding_pipeline::WorthQueryBindingLinkedArtifacts;

use super::aspect::WorthQueryContributionComposedDeclarationAspectRecord;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedDeclarationRecord {
    stage: WorthQueryDeclarationEntryOrchestrationStage,
    declaration_digest: Option<String>,
    progression_digest: Option<String>,
    envelope_digest: Option<String>,
    route_plan_digest: Option<String>,
    aspect_record: Option<WorthQueryContributionComposedDeclarationAspectRecord>,
}

impl WorthQueryContributionComposedDeclarationRecord {
    pub(super) fn from_linked_artifacts(
        stage: WorthQueryDeclarationEntryOrchestrationStage,
        linked_artifacts: &WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            stage,
            declaration_digest: linked_artifacts.declaration_digest().map(str::to_string),
            progression_digest: linked_artifacts.progression_digest().map(str::to_string),
            envelope_digest: linked_artifacts.envelope_digest().map(str::to_string),
            route_plan_digest: linked_artifacts.route_plan_digest().map(str::to_string),
            aspect_record: None,
        }
    }

    pub(super) fn from_envelope<
        D: WorthQueryDomainEntryMarker,
        I: WorthQueryDeclarationInput<D>,
    >(
        envelope: &crate::application::WorthQueryDeclarationEnvelope<D, I>,
        linked_artifacts: &WorthQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            stage: WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            declaration_digest: linked_artifacts.declaration_digest().map(str::to_string),
            progression_digest: linked_artifacts.progression_digest().map(str::to_string),
            envelope_digest: linked_artifacts.envelope_digest().map(str::to_string),
            route_plan_digest: linked_artifacts.route_plan_digest().map(str::to_string),
            aspect_record: Some(WorthQueryContributionComposedDeclarationAspectRecord::new(
                envelope.aspect_contract().clone(),
                envelope.aspect_publication().clone(),
            )),
        }
    }

    pub fn stage(&self) -> WorthQueryDeclarationEntryOrchestrationStage {
        self.stage
    }

    pub fn declaration_digest(&self) -> Option<&str> {
        self.declaration_digest.as_deref()
    }

    pub fn progression_digest(&self) -> Option<&str> {
        self.progression_digest.as_deref()
    }

    pub fn envelope_digest(&self) -> Option<&str> {
        self.envelope_digest.as_deref()
    }

    pub fn route_plan_digest(&self) -> Option<&str> {
        self.route_plan_digest.as_deref()
    }

    pub fn aspect_record(&self) -> Option<&WorthQueryContributionComposedDeclarationAspectRecord> {
        self.aspect_record.as_ref()
    }

    pub fn aspect_contract(
        &self,
    ) -> Option<&crate::application::WorthQueryDeclarationAspectContract> {
        self.aspect_record
            .as_ref()
            .map(WorthQueryContributionComposedDeclarationAspectRecord::contract)
    }

    pub fn aspect_publication(
        &self,
    ) -> Option<&crate::application::WorthQueryDeclarationAspectPublication> {
        self.aspect_record
            .as_ref()
            .map(WorthQueryContributionComposedDeclarationAspectRecord::publication)
    }
}
