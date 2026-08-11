use serde::Serialize;

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyResolutionError,
};

use super::super::ResourceRevalidationPolicyDeclaration;
use super::vocabulary::{self, ResourceRevalidationDecisionClass};

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
        let shape = vocabulary::shape_for_declaration(declaration)?;
        vocabulary::ensure_descriptor_name(frozen, &shape)?;
        Ok(Self::new(frozen, shape))
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        shape: vocabulary::RevalidationPolicyShape,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-revalidation-plan:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            frozen.frozen_digest().as_str(),
            shape.class.as_str(),
            if shape.permits_active_handle_forcing {
                "active-handle-forcing-enabled"
            } else {
                "explicit-intent-only"
            },
            if shape.permits_stale_after_revalidation {
                "stale-after-enabled"
            } else {
                "stale-after-disabled"
            },
            if shape.stale_after_requires_fulfilled_lifecycle {
                "fulfilled-only-stale-after"
            } else {
                "no-stale-after-lifecycle-gate"
            },
            if shape.permits_dependency_change_revalidation {
                "dependency-change-enabled"
            } else {
                "dependency-change-disabled"
            },
            if shape.permits_observer_demand_revalidation {
                "observer-demand-enabled"
            } else {
                "observer-demand-disabled"
            },
            if shape.permits_terminal_state_revalidation {
                "terminal-state-enabled"
            } else {
                "terminal-state-disabled"
            },
            if shape.permits_fulfilled_lifecycle_revalidation {
                "fulfilled-lifecycle-enabled"
            } else {
                "fulfilled-lifecycle-disabled"
            },
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class: shape.class,
            permits_active_handle_forcing: shape.permits_active_handle_forcing,
            permits_stale_after_revalidation: shape.permits_stale_after_revalidation,
            stale_after_requires_fulfilled_lifecycle: shape
                .stale_after_requires_fulfilled_lifecycle,
            permits_dependency_change_revalidation: shape.permits_dependency_change_revalidation,
            permits_observer_demand_revalidation: shape.permits_observer_demand_revalidation,
            permits_terminal_state_revalidation: shape.permits_terminal_state_revalidation,
            permits_fulfilled_lifecycle_revalidation: shape
                .permits_fulfilled_lifecycle_revalidation,
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
