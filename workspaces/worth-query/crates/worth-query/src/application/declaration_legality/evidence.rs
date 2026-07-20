use worth_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundarySurfaceDispositionLegality,
};

use crate::application::{
    WorthQueryAdmittedWorldBasis, WorthQueryCanonicalDeclarationArtifact,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationFamilySupportReport, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker,
};

use super::contract::WorthQueryDeclarationLegalityContract;

pub struct WorthQueryDeclarationLegalityEvidence<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration: WorthQueryCanonicalDeclarationArtifact<D, I>,
    support_report: WorthQueryDeclarationFamilySupportReport<D, I::Family>,
    legality_contract: WorthQueryDeclarationLegalityContract,
    reviewed_aspect_coverage: WorthQueryDeclarationAspectCoverage,
    world_basis: WorthQueryAdmittedWorldBasis,
    role_claim_category: FoundationalBoundaryArtifactCategory,
    role_claim_role: FoundationalBoundaryArtifactRole,
    surface_disposition: FoundationalBoundarySurfaceDispositionLegality,
    legality_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationLegalityEvidence<D, I>
{
    pub(crate) fn new(
        declaration: WorthQueryCanonicalDeclarationArtifact<D, I>,
        support_report: WorthQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: WorthQueryDeclarationLegalityContract,
        reviewed_aspect_coverage: WorthQueryDeclarationAspectCoverage,
        world_basis: WorthQueryAdmittedWorldBasis,
        role_claim_category: FoundationalBoundaryArtifactCategory,
        role_claim_role: FoundationalBoundaryArtifactRole,
        surface_disposition: FoundationalBoundarySurfaceDispositionLegality,
        legality_digest: String,
    ) -> Self {
        Self {
            declaration,
            support_report,
            legality_contract,
            reviewed_aspect_coverage,
            world_basis,
            role_claim_category,
            role_claim_role,
            surface_disposition,
            legality_digest,
        }
    }

    pub fn canonical_declaration(&self) -> &WorthQueryCanonicalDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn support_report(&self) -> &WorthQueryDeclarationFamilySupportReport<D, I::Family> {
        &self.support_report
    }

    pub fn legality_contract(&self) -> WorthQueryDeclarationLegalityContract {
        self.legality_contract
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        self.support_report.aspect_contract()
    }

    pub fn reviewed_aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.reviewed_aspect_coverage
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration.declaration_family_key()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.world_basis.operating_context_identity_digest()
    }

    pub(crate) fn world_basis(&self) -> &WorthQueryAdmittedWorldBasis {
        &self.world_basis
    }

    pub fn role_claim_category(&self) -> FoundationalBoundaryArtifactCategory {
        self.role_claim_category
    }

    pub fn role_claim_role(&self) -> FoundationalBoundaryArtifactRole {
        self.role_claim_role
    }

    pub fn surface_disposition(&self) -> FoundationalBoundarySurfaceDispositionLegality {
        self.surface_disposition
    }

    pub fn legality_digest(&self) -> &str {
        &self.legality_digest
    }

    pub fn is_structurally_legal(&self) -> bool {
        true
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> Clone
    for WorthQueryDeclarationLegalityEvidence<D, I>
{
    fn clone(&self) -> Self {
        Self {
            declaration: self.declaration.clone(),
            support_report: self.support_report.clone(),
            legality_contract: self.legality_contract,
            reviewed_aspect_coverage: self.reviewed_aspect_coverage.clone(),
            world_basis: self.world_basis.clone(),
            role_claim_category: self.role_claim_category,
            role_claim_role: self.role_claim_role,
            surface_disposition: self.surface_disposition,
            legality_digest: self.legality_digest.clone(),
        }
    }
}
