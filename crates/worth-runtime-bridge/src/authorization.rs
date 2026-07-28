mod contract;
mod denial;
mod evidence;
mod runtime;

pub use contract::{
    BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationDependencyCardinality,
    BridgeAuthorizationInstallationRequest, BridgeAuthorizationObservation,
    BridgeAuthorizationPathContract, BridgeAuthorizationPathEffect,
    BridgeAuthorizationPathObservation,
};
pub use denial::{BridgeAuthorizationDenial, BridgeAuthorizationDenialKind};
pub use evidence::BridgeAuthorizationDecisionEvidence;
pub use runtime::BridgeAuthorizationRuntime;

#[cfg(test)]
mod tests;
