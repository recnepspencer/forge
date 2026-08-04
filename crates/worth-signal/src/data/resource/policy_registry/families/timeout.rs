use super::super::super::policy::ResourceTimeoutPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_timeout(
        &self,
        policy: &ResourceTimeoutPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceTimeoutPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("timeout:disabled"),
            )?,
            ResourceTimeoutPolicyDeclaration::TransactionInheritedDeadline => self
                .built_in_policy(
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.transaction-inherited-deadline",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("timeout:transaction-inherited-deadline"),
                )?,
            ResourceTimeoutPolicyDeclaration::RuntimeInheritedDeadline => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.runtime-inherited-deadline",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("timeout:runtime-inherited-deadline"),
            )?,
            ResourceTimeoutPolicyDeclaration::PerAttemptTimeout { timeout }
            | ResourceTimeoutPolicyDeclaration::FixedTimeout { timeout }
            | ResourceTimeoutPolicyDeclaration::RuntimeTimeout { timeout } => self
                .built_in_policy(
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.fixed-timeout",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    timeout_parameter_digest("fixed-timeout", *timeout),
                )?,
            ResourceTimeoutPolicyDeclaration::TotalRequestLifetimeTimeout { timeout } => self
                .built_in_policy(
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.total-request-lifetime-timeout",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    timeout_parameter_digest("total-request-lifetime-timeout", *timeout),
                )?,
            ResourceTimeoutPolicyDeclaration::ProgressHeartbeatExtension {
                timeout,
                heartbeat_extension,
            } => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.progress-heartbeat-extension",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new(format!(
                    "timeout:progress-heartbeat-extension:{}:{}",
                    timeout.get(),
                    heartbeat_extension.get()
                )),
            )?,
            ResourceTimeoutPolicyDeclaration::TerminalTimeout { timeout } => self.built_in_policy(
                ResourcePolicyKind::Timeout,
                "signal.resource.timeout.terminal-timeout",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                timeout_parameter_digest("terminal-timeout", *timeout),
            )?,
            ResourceTimeoutPolicyDeclaration::RevalidationEligibleTimeout { timeout } => self
                .built_in_policy(
                    ResourcePolicyKind::Timeout,
                    "signal.resource.timeout.revalidation-eligible-timeout",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    timeout_parameter_digest("revalidation-eligible-timeout", *timeout),
                )?,
            ResourceTimeoutPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Timeout, name)?
            }
        })
    }
}

pub(super) fn timeout_parameter_digest(
    family: &'static str,
    timeout: crate::data::temporal::TemporalDuration,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!("timeout:{family}:{}", timeout.get()))
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            2,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.disabled",
            4,
        ),
        (
            20,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.transaction-inherited-deadline",
            4,
        ),
        (
            21,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.runtime-inherited-deadline",
            4,
        ),
        (
            3,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.fixed-timeout",
            4,
        ),
        (
            16,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.total-request-lifetime-timeout",
            4,
        ),
        (
            17,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.progress-heartbeat-extension",
            4,
        ),
        (
            18,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.terminal-timeout",
            4,
        ),
        (
            19,
            ResourcePolicyKind::Timeout,
            "signal.resource.timeout.revalidation-eligible-timeout",
            4,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
