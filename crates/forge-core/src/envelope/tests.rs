//! Tests for the operation envelope.

use std::time::Duration;

use super::*;
use crate::tracing::{
    DecisionId, DecisionKind, DecisionTier, DecisionContext,
    DecisionLog, TracedDecision, EntityRef, EntityKind,
};
use crate::policy::PolicyKind;

#[test]
fn operation_result_new_has_empty_metadata() {
    let result: OperationResult<i32> = OperationResult::new(42);
    assert_eq!(*result.get_value(), 42);
    assert!(result.get_warnings().is_empty());
    assert!(result.get_decision_log().is_empty());
    assert_eq!(result.get_metrics().entities_created, 0);
    assert!(!result.has_warnings());
    assert!(!result.has_decisions());
    assert_eq!(result.get_state_hash_before(), 0);
    assert_eq!(result.get_state_hash_after(), 0);
}

#[test]
fn operation_result_into_value() {
    let result = OperationResult::new(String::from("hello"));
    let value = result.into_value();
    assert_eq!(value, "hello");
}

#[test]
fn operation_result_add_warning() {
    let mut result = OperationResult::new(0);
    result.add_warning(KernelWarning::SliverFaceCreated {
        face_index: 3,
        area: 1e-12,
        threshold: 1e-10,
    });
    assert!(result.has_warnings());
    assert_eq!(result.get_warnings().len(), 1);
}

#[test]
fn with_metadata_constructor() {
    let log = DecisionLog::new();
    let result = OperationResult::with_metadata(
        99,
        vec![KernelWarning::AutoDecision { decision_id: DecisionId(1) }],
        log,
        OperationMetrics { duration: Duration::from_millis(5), entities_created: 3, entities_deleted: 1, entities_modified: 0, exact_predicate_calls: 10, policy_decisions_made: 2 },
        LineageDelta { faces_created: 1, ..LineageDelta::default() },
        0xAABB,
        0xCCDD,
    );
    assert_eq!(*result.get_value(), 99);
    assert!(result.has_warnings());
    assert_eq!(result.get_metrics().entities_created, 3);
    assert_eq!(result.get_metrics().exact_predicate_calls, 10);
    assert_eq!(result.get_lineage_delta().faces_created, 1);
    assert_eq!(result.get_state_hash_before(), 0xAABB);
    assert_eq!(result.get_state_hash_after(), 0xCCDD);
}

#[test]
fn serde_roundtrip_operation_result() {
    let mut log = DecisionLog::new();
    log.record(TracedDecision::new(
        DecisionId(1),
        DecisionKind::PolicyApplied {
            policy: PolicyKind::CoincidentGeometry,
            default_used: false,
        },
        DecisionTier::PolicyApplied,
        0.42,
        DecisionContext::Coincidence {
            entity_a: EntityRef::new(EntityKind::Vertex, 10),
            entity_b: EntityRef::new(EntityKind::Vertex, 20),
        },
    ));
    log.record(TracedDecision::new(
        DecisionId(2),
        DecisionKind::Ambiguous { fallback_applied: "merge".to_string() },
        DecisionTier::Escalated,
        0.001,
        DecisionContext::Tolerance { measured: 9.5e-7, threshold: 1e-6 },
    ));

    let result = OperationResult::with_metadata(
        42_i32,
        vec![KernelWarning::SliverFaceCreated { face_index: 5, area: 1e-12, threshold: 1e-10 }],
        log,
        OperationMetrics {
            duration: Duration::from_micros(1234),
            entities_created: 6,
            entities_deleted: 2,
            entities_modified: 1,
            exact_predicate_calls: 100,
            policy_decisions_made: 3,
        },
        LineageDelta { faces_created: 4, vertices_created: 8, ..LineageDelta::default() },
        0x1234_5678_9ABC_DEF0,
        0xFEDC_BA98_7654_3210,
    );

    let json = serde_json::to_string(&result).expect("serialize");
    let restored: OperationResult<i32> = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(*restored.get_value(), 42);
    assert_eq!(restored.get_decision_log().len(), 2);
    let decisions: Vec<_> = restored.get_decision_log().decisions().collect();
    assert_eq!(decisions[0].get_id(), DecisionId(1));
    assert_eq!(decisions[1].get_id(), DecisionId(2));
    assert!(!restored.get_decision_log().is_clean());
    assert_eq!(restored.get_warnings().len(), 1);
    assert_eq!(restored.get_metrics().exact_predicate_calls, 100);
    assert_eq!(restored.get_lineage_delta().faces_created, 4);
    assert_eq!(restored.get_state_hash_before(), 0x1234_5678_9ABC_DEF0);
    assert_eq!(restored.get_state_hash_after(), 0xFEDC_BA98_7654_3210);

    let summary = restored.get_decision_log().summary();
    assert_eq!(summary.total, 2);
    assert_eq!(summary.policy_applied, 1);
    assert_eq!(summary.ambiguous, 1);
}

#[test]
fn operation_result_map_preserves_metadata() {
    let mut result = OperationResult::new(10);
    result.set_state_hash_before(0xAA);
    result.set_state_hash_after(0xBB);
    result.add_warning(KernelWarning::AutoDecision { decision_id: DecisionId(1) });

    let mapped = result.map(|v| v * 2);
    assert_eq!(*mapped.get_value(), 20);
    assert_eq!(mapped.get_state_hash_before(), 0xAA);
    assert_eq!(mapped.get_state_hash_after(), 0xBB);
    assert!(mapped.has_warnings());
}
