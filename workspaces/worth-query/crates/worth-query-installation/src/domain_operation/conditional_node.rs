mod builder;
mod comparison;
mod condition;
mod condition_parameter;
mod declaration;
mod dependency;
mod location;
mod markers;
mod node_posture;
mod output_contract;
mod temporal;
mod trigger;
mod validation;

pub use builder::WorthQueryPortableConditionalNodeBuilder;
pub use condition::{
    WorthQueryConditionalConditionClass, WorthQueryConditionalEvaluationCondition,
    WorthQueryDeltaComparisonDomain, WorthQueryDeltaThreshold, WorthQueryThresholdBoundary,
};
pub use condition_parameter::{
    WorthQueryPortableConditionParameter, WorthQueryPortableConditionParameterValue,
};
pub use declaration::WorthQueryPortableConditionalNodeDeclaration;
pub use dependency::{
    WorthQueryConditionalGraphReadRole, WorthQuerySemanticDependencyCanonicalBasis,
    WorthQuerySemanticLocality, WorthQuerySemanticTruthDependency,
    WorthQuerySemanticTruthDependencyDenial, WorthQueryTruthPartitionRole,
};
pub use location::WorthQueryConditionalNodeLocation;
pub use markers::{
    WorthQueryComparatorFamily, WorthQueryDomainConditionFamily, WorthQueryOnDemandTriggerFamily,
    WorthQueryQuantityUnit, WorthQueryQuantityValueFamily, WorthQueryTypedFamilyIdentity,
};
pub use node_posture::{
    WorthQueryArtifactPosture, WorthQueryConditionalNodeContext, WorthQueryConditionalNodeRole,
    WorthQueryMaintenancePosture, WorthQueryOutputRelationship,
};
pub use output_contract::{
    WorthQueryConditionalConsequenceRole, WorthQueryConditionalNodeOutput,
    WorthQueryConditionalTouchRole,
};
pub use temporal::{WorthQueryTemporalCondition, WorthQueryTemporalWake};
pub use trigger::WorthQueryConditionalTrigger;

pub(crate) use dependency::{contract_token, dependency_token, locality_token};
pub(crate) use validation::{canonicalize_conditional_nodes, validate_conditional_nodes};

pub(crate) fn push_token(material: &mut String, label: &str, value: &str) {
    material.push_str(label);
    material.push('#');
    material.push_str(&value.len().to_string());
    material.push(':');
    material.push_str(value);
    material.push(';');
}
pub use comparison::{
    WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryOutputEquivalenceRequirement,
};
