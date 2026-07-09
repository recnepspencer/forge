use serde::Serialize;

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};

use super::ResourceRevalidationPolicyDeclaration;

const REVALIDATION_EXPLICIT_INTENT_ONLY_NAME: &str =
    "signal.resource.revalidation.explicit-intent-only";
const REVALIDATION_EXPLICIT_OR_ACTIVE_HANDLE_FORCED_NAME: &str =
    "signal.resource.revalidation.explicit-or-active-handle-forced";
const REVALIDATION_EXPLICIT_OR_STALE_AFTER_FULFILLED_NAME: &str =
    "signal.resource.revalidation.explicit-or-stale-after-fulfilled";
const REVALIDATION_EXPLICIT_OR_STALE_AFTER_FULFILLED_OR_ACTIVE_HANDLE_FORCED_NAME: &str =
    "signal.resource.revalidation.explicit-or-stale-after-fulfilled-or-active-handle-forced";
const REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_NAME: &str =
    "signal.resource.revalidation.explicit-or-dependency-change";
const REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_ACTIVE_HANDLE_FORCED_NAME: &str =
    "signal.resource.revalidation.explicit-or-dependency-change-or-active-handle-forced";
const REVALIDATION_EXPLICIT_OR_OBSERVER_DEMAND_NAME: &str =
    "signal.resource.revalidation.explicit-or-observer-demand";
const REVALIDATION_EXPLICIT_OR_OBSERVER_DEMAND_OR_ACTIVE_HANDLE_FORCED_NAME: &str =
    "signal.resource.revalidation.explicit-or-observer-demand-or-active-handle-forced";
const REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_OBSERVER_DEMAND_NAME: &str =
    "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand";
const REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_OBSERVER_DEMAND_OR_ACTIVE_HANDLE_FORCED_NAME: &str =
    "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand-or-active-handle-forced";
const REVALIDATION_EXPLICIT_OR_TERMINAL_STATE_NAME: &str =
    "signal.resource.revalidation.explicit-or-terminal-state";
const REVALIDATION_EXPLICIT_OR_TERMINAL_STATE_OR_ACTIVE_HANDLE_FORCED_NAME: &str =
    "signal.resource.revalidation.explicit-or-terminal-state-or-active-handle-forced";
const REVALIDATION_EXPLICIT_OR_FULFILLED_LIFECYCLE_NAME: &str =
    "signal.resource.revalidation.explicit-or-fulfilled-lifecycle";
const REVALIDATION_EXPLICIT_OR_FULFILLED_LIFECYCLE_OR_ACTIVE_HANDLE_FORCED_NAME: &str =
    "signal.resource.revalidation.explicit-or-fulfilled-lifecycle-or-active-handle-forced";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceRevalidationDecisionClass {
    ExplicitIntentOnly,
    ExplicitOrActiveHandleForced,
    ExplicitOrStaleAfterFulfilled,
    ExplicitOrStaleAfterFulfilledOrActiveHandleForced,
    ExplicitOrDependencyChange,
    ExplicitOrDependencyChangeOrActiveHandleForced,
    ExplicitOrObserverDemand,
    ExplicitOrObserverDemandOrActiveHandleForced,
    ExplicitOrDependencyChangeOrObserverDemand,
    ExplicitOrDependencyChangeOrObserverDemandOrActiveHandleForced,
    ExplicitOrTerminalState,
    ExplicitOrTerminalStateOrActiveHandleForced,
    ExplicitOrFulfilledLifecycle,
    ExplicitOrFulfilledLifecycleOrActiveHandleForced,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceRevalidationDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceRevalidationDecisionClass,
    permits_active_handle_forcing: bool,
    permits_stale_after_revalidation: bool,
    stale_after_requires_fulfilled_lifecycle: bool,
    permits_dependency_change_revalidation: bool,
    permits_observer_demand_revalidation: bool,
    permits_terminal_state_revalidation: bool,
    permits_fulfilled_lifecycle_revalidation: bool,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceRevalidationDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceRevalidationPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceRevalidationPolicyDeclaration::ExplicitIntentOnly => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_INTENT_ONLY_NAME,
                    "explicit intent only revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitIntentOnly,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrActiveHandleForced => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_ACTIVE_HANDLE_FORCED_NAME,
                    "explicit or active-handle forced revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrActiveHandleForced,
                    true,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilled => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_STALE_AFTER_FULFILLED_NAME,
                    "explicit or stale-after fulfilled revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrStaleAfterFulfilled,
                    false,
                    true,
                    true,
                    false,
                    false,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilledOrActiveHandleForced => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_STALE_AFTER_FULFILLED_OR_ACTIVE_HANDLE_FORCED_NAME,
                    "explicit or stale-after fulfilled or active-handle forced revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrStaleAfterFulfilledOrActiveHandleForced,
                    true,
                    true,
                    true,
                    false,
                    false,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChange => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_NAME,
                    "explicit or dependency-change revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrDependencyChange,
                    false,
                    false,
                    false,
                    true,
                    false,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrActiveHandleForced => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_ACTIVE_HANDLE_FORCED_NAME,
                    "explicit or dependency-change or active-handle forced revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrDependencyChangeOrActiveHandleForced,
                    true,
                    false,
                    false,
                    true,
                    false,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemand => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_OBSERVER_DEMAND_NAME,
                    "explicit or observer-demand revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrObserverDemand,
                    false,
                    false,
                    false,
                    false,
                    true,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemandOrActiveHandleForced => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_OBSERVER_DEMAND_OR_ACTIVE_HANDLE_FORCED_NAME,
                    "explicit or observer-demand or active-handle forced revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrObserverDemandOrActiveHandleForced,
                    true,
                    false,
                    false,
                    false,
                    true,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemand => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_OBSERVER_DEMAND_NAME,
                    "explicit or dependency-change or observer-demand revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrDependencyChangeOrObserverDemand,
                    false,
                    false,
                    false,
                    true,
                    true,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemandOrActiveHandleForced => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_OBSERVER_DEMAND_OR_ACTIVE_HANDLE_FORCED_NAME,
                    "explicit or dependency-change or observer-demand or active-handle forced revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrDependencyChangeOrObserverDemandOrActiveHandleForced,
                    true,
                    false,
                    false,
                    true,
                    true,
                    false,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalState => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_TERMINAL_STATE_NAME,
                    "explicit or terminal-state revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrTerminalState,
                    false,
                    false,
                    false,
                    false,
                    false,
                    true,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalStateOrActiveHandleForced => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_TERMINAL_STATE_OR_ACTIVE_HANDLE_FORCED_NAME,
                    "explicit or terminal-state or active-handle forced revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrTerminalStateOrActiveHandleForced,
                    true,
                    false,
                    false,
                    false,
                    false,
                    true,
                    false,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycle => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_FULFILLED_LIFECYCLE_NAME,
                    "explicit or fulfilled-lifecycle revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrFulfilledLifecycle,
                    false,
                    false,
                    false,
                    false,
                    false,
                    false,
                    true,
                ))
            }
            ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycleOrActiveHandleForced => {
                ensure_descriptor_name(
                    frozen,
                    REVALIDATION_EXPLICIT_OR_FULFILLED_LIFECYCLE_OR_ACTIVE_HANDLE_FORCED_NAME,
                    "explicit or fulfilled-lifecycle or active-handle forced revalidation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRevalidationDecisionClass::ExplicitOrFulfilledLifecycleOrActiveHandleForced,
                    true,
                    false,
                    false,
                    false,
                    false,
                    false,
                    true,
                ))
            }
            ResourceRevalidationPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Revalidation,
                    name: name.clone(),
                    reason:
                        "named revalidation policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceRevalidationDecisionClass,
        permits_active_handle_forcing: bool,
        permits_stale_after_revalidation: bool,
        stale_after_requires_fulfilled_lifecycle: bool,
        permits_dependency_change_revalidation: bool,
        permits_observer_demand_revalidation: bool,
        permits_terminal_state_revalidation: bool,
        permits_fulfilled_lifecycle_revalidation: bool,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-revalidation-plan:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str(),
            if permits_active_handle_forcing {
                "active-handle-forcing-enabled"
            } else {
                "explicit-intent-only"
            },
            if permits_stale_after_revalidation {
                "stale-after-enabled"
            } else {
                "stale-after-disabled"
            },
            if stale_after_requires_fulfilled_lifecycle {
                "fulfilled-only-stale-after"
            } else {
                "no-stale-after-lifecycle-gate"
            },
            if permits_dependency_change_revalidation {
                "dependency-change-enabled"
            } else {
                "dependency-change-disabled"
            },
            if permits_observer_demand_revalidation {
                "observer-demand-enabled"
            } else {
                "observer-demand-disabled"
            },
            if permits_terminal_state_revalidation {
                "terminal-state-enabled"
            } else {
                "terminal-state-disabled"
            },
            if permits_fulfilled_lifecycle_revalidation {
                "fulfilled-lifecycle-enabled"
            } else {
                "fulfilled-lifecycle-disabled"
            },
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            permits_active_handle_forcing,
            permits_stale_after_revalidation,
            stale_after_requires_fulfilled_lifecycle,
            permits_dependency_change_revalidation,
            permits_observer_demand_revalidation,
            permits_terminal_state_revalidation,
            permits_fulfilled_lifecycle_revalidation,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceRevalidationDecisionClass {
        self.class
    }

    pub fn permits_active_handle_forcing(&self) -> bool {
        self.permits_active_handle_forcing
    }

    pub fn permits_stale_after_revalidation(&self) -> bool {
        self.permits_stale_after_revalidation
    }

    pub fn stale_after_requires_fulfilled_lifecycle(&self) -> bool {
        self.stale_after_requires_fulfilled_lifecycle
    }

    pub fn permits_dependency_change_revalidation(&self) -> bool {
        self.permits_dependency_change_revalidation
    }

    pub fn permits_observer_demand_revalidation(&self) -> bool {
        self.permits_observer_demand_revalidation
    }

    pub fn permits_terminal_state_revalidation(&self) -> bool {
        self.permits_terminal_state_revalidation
    }

    pub fn permits_fulfilled_lifecycle_revalidation(&self) -> bool {
        self.permits_fulfilled_lifecycle_revalidation
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

impl ResourceRevalidationDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitIntentOnly => "explicit-intent-only",
            Self::ExplicitOrActiveHandleForced => "explicit-or-active-handle-forced",
            Self::ExplicitOrStaleAfterFulfilled => "explicit-or-stale-after-fulfilled",
            Self::ExplicitOrStaleAfterFulfilledOrActiveHandleForced => {
                "explicit-or-stale-after-fulfilled-or-active-handle-forced"
            }
            Self::ExplicitOrDependencyChange => "explicit-or-dependency-change",
            Self::ExplicitOrDependencyChangeOrActiveHandleForced => {
                "explicit-or-dependency-change-or-active-handle-forced"
            }
            Self::ExplicitOrObserverDemand => "explicit-or-observer-demand",
            Self::ExplicitOrObserverDemandOrActiveHandleForced => {
                "explicit-or-observer-demand-or-active-handle-forced"
            }
            Self::ExplicitOrDependencyChangeOrObserverDemand => {
                "explicit-or-dependency-change-or-observer-demand"
            }
            Self::ExplicitOrDependencyChangeOrObserverDemandOrActiveHandleForced => {
                "explicit-or-dependency-change-or-observer-demand-or-active-handle-forced"
            }
            Self::ExplicitOrTerminalState => "explicit-or-terminal-state",
            Self::ExplicitOrTerminalStateOrActiveHandleForced => {
                "explicit-or-terminal-state-or-active-handle-forced"
            }
            Self::ExplicitOrFulfilledLifecycle => "explicit-or-fulfilled-lifecycle",
            Self::ExplicitOrFulfilledLifecycleOrActiveHandleForced => {
                "explicit-or-fulfilled-lifecycle-or-active-handle-forced"
            }
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
        kind: ResourcePolicyKind::Revalidation,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
