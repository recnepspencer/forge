use super::S8LayoutStrategyCapability;
use crate::strategy::S8LayoutStrategyFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutAdmissionDeferred {
    ExactCoverageEvidenceRequired {
        family: S8LayoutStrategyFamily,
        capability: S8LayoutStrategyCapability,
    },
    LiveExactMaintenanceWitnessRequired {
        family: S8LayoutStrategyFamily,
        capability: S8LayoutStrategyCapability,
    },
}
