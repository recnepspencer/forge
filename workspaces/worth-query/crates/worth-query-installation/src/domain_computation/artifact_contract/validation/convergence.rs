use crate::domain_computation::{
    WorthQueryConvergenceContract, WorthQueryPortableArtifactContract,
};

use super::{
    portable_text, WorthQueryArtifactContractValidationDenial,
    WorthQueryArtifactContractValidationDenialKind as Kind,
};

pub(super) fn validate(
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryArtifactContractValidationDenial> {
    let valid = match &contract.convergence {
        WorthQueryConvergenceContract::NotIterative => true,
        WorthQueryConvergenceContract::Iterative {
            progress_measure_family,
            comparator_family,
            iteration_bound,
            ..
        } => {
            *iteration_bound > 0
                && portable_text(progress_measure_family)
                && portable_text(comparator_family)
        }
    };
    valid.then_some(()).ok_or_else(|| {
        WorthQueryArtifactContractValidationDenial::new(
            Kind::InvalidConvergenceContract,
            contract.family.as_str(),
        )
    })
}
