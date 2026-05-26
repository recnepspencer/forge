use std::marker::PhantomData;

use crate::application::{
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::exposure::ForgeQueryDeclarationEntryOrchestrationExposureLevel;
use super::policy::ForgeQueryDeclarationEntryOrchestrationArtifactPolicy;

pub struct ForgeQueryDeclarationEntryOrchestrationInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    artifact_policy: ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    _marker: PhantomData<(D, I)>,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEntryOrchestrationInput<D, I>
{
    pub(crate) fn new(
        handle_identity_digest: &str,
        operating_context_identity_digest: &str,
        exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
        artifact_policy: ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ) -> Self {
        Self {
            declaration_family_key: I::Family::semantic_family_key(),
            handle_identity_digest: handle_identity_digest.to_string(),
            operating_context_identity_digest: operating_context_identity_digest.to_string(),
            exposure_level,
            artifact_policy,
            _marker: PhantomData,
        }
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn exposure_level(&self) -> ForgeQueryDeclarationEntryOrchestrationExposureLevel {
        self.exposure_level
    }

    pub fn artifact_policy(&self) -> ForgeQueryDeclarationEntryOrchestrationArtifactPolicy {
        self.artifact_policy
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> Clone
    for ForgeQueryDeclarationEntryOrchestrationInput<D, I>
{
    fn clone(&self) -> Self {
        Self {
            declaration_family_key: self.declaration_family_key,
            handle_identity_digest: self.handle_identity_digest.clone(),
            operating_context_identity_digest: self.operating_context_identity_digest.clone(),
            exposure_level: self.exposure_level,
            artifact_policy: self.artifact_policy,
            _marker: PhantomData,
        }
    }
}
