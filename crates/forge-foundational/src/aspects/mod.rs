mod contracts;
mod evolution;
mod identity;
mod keys;
mod masks;
mod structs;
mod validation;

pub use contracts::{
    AbsenceLaw, AspectContract, AspectEquivalenceBasis, AspectShape, OpaqueAspectType,
    ReferenceAspectType,
};
pub use evolution::{AspectEvolutionKind, AspectEvolutionPolicy, AspectEvolutionVerdict};
pub use identity::{AspectContractRevision, AspectIdentity};
pub use keys::AspectKey;
pub use masks::{
    AspectMask, AspectMaskContract, DiagnosticMask, MaskAdmissibilityDenial, MutationMask,
    ProjectionMask,
};
pub use structs::{
    CanonicalFieldPath, FieldDeclaration, FieldKey, FieldRequirement, StructAspectShape,
    StructAspectValue,
};
pub use validation::{
    validate_aspect_value, ContractValidatedAspectArtifact, ContractValidatedAspectValue,
    ContractValidationDenial, ContractValidationInput,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "aspect_state_and_patches",
        "aspect contracts, state-map vocabulary, mask law, and patch vocabulary",
        "domain-owned truth mutation or persistence engines",
    )
}
