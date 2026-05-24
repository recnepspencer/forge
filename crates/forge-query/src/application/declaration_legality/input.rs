use crate::application::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationFamilySupportReport,
    ForgeQueryDeclarationFamilyTaxonomy, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::contract::ForgeQueryDeclarationLegalityContract;

pub struct ForgeQueryDeclarationLegalityInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
    support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
    legality_contract: ForgeQueryDeclarationLegalityContract,
    operating_context_identity_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationLegalityInput<D, I>
{
    pub(crate) fn new(
        declaration: ForgeQueryCanonicalDeclarationArtifact<D, I>,
        support_report: ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: ForgeQueryDeclarationLegalityContract,
        operating_context_identity_digest: String,
    ) -> Self {
        Self {
            declaration,
            support_report,
            legality_contract,
            operating_context_identity_digest,
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

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration.declaration_family_key()
    }

    pub fn declaration_taxonomy(&self) -> ForgeQueryDeclarationFamilyTaxonomy {
        self.declaration.declaration_taxonomy()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.declaration.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ForgeQueryCanonicalDeclarationArtifact<D, I>,
        ForgeQueryDeclarationFamilySupportReport<D, I::Family>,
        ForgeQueryDeclarationLegalityContract,
        String,
    ) {
        (
            self.declaration,
            self.support_report,
            self.legality_contract,
            self.operating_context_identity_digest,
        )
    }
}
