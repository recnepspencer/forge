use serde::Serialize;

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};

use super::ResourceOutputContinuityPolicyDeclaration;

const PRESERVE_LIFECYCLE_OUTPUT_SEPARATION_NAME: &str =
    "signal.resource.output-continuity.preserve-lifecycle-output-separation";
const HIDE_WHILE_PENDING_NAME: &str = "signal.resource.output-continuity.hide-while-pending";
const HIDE_AFTER_REJECTION_NAME: &str = "signal.resource.output-continuity.hide-after-rejection";
const HIDE_AFTER_TIMEOUT_NAME: &str = "signal.resource.output-continuity.hide-after-timeout";
const HIDE_AFTER_CANCELLATION_NAME: &str =
    "signal.resource.output-continuity.hide-after-cancellation";
const HIDE_AFTER_SUPERSESSION_NAME: &str =
    "signal.resource.output-continuity.hide-after-supersession";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceOutputContinuityDecisionClass {
    PreserveWhilePending,
    HideWhilePending,
    HideAfterRejection,
    HideAfterTimeout,
    HideAfterCancellation,
    HideAfterSupersession,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceOutputContinuityDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceOutputContinuityDecisionClass,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceOutputContinuityDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceOutputContinuityPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceOutputContinuityPolicyDeclaration::PreserveLifecycleOutputSeparation => {
                ensure_descriptor_name(
                    frozen,
                    PRESERVE_LIFECYCLE_OUTPUT_SEPARATION_NAME,
                    "preserve-lifecycle-output-separation output continuity",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceOutputContinuityDecisionClass::PreserveWhilePending,
                ))
            }
            ResourceOutputContinuityPolicyDeclaration::HideWhilePending => {
                ensure_descriptor_name(
                    frozen,
                    HIDE_WHILE_PENDING_NAME,
                    "hide-while-pending output continuity",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceOutputContinuityDecisionClass::HideWhilePending,
                ))
            }
            ResourceOutputContinuityPolicyDeclaration::HideAfterRejection => {
                ensure_descriptor_name(
                    frozen,
                    HIDE_AFTER_REJECTION_NAME,
                    "hide-after-rejection output continuity",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceOutputContinuityDecisionClass::HideAfterRejection,
                ))
            }
            ResourceOutputContinuityPolicyDeclaration::HideAfterTimeout => {
                ensure_descriptor_name(
                    frozen,
                    HIDE_AFTER_TIMEOUT_NAME,
                    "hide-after-timeout output continuity",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceOutputContinuityDecisionClass::HideAfterTimeout,
                ))
            }
            ResourceOutputContinuityPolicyDeclaration::HideAfterCancellation => {
                ensure_descriptor_name(
                    frozen,
                    HIDE_AFTER_CANCELLATION_NAME,
                    "hide-after-cancellation output continuity",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceOutputContinuityDecisionClass::HideAfterCancellation,
                ))
            }
            ResourceOutputContinuityPolicyDeclaration::HideAfterSupersession => {
                ensure_descriptor_name(
                    frozen,
                    HIDE_AFTER_SUPERSESSION_NAME,
                    "hide-after-supersession output continuity",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceOutputContinuityDecisionClass::HideAfterSupersession,
                ))
            }
            ResourceOutputContinuityPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::OutputContinuity,
                    name: name.clone(),
                    reason:
                        "named output-continuity policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceOutputContinuityDecisionClass,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-output-continuity-plan:{}:{}",
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

    pub fn class(&self) -> ResourceOutputContinuityDecisionClass {
        self.class
    }

    pub fn preserves_previous_output_while_pending(&self) -> bool {
        matches!(
            self.class,
            ResourceOutputContinuityDecisionClass::PreserveWhilePending
                | ResourceOutputContinuityDecisionClass::HideAfterTimeout
                | ResourceOutputContinuityDecisionClass::HideAfterRejection
                | ResourceOutputContinuityDecisionClass::HideAfterCancellation
                | ResourceOutputContinuityDecisionClass::HideAfterSupersession
        )
    }

    pub fn preserves_previous_output_after_rejection(&self) -> bool {
        !matches!(
            self.class,
            ResourceOutputContinuityDecisionClass::HideAfterRejection
        )
    }

    pub fn preserves_previous_output_after_timeout(&self) -> bool {
        !matches!(
            self.class,
            ResourceOutputContinuityDecisionClass::HideAfterTimeout
        )
    }

    pub fn preserves_previous_output_after_cancellation(&self) -> bool {
        !matches!(
            self.class,
            ResourceOutputContinuityDecisionClass::HideAfterCancellation
        )
    }

    pub fn preserves_previous_output_after_supersession(&self) -> bool {
        !matches!(
            self.class,
            ResourceOutputContinuityDecisionClass::HideAfterSupersession
        )
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

impl ResourceOutputContinuityDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::PreserveWhilePending => "preserve-while-pending",
            Self::HideWhilePending => "hide-while-pending",
            Self::HideAfterRejection => "hide-after-rejection",
            Self::HideAfterTimeout => "hide-after-timeout",
            Self::HideAfterCancellation => "hide-after-cancellation",
            Self::HideAfterSupersession => "hide-after-supersession",
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
        kind: ResourcePolicyKind::OutputContinuity,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
