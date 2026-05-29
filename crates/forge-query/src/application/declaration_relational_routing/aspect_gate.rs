use crate::application::{
    aspect_coverage_from_publication, merged_authority_aspect_contract,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

pub(super) struct RelationalAuthorityAspectGate {
    contract: ForgeQueryDeclarationAspectContract,
    coverage: ForgeQueryDeclarationAspectCoverage,
    coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    fit: ForgeQueryDeclarationAspectFit,
}

impl RelationalAuthorityAspectGate {
    pub(super) fn from_envelope<
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
        authority_contract: &ForgeQueryDeclarationAspectContract,
    ) -> Self {
        let contract =
            merged_authority_aspect_contract(envelope.aspect_contract(), authority_contract);
        let coverage = aspect_coverage_from_publication(envelope.aspect_publication());
        let fit = coverage.fit_against(&contract);
        Self {
            contract,
            coverage,
            coverage_basis: ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
            fit,
        }
    }

    pub(super) fn contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.contract
    }

    pub(super) fn coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.coverage
    }

    pub(super) fn coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.coverage_basis
    }

    pub(super) fn fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.fit
    }
}
