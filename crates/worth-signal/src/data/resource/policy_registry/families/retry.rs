use super::super::super::declaration::ResourceNodeDeclaration;
use super::super::super::policy::{ResourceRetryBudgetScope, ResourceRetryPolicyDeclaration};
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_retry(
        &self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        let policy = declaration.retry_policy();
        let max_attempts = declaration.retry_max_attempts();
        let max_jitter = declaration.retry_deterministic_jitter();
        let retry_budget_scope = declaration.retry_budget_scope();
        let retry_budget_limit = declaration.retry_budget_limit();
        Ok(match policy {
            ResourceRetryPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                retry_parameter_digest(
                    "disabled",
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ),
            )?,
            ResourceRetryPolicyDeclaration::FixedDelay { delay }
            | ResourceRetryPolicyDeclaration::RuntimeBackoff { delay } => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.fixed-delay",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                retry_parameter_digest(
                    &format!("fixed-delay:{}", delay.get()),
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ),
            )?,
            ResourceRetryPolicyDeclaration::ExponentialBackoff {
                initial_delay,
                multiplier,
            } => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.exponential-backoff",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                retry_parameter_digest(
                    &format!("exponential-backoff:{}:{}", initial_delay.get(), multiplier),
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ),
            )?,
            ResourceRetryPolicyDeclaration::CappedExponentialBackoff {
                initial_delay,
                multiplier,
                max_delay,
            } => self.built_in_policy(
                ResourcePolicyKind::Retry,
                "signal.resource.retry.capped-exponential-backoff",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                retry_parameter_digest(
                    &format!(
                        "capped-exponential-backoff:{}:{}:{}",
                        initial_delay.get(),
                        multiplier,
                        max_delay.get()
                    ),
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ),
            )?,
            ResourceRetryPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Retry, name)?
            }
        })
    }
}

pub(super) fn retry_parameter_digest(
    base: &str,
    max_attempts: Option<u32>,
    max_jitter: Option<crate::data::temporal::TemporalDuration>,
    retry_budget_scope: Option<ResourceRetryBudgetScope>,
    retry_budget_limit: Option<u32>,
) -> ResourcePolicyDigest {
    ResourcePolicyDigest::new(format!(
        "retry:{}:max-attempts:{}:deterministic-jitter:{}:retry-budget-scope:{}:retry-budget-limit:{}",
        base,
        max_attempts
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unbounded".to_owned()),
        max_jitter
            .map(|value| value.get().to_string())
            .unwrap_or_else(|| "none".to_owned()),
        retry_budget_scope
            .map(|value| value.as_str().to_owned())
            .unwrap_or_else(|| "none".to_owned()),
        retry_budget_limit
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unbounded".to_owned())
    ))
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            0,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.disabled",
            5,
        ),
        (
            1,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.fixed-delay",
            5,
        ),
        (
            14,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.exponential-backoff",
            5,
        ),
        (
            15,
            ResourcePolicyKind::Retry,
            "signal.resource.retry.capped-exponential-backoff",
            5,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
