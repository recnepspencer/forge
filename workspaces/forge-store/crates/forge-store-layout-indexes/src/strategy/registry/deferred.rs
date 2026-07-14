use super::LayoutStrategyCapability;
use crate::strategy::LayoutStrategyFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutAdmissionDeferred {
    ExactCoverageEvidenceRequired {
        family: LayoutStrategyFamily,
        capability: LayoutStrategyCapability,
    },
}
