//! Decision resolution method and context.
//!
//! DOMAIN: `DecisionKind` captures *how* a decision was resolved.
//! `DecisionContext` captures *what* the decision was about.
//! Together they answer: "What happened?" + "How was it resolved?"

use std::fmt;

use serde::{Deserialize, Serialize};

use super::entity_ref::EntityRef;
use crate::policy::PolicyKind;

/// How a kernel decision was resolved.
///
/// This captures the *resolution method*, not the *subject*. The subject
/// is captured in `DecisionContext`. Together they answer:
/// "What was decided?" (context) + "How was it resolved?" (kind).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionKind {
    /// Predicate resolved exactly — zero ambiguity.
    Exact,
    /// Ambiguity detected, resolved by a configured `ModelingContext` policy.
    PolicyApplied {
        /// Which policy category was applied.
        policy: PolicyKind,
        /// Whether the system default was used (no user override).
        default_used: bool,
    },
    /// Near a threshold but resolved with confidence.
    /// Logged for transparency even though no policy was needed.
    NearBoundary {
        /// The threshold that was approached.
        threshold: f64,
    },
    /// Could not be resolved by policy — safe default applied, flagged for review.
    Ambiguous {
        /// Description of the fallback that was applied.
        fallback_applied: String,
    },
    /// Hard constraint forced a specific outcome (e.g., manifoldness requirement).
    Forced {
        /// Why this outcome was forced.
        reason: String,
    },
}

impl fmt::Display for DecisionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionKind::Exact => write!(f, "Exact"),
            DecisionKind::PolicyApplied {
                policy,
                default_used,
            } => {
                write!(f, "PolicyApplied({:?}, default={})", policy, default_used)
            }
            DecisionKind::NearBoundary { threshold } => {
                write!(f, "NearBoundary(threshold={:.2e})", threshold)
            }
            DecisionKind::Ambiguous { fallback_applied } => {
                write!(f, "Ambiguous(fallback={})", fallback_applied)
            }
            DecisionKind::Forced { reason } => {
                write!(f, "Forced({})", reason)
            }
        }
    }
}

/// What a kernel decision was about.
///
/// Provides structured, machine-readable context for *what* prompted a
/// decision. Paired with `DecisionKind` (how it was resolved).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DecisionContext {
    /// Point classification (in/out/on boundary).
    Classification {
        /// The 3D point being classified.
        point: [f64; 3],
        /// The classification result (e.g., "Inside", "Outside").
        result: String,
    },
    /// Two entities are coincident or nearly so.
    Coincidence {
        /// First entity in the coincidence pair.
        entity_a: EntityRef,
        /// Second entity in the coincidence pair.
        entity_b: EntityRef,
    },
    /// A measured value was compared against a tolerance threshold.
    Tolerance {
        /// The measured value.
        measured: f64,
        /// The threshold it was compared against.
        threshold: f64,
    },
    /// A degenerate geometric configuration was detected.
    Degeneracy {
        /// Human-readable description of the degeneracy.
        description: String,
    },
    /// A predicate evaluated near or at zero and escalated to higher precision.
    PrecisionEscalation {
        /// Details of the precision escalation.
        escalation: forge_math::arithmetic::precision::PrecisionEscalation,
    },
}

impl fmt::Display for DecisionContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionContext::Classification { point, result } => {
                write!(
                    f,
                    "Classification [{:.4}, {:.4}, {:.4}] → {}",
                    point[0], point[1], point[2], result
                )
            }
            DecisionContext::Coincidence { entity_a, entity_b } => {
                write!(f, "Coincidence {} ↔ {}", entity_a, entity_b)
            }
            DecisionContext::Tolerance {
                measured,
                threshold,
            } => {
                write!(
                    f,
                    "Tolerance measured={:.2e} threshold={:.2e}",
                    measured, threshold
                )
            }
            DecisionContext::Degeneracy { description } => {
                write!(f, "Degeneracy: {}", description)
            }
            DecisionContext::PrecisionEscalation { escalation } => {
                write!(
                    f,
                    "Escalation to {:?}: Δ={:.2e} [{}]",
                    escalation.resolved_at,
                    escalation.disagreement_magnitude.unwrap_or(0.0),
                    escalation.target_triple
                )
            }
        }
    }
}
