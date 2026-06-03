use forge_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundarySurfaceDispositionLegality,
};

use crate::application::{
    ForgeQueryAdmittedWorldBasis, ForgeQueryCanonicalDeclarationArtifact,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationFamilySupportReport, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

use super::contract::ForgeQueryDeclarationLegalityContract;

pub struct ForgeQueryDeclarationLegalityEvidence<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
    support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
    legality_contract: ForgeQueryDeclarationLegalityContract,
    reviewed_aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    world_basis: ForgeQueryAdmittedWorldBasis,
    role_claim_category: FoundationalBoundaryArtifactCategory,
    role_claim_role: FoundationalBoundaryArtifactRole,
    surface_disposition: FoundationalBoundarySurfaceDispositionLegality,
    legality_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationLegalityEvidence<D, I>
{
    pub(crate) fn new(
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
        reviewed_aspect_coverage: ForgeQueryDeclarationAspectCoverage,
        world_basis: ForgeQueryAdmittedWorldBasis,
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

    pub fn canonical_declaration(&self) -> &ForgeQueryCanonicalDeclarationArtifact<D, I> {
        &self.declaration
    }

    pub fn support_report(&self) -> &ForgeQueryDeclarationFamilySupportReport<D, I::Family> {
        &self.support_report
    }

    pub fn legality_contract(&self) -> ForgeQueryDeclarationLegalityContract {
        self.legality_contract
    }

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        self.support_report.aspect_contract()
    }

    pub fn reviewed_aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.reviewed_aspect_coverage
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration.declaration_family_key()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.world_basis.operating_context_identity_digest()
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

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> Clone
    for ForgeQueryDeclarationLegalityEvidence<D, I>
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
