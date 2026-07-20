use crate::application::{
    aspect_coverage_from_publication, authority_scoped_envelope_aspect_contract,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::contract::WorthQueryDeclarationSignalCompatibilityContract;

pub(super) struct SignalAuthorityAspectGate {
    authority_contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
    coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    fit: WorthQueryDeclarationAspectFit,
    dependency_aspects: WorthQueryDeclarationAspectContract,
    produced_aspects: WorthQueryDeclarationAspectContract,
}

impl SignalAuthorityAspectGate {
    pub(super) fn from_envelope<
        D: WorthQueryDomainEntryMarker,
        I: WorthQueryDeclarationInput<D>,
    >(
        envelope: &WorthQueryDeclarationEnvelope<D, I>,
        contract: &WorthQueryDeclarationSignalCompatibilityContract,
    ) -> Self {
        let authority_contract =
            authority_scoped_envelope_aspect_contract(envelope.aspect_contract());
        let coverage = aspect_coverage_from_publication(envelope.aspect_publication());
        let dependency_aspects = {
            let declared = contract.dependency_aspects();
            if declared == WorthQueryDeclarationAspectContract::empty() {
                authority_contract.clone()
            } else {
                declared
            }
        };
        let produced_aspects = contract.produced_aspects();
        let fit = coverage.fit_against(&dependency_aspects);
        Self {
            authority_contract,
            coverage,
            coverage_basis: WorthQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
            fit,
            dependency_aspects,
            produced_aspects,
        }
    }

    pub(super) fn authority_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.authority_contract
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

    pub(super) fn dependency_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.dependency_aspects
    }

    pub(super) fn produced_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.produced_aspects
    }
}
