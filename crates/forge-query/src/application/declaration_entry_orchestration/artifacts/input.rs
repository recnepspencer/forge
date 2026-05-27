use std::marker::PhantomData;

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
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
    aspect_contract: ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
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
        aspect_contract: ForgeQueryDeclarationAspectContract,
        aspect_coverage: ForgeQueryDeclarationAspectCoverage,
        aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
        exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel,
        artifact_policy: ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ) -> Self {
        Self {
            declaration_family_key: I::Family::semantic_family_key(),
            handle_identity_digest: handle_identity_digest.to_string(),
            operating_context_identity_digest: operating_context_identity_digest.to_string(),
            aspect_contract,
            aspect_coverage,
            aspect_coverage_basis,
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

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
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
            aspect_contract: self.aspect_contract.clone(),
            aspect_coverage: self.aspect_coverage.clone(),
            aspect_coverage_basis: self.aspect_coverage_basis,
            exposure_level: self.exposure_level,
            artifact_policy: self.artifact_policy,
            _marker: PhantomData,
        }
    }
}
