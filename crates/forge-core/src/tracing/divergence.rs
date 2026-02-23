//! Divergence detection and reporting (Milestone P2.3).
//!
//! DOMAIN: Scan a `DecisionLog` for decisions where the f64 fast-path
//! disagreed with the higher-precision answer. Aggregate into a
//! structured `DivergenceReport` that is serializable and parseable
//! by AI agents (PV-30).
//!
//! INVARIANTS: A clean operation always produces `divergence_rate == 0.0`.
//! DEPENDENCIES: `schema` (TracedDecision, DecisionContext, EntityRef),
//!               `decision_log` (DecisionLog), `forge_math` (PrecisionMode).

use serde::{Deserialize, Serialize};

use super::decision_log::DecisionLog;
use super::schema::{DecisionContext, DecisionId, DecisionTier, EntityRef, TracedDecision};
use forge_math::arithmetic::precision::PrecisionMode;
use forge_math::sign::TriSign;

/// Detail record for a single divergent decision.
///
/// Captures what the f64 fast-path would have said versus the
/// exact answer, the margin, and whether the divergence would
/// have affected topology.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DivergenceDetail {
    /// The decision that diverged.
    decision_id: DecisionId,
    /// What the f64 fast-path computed (None if f64 was inconclusive).
    float_answer: Option<TriSign>,
    /// What the higher-precision mode computed.
    exact_answer: TriSign,
    /// How close to the threshold (from TracedDecision::margin).
    margin: f64,
    /// Which entity this decision affected.
    entity_scope: Option<EntityRef>,
    /// Which precision mode ultimately resolved the decision.
    resolved_at: PrecisionMode,
    /// Expansion length at the point of escalation.
    expansion_length: Option<usize>,
    /// Would the float answer have changed the topology?
    topology_affecting: bool,
}

impl DivergenceDetail {
    /// The divergent decision's ID.
    pub fn get_decision_id(&self) -> DecisionId {
        self.decision_id
    }

    /// What f64 would have returned.
    pub fn get_float_answer(&self) -> Option<TriSign> {
        self.float_answer
    }

    /// The correct higher-precision answer.
    pub fn get_exact_answer(&self) -> TriSign {
        self.exact_answer
    }

    /// How close to the threshold.
    pub fn get_margin(&self) -> f64 {
        self.margin
    }

    /// The affected entity, if any.
    pub fn get_entity_scope(&self) -> Option<&EntityRef> {
        self.entity_scope.as_ref()
    }

    /// Which precision mode resolved this.
    pub fn get_resolved_at(&self) -> PrecisionMode {
        self.resolved_at
    }

    /// Expansion length at escalation.
    pub fn get_expansion_length(&self) -> Option<usize> {
        self.expansion_length
    }

    /// Whether this divergence would have changed the topology.
    pub fn is_topology_affecting(&self) -> bool {
        self.topology_affecting
    }
}

/// Aggregate divergence report across all decisions in an operation.
///
/// Produced by [`scan_for_divergences`]. If
/// `topology_affecting_divergences > 0`, the float-precision path
/// would have produced a different topological result — a critical finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DivergenceReport {
    /// Total number of decisions in the log.
    total_decisions: usize,
    /// Number of decisions where float disagreed with exact.
    divergent_decisions: usize,
    /// Ratio of divergent to total decisions.
    divergence_rate: f64,
    /// Number of divergences that would have changed topology.
    topology_affecting_divergences: usize,
    /// Smallest margin across all divergent decisions.
    min_margin: f64,
    /// Per-decision detail records.
    details: Vec<DivergenceDetail>,
}

impl DivergenceReport {
    /// Total decisions scanned.
    pub fn get_total_decisions(&self) -> usize {
        self.total_decisions
    }

    /// Number of divergent decisions.
    pub fn get_divergent_decisions(&self) -> usize {
        self.divergent_decisions
    }

    /// Divergence rate (0.0 = clean, 1.0 = all divergent).
    pub fn get_divergence_rate(&self) -> f64 {
        self.divergence_rate
    }

    /// Number of topology-affecting divergences.
    pub fn get_topology_affecting_divergences(&self) -> usize {
        self.topology_affecting_divergences
    }

    /// Smallest margin across divergent decisions.
    pub fn get_min_margin(&self) -> f64 {
        self.min_margin
    }

    /// Per-decision divergence details.
    pub fn get_details(&self) -> &[DivergenceDetail] {
        &self.details
    }

    /// Whether this report contains critical findings.
    pub fn has_critical_findings(&self) -> bool {
        self.topology_affecting_divergences > 0
    }
}

/// Classify whether a divergence would have affected topology.
///
/// A divergence is topology-affecting when:
/// 1. The float sign differs from the exact sign (not just inconclusive → resolved), AND
/// 2. The decision tier is NearBoundary or higher (close enough to threshold
///    that classification could have flipped the result).
fn classify_topology_impact(
    float_sign: Option<TriSign>,
    exact_sign: TriSign,
    tier: DecisionTier,
) -> bool {
    let signs_differ = match float_sign {
        Some(fs) => fs != exact_sign,
        None => tier >= DecisionTier::NearBoundary,
    };
    signs_differ && tier >= DecisionTier::NearBoundary
}

/// Scan a `DecisionLog` for divergent decisions (where `float_agreed == false`).
///
/// Extracts escalation metadata from each `DecisionContext::PrecisionEscalation`
/// and aggregates into a `DivergenceReport`.
pub fn scan_for_divergences(log: &DecisionLog) -> DivergenceReport {
    let all_decisions: Vec<&TracedDecision> = log.decisions().collect();
    let total_decisions = all_decisions.len();

    let mut details = Vec::new();
    let mut min_margin = f64::INFINITY;

    for decision in &all_decisions {
        let escalation = match decision.get_context() {
            DecisionContext::PrecisionEscalation { escalation } => escalation,
            _ => {
                // Non-escalation decisions are not divergence candidates
                continue;
            }
        };

        if escalation.float_agreed {
            continue;
        }

        let margin = decision.get_margin();
        if margin < min_margin {
            min_margin = margin;
        }

        let float_sign = escalation.float_sign;
        let exact_sign = extract_exact_sign(decision);
        let topology_affecting =
            classify_topology_impact(float_sign, exact_sign, decision.get_tier());

        details.push(DivergenceDetail {
            decision_id: decision.get_id(),
            float_answer: float_sign,
            exact_answer: exact_sign,
            margin,
            entity_scope: decision.get_entity_scope().cloned(),
            resolved_at: escalation.resolved_at,
            expansion_length: escalation.expansion_length,
            topology_affecting,
        });
    }

    let divergent_decisions = details.len();
    let topology_affecting_divergences = details.iter().filter(|d| d.topology_affecting).count();

    if min_margin == f64::INFINITY {
        min_margin = 0.0;
    }

    let divergence_rate = if total_decisions > 0 {
        divergent_decisions as f64 / total_decisions as f64
    } else {
        0.0
    };

    DivergenceReport {
        total_decisions,
        divergent_decisions,
        divergence_rate,
        topology_affecting_divergences,
        min_margin,
        details,
    }
}

/// Extract the exact sign from a traced decision.
///
/// The exact sign is the sign that was ultimately committed. For escalation
/// decisions the resolution is embedded in the kind — exact decisions
/// produce deterministic signs. We infer from the margin: positive margin
/// with Exact kind means the resolved sign was definite.
fn extract_exact_sign(decision: &TracedDecision) -> TriSign {
    let margin = decision.get_margin();
    if margin > 0.0 {
        TriSign::Pos
    } else if margin < 0.0 {
        TriSign::Neg
    } else {
        TriSign::Zero
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::schema::{DecisionKind, DecisionTier};
    use forge_math::arithmetic::precision::{build_target_description, PrecisionEscalation};

    fn make_clean_escalation() -> PrecisionEscalation {
        PrecisionEscalation {
            resolved_at: PrecisionMode::Float64,
            float_agreed: true,
            expansion_length: None,
            target_triple: build_target_description(),
            disagreement_magnitude: None,
            float_sign: Some(TriSign::Pos),
        }
    }

    fn make_divergent_escalation() -> PrecisionEscalation {
        PrecisionEscalation {
            resolved_at: PrecisionMode::ExpansionB,
            float_agreed: false,
            expansion_length: Some(4),
            target_triple: build_target_description(),
            disagreement_magnitude: Some(1e-15),
            float_sign: Some(TriSign::Neg),
        }
    }

    #[test]
    fn clean_log_produces_zero_divergence() {
        let mut log = DecisionLog::new();
        let decision = TracedDecision::new(
            DecisionId(0),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::PrecisionEscalation {
                escalation: make_clean_escalation(),
            },
        );
        log.record(decision);

        let report = scan_for_divergences(&log);
        assert_eq!(report.get_total_decisions(), 1);
        assert_eq!(report.get_divergent_decisions(), 0);
        assert_eq!(report.get_divergence_rate(), 0.0);
        assert!(!report.has_critical_findings());
    }

    #[test]
    fn divergent_decision_detected() {
        let mut log = DecisionLog::new();
        let mut decision = TracedDecision::new(
            DecisionId(0),
            DecisionKind::NearBoundary { threshold: 1e-10 },
            DecisionTier::NearBoundary,
            1e-15,
            DecisionContext::PrecisionEscalation {
                escalation: make_divergent_escalation(),
            },
        );
        decision.set_entity_scope(EntityRef::new(EntityKind::Face, 42));
        log.record(decision);

        let report = scan_for_divergences(&log);
        assert_eq!(report.get_divergent_decisions(), 1);
        assert!(report.get_divergence_rate() > 0.0);
        assert_eq!(report.get_details().len(), 1);

        let detail = &report.get_details()[0];
        assert_eq!(detail.get_float_answer(), Some(TriSign::Neg));
        assert_eq!(detail.get_resolved_at(), PrecisionMode::ExpansionB);
        assert!(detail.is_topology_affecting());
    }

    #[test]
    fn report_serialization_round_trip() {
        let mut log = DecisionLog::new();
        let decision = TracedDecision::new(
            DecisionId(0),
            DecisionKind::NearBoundary { threshold: 1e-10 },
            DecisionTier::NearBoundary,
            1e-15,
            DecisionContext::PrecisionEscalation {
                escalation: make_divergent_escalation(),
            },
        );
        log.record(decision);

        let report = scan_for_divergences(&log);
        let json = serde_json::to_string(&report).expect("serialize");
        let deserialized: DivergenceReport =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(report, deserialized);
    }
}
