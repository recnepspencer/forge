mod aspect_entries;
mod boundary_entries;
mod common_entries;
mod mask_entries;
mod transition_entries;

use worth_proof::TransitionOutcome;

use crate::locators::{
    AspectContractLocator, AspectFieldLocator, AspectLocator, AspectValueLocator,
    BoundaryArtifactLocator, BoundaryMismatchLocator, BoundarySourceLocator,
    FoundationalTransitionLocator,
};

use super::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};

pub use mask_entries::{
    diagnostic_mask_locator_canonical_basis_entries, mutation_mask_locator_canonical_basis_entries,
    projection_mask_locator_canonical_basis_entries,
};

use aspect_entries::{
    aspect_contract_locator_entries, aspect_field_locator_entries, aspect_locator_entries,
    value_locator_entries,
};
use boundary_entries::{
    boundary_artifact_locator_entries, mismatch_locator_entries, source_locator_entries,
};
use transition_entries::transition_locator_entries;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalLocatorInput {
    BoundaryArtifact(BoundaryArtifactLocator),
    Aspect(AspectLocator),
    AspectField(AspectFieldLocator),
    AspectContract(AspectContractLocator),
    Value(AspectValueLocator),
    Source(BoundarySourceLocator),
    Mismatch(BoundaryMismatchLocator),
    Transition(FoundationalTransitionLocator),
}

pub fn prepare_locator_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    locator: CanonicalLocatorInput,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Locator,
        canonical_locator_entries(locator),
    )
}

pub fn locator_canonical_basis_entries(
    ready: &CanonicalBasisReadyArtifact,
) -> &[CanonicalBasisEntry] {
    ready.payload().entries()
}

fn canonical_locator_entries(locator: CanonicalLocatorInput) -> Vec<CanonicalBasisEntry> {
    match locator {
        CanonicalLocatorInput::BoundaryArtifact(locator) => {
            boundary_artifact_locator_entries("boundary_artifact", locator)
        }
        CanonicalLocatorInput::Aspect(locator) => aspect_locator_entries("aspect", &locator),
        CanonicalLocatorInput::AspectField(locator) => {
            aspect_field_locator_entries("aspect_field", &locator)
        }
        CanonicalLocatorInput::AspectContract(locator) => {
            aspect_contract_locator_entries("aspect_contract", &locator)
        }
        CanonicalLocatorInput::Value(locator) => value_locator_entries(locator),
        CanonicalLocatorInput::Source(locator) => source_locator_entries(locator),
        CanonicalLocatorInput::Mismatch(locator) => mismatch_locator_entries(locator),
        CanonicalLocatorInput::Transition(locator) => transition_locator_entries(locator),
    }
}
