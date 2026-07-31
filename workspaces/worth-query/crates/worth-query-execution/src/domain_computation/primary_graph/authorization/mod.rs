mod admission;
mod admitted_capability_access;
mod admitted_operation;
mod bridge_observation;
mod capability_admission;
mod capability_binding_lowering;
mod capability_currentness;
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

pub use admitted_capability_access::WorthQueryAdmittedApplicationCapabilityAccess;
pub use admitted_operation::WorthQueryAdmittedApplicationOperation;
pub(in crate::domain_computation::primary_graph) use admitted_operation::WorthQueryOperationAdmissionIdentity;
pub use capability_registry::WorthQueryCapabilityPlanCompilationEvidence;
pub(in crate::domain_computation::primary_graph) use decision_facts::{
    WorthQueryAuthorizationDecisionFact, WorthQueryPrincipalCurrentnessDependency,
    WorthQueryRetainedAuthorizationDecisionFacts,
};
pub use denial::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
pub(in crate::domain_computation::primary_graph) use installed_policy::WorthQueryInstalledAuthorizationRegistry;
pub use operation_scope_binding::{
    WorthQueryOperationScopeBinding, WorthQueryOperationScopeEntityBinding,
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
