//! Decision significance tier.
//!
//! DOMAIN: Classifies kernel decisions by how much agent attention they need.
//! `Ord` is derived so `tier_at_least()` uses simple comparison.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Significance tier for a kernel decision.
///
/// Set at record-time by the caller, not inferred by the view layer.
/// `Ord` is derived so `tier_at_least()` uses simple comparison.
/// Deterministic < Resolved < NearBoundary < PolicyApplied < Escalated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DecisionTier {
    /// Tier 0: Predicate resolved exactly. Zero agent value.
    Deterministic,
    /// Tier 1: Unambiguous but involved a tolerance comparison. Auditable.
    Resolved,
    /// Tier 2: Result correct but margin is small. Brittle.
    NearBoundary,
    /// Tier 3: Kernel applied a fallback policy. Agent can override.
    PolicyApplied,
    /// Tier 4: Kernel could not proceed. Agent must act.
    Escalated,
}

impl fmt::Display for DecisionTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionTier::Deterministic => write!(f, "deterministic"),
            DecisionTier::Resolved => write!(f, "resolved"),
            DecisionTier::NearBoundary => write!(f, "near-boundary"),
            DecisionTier::PolicyApplied => write!(f, "policy-applied"),
            DecisionTier::Escalated => write!(f, "escalated"),
        }
    }
}
