use serde::Serialize;

use crate::data::resource::lifecycle::ResourceLifecycleClass;
use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};

use super::ResourceRetentionPolicyDeclaration;

const RETAIN_ALL_TRANSITIONS_NAME: &str = "signal.resource.retention.retain-all-transitions";
const TERMINAL_SUMMARIES_ONLY_NAME: &str = "signal.resource.retention.terminal-summaries-only";
const COMPACT_SUPERSEDED_NAME: &str = "signal.resource.retention.compact-superseded";
const COMPACT_CANCELLED_NAME: &str = "signal.resource.retention.compact-cancelled";
const COMPACT_TIMED_OUT_NAME: &str = "signal.resource.retention.compact-timed-out";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceRetentionDecisionClass {
    RetainAllTransitions,
    TerminalSummariesOnly,
    CompactSuperseded,
    CompactCancelled,
    CompactTimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceRetentionDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceRetentionDecisionClass,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceRetentionDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceRetentionPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceRetentionPolicyDeclaration::RetainAllTransitions => {
                ensure_descriptor_name(
                    frozen,
                    RETAIN_ALL_TRANSITIONS_NAME,
                    "retain-all-transitions retention policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRetentionDecisionClass::RetainAllTransitions,
                ))
            }
            ResourceRetentionPolicyDeclaration::RetainOperationalLifecycleSummary
            | ResourceRetentionPolicyDeclaration::TerminalSummariesOnly => {
                ensure_descriptor_name(
                    frozen,
                    TERMINAL_SUMMARIES_ONLY_NAME,
                    "terminal-summaries-only retention policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRetentionDecisionClass::TerminalSummariesOnly,
                ))
            }
            ResourceRetentionPolicyDeclaration::CompactSuperseded => {
                ensure_descriptor_name(
                    frozen,
                    COMPACT_SUPERSEDED_NAME,
                    "compact-superseded retention policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRetentionDecisionClass::CompactSuperseded,
                ))
            }
            ResourceRetentionPolicyDeclaration::CompactCancelled => {
                ensure_descriptor_name(
                    frozen,
                    COMPACT_CANCELLED_NAME,
                    "compact-cancelled retention policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRetentionDecisionClass::CompactCancelled,
                ))
            }
            ResourceRetentionPolicyDeclaration::CompactTimedOut => {
                ensure_descriptor_name(
                    frozen,
                    COMPACT_TIMED_OUT_NAME,
                    "compact-timed-out retention policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceRetentionDecisionClass::CompactTimedOut,
                ))
            }
            ResourceRetentionPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Retention,
                    name: name.clone(),
                    reason:
                        "named retention policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(frozen: &FrozenResourcePolicyDescriptor, class: ResourceRetentionDecisionClass) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-retention-plan:{}:{}",
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

    pub fn class(&self) -> ResourceRetentionDecisionClass {
        self.class
    }

    pub fn retains_rich_history(&self) -> bool {
        matches!(
            self.class,
            ResourceRetentionDecisionClass::RetainAllTransitions
        )
    }

    pub fn permits_compaction_for_lifecycle(&self, lifecycle: ResourceLifecycleClass) -> bool {
        match self.class {
            ResourceRetentionDecisionClass::RetainAllTransitions => false,
            ResourceRetentionDecisionClass::TerminalSummariesOnly => {
                lifecycle.is_terminal() && lifecycle.is_runtime_truth()
            }
            ResourceRetentionDecisionClass::CompactSuperseded => {
                lifecycle == ResourceLifecycleClass::Superseded
            }
            ResourceRetentionDecisionClass::CompactCancelled => {
                lifecycle == ResourceLifecycleClass::Cancelled
            }
            ResourceRetentionDecisionClass::CompactTimedOut => {
                lifecycle == ResourceLifecycleClass::TimedOut
            }
        }
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

impl ResourceRetentionDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::RetainAllTransitions => "retain-all-transitions",
            Self::TerminalSummariesOnly => "terminal-summaries-only",
            Self::CompactSuperseded => "compact-superseded",
            Self::CompactCancelled => "compact-cancelled",
            Self::CompactTimedOut => "compact-timed-out",
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
        kind: ResourcePolicyKind::Retention,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
