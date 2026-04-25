use serde::{Deserialize, Serialize};

use crate::data::temporal::TemporalDuration;

use super::lifecycle::ResourceLifecycleClass;

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
    RuntimeBackoff { delay: TemporalDuration },
    Named { name: ResourcePolicyName },
}

impl Default for ResourceRetryPolicyDeclaration {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceTimeoutPolicyDeclaration {
    Disabled,
    RuntimeTimeout { timeout: TemporalDuration },
    Named { name: ResourcePolicyName },
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
    Named { name: ResourcePolicyName },
}

impl Default for ResourceOutputContinuityPolicyDeclaration {
    fn default() -> Self {
        Self::PreserveLifecycleOutputSeparation
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceRetentionPolicyDeclaration {
    RetainOperationalLifecycleSummary,
    Named { name: ResourcePolicyName },
}

impl Default for ResourceRetentionPolicyDeclaration {
    fn default() -> Self {
        Self::RetainOperationalLifecycleSummary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLifecyclePolicyDeclaration {
    initial: ResourceLifecycleClass,
}

impl ResourceLifecyclePolicyDeclaration {
    pub fn new(initial: ResourceLifecycleClass) -> Self {
        Self { initial }
    }

    pub fn initial(self) -> ResourceLifecycleClass {
        self.initial
    }
}

impl Default for ResourceLifecyclePolicyDeclaration {
    fn default() -> Self {
        Self::new(ResourceLifecycleClass::Unrequested)
    }
}
