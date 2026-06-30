use serde::Serialize;

use crate::compiled_product_family::TopologyCompiledProductFamilyAdmittedInput;

use super::locality_basis::TopologyCompiledProductLocalityBasis;
use super::prior_proof_basis::TopologyCompiledProductPriorProofBasis;
use super::source_authority_basis::TopologyCompiledProductSourceAuthorityBasis;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopologyCompiledProductAdmittedInput {
    family_admitted_input: TopologyCompiledProductFamilyAdmittedInput,
    source_authority_basis: TopologyCompiledProductSourceAuthorityBasis,
    locality_basis: TopologyCompiledProductLocalityBasis,
    prior_proof_basis: TopologyCompiledProductPriorProofBasis,
}

impl TopologyCompiledProductAdmittedInput {
    pub(crate) fn new(
        family_admitted_input: TopologyCompiledProductFamilyAdmittedInput,
        source_authority_basis: TopologyCompiledProductSourceAuthorityBasis,
        locality_basis: TopologyCompiledProductLocalityBasis,
        prior_proof_basis: TopologyCompiledProductPriorProofBasis,
    ) -> Self {
        Self {
            family_admitted_input,
            source_authority_basis,
            locality_basis,
            prior_proof_basis,
        }
    }

    pub fn family_admitted_input(&self) -> &TopologyCompiledProductFamilyAdmittedInput {
        &self.family_admitted_input
    }

    pub fn into_family_admitted_input(self) -> TopologyCompiledProductFamilyAdmittedInput {
        self.family_admitted_input
    }

    pub fn source_authority_basis(&self) -> &TopologyCompiledProductSourceAuthorityBasis {
        &self.source_authority_basis
    }

    pub fn locality_basis(&self) -> &TopologyCompiledProductLocalityBasis {
        &self.locality_basis
    }

    pub fn prior_proof_basis(&self) -> &TopologyCompiledProductPriorProofBasis {
        &self.prior_proof_basis
    }
}
