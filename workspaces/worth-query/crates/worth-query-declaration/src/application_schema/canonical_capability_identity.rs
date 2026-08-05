use crate::application_capability::{
    application_capability_canonical_components, ErasedApplicationCapabilityContract,
};

use super::canonical_basis::ApplicationSchemaCanonicalBasis;

pub(super) fn append_capability_contract(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    contract: &ErasedApplicationCapabilityContract,
) {
    for component in application_capability_canonical_components(contract) {
        basis.value(
            format!("{prefix}.{}", component.locus()),
            component.value().clone(),
        );
    }
}
