mod admission;
mod admitted_operation;
mod bridge_observation;
mod capability_binding_lowering;
mod capability_admission;
mod capability_currentness;
mod capability_lowering;
mod capability_observation;
mod capability_registry;
mod capability_request_resolution;
mod denial;
mod installed_policy;
mod lowering;
mod operation_scope_binding;

pub use admitted_operation::{
    WorthQueryAdmittedApplicationOperation,
};
pub use operation_scope_binding::{
    WorthQueryOperationScopeBinding, WorthQueryOperationScopeEntityBinding,
};
pub(in crate::domain_computation::primary_graph) use admitted_operation::{
    WorthQueryAuthorizationCommitDependency, WorthQueryOperationAdmissionIdentity,
};
pub use capability_registry::WorthQueryCapabilityPlanCompilationEvidence;
pub use denial::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
pub(in crate::domain_computation::primary_graph) use installed_policy::WorthQueryInstalledAuthorizationRegistry;

fn authorization_denial(
    subject: impl Into<String>,
    _detail: &'static str,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::InvalidInstalledPolicy,
        subject,
    )
}
