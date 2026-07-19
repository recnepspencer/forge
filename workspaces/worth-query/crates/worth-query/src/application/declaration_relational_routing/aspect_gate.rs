use crate::application::{
    aspect_coverage_from_publication, merged_authority_aspect_contract,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

pub(super) struct RelationalAuthorityAspectGate {
    contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
    coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    fit: WorthQueryDeclarationAspectFit,
}

impl RelationalAuthorityAspectGate {
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
        Self {
            contract,
            coverage,
            coverage_basis: WorthQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
            fit,
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
}
