use serde::Serialize;

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};
use crate::data::temporal::TemporalDuration;

use super::ResourceStaleAfterPolicyDeclaration;

const STALE_AFTER_DISABLED_NAME: &str = "signal.resource.stale-after.disabled";
const STALE_AFTER_RUNTIME_NAME: &str = "signal.resource.stale-after.runtime-stale-after";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceStaleAfterDecisionClass {
    Disabled,
    RuntimeStaleAfter,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceStaleAfterDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceStaleAfterDecisionClass,
    stale_after: Option<TemporalDuration>,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceStaleAfterDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceStaleAfterPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceStaleAfterPolicyDeclaration::Disabled => {
                ensure_descriptor_name(frozen, STALE_AFTER_DISABLED_NAME, "disabled stale-after")?;
                Ok(Self::new(
                    frozen,
                    ResourceStaleAfterDecisionClass::Disabled,
                    None,
                ))
            }
            ResourceStaleAfterPolicyDeclaration::RuntimeStaleAfter { stale_after } => {
                ensure_descriptor_name(frozen, STALE_AFTER_RUNTIME_NAME, "runtime stale-after")?;
                Ok(Self::new(
                    frozen,
                    ResourceStaleAfterDecisionClass::RuntimeStaleAfter,
                    Some(*stale_after),
                ))
            }
            ResourceStaleAfterPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::StaleAfter,
                    name: name.clone(),
                    reason:
                        "named stale-after policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceStaleAfterDecisionClass,
        stale_after: Option<TemporalDuration>,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-stale-after-plan:{}:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str(),
            stale_after
                .map(|value| value.get().to_string())
                .unwrap_or_else(|| "disabled".to_owned())
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            stale_after,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceStaleAfterDecisionClass {
        self.class
    }

    pub fn stale_after(&self) -> Option<TemporalDuration> {
        self.stale_after
    }

    pub fn is_enabled(&self) -> bool {
        self.stale_after.is_some()
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

impl ResourceStaleAfterDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::RuntimeStaleAfter => "runtime-stale-after",
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
        kind: ResourcePolicyKind::StaleAfter,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
