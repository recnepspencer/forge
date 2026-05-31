use crate::application::{
    aspect_coverage_from_publication, authority_scoped_envelope_aspect_contract,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::contract::ForgeQueryDeclarationSignalCompatibilityContract;

pub(super) struct SignalAuthorityAspectGate {
    authority_contract: ForgeQueryDeclarationAspectContract,
    coverage: ForgeQueryDeclarationAspectCoverage,
    coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    fit: ForgeQueryDeclarationAspectFit,
    dependency_aspects: ForgeQueryDeclarationAspectContract,
    produced_aspects: ForgeQueryDeclarationAspectContract,
}

impl SignalAuthorityAspectGate {
    pub(super) fn from_envelope<
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
        contract: &ForgeQueryDeclarationSignalCompatibilityContract,
    ) -> Self {
        let authority_contract =
            authority_scoped_envelope_aspect_contract(envelope.aspect_contract());
        let coverage = aspect_coverage_from_publication(envelope.aspect_publication());
        let dependency_aspects = {
            let declared = contract.dependency_aspects();
            if declared == ForgeQueryDeclarationAspectContract::empty() {
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
            coverage_basis: ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
            fit,
            dependency_aspects,
            produced_aspects,
        }
    }

    pub(super) fn authority_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.authority_contract
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

    pub(super) fn dependency_aspects(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.dependency_aspects
    }

    pub(super) fn produced_aspects(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.produced_aspects
    }
}
