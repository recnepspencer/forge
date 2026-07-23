use crate::data::aspect::AspectMask;

use super::super::{
    SignalThresholdBoundary, SignalThresholdComparisonDomain, SignalThresholdValueFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalConditionalConditionClass {
    Always,
    AspectFilter,
    DeltaThreshold,
    OnDemand,
    RuntimePredicate,
    TemporalWake,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalConditionalComparatorClass {
    Exact,
    Tolerance,
    OutputIdentity,
    Custom,
    RuntimeResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalConditionalComparatorPosition {
    Dependency,
    Output,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalConditionalArtifactReuseClass {
    NotReusable,
    DependencyAndOutputEquivalent,
    OutputEquivalent,
    RuntimeResolved,
}

/// The first semantic dimension that prevents conditional continuity.
///
/// Values are retained only where they are portable meaning. Installed
/// condition and comparator identities belong to execution affinity instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalConditionalSemanticMismatch {
    ConditionClass {
        current: SignalConditionalConditionClass,
        candidate: SignalConditionalConditionClass,
    },
    AspectFilterMask {
        current: AspectMask,
        candidate: AspectMask,
    },
    ThresholdValue {
        current: worth_foundational::facade::AspectValue,
        candidate: worth_foundational::facade::AspectValue,
    },
    ThresholdUnitIdentity {
        current: String,
        candidate: String,
    },
    ThresholdValueFamily {
        current: SignalThresholdValueFamily,
        candidate: SignalThresholdValueFamily,
    },
    ThresholdComparisonDomain {
        current: SignalThresholdComparisonDomain,
        candidate: SignalThresholdComparisonDomain,
    },
    ThresholdBoundary {
        current: SignalThresholdBoundary,
        candidate: SignalThresholdBoundary,
    },
    InstalledConditionMeaningUnproven,
    DependencyAspects {
        current: AspectMask,
        candidate: AspectMask,
    },
    TriggerAspects {
        current: AspectMask,
        candidate: AspectMask,
    },
    ComparatorClass {
        position: SignalConditionalComparatorPosition,
        current: SignalConditionalComparatorClass,
        candidate: SignalConditionalComparatorClass,
    },
    ComparatorTolerance {
        position: SignalConditionalComparatorPosition,
        current_epsilon: u64,
        candidate_epsilon: u64,
    },
    ComparatorCustomKey {
        position: SignalConditionalComparatorPosition,
        current: String,
        candidate: String,
    },
    InstalledComparatorMeaningUnproven {
        position: SignalConditionalComparatorPosition,
    },
    ArtifactReuseClass {
        current: SignalConditionalArtifactReuseClass,
        candidate: SignalConditionalArtifactReuseClass,
    },
    InstalledArtifactReuseMeaningUnproven,
}
