use crate::planar_contracts::contract_bundle::{
    PlanarContractBundleFamily, PlanarContractBundleValidationBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarContractBundleInspectionRow {
    family: PlanarContractBundleFamily,
    receipt_count: usize,
}

impl PlanarContractBundleInspectionRow {
    pub(crate) fn from_basis(basis: &PlanarContractBundleValidationBasis) -> Vec<Self> {
        basis
            .family_rows()
            .iter()
            .map(|row| Self {
                family: row.family(),
                receipt_count: row.receipt_count(),
            })
            .collect()
    }

    pub fn family(&self) -> PlanarContractBundleFamily {
        self.family
    }

    pub fn receipt_count(&self) -> usize {
        self.receipt_count
    }
}
