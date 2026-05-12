mod contracts;
mod evolution;
mod identity;
mod keys;
mod masks;
mod patches;
mod state;
mod structs;
mod validation;

pub use contracts::{
    AbsenceLaw, AspectContract, AspectEquivalenceBasis, AspectShape, OpaqueAspectType,
    ReferenceAspectType,
};
pub use evolution::{
    classify_aspect_contract_evolution, AspectEvolutionClassified,
    AspectEvolutionClassifiedContractArtifact, AspectEvolutionClassifiedContracts,
    AspectEvolutionKind, AspectEvolutionPolicy, AspectEvolutionVerdict,
};
pub use identity::{AspectContractRevision, AspectIdentity};
pub use keys::AspectKey;
pub use masks::{
    AspectMask, AspectMaskContract, DiagnosticMask, MaskAdmissibilityDenial, MutationMask,
    ProjectionMask,
};
pub use patches::{
    AuthoritativePatchApplicationDenial, AuthoritativePatchConstructionDenial,
    AuthoritativeRecordAspectPatch, FieldLevelAspectPatch,
};
pub use state::{
    admit_authoritative_record_aspect_state, AuthoritativeRecordAspectState,
    AuthoritativeRecordAspectStateAdmitted, AuthoritativeRecordAspectStateArtifact,
    AuthoritativeStateAdmissionDenial, CanonicalAspectStateMap,
};
pub use structs::{
    CanonicalFieldPath, FieldDeclaration, FieldKey, FieldRequirement, StructAspectShape,
    StructAspectValue, StructAspectValueConstructionDenial,
};
pub use validation::{
    validate_aspect_value, ContractValidatedAspectArtifact, ContractValidatedAspectValue,
    ContractValidatedAspectValueView, ContractValidationDenial, ContractValidationInput,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "aspect_state_and_patches",
        "aspect contracts, state-map vocabulary, mask law, and patch vocabulary",
        "domain-owned truth mutation or persistence engines",
    )
}
