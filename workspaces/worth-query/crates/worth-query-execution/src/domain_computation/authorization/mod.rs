mod admission;
mod admitted_capability_access;
mod admitted_operation;
mod application_commit_authorization;
mod authorization_revalidation;
mod bridge_binding;
mod bridge_observation;
mod capability_admission;
mod capability_binding_lowering;
mod capability_decision_fact;
mod capability_lowering;
mod capability_observation;
mod capability_operation_progression;
mod capability_registry;
mod capability_request_resolution;
mod capability_revalidation;
mod decision_facts;
mod denial;
mod installed_policy;
mod lowering;
mod operation_scope_binding;
mod retained_capability_request;
mod time_basis;

pub use admitted_capability_access::WorthQueryAdmittedApplicationCapabilityAccess;
pub use admitted_operation::WorthQueryAdmittedApplicationOperation;
pub(in crate::domain_computation) use admitted_operation::WorthQueryOperationAdmissionIdentity;
pub(in crate::domain_computation) use application_commit_authorization::WorthQueryApplicationCommitAuthorization;
pub(super) use bridge_binding::bridge_authorization_binding_identity;
pub(in crate::domain_computation) use capability_decision_fact::{
    WorthQueryCapabilityCommitBasis, WorthQueryRetainedCapabilityAuthorization,
};
pub use capability_registry::WorthQueryCapabilityPlanCompilationEvidence;
pub(in crate::domain_computation) use decision_facts::{
    WorthQueryAuthorizationDecisionFact, WorthQueryCommitAuthorizationBasis,
    WorthQueryPrincipalCurrentnessDependency, WorthQueryProviderAuthorizationDecisionFacts,
    WorthQueryRetainedAuthorizationDecisionFacts,
};
pub use denial::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
pub(in crate::domain_computation) use installed_policy::WorthQueryInstalledAuthorizationRegistry;
pub use operation_scope_binding::{
    WorthQueryOperationScopeBinding, WorthQueryOperationScopeEntityBinding,
};
pub(in crate::domain_computation) use time_basis::{
    WorthQueryAuthorizationClock, WorthQueryAuthorizationTimeSample,
};

fn authorization_denial(
    subject: impl Into<String>,
    _detail: &'static str,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::InvalidInstalledPolicy,
        subject,
    )
}
