use std::marker::PhantomData;

use crate::application::{
    WorthQueryAdmittedWorldBasis, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectCoverageBasis,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::exposure::WorthQueryDeclarationEntryOrchestrationExposureLevel;
use super::policy::WorthQueryDeclarationEntryOrchestrationArtifactPolicy;

pub struct WorthQueryDeclarationEntryOrchestrationInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration_family_key: &'static str,
    world_basis: WorthQueryAdmittedWorldBasis,
    aspect_contract: WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel,
    artifact_policy: WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
    _marker: PhantomData<(D, I)>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEntryOrchestrationInput<D, I>
{
    pub(crate) fn new(
        world_basis: WorthQueryAdmittedWorldBasis,
        aspect_contract: WorthQueryDeclarationAspectContract,
        aspect_coverage: WorthQueryDeclarationAspectCoverage,
        aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
        exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel,
        artifact_policy: WorthQueryDeclarationEntryOrchestrationArtifactPolicy,
    ) -> Self {
        Self {
            declaration_family_key: I::Family::semantic_family_key(),
            world_basis,
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
        self.world_basis.handle_identity_for_reporting()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.world_basis.operating_context_identity_digest()
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn exposure_level(&self) -> WorthQueryDeclarationEntryOrchestrationExposureLevel {
        self.exposure_level
    }

    pub fn artifact_policy(&self) -> WorthQueryDeclarationEntryOrchestrationArtifactPolicy {
        self.artifact_policy
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> Clone
    for WorthQueryDeclarationEntryOrchestrationInput<D, I>
{
    fn clone(&self) -> Self {
        Self {
            declaration_family_key: self.declaration_family_key,
            world_basis: self.world_basis.clone(),
            aspect_contract: self.aspect_contract.clone(),
            aspect_coverage: self.aspect_coverage.clone(),
            aspect_coverage_basis: self.aspect_coverage_basis,
            exposure_level: self.exposure_level,
            artifact_policy: self.artifact_policy,
            _marker: PhantomData,
        }
    }
}
