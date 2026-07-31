mod admission;
mod admitted_operation;
mod bridge_observation;
mod denial;
mod installed_policy;
mod lowering;

pub use admitted_operation::{
    WorthQueryAdmittedApplicationOperation, WorthQueryOperationScopeFingerprint,
};
pub(in crate::domain_computation::primary_graph) use admitted_operation::{
    WorthQueryAuthorizationCommitDependency, WorthQueryOperationAdmissionIdentity,
};
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
