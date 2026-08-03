mod contract;
mod denial;
mod evidence;
mod runtime;

pub use contract::{
    BridgeAuthorizationBindingIdentity, BridgeAuthorizationClauseContract,
    BridgeAuthorizationClauseObservation, BridgeAuthorizationCorrespondenceIdentity,
    BridgeAuthorizationDependencyCardinality, BridgeAuthorizationInstallationRequest,
    BridgeAuthorizationObservation, BridgeAuthorizationRequirementContract,
    BridgeAuthorizationRequirementObservation, BridgeAuthorizationRuleContract,
    BridgeAuthorizationRuleEffect, BridgeAuthorizationRuleObservation,
};
pub use denial::{BridgeAuthorizationDenial, BridgeAuthorizationDenialKind};
pub use evidence::BridgeAuthorizationDecisionEvidence;
pub use runtime::BridgeAuthorizationRuntime;

#[cfg(test)]
mod tests;
