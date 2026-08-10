use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactRetentionPolicy {
    Retain,
    Reconstruct,
    Omit,
}

impl ArtifactRetentionPolicy {
    pub fn description(self) -> &'static str {
        match self {
            Self::Retain => "retain eagerly in runtime state",
            Self::Reconstruct => "reconstruct deterministically on demand",
            Self::Omit => "omit unless a richer policy is configured",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DiagnosticsAvailability {
    RetainedAvailable,
    ReconstructedAvailable,
    OmittedByTier,
    DeniedByBudget,
    #[default]
    UnavailableNotRetained,
    UnavailableNotReconstructable,
}

impl DiagnosticsAvailability {
    pub fn is_available(self) -> bool {
        matches!(self, Self::RetainedAvailable | Self::ReconstructedAvailable)
    }

    pub fn is_reconstructed(self) -> bool {
        matches!(self, Self::ReconstructedAvailable)
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::RetainedAvailable => {
                "artifact detail was available from retained diagnostics state"
            }
            Self::ReconstructedAvailable => {
                "artifact detail was reconstructed through explicit cold materialization"
            }
            Self::OmittedByTier => "artifact detail is omitted by the active diagnostics tier",
            Self::DeniedByBudget => {
                "artifact detail was denied by the active reconstruction budget"
            }
            Self::UnavailableNotRetained => {
                "artifact detail is not retained in the active diagnostics envelope"
            }
            Self::UnavailableNotReconstructable => {
                "artifact detail is not reconstructable under the active diagnostics policy"
            }
        }
    }
}

impl fmt::Display for DiagnosticsAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RetainedAvailable => write!(f, "RetainedAvailable"),
            Self::ReconstructedAvailable => write!(f, "ReconstructedAvailable"),
            Self::OmittedByTier => write!(f, "OmittedByTier"),
            Self::DeniedByBudget => write!(f, "DeniedByBudget"),
            Self::UnavailableNotRetained => write!(f, "UnavailableNotRetained"),
            Self::UnavailableNotReconstructable => write!(f, "UnavailableNotReconstructable"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrdinaryAccessLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetainedForensicAccessLane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplicitColdAccessLane;
