use serde::{Deserialize, Serialize};

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};
use crate::data::temporal::{ClockTick, TemporalDuration};

use super::ResourceTimeoutPolicyDeclaration;

const TIMEOUT_DISABLED_NAME: &str = "signal.resource.timeout.disabled";
const TIMEOUT_TRANSACTION_INHERITED_NAME: &str =
    "signal.resource.timeout.transaction-inherited-deadline";
const TIMEOUT_RUNTIME_INHERITED_NAME: &str = "signal.resource.timeout.runtime-inherited-deadline";
const TIMEOUT_FIXED_NAME: &str = "signal.resource.timeout.fixed-timeout";
const TIMEOUT_TOTAL_REQUEST_LIFETIME_NAME: &str =
    "signal.resource.timeout.total-request-lifetime-timeout";
const TIMEOUT_HEARTBEAT_EXTENSION_NAME: &str =
    "signal.resource.timeout.progress-heartbeat-extension";
const TIMEOUT_TERMINAL_NAME: &str = "signal.resource.timeout.terminal-timeout";
const TIMEOUT_REVALIDATION_ELIGIBLE_NAME: &str =
    "signal.resource.timeout.revalidation-eligible-timeout";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceTimeoutOutcomeClass {
    Terminal,
    RevalidationEligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceTimeoutDecisionClass {
    Disabled,
    TransactionInheritedDeadline,
    RuntimeInheritedDeadline,
    FixedTimeout,
    TotalRequestLifetimeTimeout,
    ProgressHeartbeatExtension,
    TerminalTimeout,
    RevalidationEligibleTimeout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceTimeoutDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceTimeoutDecisionClass,
    timeout: Option<TemporalDuration>,
    heartbeat_extension: Option<TemporalDuration>,
    outcome_class: ResourceTimeoutOutcomeClass,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceTimeoutDecisionPlan {
    pub(crate) fn disabled_builtin_default() -> Self {
        Self {
            descriptor_id: ResourcePolicyDescriptorId::new(u64::MAX),
            semantic_name: TIMEOUT_DISABLED_NAME.to_owned(),
            class: ResourceTimeoutDecisionClass::Disabled,
            timeout: None,
            heartbeat_extension: None,
            outcome_class: ResourceTimeoutOutcomeClass::Terminal,
            decision_digest: ResourcePolicyDigest::new(
                "resource-policy-timeout-plan:disabled-default",
            ),
        }
    }

    pub(crate) fn lower(
        declaration: &ResourceTimeoutPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceTimeoutPolicyDeclaration::Disabled => {
                ensure_descriptor_name(frozen, TIMEOUT_DISABLED_NAME, "disabled timeout")?;
                Ok(Self::new(
                    frozen,
                    ResourceTimeoutDecisionClass::Disabled,
                    None,
                    None,
                    ResourceTimeoutOutcomeClass::Terminal,
                ))
            }
            ResourceTimeoutPolicyDeclaration::TransactionInheritedDeadline => {
                ensure_descriptor_name(
                    frozen,
                    TIMEOUT_TRANSACTION_INHERITED_NAME,
                    "transaction inherited deadline",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceTimeoutDecisionClass::TransactionInheritedDeadline,
                    None,
                    None,
                    ResourceTimeoutOutcomeClass::Terminal,
                ))
            }
            ResourceTimeoutPolicyDeclaration::RuntimeInheritedDeadline => {
                ensure_descriptor_name(
                    frozen,
                    TIMEOUT_RUNTIME_INHERITED_NAME,
                    "runtime inherited deadline",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceTimeoutDecisionClass::RuntimeInheritedDeadline,
                    None,
                    None,
                    ResourceTimeoutOutcomeClass::Terminal,
                ))
            }
            ResourceTimeoutPolicyDeclaration::PerAttemptTimeout { timeout }
            | ResourceTimeoutPolicyDeclaration::FixedTimeout { timeout }
            | ResourceTimeoutPolicyDeclaration::RuntimeTimeout { timeout } => {
                ensure_descriptor_name(frozen, TIMEOUT_FIXED_NAME, "fixed timeout")?;
                Ok(Self::new(
                    frozen,
                    ResourceTimeoutDecisionClass::FixedTimeout,
                    Some(*timeout),
                    None,
                    ResourceTimeoutOutcomeClass::Terminal,
                ))
            }
            ResourceTimeoutPolicyDeclaration::TotalRequestLifetimeTimeout { timeout } => {
                ensure_descriptor_name(
                    frozen,
                    TIMEOUT_TOTAL_REQUEST_LIFETIME_NAME,
                    "total request lifetime timeout",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceTimeoutDecisionClass::TotalRequestLifetimeTimeout,
                    Some(*timeout),
                    None,
                    ResourceTimeoutOutcomeClass::Terminal,
                ))
            }
            ResourceTimeoutPolicyDeclaration::ProgressHeartbeatExtension {
                timeout,
                heartbeat_extension,
            } => {
                ensure_descriptor_name(
                    frozen,
                    TIMEOUT_HEARTBEAT_EXTENSION_NAME,
                    "progress heartbeat extension timeout",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceTimeoutDecisionClass::ProgressHeartbeatExtension,
                    Some(*timeout),
                    Some(*heartbeat_extension),
                    ResourceTimeoutOutcomeClass::Terminal,
                ))
            }
            ResourceTimeoutPolicyDeclaration::TerminalTimeout { timeout } => {
                ensure_descriptor_name(frozen, TIMEOUT_TERMINAL_NAME, "terminal timeout")?;
                Ok(Self::new(
                    frozen,
                    ResourceTimeoutDecisionClass::TerminalTimeout,
                    Some(*timeout),
                    None,
                    ResourceTimeoutOutcomeClass::Terminal,
                ))
            }
            ResourceTimeoutPolicyDeclaration::RevalidationEligibleTimeout { timeout } => {
                ensure_descriptor_name(
                    frozen,
                    TIMEOUT_REVALIDATION_ELIGIBLE_NAME,
                    "revalidation-eligible timeout",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceTimeoutDecisionClass::RevalidationEligibleTimeout,
                    Some(*timeout),
                    None,
                    ResourceTimeoutOutcomeClass::RevalidationEligible,
                ))
            }
            ResourceTimeoutPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Timeout,
                    name: name.clone(),
                    reason: "named timeout policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceTimeoutDecisionClass,
        timeout: Option<TemporalDuration>,
        heartbeat_extension: Option<TemporalDuration>,
        outcome_class: ResourceTimeoutOutcomeClass,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-timeout-plan:{}:{}:{}:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str(),
            timeout
                .map(|value| value.get().to_string())
                .unwrap_or_else(|| "none".to_owned()),
            heartbeat_extension
                .map(|value| value.get().to_string())
                .unwrap_or_else(|| "none".to_owned()),
            outcome_class.as_str(),
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            timeout,
            heartbeat_extension,
            outcome_class,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceTimeoutDecisionClass {
        self.class
    }

    pub fn timeout(&self) -> Option<TemporalDuration> {
        self.timeout
    }

    pub fn heartbeat_extension(&self) -> Option<TemporalDuration> {
        self.heartbeat_extension
    }

    pub fn outcome_class(&self) -> ResourceTimeoutOutcomeClass {
        self.outcome_class
    }

    pub fn timeout_for_lineage(
        &self,
        current_tick: ClockTick,
        generation_started_tick: ClockTick,
    ) -> Option<TemporalDuration> {
        let timeout = self.timeout?;
        match self.class {
            ResourceTimeoutDecisionClass::Disabled => None,
            ResourceTimeoutDecisionClass::TransactionInheritedDeadline
            | ResourceTimeoutDecisionClass::RuntimeInheritedDeadline => None,
            ResourceTimeoutDecisionClass::FixedTimeout
            | ResourceTimeoutDecisionClass::ProgressHeartbeatExtension
            | ResourceTimeoutDecisionClass::TerminalTimeout
            | ResourceTimeoutDecisionClass::RevalidationEligibleTimeout => Some(timeout),
            ResourceTimeoutDecisionClass::TotalRequestLifetimeTimeout => {
                let elapsed = current_tick
                    .get()
                    .saturating_sub(generation_started_tick.get());
                let remaining = timeout.get().saturating_sub(elapsed);
                if remaining == 0 {
                    None
                } else {
                    Some(
                        TemporalDuration::temporal_duration(remaining)
                            .expect("positive remaining timeout must stay valid"),
                    )
                }
            }
        }
    }

    pub fn retry_window_exhausted(
        &self,
        current_tick: ClockTick,
        generation_started_tick: ClockTick,
    ) -> bool {
        matches!(
            self.class,
            ResourceTimeoutDecisionClass::TotalRequestLifetimeTimeout
        ) && self.timeout.is_some_and(|timeout| {
            current_tick
                .get()
                .saturating_sub(generation_started_tick.get())
                >= timeout.get()
        })
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }

    pub fn allows_heartbeat_extension(&self) -> bool {
        matches!(
            self.class,
            ResourceTimeoutDecisionClass::ProgressHeartbeatExtension
        ) && self.heartbeat_extension.is_some()
    }
}

impl ResourceTimeoutDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::TransactionInheritedDeadline => "transaction-inherited-deadline",
            Self::RuntimeInheritedDeadline => "runtime-inherited-deadline",
            Self::FixedTimeout => "fixed-timeout",
            Self::TotalRequestLifetimeTimeout => "total-request-lifetime-timeout",
            Self::ProgressHeartbeatExtension => "progress-heartbeat-extension",
            Self::TerminalTimeout => "terminal-timeout",
            Self::RevalidationEligibleTimeout => "revalidation-eligible-timeout",
        }
    }
}

impl ResourceTimeoutOutcomeClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::RevalidationEligible => "revalidation-eligible",
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
        kind: ResourcePolicyKind::Timeout,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
