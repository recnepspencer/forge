use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;

use super::aspect::ForgeQueryContributionComposedDeclarationAspectRecord;
use super::lower::DeclarationLowering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedDeclarationRecord {
    stage: ForgeQueryDeclarationEntryOrchestrationStage,
    declaration_digest: Option<String>,
    progression_digest: Option<String>,
    envelope_digest: Option<String>,
    route_plan_digest: Option<String>,
    aspect_record: Option<ForgeQueryContributionComposedDeclarationAspectRecord>,
}

impl ForgeQueryContributionComposedDeclarationRecord {
    pub(super) fn from_linked_artifacts(
        stage: ForgeQueryDeclarationEntryOrchestrationStage,
        linked_artifacts: &ForgeQueryBindingLinkedArtifacts,
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
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        envelope: &crate::application::ForgeQueryDeclarationEnvelope<D, I>,
        linked_artifacts: &ForgeQueryBindingLinkedArtifacts,
    ) -> Self {
        Self {
            stage: ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
            declaration_digest: linked_artifacts.declaration_digest().map(str::to_string),
            progression_digest: linked_artifacts.progression_digest().map(str::to_string),
            envelope_digest: linked_artifacts.envelope_digest().map(str::to_string),
            route_plan_digest: linked_artifacts.route_plan_digest().map(str::to_string),
            aspect_record: Some(ForgeQueryContributionComposedDeclarationAspectRecord::new(
                envelope.aspect_contract().clone(),
                envelope.aspect_publication().clone(),
            )),
        }
    }

    pub fn stage(&self) -> ForgeQueryDeclarationEntryOrchestrationStage {
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

    pub fn aspect_record(&self) -> Option<&ForgeQueryContributionComposedDeclarationAspectRecord> {
        self.aspect_record.as_ref()
    }

    pub fn aspect_contract(
        &self,
    ) -> Option<&crate::application::ForgeQueryDeclarationAspectContract> {
        self.aspect_record
            .as_ref()
            .map(ForgeQueryContributionComposedDeclarationAspectRecord::contract)
    }

    pub fn aspect_publication(
        &self,
    ) -> Option<&crate::application::ForgeQueryDeclarationAspectPublication> {
        self.aspect_record
            .as_ref()
            .map(ForgeQueryContributionComposedDeclarationAspectRecord::publication)
    }
}

pub(super) fn declaration_aspect_record_from_lowering<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    declaration: &DeclarationLowering<D, I>,
) -> ForgeQueryContributionComposedDeclarationAspectRecord {
    ForgeQueryContributionComposedDeclarationAspectRecord::new(
        declaration.envelope.aspect_contract().clone(),
        declaration.envelope.aspect_publication().clone(),
    )
}
