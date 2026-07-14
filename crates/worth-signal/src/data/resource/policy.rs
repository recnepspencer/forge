pub mod cancellation;
pub mod compatibility;
pub mod diagnostics;
pub mod observation;
pub mod output_continuity;
pub mod replay;
pub mod retention;
pub mod retry;
pub mod revalidation;
pub mod stale_after;
pub mod supersession;
pub mod timeout;

use serde::{Deserialize, Deserializer, Serialize};

use crate::data::temporal::TemporalDuration;

use super::lifecycle::ResourceLifecycleClass;

pub use cancellation::{ResourceCancellationDecisionClass, ResourceCancellationDecisionPlan};
pub use compatibility::{
    DeniedResourcePolicyRestoreCompatibility, ResourcePolicyCompatibilityClass,
    ResourcePolicyCompatibilityFamilyReport, ResourcePolicyCompatibilityReport,
    ResourcePolicyRestoreCompatibilityDenialClass, ResourcePolicyRestoreCompatibilityProof,
};
pub use diagnostics::{ResourceDiagnosticsDecisionClass, ResourceDiagnosticsDecisionPlan};
pub use observation::{ResourceObservationDecisionClass, ResourceObservationDecisionPlan};
pub use output_continuity::{
    ResourceOutputContinuityDecisionClass, ResourceOutputContinuityDecisionPlan,
};
pub use replay::{ResourceReplayDecisionClass, ResourceReplayDecisionPlan};
pub use retention::{ResourceRetentionDecisionClass, ResourceRetentionDecisionPlan};
pub use retry::{ResourceRetryBudgetScope, ResourceRetryDecisionClass, ResourceRetryDecisionPlan};
pub use revalidation::{ResourceRevalidationDecisionClass, ResourceRevalidationDecisionPlan};
pub use stale_after::{ResourceStaleAfterDecisionClass, ResourceStaleAfterDecisionPlan};
pub use supersession::{
    ResourceSupersessionDecisionClass, ResourceSupersessionDecisionPlan,
    ResourceSupersessionOldHostWorkPosture, ResourceSupersessionOverlapDisposition,
};
pub use timeout::{
    ResourceTimeoutDecisionClass, ResourceTimeoutDecisionPlan, ResourceTimeoutOutcomeClass,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourcePolicyName(String);

impl ResourcePolicyName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRetryPolicyDeclaration {
    Disabled,
    FixedDelay {
        delay: TemporalDuration,
    },
    ExponentialBackoff {
        initial_delay: TemporalDuration,
        multiplier: u32,
    },
    CappedExponentialBackoff {
        initial_delay: TemporalDuration,
        multiplier: u32,
        max_delay: TemporalDuration,
    },
    RuntimeBackoff {
        delay: TemporalDuration,
    },
    Named {
        name: ResourcePolicyName,
    },
}

impl Default for ResourceRetryPolicyDeclaration {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceTimeoutPolicyDeclaration {
    Disabled,
    TransactionInheritedDeadline,
    RuntimeInheritedDeadline,
    PerAttemptTimeout {
        timeout: TemporalDuration,
    },
    FixedTimeout {
        timeout: TemporalDuration,
    },
    TotalRequestLifetimeTimeout {
        timeout: TemporalDuration,
    },
    ProgressHeartbeatExtension {
        timeout: TemporalDuration,
        heartbeat_extension: TemporalDuration,
    },
    TerminalTimeout {
        timeout: TemporalDuration,
    },
    RevalidationEligibleTimeout {
        timeout: TemporalDuration,
    },
    RuntimeTimeout {
        timeout: TemporalDuration,
    },
    Named {
        name: ResourcePolicyName,
    },
}

impl Default for ResourceTimeoutPolicyDeclaration {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceCancellationPolicyDeclaration {
    RuntimeDenialOnly,
    BestEffortHostSignalAndRuntimeDenial,
    Named { name: ResourcePolicyName },
}

impl Default for ResourceCancellationPolicyDeclaration {
    fn default() -> Self {
        Self::BestEffortHostSignalAndRuntimeDenial
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceStaleAfterPolicyDeclaration {
    Disabled,
    RuntimeStaleAfter { stale_after: TemporalDuration },
    Named { name: ResourcePolicyName },
}

impl Default for ResourceStaleAfterPolicyDeclaration {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceSupersessionPolicyDeclaration {
    NewGenerationSupersedesPrior,
    OverlappingGenerationRetainsOldHostWork,
    OverlappingGenerationCancelsOldHostWork,
    IntentEquivalentCoalescesToActive,
    Named { name: ResourcePolicyName },
}

impl Default for ResourceSupersessionPolicyDeclaration {
    fn default() -> Self {
        Self::NewGenerationSupersedesPrior
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceObservationPolicyDeclaration {
    LifecycleOnly,
    LifecycleAndOutput,
    LifecycleOutputAndDeniedCompletion,
    LifecycleOutputAndRetrySchedule,
    LifecycleOutputAndDeniedCompletionAndRetrySchedule,
    Named { name: ResourcePolicyName },
}

impl Default for ResourceObservationPolicyDeclaration {
    fn default() -> Self {
        Self::LifecycleAndOutput
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRevalidationPolicyDeclaration {
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
    Named { name: ResourcePolicyName },
}

impl Default for ResourceRevalidationPolicyDeclaration {
    fn default() -> Self {
        Self::ExplicitIntentOnly
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceOutputContinuityPolicyDeclaration {
    PreserveLifecycleOutputSeparation,
    HideWhilePending,
    HideAfterRejection,
    HideAfterTimeout,
    HideAfterCancellation,
    HideAfterSupersession,
    Named { name: ResourcePolicyName },
}

impl Default for ResourceOutputContinuityPolicyDeclaration {
    fn default() -> Self {
        Self::PreserveLifecycleOutputSeparation
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRetentionPolicyDeclaration {
    RetainAllTransitions,
    RetainOperationalLifecycleSummary,
    TerminalSummariesOnly,
    CompactSuperseded,
    CompactCancelled,
    CompactTimedOut,
    Named { name: ResourcePolicyName },
}

impl Default for ResourceRetentionPolicyDeclaration {
    fn default() -> Self {
        Self::RetainOperationalLifecycleSummary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceDiagnosticsPolicyDeclaration {
    RetainedOnly,
    BudgetedExpansion {
        max_replay_reconstruction_width: u32,
    },
    ForensicExpansionBudget {
        max_replay_reconstruction_width: u32,
        max_forensic_reconstruction_width: u32,
    },
    DenyColdExpansion,
    Named {
        name: ResourcePolicyName,
    },
}

impl Default for ResourceDiagnosticsPolicyDeclaration {
    fn default() -> Self {
        Self::BudgetedExpansion {
            max_replay_reconstruction_width: u32::MAX,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceReplayPolicyDeclaration {
    IdenticalOnly,
    CompatibleParameterExpansion,
    CompatibleRetentionNarrowing,
    CompatibleDiagnosticsRichnessChange,
    CompatibleParameterExpansionAndRetentionNarrowing,
    CompatibleParameterExpansionAndDiagnosticsRichnessChange,
    CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange,
    CompatibleRetentionNarrowingAndDiagnosticsRichnessChange,
    DenyOnUnknownOrMissing,
    Named { name: ResourcePolicyName },
}

impl Default for ResourceReplayPolicyDeclaration {
    fn default() -> Self {
        Self::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct ResourceInitialLifecycleClass {
    lifecycle: ResourceLifecycleClass,
}

impl ResourceInitialLifecycleClass {
    pub const UNREQUESTED: Self = Self {
        lifecycle: ResourceLifecycleClass::Unrequested,
    };

    pub fn unrequested() -> Self {
        Self::UNREQUESTED
    }

    pub fn lifecycle(self) -> ResourceLifecycleClass {
        self.lifecycle
    }
}

impl Default for ResourceInitialLifecycleClass {
    fn default() -> Self {
        Self::UNREQUESTED
    }
}

impl<'de> Deserialize<'de> for ResourceInitialLifecycleClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let lifecycle = ResourceLifecycleClass::deserialize(deserializer)?;
        if lifecycle == ResourceLifecycleClass::Unrequested {
            Ok(Self::UNREQUESTED)
        } else {
            Err(serde::de::Error::custom(
                "resource initial lifecycle policy must deserialize to Unrequested",
            ))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLifecyclePolicyDeclaration {
    initial: ResourceInitialLifecycleClass,
}

impl ResourceLifecyclePolicyDeclaration {
    pub fn new(initial: ResourceInitialLifecycleClass) -> Self {
        Self { initial }
    }

    pub fn initial(self) -> ResourceLifecycleClass {
        self.initial.lifecycle()
    }
}

impl Default for ResourceLifecyclePolicyDeclaration {
    fn default() -> Self {
        Self::new(ResourceInitialLifecycleClass::UNREQUESTED)
    }
}
