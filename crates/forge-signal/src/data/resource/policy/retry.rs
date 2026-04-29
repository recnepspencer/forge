use serde::{Deserialize, Serialize};

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};
use crate::data::resource::request::ResourceAttemptId;
use crate::data::resource::request::ResourceRequestHandle;
use crate::data::temporal::TemporalDuration;

use super::super::declaration::ResourceNodeDeclaration;
use super::ResourceRetryPolicyDeclaration;

const RETRY_DISABLED_NAME: &str = "signal.resource.retry.disabled";
const RETRY_FIXED_DELAY_NAME: &str = "signal.resource.retry.fixed-delay";
const RETRY_EXPONENTIAL_BACKOFF_NAME: &str = "signal.resource.retry.exponential-backoff";
const RETRY_CAPPED_EXPONENTIAL_BACKOFF_NAME: &str =
    "signal.resource.retry.capped-exponential-backoff";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRetryBudgetScope {
    Request,
    ResourceNode,
    Runtime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceRetryDecisionClass {
    Disabled,
    FixedDelay,
    ExponentialBackoff,
    CappedExponentialBackoff,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceRetryDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceRetryDecisionClass,
    initial_delay: Option<TemporalDuration>,
    multiplier: Option<u32>,
    max_delay: Option<TemporalDuration>,
    max_attempts: Option<u32>,
    max_jitter: Option<TemporalDuration>,
    retry_budget_scope: Option<ResourceRetryBudgetScope>,
    retry_budget_limit: Option<u32>,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceRetryDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceNodeDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        let policy = declaration.retry_policy();
        let max_attempts = declaration.retry_max_attempts();
        let max_jitter = declaration.retry_deterministic_jitter();
        let retry_budget_scope = declaration.retry_budget_scope();
        let retry_budget_limit = declaration.retry_budget_limit();
        match policy {
            ResourceRetryPolicyDeclaration::Disabled => {
                ensure_descriptor_name(frozen, RETRY_DISABLED_NAME, "disabled retry")?;
                Ok(Self::new(
                    frozen,
                    ResourceRetryDecisionClass::Disabled,
                    None,
                    None,
                    None,
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ))
            }
            ResourceRetryPolicyDeclaration::FixedDelay { delay }
            | ResourceRetryPolicyDeclaration::RuntimeBackoff { delay } => {
                ensure_descriptor_name(frozen, RETRY_FIXED_DELAY_NAME, "fixed-delay retry")?;
                Ok(Self::new(
                    frozen,
                    ResourceRetryDecisionClass::FixedDelay,
                    Some(*delay),
                    None,
                    None,
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ))
            }
            ResourceRetryPolicyDeclaration::ExponentialBackoff {
                initial_delay,
                multiplier,
            } => {
                ensure_descriptor_name(
                    frozen,
                    RETRY_EXPONENTIAL_BACKOFF_NAME,
                    "exponential backoff retry",
                )?;
                validate_multiplier(*multiplier, frozen)?;
                Ok(Self::new(
                    frozen,
                    ResourceRetryDecisionClass::ExponentialBackoff,
                    Some(*initial_delay),
                    Some(*multiplier),
                    None,
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ))
            }
            ResourceRetryPolicyDeclaration::CappedExponentialBackoff {
                initial_delay,
                multiplier,
                max_delay,
            } => {
                ensure_descriptor_name(
                    frozen,
                    RETRY_CAPPED_EXPONENTIAL_BACKOFF_NAME,
                    "capped exponential backoff retry",
                )?;
                validate_multiplier(*multiplier, frozen)?;
                Ok(Self::new(
                    frozen,
                    ResourceRetryDecisionClass::CappedExponentialBackoff,
                    Some(*initial_delay),
                    Some(*multiplier),
                    Some(*max_delay),
                    max_attempts,
                    max_jitter,
                    retry_budget_scope,
                    retry_budget_limit,
                ))
            }
            ResourceRetryPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Retry,
                    name: name.clone(),
                    reason: "named retry policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceRetryDecisionClass,
        initial_delay: Option<TemporalDuration>,
        multiplier: Option<u32>,
        max_delay: Option<TemporalDuration>,
        max_attempts: Option<u32>,
        max_jitter: Option<TemporalDuration>,
        retry_budget_scope: Option<ResourceRetryBudgetScope>,
        retry_budget_limit: Option<u32>,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-retry-plan:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str(),
            initial_delay
                .map(|value| value.get().to_string())
                .unwrap_or_else(|| "none".to_owned()),
            multiplier
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_owned()),
            max_delay
                .map(|value| value.get().to_string())
                .unwrap_or_else(|| "none".to_owned()),
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
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            initial_delay,
            multiplier,
            max_delay,
            max_attempts,
            max_jitter,
            retry_budget_scope,
            retry_budget_limit,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceRetryDecisionClass {
        self.class
    }

    pub fn initial_delay(&self) -> Option<TemporalDuration> {
        self.initial_delay
    }

    pub fn multiplier(&self) -> Option<u32> {
        self.multiplier
    }

    pub fn max_delay(&self) -> Option<TemporalDuration> {
        self.max_delay
    }

    pub fn max_attempts(&self) -> Option<u32> {
        self.max_attempts
    }

    pub fn max_jitter(&self) -> Option<TemporalDuration> {
        self.max_jitter
    }

    pub fn retry_budget_scope(&self) -> Option<ResourceRetryBudgetScope> {
        self.retry_budget_scope
    }

    pub fn retry_budget_limit(&self) -> Option<u32> {
        self.retry_budget_limit
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }

    pub fn admits_attempt(&self, next_attempt: ResourceAttemptId) -> bool {
        self.max_attempts
            .map(|max_attempts| next_attempt.get() < u64::from(max_attempts))
            .unwrap_or(true)
    }

    pub fn delay_for_attempt(
        &self,
        previous_handle: ResourceRequestHandle,
        next_attempt: ResourceAttemptId,
    ) -> Option<TemporalDuration> {
        let base_delay = match self.class {
            ResourceRetryDecisionClass::Disabled => None,
            ResourceRetryDecisionClass::FixedDelay => self.initial_delay,
            ResourceRetryDecisionClass::ExponentialBackoff => Some(exponential_delay(
                self.initial_delay.expect("fixed exponential initial delay"),
                self.multiplier.expect("fixed exponential multiplier"),
                next_attempt,
                None,
            )),
            ResourceRetryDecisionClass::CappedExponentialBackoff => Some(exponential_delay(
                self.initial_delay.expect("fixed capped initial delay"),
                self.multiplier.expect("fixed capped multiplier"),
                next_attempt,
                self.max_delay,
            )),
        }?;
        Some(apply_deterministic_jitter(
            base_delay,
            self.max_jitter,
            self.decision_digest(),
            previous_handle,
            next_attempt,
        ))
    }
}

fn apply_deterministic_jitter(
    base_delay: TemporalDuration,
    max_jitter: Option<TemporalDuration>,
    decision_digest: &ResourcePolicyDigest,
    previous_handle: ResourceRequestHandle,
    next_attempt: ResourceAttemptId,
) -> TemporalDuration {
    let Some(max_jitter) = max_jitter else {
        return base_delay;
    };
    let jitter_window = max_jitter.get();
    if jitter_window == 0 {
        return base_delay;
    }
    let seed = deterministic_seed(decision_digest, previous_handle, next_attempt);
    let jitter = seed % (jitter_window.saturating_add(1));
    TemporalDuration::temporal_duration(base_delay.get().saturating_add(jitter))
        .expect("jittered retry delay stays positive")
}

fn deterministic_seed(
    decision_digest: &ResourcePolicyDigest,
    previous_handle: ResourceRequestHandle,
    next_attempt: ResourceAttemptId,
) -> u64 {
    let mut hash = 1469598103934665603u64;
    for byte in decision_digest.as_str().bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(1099511628211);
    }
    hash ^= previous_handle.request_id().get();
    hash = hash.wrapping_mul(1099511628211);
    hash ^= previous_handle.generation().get();
    hash = hash.wrapping_mul(1099511628211);
    hash ^= previous_handle.branch_epoch().restore_epoch();
    hash = hash.wrapping_mul(1099511628211);
    hash ^= next_attempt.get();
    hash
}

impl ResourceRetryDecisionClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::FixedDelay => "fixed-delay",
            Self::ExponentialBackoff => "exponential-backoff",
            Self::CappedExponentialBackoff => "capped-exponential-backoff",
        }
    }
}

impl ResourceRetryBudgetScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Request => "request",
            Self::ResourceNode => "resource-node",
            Self::Runtime => "runtime",
        }
    }
}

fn ensure_descriptor_name(
    frozen: &FrozenResourcePolicyDescriptor,
    expected: &str,
    reason: &'static str,
) -> Result<(), ResourcePolicyResolutionError> {
    if frozen.descriptor().semantic_name().as_str() == expected {
        return Ok(());
    }
    Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
        kind: ResourcePolicyKind::Retry,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}

fn validate_multiplier(
    multiplier: u32,
    frozen: &FrozenResourcePolicyDescriptor,
) -> Result<(), ResourcePolicyResolutionError> {
    if multiplier >= 2 {
        return Ok(());
    }
    Err(ResourcePolicyResolutionError::MalformedDescriptor {
        kind: ResourcePolicyKind::Retry,
        name: frozen.descriptor().semantic_name().clone(),
        reason: "retry multiplier must be at least 2",
    })
}

fn exponential_delay(
    initial_delay: TemporalDuration,
    multiplier: u32,
    attempt: ResourceAttemptId,
    max_delay: Option<TemporalDuration>,
) -> TemporalDuration {
    let mut delay = u128::from(initial_delay.get());
    let factor = u128::from(multiplier);
    for _ in 0..attempt.get() {
        delay = delay.saturating_mul(factor);
    }
    let mut value = delay.min(u128::from(u64::MAX)) as u64;
    if let Some(max_delay) = max_delay {
        value = value.min(max_delay.get());
    }
    TemporalDuration::temporal_duration(value).expect("retry delay stays positive")
}
