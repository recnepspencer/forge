use std::collections::BTreeMap;

use crate::capability::MosaicSizingContractId;
use crate::declaration::UiDeclaredMeasurementConstraintModifier;
use crate::facade::WorthUiDslPackage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSourceBackedDeclarationClaims {
    mosaic_membership_name: Box<str>,
    measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
    mosaic_sizing_contract_id: MosaicSizingContractId,
}

impl WorthUiSourceBackedDeclarationClaims {
    pub(crate) fn new(
        mosaic_membership_name: impl Into<Box<str>>,
        measurement_constraint_modifier: Option<UiDeclaredMeasurementConstraintModifier>,
        mosaic_sizing_contract_id: MosaicSizingContractId,
    ) -> Self {
        Self {
            mosaic_membership_name: mosaic_membership_name.into(),
            measurement_constraint_modifier,
            mosaic_sizing_contract_id,
        }
    }

    pub(crate) fn mosaic_membership_name(&self) -> &str {
        &self.mosaic_membership_name
    }

    pub(crate) fn measurement_constraint_modifier(
        &self,
    ) -> Option<UiDeclaredMeasurementConstraintModifier> {
        self.measurement_constraint_modifier
    }

    pub(crate) fn mosaic_sizing_contract_id(&self) -> &MosaicSizingContractId {
        &self.mosaic_sizing_contract_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSourceBackedDeclarationWitness {
    declaration_claims: BTreeMap<(String, usize), WorthUiSourceBackedDeclarationClaims>,
}

impl WorthUiSourceBackedDeclarationWitness {
    pub(crate) fn new(
        declaration_claims: BTreeMap<(String, usize), WorthUiSourceBackedDeclarationClaims>,
    ) -> Self {
        Self { declaration_claims }
    }

    pub(crate) fn claims_for(
        &self,
        module_path: &str,
        declaration_index: usize,
    ) -> Option<&WorthUiSourceBackedDeclarationClaims> {
        self.declaration_claims
            .get(&(module_path.to_owned(), declaration_index))
    }

    #[cfg(test)]
    pub(crate) fn sorted_mosaic_membership_names(&self) -> Vec<&str> {
        let mut names = self
            .declaration_claims
            .values()
            .map(WorthUiSourceBackedDeclarationClaims::mosaic_membership_name)
            .collect::<Vec<_>>();
        names.sort_unstable();
        names
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourceBackedDslPackage {
    dsl_package: WorthUiDslPackage,
    declaration_witness: WorthUiSourceBackedDeclarationWitness,
}

impl WorthUiSourceBackedDslPackage {
    pub(crate) fn new(
        dsl_package: WorthUiDslPackage,
        declaration_witness: WorthUiSourceBackedDeclarationWitness,
    ) -> Self {
        Self {
            dsl_package,
            declaration_witness,
        }
    }

    pub fn dsl_package(&self) -> &WorthUiDslPackage {
        &self.dsl_package
    }

    #[cfg(test)]
    pub(crate) fn declaration_witness(&self) -> &WorthUiSourceBackedDeclarationWitness {
        &self.declaration_witness
    }

    pub(crate) fn into_parts(self) -> (WorthUiDslPackage, WorthUiSourceBackedDeclarationWitness) {
        (self.dsl_package, self.declaration_witness)
    }
}
