use serde::Serialize;

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};

use super::ResourceObservationPolicyDeclaration;

const LIFECYCLE_ONLY_NAME: &str = "signal.resource.observation.lifecycle-only";
const LIFECYCLE_AND_OUTPUT_NAME: &str = "signal.resource.observation.lifecycle-and-output";
const LIFECYCLE_OUTPUT_AND_DENIED_COMPLETION_NAME: &str =
    "signal.resource.observation.lifecycle-output-and-denied-completion";
const LIFECYCLE_OUTPUT_AND_RETRY_SCHEDULE_NAME: &str =
    "signal.resource.observation.lifecycle-output-and-retry-schedule";
const LIFECYCLE_OUTPUT_AND_DENIED_COMPLETION_AND_RETRY_SCHEDULE_NAME: &str =
    "signal.resource.observation.lifecycle-output-and-denied-completion-and-retry-schedule";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceObservationDecisionClass {
    LifecycleOnly,
    LifecycleAndOutput,
    LifecycleOutputAndDeniedCompletion,
    LifecycleOutputAndRetrySchedule,
    LifecycleOutputAndDeniedCompletionAndRetrySchedule,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceObservationDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceObservationDecisionClass,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceObservationDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceObservationPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceObservationPolicyDeclaration::LifecycleOnly => {
                ensure_descriptor_name(frozen, LIFECYCLE_ONLY_NAME, "lifecycle-only observation")?;
                Ok(Self::new(
                    frozen,
                    ResourceObservationDecisionClass::LifecycleOnly,
                ))
            }
            ResourceObservationPolicyDeclaration::LifecycleAndOutput => {
                ensure_descriptor_name(
                    frozen,
                    LIFECYCLE_AND_OUTPUT_NAME,
                    "lifecycle-and-output observation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceObservationDecisionClass::LifecycleAndOutput,
                ))
            }
            ResourceObservationPolicyDeclaration::LifecycleOutputAndDeniedCompletion => {
                ensure_descriptor_name(
                    frozen,
                    LIFECYCLE_OUTPUT_AND_DENIED_COMPLETION_NAME,
                    "lifecycle-output-and-denied-completion observation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceObservationDecisionClass::LifecycleOutputAndDeniedCompletion,
                ))
            }
            ResourceObservationPolicyDeclaration::LifecycleOutputAndRetrySchedule => {
                ensure_descriptor_name(
                    frozen,
                    LIFECYCLE_OUTPUT_AND_RETRY_SCHEDULE_NAME,
                    "lifecycle-output-and-retry-schedule observation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceObservationDecisionClass::LifecycleOutputAndRetrySchedule,
                ))
            }
            ResourceObservationPolicyDeclaration::LifecycleOutputAndDeniedCompletionAndRetrySchedule => {
                ensure_descriptor_name(
                    frozen,
                    LIFECYCLE_OUTPUT_AND_DENIED_COMPLETION_AND_RETRY_SCHEDULE_NAME,
                    "lifecycle-output-and-denied-completion-and-retry-schedule observation",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceObservationDecisionClass::LifecycleOutputAndDeniedCompletionAndRetrySchedule,
                ))
            }
            ResourceObservationPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Observation,
                    name: name.clone(),
                    reason:
                        "named observation policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceObservationDecisionClass,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-observation-plan:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str()
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceObservationDecisionClass {
        self.class
    }

    pub fn includes_output_continuity(&self) -> bool {
        !matches!(self.class, ResourceObservationDecisionClass::LifecycleOnly)
    }

    pub fn includes_denied_completion(&self) -> bool {
        matches!(
            self.class,
            ResourceObservationDecisionClass::LifecycleOutputAndDeniedCompletion
                | ResourceObservationDecisionClass::LifecycleOutputAndDeniedCompletionAndRetrySchedule
        )
    }

    pub fn includes_retry_schedule(&self) -> bool {
        matches!(
            self.class,
            ResourceObservationDecisionClass::LifecycleOutputAndRetrySchedule
                | ResourceObservationDecisionClass::LifecycleOutputAndDeniedCompletionAndRetrySchedule
        )
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

impl ResourceObservationDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleOnly => "lifecycle-only",
            Self::LifecycleAndOutput => "lifecycle-and-output",
            Self::LifecycleOutputAndDeniedCompletion => "lifecycle-output-and-denied-completion",
            Self::LifecycleOutputAndRetrySchedule => "lifecycle-output-and-retry-schedule",
            Self::LifecycleOutputAndDeniedCompletionAndRetrySchedule => {
                "lifecycle-output-and-denied-completion-and-retry-schedule"
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
        kind: ResourcePolicyKind::Observation,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
