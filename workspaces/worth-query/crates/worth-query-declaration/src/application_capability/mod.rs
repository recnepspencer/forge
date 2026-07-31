mod canonical_components;
mod composition;
mod contract;
mod reference;
mod scope;
mod scope_narrowing;

#[cfg(test)]
mod scope_narrowing_tests;

pub use canonical_components::{
    application_capability_canonical_components, ApplicationCapabilityCanonicalComponent,
};
pub use composition::{
    ApplicationCapabilityActorComposition, ApplicationCapabilityComposition,
    ApplicationCapabilityDecisionComposition, ApplicationCapabilityPropagationComposition,
    ApplicationCapabilityRule,
};
pub use contract::{
    ApplicationCapabilityContract, ApplicationCapabilityContractBuilder,
    ErasedApplicationCapabilityContract,
};
pub use reference::{
    ApplicationCapabilityContextRef, ApplicationCapabilityProvenanceRef, ApplicationCapabilityRef,
};
pub use scope::{
    ApplicationCapabilityAmountDimension, ApplicationCapabilityCardinalityDimension,
    ApplicationCapabilityConstraintDefinition, ApplicationCapabilityDelegationDefinition,
    ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityTargetDefinition, ApplicationCapabilityValidityDefinition,
    ApplicationCapabilityValueBinding,
};
pub use scope_narrowing::{
    ApplicationCapabilityAmountScope, ApplicationCapabilityAmountValue,
    ApplicationCapabilityContextScope, ApplicationCapabilityDelegationScope,
    ApplicationCapabilityLimitScope, ApplicationCapabilityOptionalValueSet,
    ApplicationCapabilityScope, ApplicationCapabilityTargetScope,
    ApplicationCapabilityValidityWindow, ApplicationCapabilityValue, ApplicationCapabilityValueSet,
};
