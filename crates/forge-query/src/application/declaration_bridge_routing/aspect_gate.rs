use crate::application::{
    aspect_coverage_from_publication, merged_authority_aspect_contract,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

pub(super) struct BridgeAuthorityAspectGate {
    contract: ForgeQueryDeclarationAspectContract,
    coverage: ForgeQueryDeclarationAspectCoverage,
    coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    fit: ForgeQueryDeclarationAspectFit,
    mapped_aspects: ForgeQueryDeclarationAspectCoverage,
    mapping_fit: ForgeQueryDeclarationAspectFit,
}

impl BridgeAuthorityAspectGate {
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
        let mapped_aspects = coverage.scoped_to_contract(authority_contract);
        let mapping_fit = mapped_aspects.fit_against(authority_contract);
        Self {
            contract,
            coverage,
            coverage_basis: ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
            fit,
            mapped_aspects,
            mapping_fit,
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

    pub(super) fn mapped_aspects(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.mapped_aspects
    }

    pub(super) fn mapping_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.mapping_fit
    }
}
