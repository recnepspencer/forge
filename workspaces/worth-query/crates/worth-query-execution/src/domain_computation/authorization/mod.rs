mod admission;
mod admitted_operation;
mod application_commit_authorization;
mod authorization_revalidation;
mod bridge_observation;
mod capability_admission;
mod capability_binding_lowering;
mod capability_decision_fact;
mod capability_lowering;
mod capability_observation;
mod capability_operation_progression;
mod capability_projection_validation;
mod capability_registry;
mod capability_request_resolution;
mod capability_revalidation;
mod decision_facts;
mod denial;
mod graph_work_session;
mod installed_policy;
mod lowering;
mod operation_scope_binding;
mod prepared_capability_access;
mod retained_capability_request;
mod time_basis;

pub use admitted_operation::WorthQueryAdmittedApplicationOperation;
pub(in crate::domain_computation) use admitted_operation::WorthQueryOperationAdmissionIdentity;
pub(in crate::domain_computation) use application_commit_authorization::WorthQueryApplicationCommitAuthorization;
pub(in crate::domain_computation) use capability_decision_fact::{
    WorthQueryCapabilityCommitBasis, WorthQueryRetainedCapabilityAuthorization,
};
pub(in crate::domain_computation) use capability_observation::observe_capability;
pub use capability_registry::WorthQueryCapabilityPlanCompilationEvidence;
pub(in crate::domain_computation) use capability_request_resolution::resolve_capability_request;
pub(in crate::domain_computation) use decision_facts::{
    WorthQueryAuthorizationDecisionFact, WorthQueryCommitAuthorizationBasis,
    WorthQueryPrincipalCurrentnessDependency, WorthQueryProviderAuthorizationDecisionFacts,
    WorthQueryRetainedAuthorizationDecisionFacts,
};
pub use denial::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
#[cfg(test)]
pub(in crate::domain_computation) use graph_work_session::bind_operation_execution_authority;
pub(in crate::domain_computation) use graph_work_session::WorthQueryOperationGraphWorkSession;
pub(in crate::domain_computation) use installed_policy::WorthQueryInstalledAuthorizationRegistry;
pub use operation_scope_binding::{
    WorthQueryOperationScopeBinding, WorthQueryOperationScopeEntityBinding,
};
pub use prepared_capability_access::WorthQueryPreparedApplicationCapabilityAccess;
pub(in crate::domain_computation) use retained_capability_request::WorthQueryRetainedCapabilityRequest;
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
