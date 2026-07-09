use crate::application::{
    aspect_coverage_from_publication, merged_authority_aspect_contract,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

pub(super) struct BridgeAuthorityAspectGate {
    contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
    coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    fit: WorthQueryDeclarationAspectFit,
    mapped_aspects: WorthQueryDeclarationAspectCoverage,
    mapping_fit: WorthQueryDeclarationAspectFit,
}

impl BridgeAuthorityAspectGate {
    pub(super) fn from_envelope<
        D: WorthQueryDomainEntryMarker,
        I: WorthQueryDeclarationInput<D>,
    >(
        envelope: &WorthQueryDeclarationEnvelope<D, I>,
        authority_contract: &WorthQueryDeclarationAspectContract,
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
            coverage_basis: WorthQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
            fit,
            mapped_aspects,
            mapping_fit,
        }
    }

    pub(super) fn contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.contract
    }

    pub(super) fn coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.coverage
    }

    pub(super) fn coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.coverage_basis
    }

    pub(super) fn fit(&self) -> WorthQueryDeclarationAspectFit {
        self.fit
    }

    pub(super) fn mapped_aspects(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.mapped_aspects
    }

    pub(super) fn mapping_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.mapping_fit
    }
}
