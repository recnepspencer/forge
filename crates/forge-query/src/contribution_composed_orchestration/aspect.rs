use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectPublication,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedIntentAspectRecord {
    declaration_contract: ForgeQueryDeclarationAspectContract,
    declaration_coverage: ForgeQueryDeclarationAspectCoverage,
}

impl ForgeQueryContributionComposedIntentAspectRecord {
    pub fn new(
        declaration_contract: ForgeQueryDeclarationAspectContract,
        declaration_coverage: ForgeQueryDeclarationAspectCoverage,
    ) -> Self {
        Self {
            declaration_contract,
            declaration_coverage,
        }
    }

    pub fn declaration_contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.declaration_contract
    }

    pub fn declaration_coverage(&self) -> &ForgeQueryDeclarationAspectCoverage {
        &self.declaration_coverage
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContributionComposedDeclarationAspectRecord {
    contract: ForgeQueryDeclarationAspectContract,
    publication: ForgeQueryDeclarationAspectPublication,
}

impl ForgeQueryContributionComposedDeclarationAspectRecord {
    pub fn new(
        contract: ForgeQueryDeclarationAspectContract,
        publication: ForgeQueryDeclarationAspectPublication,
    ) -> Self {
        Self {
            contract,
            publication,
        }
    }

    pub fn contract(&self) -> &ForgeQueryDeclarationAspectContract {
        &self.contract
    }

    pub fn publication(&self) -> &ForgeQueryDeclarationAspectPublication {
        &self.publication
    }
}
