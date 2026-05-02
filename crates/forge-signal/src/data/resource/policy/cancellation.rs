use serde::Serialize;

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};
use crate::data::temporal::TemporalDuration;

use super::ResourceCancellationPolicyDeclaration;
use crate::data::resource::request::ResourceNodeId;

const CANCELLATION_RUNTIME_DENIAL_ONLY_NAME: &str =
    "signal.resource.cancellation.runtime-denial-only";
const CANCELLATION_BEST_EFFORT_HOST_SIGNAL_AND_RUNTIME_DENIAL_NAME: &str =
    "signal.resource.cancellation.best-effort-host-signal-and-runtime-denial";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceCancellationDecisionClass {
    RuntimeDenialOnly,
    BestEffortHostSignalAndRuntimeDenial,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceCancellationDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceCancellationDecisionClass,
    requests_host_advisory: bool,
    grace_period: Option<TemporalDuration>,
    declared_dependent_cancellation_nodes: Vec<ResourceNodeId>,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceCancellationDecisionPlan {
    pub(crate) fn lower(
        owner: ResourceNodeId,
        declaration: &ResourceCancellationPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
        grace_period: Option<TemporalDuration>,
        declared_dependent_cancellation_nodes: &[ResourceNodeId],
    ) -> Result<Self, ResourcePolicyResolutionError> {
        let canonical_dependent_nodes =
            canonical_dependent_nodes(owner, frozen, declared_dependent_cancellation_nodes)?;
        match declaration {
            ResourceCancellationPolicyDeclaration::RuntimeDenialOnly => {
                ensure_descriptor_name(
                    frozen,
                    CANCELLATION_RUNTIME_DENIAL_ONLY_NAME,
                    "runtime denial only cancellation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceCancellationDecisionClass::RuntimeDenialOnly,
                    false,
                    grace_period,
                    canonical_dependent_nodes,
                ))
            }
            ResourceCancellationPolicyDeclaration::BestEffortHostSignalAndRuntimeDenial => {
                ensure_descriptor_name(
                    frozen,
                    CANCELLATION_BEST_EFFORT_HOST_SIGNAL_AND_RUNTIME_DENIAL_NAME,
                    "best effort host signal and runtime denial cancellation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceCancellationDecisionClass::BestEffortHostSignalAndRuntimeDenial,
                    true,
                    grace_period,
                    canonical_dependent_nodes,
                ))
            }
            ResourceCancellationPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Cancellation,
                    name: name.clone(),
                    reason:
                        "named cancellation policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceCancellationDecisionClass,
        requests_host_advisory: bool,
        grace_period: Option<TemporalDuration>,
        declared_dependent_cancellation_nodes: Vec<ResourceNodeId>,
    ) -> Self {
        let dependent_digest = if declared_dependent_cancellation_nodes.is_empty() {
            String::from("no-dependents")
        } else {
            declared_dependent_cancellation_nodes
                .iter()
                .map(|node| format!("{}:{}", node.node().index(), node.node().generation()))
                .collect::<Vec<_>>()
                .join(",")
        };
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-cancellation-plan:{}:{}:{}:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str(),
            if requests_host_advisory {
                "host-advisory"
            } else {
                "runtime-denial-only"
            },
            grace_period
                .map(|duration| duration.get().to_string())
                .unwrap_or_else(|| "no-grace".to_owned()),
            dependent_digest,
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            requests_host_advisory,
            grace_period,
            declared_dependent_cancellation_nodes,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceCancellationDecisionClass {
        self.class
    }

    pub fn requests_host_advisory(&self) -> bool {
        self.requests_host_advisory
    }

    pub fn grace_period(&self) -> Option<TemporalDuration> {
        self.grace_period
    }

    pub fn declared_dependent_cancellation_nodes(&self) -> &[ResourceNodeId] {
        &self.declared_dependent_cancellation_nodes
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

fn canonical_dependent_nodes(
    owner: ResourceNodeId,
    frozen: &FrozenResourcePolicyDescriptor,
    declared_dependent_cancellation_nodes: &[ResourceNodeId],
) -> Result<Vec<ResourceNodeId>, ResourcePolicyResolutionError> {
    let mut canonical = declared_dependent_cancellation_nodes.to_vec();
    canonical.sort();
    canonical.dedup();
    if canonical.iter().any(|node| *node == owner) {
        return Err(ResourcePolicyResolutionError::MalformedDescriptor {
            kind: ResourcePolicyKind::Cancellation,
            name: frozen.descriptor().semantic_name().clone(),
            reason: "declared dependent cancellation footprint cannot include the owner node",
        });
    }
    Ok(canonical)
}

impl ResourceCancellationDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeDenialOnly => "runtime-denial-only",
            Self::BestEffortHostSignalAndRuntimeDenial => {
                "best-effort-host-signal-and-runtime-denial"
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
        kind: ResourcePolicyKind::Cancellation,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
