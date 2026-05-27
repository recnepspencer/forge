use crate::application::{
    aspect_coverage_from_publication, authority_mismatch_from_fit,
    authority_scoped_envelope_aspect_contract, merged_authority_aspect_contract,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectCoverageBasis, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationAspectPublication, ForgeQueryDeclarationAuthorityAspectMismatch,
    ForgeQueryDeclarationBridgeContinuationContract, ForgeQueryDeclarationRelationalTruthContract,
    ForgeQueryDeclarationSignalCompatibilityContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRelationalAuthorityAspectSummary {
    aspect_contract: ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    aspect_fit: ForgeQueryDeclarationAspectFit,
    aspect_mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
}

impl ForgeQueryDeclarationRelationalAuthorityAspectSummary {
    pub(crate) fn new(
        aspect_contract: ForgeQueryDeclarationAspectContract,
        aspect_coverage: ForgeQueryDeclarationAspectCoverage,
        aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
        aspect_fit: ForgeQueryDeclarationAspectFit,
        aspect_mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
    ) -> Self {
        Self {
            aspect_contract,
            aspect_coverage,
            aspect_coverage_basis,
            aspect_fit,
            aspect_mismatch,
        }
    }

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<ForgeQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationBridgeAuthorityAspectSummary {
    aspect_contract: ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    aspect_fit: ForgeQueryDeclarationAspectFit,
    aspect_mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
    mapped_aspects: ForgeQueryDeclarationAspectCoverage,
    mapped_aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    mapping_fit: ForgeQueryDeclarationAspectFit,
}

impl ForgeQueryDeclarationBridgeAuthorityAspectSummary {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        aspect_contract: ForgeQueryDeclarationAspectContract,
        aspect_coverage: ForgeQueryDeclarationAspectCoverage,
        aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
        aspect_fit: ForgeQueryDeclarationAspectFit,
        aspect_mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
        mapped_aspects: ForgeQueryDeclarationAspectCoverage,
        mapped_aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
        mapping_fit: ForgeQueryDeclarationAspectFit,
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

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<ForgeQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }

    pub fn mapped_aspects(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.mapped_aspects
    }

    pub fn mapped_aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.mapped_aspect_coverage_basis
    }

    pub fn mapping_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.mapping_fit
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationSignalAuthorityAspectSummary {
    aspect_contract: ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    aspect_fit: ForgeQueryDeclarationAspectFit,
    aspect_mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
    dependency_aspects: ForgeQueryDeclarationAspectContract,
    produced_aspects: ForgeQueryDeclarationAspectContract,
}

impl ForgeQueryDeclarationSignalAuthorityAspectSummary {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        aspect_contract: ForgeQueryDeclarationAspectContract,
        aspect_coverage: ForgeQueryDeclarationAspectCoverage,
        aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
        aspect_fit: ForgeQueryDeclarationAspectFit,
        aspect_mismatch: Option<ForgeQueryDeclarationAuthorityAspectMismatch>,
        dependency_aspects: ForgeQueryDeclarationAspectContract,
        produced_aspects: ForgeQueryDeclarationAspectContract,
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

    pub fn aspect_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.aspect_contract
    }

    pub fn aspect_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.aspect_coverage
    }

    pub fn aspect_coverage_basis(&self) -> ForgeQueryDeclarationAspectCoverageBasis {
        self.aspect_coverage_basis
    }

    pub fn aspect_fit(&self) -> ForgeQueryDeclarationAspectFit {
        self.aspect_fit
    }

    pub fn aspect_mismatch(&self) -> Option<ForgeQueryDeclarationAuthorityAspectMismatch> {
        self.aspect_mismatch
    }

    pub fn dependency_aspects(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.dependency_aspects
    }

    pub fn produced_aspects(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.produced_aspects
    }
}

pub(crate) fn relational_authority_summary_from_coverage(
    envelope_contract: &ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    contract: Option<&ForgeQueryDeclarationRelationalTruthContract>,
) -> ForgeQueryDeclarationRelationalAuthorityAspectSummary {
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

    ForgeQueryDeclarationRelationalAuthorityAspectSummary::new(
        aspect_contract,
        aspect_coverage,
        aspect_coverage_basis,
        aspect_fit,
        aspect_mismatch,
    )
}

pub(crate) fn relational_authority_summary_from_publication(
    envelope_contract: &ForgeQueryDeclarationAspectContract,
    publication: &ForgeQueryDeclarationAspectPublication,
    contract: Option<&ForgeQueryDeclarationRelationalTruthContract>,
) -> ForgeQueryDeclarationRelationalAuthorityAspectSummary {
    relational_authority_summary_from_coverage(
        envelope_contract,
        aspect_coverage_from_publication(publication),
        ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
        contract,
    )
}

pub(crate) fn bridge_authority_summary_from_coverage(
    envelope_contract: &ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    contract: Option<&ForgeQueryDeclarationBridgeContinuationContract>,
) -> ForgeQueryDeclarationBridgeAuthorityAspectSummary {
    let envelope_contract = authority_scoped_envelope_aspect_contract(envelope_contract);
    let bridge_required_aspects = contract
        .map(|contract| contract.required_aspects().clone())
        .unwrap_or_else(ForgeQueryDeclarationAspectContract::empty);
    let aspect_contract = if bridge_required_aspects == ForgeQueryDeclarationAspectContract::empty()
    {
        envelope_contract.clone()
    } else {
        merged_authority_aspect_contract(&envelope_contract, &bridge_required_aspects)
    };
    let aspect_fit = aspect_coverage.fit_against(&aspect_contract);
    let aspect_mismatch = authority_mismatch_from_fit(aspect_fit);
    let mapped_aspects = aspect_coverage.scoped_to_contract(&bridge_required_aspects);
    let mapping_fit = mapped_aspects.fit_against(&bridge_required_aspects);

    ForgeQueryDeclarationBridgeAuthorityAspectSummary::new(
        aspect_contract,
        aspect_coverage,
        aspect_coverage_basis,
        aspect_fit,
        aspect_mismatch,
        mapped_aspects,
        ForgeQueryDeclarationAspectCoverageBasis::BridgeMappedCoverage,
        mapping_fit,
    )
}

pub(crate) fn bridge_authority_summary_from_publication(
    envelope_contract: &ForgeQueryDeclarationAspectContract,
    publication: &ForgeQueryDeclarationAspectPublication,
    contract: Option<&ForgeQueryDeclarationBridgeContinuationContract>,
) -> ForgeQueryDeclarationBridgeAuthorityAspectSummary {
    bridge_authority_summary_from_coverage(
        envelope_contract,
        aspect_coverage_from_publication(publication),
        ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
        contract,
    )
}

pub(crate) fn signal_authority_summary_from_coverage(
    envelope_contract: &ForgeQueryDeclarationAspectContract,
    aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    aspect_coverage_basis: ForgeQueryDeclarationAspectCoverageBasis,
    contract: Option<&ForgeQueryDeclarationSignalCompatibilityContract>,
) -> ForgeQueryDeclarationSignalAuthorityAspectSummary {
    let envelope_aspects = authority_scoped_envelope_aspect_contract(envelope_contract);
    let (dependency_aspects, produced_aspects) = contract
        .map(|contract| {
            let dependency_aspects =
                if contract.dependency_aspects() == ForgeQueryDeclarationAspectContract::empty() {
                    envelope_aspects.clone()
                } else {
                    contract.dependency_aspects().clone()
                };
            (dependency_aspects, contract.produced_aspects().clone())
        })
        .unwrap_or_else(|| {
            (
                envelope_aspects.clone(),
                ForgeQueryDeclarationAspectContract::empty(),
            )
        });
    let aspect_fit = aspect_coverage.fit_against(&dependency_aspects);
    let aspect_mismatch = authority_mismatch_from_fit(aspect_fit);

    ForgeQueryDeclarationSignalAuthorityAspectSummary::new(
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
    envelope_contract: &ForgeQueryDeclarationAspectContract,
    publication: &ForgeQueryDeclarationAspectPublication,
    contract: Option<&ForgeQueryDeclarationSignalCompatibilityContract>,
) -> ForgeQueryDeclarationSignalAuthorityAspectSummary {
    signal_authority_summary_from_coverage(
        envelope_contract,
        aspect_coverage_from_publication(publication),
        ForgeQueryDeclarationAspectCoverageBasis::EnvelopePublishedCoverage,
        contract,
    )
}
