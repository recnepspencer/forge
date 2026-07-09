use crate::application::{
    WorthQueryAdmittedWorldBasis, WorthQueryCanonicalDeclarationArtifact,
    WorthQueryDeclarationFamilySupportReport, WorthQueryDeclarationFamilyTaxonomy,
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};
use crate::runtime::WorthQueryRuntimeFamilySupportStatus;

use super::contract::WorthQueryDeclarationLegalityContract;

pub struct WorthQueryDeclarationLegalityInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    declaration: WorthQueryCanonicalDeclarationArtifact<D, I>,
    support_report: WorthQueryDeclarationFamilySupportReport<D, I::Family>,
    legality_contract: WorthQueryDeclarationLegalityContract,
    world_basis: WorthQueryAdmittedWorldBasis,
    temporal_runtime_support_status: Option<WorthQueryRuntimeFamilySupportStatus>,
    async_runtime_support_status: Option<WorthQueryRuntimeFamilySupportStatus>,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationLegalityInput<D, I>
{
    pub(crate) fn new(
        declaration: WorthQueryCanonicalDeclarationArtifact<D, I>,
        support_report: WorthQueryDeclarationFamilySupportReport<D, I::Family>,
        legality_contract: WorthQueryDeclarationLegalityContract,
        world_basis: WorthQueryAdmittedWorldBasis,
        temporal_runtime_support_status: Option<WorthQueryRuntimeFamilySupportStatus>,
        async_runtime_support_status: Option<WorthQueryRuntimeFamilySupportStatus>,
    ) -> Self {
        Self {
            declaration,
            support_report,
            legality_contract,
            world_basis,
            temporal_runtime_support_status,
            async_runtime_support_status,
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

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration.declaration_family_key()
    }

    pub fn declaration_taxonomy(&self) -> WorthQueryDeclarationFamilyTaxonomy {
        self.declaration.declaration_taxonomy()
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.declaration.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.world_basis.operating_context_identity_digest()
    }

    pub fn temporal_runtime_support_status(&self) -> Option<WorthQueryRuntimeFamilySupportStatus> {
        self.temporal_runtime_support_status
    }

    pub fn async_runtime_support_status(&self) -> Option<WorthQueryRuntimeFamilySupportStatus> {
        self.async_runtime_support_status
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        WorthQueryCanonicalDeclarationArtifact<D, I>,
        WorthQueryDeclarationFamilySupportReport<D, I::Family>,
        WorthQueryDeclarationLegalityContract,
        WorthQueryAdmittedWorldBasis,
        Option<WorthQueryRuntimeFamilySupportStatus>,
        Option<WorthQueryRuntimeFamilySupportStatus>,
    ) {
        (
            self.declaration,
            self.support_report,
            self.legality_contract,
            self.world_basis,
            self.temporal_runtime_support_status,
            self.async_runtime_support_status,
        )
    }
}
