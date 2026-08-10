use serde::Serialize;

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyKind, ResourcePolicyResolutionError,
};

use super::super::ResourceRevalidationPolicyDeclaration;

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
const REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_OBSERVER_DEMAND_OR_ACTIVE_HANDLE_FORCED_NAME:
    &str = "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand-or-active-handle-forced";
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

pub(super) struct RevalidationPolicyShape {
    pub(super) descriptor_name: &'static str,
    pub(super) reason: &'static str,
    pub(super) class: ResourceRevalidationDecisionClass,
    pub(super) permits_active_handle_forcing: bool,
    pub(super) permits_stale_after_revalidation: bool,
    pub(super) stale_after_requires_fulfilled_lifecycle: bool,
    pub(super) permits_dependency_change_revalidation: bool,
    pub(super) permits_observer_demand_revalidation: bool,
    pub(super) permits_terminal_state_revalidation: bool,
    pub(super) permits_fulfilled_lifecycle_revalidation: bool,
}

pub(super) fn shape_for_declaration(
    declaration: &ResourceRevalidationPolicyDeclaration,
) -> Result<RevalidationPolicyShape, ResourcePolicyResolutionError> {
    let shape = match declaration {
        ResourceRevalidationPolicyDeclaration::ExplicitIntentOnly => RevalidationPolicyShape {
            descriptor_name: REVALIDATION_EXPLICIT_INTENT_ONLY_NAME,
            reason: "explicit intent only revalidation",
            class: ResourceRevalidationDecisionClass::ExplicitIntentOnly,
            permits_active_handle_forcing: false,
            permits_stale_after_revalidation: false,
            stale_after_requires_fulfilled_lifecycle: false,
            permits_dependency_change_revalidation: false,
            permits_observer_demand_revalidation: false,
            permits_terminal_state_revalidation: false,
            permits_fulfilled_lifecycle_revalidation: false,
        },
        ResourceRevalidationPolicyDeclaration::ExplicitOrActiveHandleForced => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_ACTIVE_HANDLE_FORCED_NAME,
                reason: "explicit or active-handle forced revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrActiveHandleForced,
                permits_active_handle_forcing: true,
                permits_stale_after_revalidation: false,
                stale_after_requires_fulfilled_lifecycle: false,
                permits_dependency_change_revalidation: false,
                permits_observer_demand_revalidation: false,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: false,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilled => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_STALE_AFTER_FULFILLED_NAME,
                reason: "explicit or stale-after fulfilled revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrStaleAfterFulfilled,
                permits_active_handle_forcing: false,
                permits_stale_after_revalidation: true,
                stale_after_requires_fulfilled_lifecycle: true,
                permits_dependency_change_revalidation: false,
                permits_observer_demand_revalidation: false,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: false,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilledOrActiveHandleForced => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_STALE_AFTER_FULFILLED_OR_ACTIVE_HANDLE_FORCED_NAME,
                reason: "explicit or stale-after fulfilled or active-handle forced revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrStaleAfterFulfilledOrActiveHandleForced,
                permits_active_handle_forcing: true,
                permits_stale_after_revalidation: true,
                stale_after_requires_fulfilled_lifecycle: true,
                permits_dependency_change_revalidation: false,
                permits_observer_demand_revalidation: false,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: false,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChange => RevalidationPolicyShape {
            descriptor_name: REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_NAME,
            reason: "explicit or dependency-change revalidation",
            class: ResourceRevalidationDecisionClass::ExplicitOrDependencyChange,
            permits_active_handle_forcing: false,
            permits_stale_after_revalidation: false,
            stale_after_requires_fulfilled_lifecycle: false,
            permits_dependency_change_revalidation: true,
            permits_observer_demand_revalidation: false,
            permits_terminal_state_revalidation: false,
            permits_fulfilled_lifecycle_revalidation: false,
        },
        ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrActiveHandleForced => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_ACTIVE_HANDLE_FORCED_NAME,
                reason: "explicit or dependency-change or active-handle forced revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrDependencyChangeOrActiveHandleForced,
                permits_active_handle_forcing: true,
                permits_stale_after_revalidation: false,
                stale_after_requires_fulfilled_lifecycle: false,
                permits_dependency_change_revalidation: true,
                permits_observer_demand_revalidation: false,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: false,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemand => RevalidationPolicyShape {
            descriptor_name: REVALIDATION_EXPLICIT_OR_OBSERVER_DEMAND_NAME,
            reason: "explicit or observer-demand revalidation",
            class: ResourceRevalidationDecisionClass::ExplicitOrObserverDemand,
            permits_active_handle_forcing: false,
            permits_stale_after_revalidation: false,
            stale_after_requires_fulfilled_lifecycle: false,
            permits_dependency_change_revalidation: false,
            permits_observer_demand_revalidation: true,
            permits_terminal_state_revalidation: false,
            permits_fulfilled_lifecycle_revalidation: false,
        },
        ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemandOrActiveHandleForced => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_OBSERVER_DEMAND_OR_ACTIVE_HANDLE_FORCED_NAME,
                reason: "explicit or observer-demand or active-handle forced revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrObserverDemandOrActiveHandleForced,
                permits_active_handle_forcing: true,
                permits_stale_after_revalidation: false,
                stale_after_requires_fulfilled_lifecycle: false,
                permits_dependency_change_revalidation: false,
                permits_observer_demand_revalidation: true,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: false,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemand => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_OBSERVER_DEMAND_NAME,
                reason: "explicit or dependency-change or observer-demand revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrDependencyChangeOrObserverDemand,
                permits_active_handle_forcing: false,
                permits_stale_after_revalidation: false,
                stale_after_requires_fulfilled_lifecycle: false,
                permits_dependency_change_revalidation: true,
                permits_observer_demand_revalidation: true,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: false,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemandOrActiveHandleForced => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_DEPENDENCY_CHANGE_OR_OBSERVER_DEMAND_OR_ACTIVE_HANDLE_FORCED_NAME,
                reason: "explicit or dependency-change or observer-demand or active-handle forced revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrDependencyChangeOrObserverDemandOrActiveHandleForced,
                permits_active_handle_forcing: true,
                permits_stale_after_revalidation: false,
                stale_after_requires_fulfilled_lifecycle: false,
                permits_dependency_change_revalidation: true,
                permits_observer_demand_revalidation: true,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: false,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalState => RevalidationPolicyShape {
            descriptor_name: REVALIDATION_EXPLICIT_OR_TERMINAL_STATE_NAME,
            reason: "explicit or terminal-state revalidation",
            class: ResourceRevalidationDecisionClass::ExplicitOrTerminalState,
            permits_active_handle_forcing: false,
            permits_stale_after_revalidation: false,
            stale_after_requires_fulfilled_lifecycle: false,
            permits_dependency_change_revalidation: false,
            permits_observer_demand_revalidation: false,
            permits_terminal_state_revalidation: true,
            permits_fulfilled_lifecycle_revalidation: false,
        },
        ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalStateOrActiveHandleForced => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_TERMINAL_STATE_OR_ACTIVE_HANDLE_FORCED_NAME,
                reason: "explicit or terminal-state or active-handle forced revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrTerminalStateOrActiveHandleForced,
                permits_active_handle_forcing: true,
                permits_stale_after_revalidation: false,
                stale_after_requires_fulfilled_lifecycle: false,
                permits_dependency_change_revalidation: false,
                permits_observer_demand_revalidation: false,
                permits_terminal_state_revalidation: true,
                permits_fulfilled_lifecycle_revalidation: false,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycle => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_FULFILLED_LIFECYCLE_NAME,
                reason: "explicit or fulfilled-lifecycle revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrFulfilledLifecycle,
                permits_active_handle_forcing: false,
                permits_stale_after_revalidation: false,
                stale_after_requires_fulfilled_lifecycle: false,
                permits_dependency_change_revalidation: false,
                permits_observer_demand_revalidation: false,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: true,
            }
        }
        ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycleOrActiveHandleForced => {
            RevalidationPolicyShape {
                descriptor_name: REVALIDATION_EXPLICIT_OR_FULFILLED_LIFECYCLE_OR_ACTIVE_HANDLE_FORCED_NAME,
                reason: "explicit or fulfilled-lifecycle or active-handle forced revalidation",
                class: ResourceRevalidationDecisionClass::ExplicitOrFulfilledLifecycleOrActiveHandleForced,
                permits_active_handle_forcing: true,
                permits_stale_after_revalidation: false,
                stale_after_requires_fulfilled_lifecycle: false,
                permits_dependency_change_revalidation: false,
                permits_observer_demand_revalidation: false,
                permits_terminal_state_revalidation: false,
                permits_fulfilled_lifecycle_revalidation: true,
            }
        }
        ResourceRevalidationPolicyDeclaration::Named { name } => {
            return Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                kind: ResourcePolicyKind::Revalidation,
                name: name.clone(),
                reason: "named revalidation policies are descriptor-only in the first ship runtime",
            });
        }
    };
    Ok(shape)
}

pub(super) fn ensure_descriptor_name(
    frozen: &FrozenResourcePolicyDescriptor,
    shape: &RevalidationPolicyShape,
) -> Result<(), ResourcePolicyResolutionError> {
    if frozen.descriptor().semantic_name().as_str() == shape.descriptor_name {
        return Ok(());
    }
    Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
        kind: ResourcePolicyKind::Revalidation,
        name: frozen.descriptor().semantic_name().clone(),
        reason: shape.reason,
    })
}
