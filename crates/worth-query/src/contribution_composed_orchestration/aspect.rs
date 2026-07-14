use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectPublication,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedIntentAspectRecord {
    declaration_contract: WorthQueryDeclarationAspectContract,
    declaration_coverage: WorthQueryDeclarationAspectCoverage,
}

impl WorthQueryContributionComposedIntentAspectRecord {
    pub fn new(
        declaration_contract: WorthQueryDeclarationAspectContract,
        declaration_coverage: WorthQueryDeclarationAspectCoverage,
    ) -> Self {
        Self {
            declaration_contract,
            declaration_coverage,
        }
    }

    pub fn declaration_contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.declaration_contract
    }

    pub fn declaration_coverage(&self) -> &WorthQueryDeclarationAspectCoverage {
        &self.declaration_coverage
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryContributionComposedDeclarationAspectRecord {
    contract: WorthQueryDeclarationAspectContract,
    publication: WorthQueryDeclarationAspectPublication,
}

impl WorthQueryContributionComposedDeclarationAspectRecord {
    pub fn new(
        contract: WorthQueryDeclarationAspectContract,
        publication: WorthQueryDeclarationAspectPublication,
    ) -> Self {
        Self {
            contract,
            publication,
        }
    }

    pub fn contract(&self) -> &WorthQueryDeclarationAspectContract {
        &self.contract
    }

    pub fn publication(&self) -> &WorthQueryDeclarationAspectPublication {
        &self.publication
    }
}
