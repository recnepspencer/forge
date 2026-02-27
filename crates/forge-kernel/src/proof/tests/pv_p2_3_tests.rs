//! PV-28, PV-29, PV-30: Divergence Detection & Reporting (Milestone P2.3).
//!
//! DOMAIN: Acceptance tests for the divergence analysis infrastructure.
//! PV-28: Clean operation → divergence rate = 0.0
//! PV-29: Near-degenerate operation → non-zero divergence rate
//! PV-30: Report is serializable and parseable

use forge_core::tracing::divergence::{scan_for_divergences, DivergenceReport};
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionLog, DecisionTier, EntityRef, TracedDecision,
};
use forge_math::arithmetic::precision::{
    build_target_description, PrecisionEscalation, PrecisionMode,
};
use forge_math::sign::TriSign;

fn make_clean_decision(id: u64) -> TracedDecision {
    TracedDecision::new(
        DecisionId(id),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::PrecisionEscalation {
            escalation: PrecisionEscalation {
                resolved_at: PrecisionMode::Float64,
                float_agreed: true,
                expansion_length: None,
                target_triple: build_target_description(),
                disagreement_magnitude: None,
                float_sign: Some(TriSign::Pos),
            },
        },
    )
}

fn make_divergent_decision(id: u64, float_sign: Option<TriSign>) -> TracedDecision {
    let mut decision = TracedDecision::new(
        DecisionId(id),
        DecisionKind::NearBoundary { threshold: 1e-10 },
        DecisionTier::NearBoundary,
        1e-15,
        DecisionContext::PrecisionEscalation {
            escalation: PrecisionEscalation {
                resolved_at: PrecisionMode::ExpansionB,
                float_agreed: false,
                expansion_length: Some(4),
                target_triple: build_target_description(),
                disagreement_magnitude: Some(1e-15),
                float_sign,
            },
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::Face, id as u32));
    decision
}

#[test]
fn pv_28_clean_operation_zero_divergence() {
    let mut log = DecisionLog::new();
    for i in 0..10 {
        log.record(make_clean_decision(i));
    }

    let report = scan_for_divergences(&log);

    assert_eq!(report.get_total_decisions(), 10);
    assert_eq!(report.get_divergent_decisions(), 0);
    assert_eq!(report.get_divergence_rate(), 0.0);
    assert_eq!(report.get_topology_affecting_divergences(), 0);
    assert!(!report.has_critical_findings());
    assert!(report.get_details().is_empty());
}

#[test]
fn pv_29_near_degenerate_nonzero_divergence() {
    let mut log = DecisionLog::new();

    for i in 0..8 {
        log.record(make_clean_decision(i));
    }

    log.record(make_divergent_decision(8, Some(TriSign::Neg)));
    log.record(make_divergent_decision(9, Some(TriSign::Neg)));

    let report = scan_for_divergences(&log);

    assert_eq!(report.get_total_decisions(), 10);
    assert_eq!(report.get_divergent_decisions(), 2);
    assert!(report.get_divergence_rate() > 0.0);
    assert!(report.get_divergence_rate() < 1.0);

    assert!(report.get_topology_affecting_divergences() > 0);
    assert!(report.has_critical_findings());

    assert!(report.get_min_margin() <= 1e-15);

    let detail = &report.get_details()[0];
    assert_eq!(detail.get_float_answer(), Some(TriSign::Neg));
    assert_eq!(detail.get_resolved_at(), PrecisionMode::ExpansionB);
    assert!(detail.get_entity_scope().is_some());
    assert!(detail.is_topology_affecting());
}

#[test]
fn pv_30_report_serializable_round_trip() {
    let mut log = DecisionLog::new();
    log.record(make_clean_decision(0));
    log.record(make_divergent_decision(1, Some(TriSign::Neg)));

    let report = scan_for_divergences(&log);

    let json = serde_json::to_string_pretty(&report).expect("serialize to JSON");

    assert!(json.contains("divergent_decisions"));
    assert!(json.contains("divergence_rate"));
    assert!(json.contains("topology_affecting"));

    let deserialized: DivergenceReport =
        serde_json::from_str(&json).expect("deserialize from JSON");

    assert_eq!(report, deserialized);
    assert_eq!(
        deserialized.get_divergent_decisions(),
        report.get_divergent_decisions()
    );
    assert_eq!(
        deserialized.get_topology_affecting_divergences(),
        report.get_topology_affecting_divergences()
    );
}
