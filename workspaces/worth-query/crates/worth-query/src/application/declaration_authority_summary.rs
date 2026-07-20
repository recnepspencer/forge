use crate::application::{
    aspect_coverage_from_publication, authority_mismatch_from_fit,
    authority_scoped_envelope_aspect_contract, merged_authority_aspect_contract,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectCoverageBasis, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationAspectPublication, WorthQueryDeclarationAuthorityAspectMismatch,
    WorthQueryDeclarationBridgeContinuationContract, WorthQueryDeclarationRelationalTruthContract,
    WorthQueryDeclarationSignalCompatibilityContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationRelationalAuthorityAspectSummary {
    aspect_contract: WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    aspect_fit: WorthQueryDeclarationAspectFit,
    aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
}

impl WorthQueryDeclarationRelationalAuthorityAspectSummary {
    pub(crate) fn new(
        aspect_contract: WorthQueryDeclarationAspectContract,
        aspect_coverage: WorthQueryDeclarationAspectCoverage,
        aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
        aspect_fit: WorthQueryDeclarationAspectFit,
        aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
    ) -> Self {
        Self {
            aspect_contract,
            aspect_coverage,
            aspect_coverage_basis,
            aspect_fit,
            aspect_mismatch,
        }
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<WorthQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationBridgeAuthorityAspectSummary {
    aspect_contract: WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    aspect_fit: WorthQueryDeclarationAspectFit,
    aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
    mapped_aspects: WorthQueryDeclarationAspectCoverage,
    mapped_aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    mapping_fit: WorthQueryDeclarationAspectFit,
}

impl WorthQueryDeclarationBridgeAuthorityAspectSummary {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        aspect_contract: WorthQueryDeclarationAspectContract,
        aspect_coverage: WorthQueryDeclarationAspectCoverage,
        aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
        aspect_fit: WorthQueryDeclarationAspectFit,
        aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
        mapped_aspects: WorthQueryDeclarationAspectCoverage,
        mapped_aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
        mapping_fit: WorthQueryDeclarationAspectFit,
    ) -> Self {
        Self {
            aspect_contract,
            aspect_coverage,
            aspect_coverage_basis,
            aspect_fit,
            aspect_mismatch,
            mapped_aspects,
            mapped_aspect_coverage_basis,
            mapping_fit,
        }
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<WorthQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }

    pub fn mapped_aspects(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.mapped_aspects
    }

    pub fn mapped_aspect_coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.mapped_aspect_coverage_basis
    }

    pub fn mapping_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.mapping_fit
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationSignalAuthorityAspectSummary {
    aspect_contract: WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    aspect_fit: WorthQueryDeclarationAspectFit,
    aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
    dependency_aspects: WorthQueryDeclarationAspectContract,
    produced_aspects: WorthQueryDeclarationAspectContract,
}

impl WorthQueryDeclarationSignalAuthorityAspectSummary {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        aspect_contract: WorthQueryDeclarationAspectContract,
        aspect_coverage: WorthQueryDeclarationAspectCoverage,
        aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
        aspect_fit: WorthQueryDeclarationAspectFit,
        aspect_mismatch: Option<WorthQueryDeclarationAuthorityAspectMismatch>,
        dependency_aspects: WorthQueryDeclarationAspectContract,
        produced_aspects: WorthQueryDeclarationAspectContract,
    ) -> Self {
        Self {
            aspect_contract,
            aspect_coverage,
            aspect_coverage_basis,
            aspect_fit,
            aspect_mismatch,
            dependency_aspects,
            produced_aspects,
        }
    }

    pub fn aspect_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> WorthQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> WorthQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<WorthQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }

    pub fn dependency_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.dependency_aspects
    }

    pub fn produced_aspects(&self) -> &WorthQueryDeclarationAspectContract {
        &self.produced_aspects
    }
}

pub(crate) fn relational_authority_summary_from_coverage(
    envelope_contract: &WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    contract: Option<&WorthQueryDeclarationRelationalTruthContract>,
) -> WorthQueryDeclarationRelationalAuthorityAspectSummary {
    let aspect_contract = contract
        .map(|contract| {
            merged_authority_aspect_contract(
                &authority_scoped_envelope_aspect_contract(envelope_contract),
                contract.required_aspects(),
            )
        })
        .unwrap_or_else(|| authority_scoped_envelope_aspect_contract(envelope_contract));
    let aspect_fit = aspect_coverage.fit_against(&aspect_contract);
    let aspect_mismatch = authority_mismatch_from_fit(aspect_fit);

    WorthQueryDeclarationRelationalAuthorityAspectSummary::new(
        aspect_contract,
        aspect_coverage,
        aspect_coverage_basis,
        aspect_fit,
        aspect_mismatch,
    )
}

pub(crate) fn relational_authority_summary_from_publication(
    envelope_contract: &WorthQueryDeclarationAspectContract,
    publication: &WorthQueryDeclarationAspectPublication,
    contract: Option<&WorthQueryDeclarationRelationalTruthContract>,
) -> WorthQueryDeclarationRelationalAuthorityAspectSummary {
    relational_authority_summary_from_coverage(
        envelope_contract,
        aspect_coverage_from_publication(publication),
        WorthQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
        contract,
    )
}

pub(crate) fn bridge_authority_summary_from_coverage(
    envelope_contract: &WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    contract: Option<&WorthQueryDeclarationBridgeContinuationContract>,
) -> WorthQueryDeclarationBridgeAuthorityAspectSummary {
    let envelope_contract = authority_scoped_envelope_aspect_contract(envelope_contract);
    let bridge_required_aspects = contract
        .map(|contract| contract.required_aspects().clone())
        .unwrap_or_else(WorthQueryDeclarationAspectContract::empty);
    let aspect_contract = if bridge_required_aspects == WorthQueryDeclarationAspectContract::empty()
    {
        envelope_contract.clone()
    } else {
        merged_authority_aspect_contract(&envelope_contract, &bridge_required_aspects)
    };
    let aspect_fit = aspect_coverage.fit_against(&aspect_contract);
    let aspect_mismatch = authority_mismatch_from_fit(aspect_fit);
    let mapped_aspects = aspect_coverage.scoped_to_contract(&bridge_required_aspects);
    let mapping_fit = mapped_aspects.fit_against(&bridge_required_aspects);

    WorthQueryDeclarationBridgeAuthorityAspectSummary::new(
        aspect_contract,
        aspect_coverage,
        aspect_coverage_basis,
        aspect_fit,
        aspect_mismatch,
        mapped_aspects,
        WorthQueryDeclarationAspectCoverageBasis::BridgeMappedCoverage,
        mapping_fit,
    )
}

pub(crate) fn bridge_authority_summary_from_publication(
    envelope_contract: &WorthQueryDeclarationAspectContract,
    publication: &WorthQueryDeclarationAspectPublication,
    contract: Option<&WorthQueryDeclarationBridgeContinuationContract>,
) -> WorthQueryDeclarationBridgeAuthorityAspectSummary {
    bridge_authority_summary_from_coverage(
        envelope_contract,
        aspect_coverage_from_publication(publication),
        WorthQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
        contract,
    )
}

pub(crate) fn signal_authority_summary_from_coverage(
    envelope_contract: &WorthQueryDeclarationAspectContract,
    aspect_coverage: WorthQueryDeclarationAspectCoverage,
    aspect_coverage_basis: WorthQueryDeclarationAspectCoverageBasis,
    contract: Option<&WorthQueryDeclarationSignalCompatibilityContract>,
) -> WorthQueryDeclarationSignalAuthorityAspectSummary {
    let envelope_aspects = authority_scoped_envelope_aspect_contract(envelope_contract);
    let (dependency_aspects, produced_aspects) = contract
        .map(|contract| {
            let dependency_aspects =
                if contract.dependency_aspects() == WorthQueryDeclarationAspectContract::empty() {
                    envelope_aspects.clone()
                } else {
                    contract.dependency_aspects().clone()
                };
            (dependency_aspects, contract.produced_aspects().clone())
        })
        .unwrap_or_else(|| {
            (
                envelope_aspects.clone(),
                WorthQueryDeclarationAspectContract::empty(),
            )
        });
    let aspect_fit = aspect_coverage.fit_against(&dependency_aspects);
    let aspect_mismatch = authority_mismatch_from_fit(aspect_fit);

    WorthQueryDeclarationSignalAuthorityAspectSummary::new(
        dependency_aspects.clone(),
        aspect_coverage,
        aspect_coverage_basis,
        aspect_fit,
        aspect_mismatch,
        dependency_aspects,
        produced_aspects,
    )
}

pub(crate) fn signal_authority_summary_from_publication(
    envelope_contract: &WorthQueryDeclarationAspectContract,
    publication: &WorthQueryDeclarationAspectPublication,
    contract: Option<&WorthQueryDeclarationSignalCompatibilityContract>,
) -> WorthQueryDeclarationSignalAuthorityAspectSummary {
    signal_authority_summary_from_coverage(
        envelope_contract,
        aspect_coverage_from_publication(publication),
        WorthQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
        contract,
    )
}
