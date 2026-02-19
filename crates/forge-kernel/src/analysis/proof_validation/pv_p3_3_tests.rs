//! P3.3 acceptance tests — Causal Decision Chain Reconstruction.
//!
//! PV-37: Face created by Boolean → causal chain traces back through
//!        split → classification → assembly → origin feature.
//! PV-38: Causal chain for a face in a 50-step chain has exactly 5
//!        relevant steps (not < 10, exactly 5).
//! PV-54: ChainSummary for a 50-step entity is < 200 tokens
//!        and contains the tightest-margin decision.
//! PV-54.5: Causal chain excludes non-mutating ops but retains 100%
//!          of operations that altered bounding vertices/edges.

use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionLog, DecisionTier,
    EntityRef, TracedDecision,
};
use forge_topo::lineage::{EntityKind, Lineage, LineageEvent, OpSignature};
use forge_topo::replay::{ReplayEntry, ReplayLog};

use crate::analysis::causal_chain::{query_causal_chain, query_causal_summary};

/// Helper: create a TracedDecision scoped to a specific entity.
fn make_entity_decision(
    id: u64,
    entity: &EntityRef,
    tier: DecisionTier,
    margin: f64,
) -> TracedDecision {
    let mut d = TracedDecision::new(
        DecisionId(id),
        DecisionKind::Exact,
        tier,
        margin,
        DecisionContext::Classification {
            point: [0.0, 0.0, 0.0],
            result: format!("step_{}", id),
        },
    );
    d.set_entity_scope(entity.clone());
    d
}

/// Helper: create a ReplayEntry for a named operation.
fn make_replay_entry(name: &str, invocation: u64, pre: u128, post: u128) -> ReplayEntry {
    let mut entry = ReplayEntry::new(
        OpSignature::with_id(name, invocation),
        "{}".to_string(),
        invocation,
        pre,
    );
    entry.set_post_hash(post);
    entry
}

/// PV-37: Face created by Boolean → causal chain traces back through
/// make_cube → boolean_split → classify_faces → assemble_result.
///
/// Verifies:
/// - Chain has exactly 4 steps
/// - Steps are in the correct causal order
/// - Each step has the correct operation signature name
/// - Min margin captures the NearBoundary decision
#[test]
fn pv_37_face_causal_chain_traces_ancestry() {
    let target_face = EntityRef::new("Face", 3);

    let op_names = ["make_cube", "boolean_split", "classify_faces", "assemble_result"];

    let mut replay_log = ReplayLog::new();
    replay_log.record(make_replay_entry("make_cube", 1, 0, 100));
    replay_log.record(make_replay_entry("boolean_split", 2, 100, 200));
    replay_log.record(make_replay_entry("classify_faces", 3, 200, 300));
    replay_log.record(make_replay_entry("assemble_result", 4, 300, 400));

    let root_lineage = Lineage::root(1, OpSignature::with_id("make_cube", 1));
    let split_lineage = Lineage::derive(&root_lineage, OpSignature::with_id("boolean_split", 2));
    let classify_lineage = Lineage::derive(&split_lineage, OpSignature::with_id("classify_faces", 3));
    let assemble_lineage = Lineage::derive(&classify_lineage, OpSignature::with_id("assemble_result", 4));

    let lineage_events = vec![
        LineageEvent::EntityCreated {
            entity_kind: EntityKind::Face,
            lineage: root_lineage,
        },
        LineageEvent::EntityModified {
            entity_kind: EntityKind::Face,
            old_lineage: Lineage::root(1, OpSignature::with_id("make_cube", 1)),
            new_lineage: split_lineage,
        },
        LineageEvent::EntityModified {
            entity_kind: EntityKind::Face,
            old_lineage: Lineage::derive(
                &Lineage::root(1, OpSignature::with_id("make_cube", 1)),
                OpSignature::with_id("boolean_split", 2),
            ),
            new_lineage: classify_lineage,
        },
        LineageEvent::EntityModified {
            entity_kind: EntityKind::Face,
            old_lineage: Lineage::derive(
                &Lineage::derive(
                    &Lineage::root(1, OpSignature::with_id("make_cube", 1)),
                    OpSignature::with_id("boolean_split", 2),
                ),
                OpSignature::with_id("classify_faces", 3),
            ),
            new_lineage: assemble_lineage,
        },
    ];

    let mut decision_log = DecisionLog::new();
    decision_log.record(make_entity_decision(1, &target_face, DecisionTier::Deterministic, 1.0));
    decision_log.record(make_entity_decision(2, &target_face, DecisionTier::NearBoundary, 0.01));
    decision_log.record(make_entity_decision(3, &target_face, DecisionTier::Resolved, 0.5));
    decision_log.record(make_entity_decision(4, &target_face, DecisionTier::Deterministic, 1.0));

    let chain = query_causal_chain(
        &target_face,
        &replay_log,
        &decision_log,
        &lineage_events,
        &[],
    );

    assert_eq!(
        chain.get_steps().len(),
        4,
        "Causal chain must have exactly 4 steps, got {}",
        chain.get_steps().len()
    );

    for (i, step) in chain.get_steps().iter().enumerate() {
        assert_eq!(
            step.get_operation().get_name(),
            op_names[i],
            "Step {} must be '{}', got '{}'",
            i,
            op_names[i],
            step.get_operation().get_name()
        );
    }

    assert_eq!(
        chain.get_steps()[0].get_topology_hashes(),
        (0, 100),
        "Step 0 pre/post hashes must be (0, 100)"
    );
    assert_eq!(
        chain.get_steps()[3].get_topology_hashes(),
        (300, 400),
        "Step 3 pre/post hashes must be (300, 400)"
    );

    assert_eq!(chain.get_target().get_kind(), "Face");
    assert_eq!(chain.get_target().get_index(), 3);

    assert!(
        (chain.get_summary().get_min_margin() - 0.01).abs() < 1e-10,
        "Min margin must be exactly 0.01 (NearBoundary decision), got {}",
        chain.get_summary().get_min_margin()
    );
}

/// PV-38: 50-step chain with exactly 5 relevant ops → chain has exactly 5 steps.
///
/// Injects 5 Face-type lineage events whose op names match exactly 5
/// of the 50 replay entries. The other 45 entries have unique names
/// that never appear in any lineage event.
#[test]
fn pv_38_fifty_step_chain_exactly_five_relevant_steps() {
    let target_face = EntityRef::new("Face", 7);
    let unrelated = EntityRef::new("Face", 99);

    let relevant_indices: Vec<u64> = vec![1, 10, 20, 35, 50];

    let mut replay_log = ReplayLog::new();
    for i in 1..=50u64 {
        let name = if relevant_indices.contains(&i) {
            format!("face_op_{}", i)
        } else {
            format!("unrelated_op_{}", i)
        };
        replay_log.record(make_replay_entry(
            &name,
            i,
            (i as u128 - 1) * 100,
            i as u128 * 100,
        ));
    }

    let mut lineage_events = Vec::new();
    let mut decision_log = DecisionLog::new();

    for (idx, &step) in relevant_indices.iter().enumerate() {
        let op_name = format!("face_op_{}", step);
        let op_sig = OpSignature::with_id(&op_name, step);
        let lineage = if idx == 0 {
            Lineage::root(1, op_sig.clone())
        } else {
            let parent_sig = OpSignature::with_id(
                &format!("face_op_{}", relevant_indices[idx - 1]),
                relevant_indices[idx - 1],
            );
            let parent = Lineage::root(1, parent_sig);
            Lineage::derive(&parent, op_sig.clone())
        };

        if idx == 0 {
            lineage_events.push(LineageEvent::EntityCreated {
                entity_kind: EntityKind::Face,
                lineage,
            });
        } else {
            lineage_events.push(LineageEvent::EntityModified {
                entity_kind: EntityKind::Face,
                old_lineage: Lineage::root(1, OpSignature::with_id("dummy", 0)),
                new_lineage: lineage,
            });
        }

        decision_log.record(make_entity_decision(
            step * 100,
            &target_face,
            DecisionTier::Deterministic,
            1.0,
        ));
    }

    for step in 1..=50u64 {
        if !relevant_indices.contains(&step) {
            let op_name = format!("vertex_unrelated_{}", step);
            lineage_events.push(LineageEvent::EntityModified {
                entity_kind: EntityKind::Vertex,
                old_lineage: Lineage::root(99, OpSignature::with_id(&op_name, step)),
                new_lineage: Lineage::root(99, OpSignature::with_id(&op_name, step + 1)),
            });

            decision_log.record(make_entity_decision(
                step * 100 + 50,
                &unrelated,
                DecisionTier::Deterministic,
                1.0,
            ));
        }
    }

    let chain = query_causal_chain(
        &target_face,
        &replay_log,
        &decision_log,
        &lineage_events,
        &[],
    );

    assert_eq!(
        chain.get_steps().len(),
        5,
        "50-step chain with 5 relevant ops must produce exactly 5 steps, got {}",
        chain.get_steps().len()
    );

    let step_names: Vec<&str> = chain.get_steps().iter()
        .map(|s| s.get_operation().get_name())
        .collect();
    for &idx in &relevant_indices {
        let expected_name = format!("face_op_{}", idx);
        assert!(
            step_names.contains(&expected_name.as_str()),
            "Step for '{}' must be present in chain, got {:?}",
            expected_name,
            step_names
        );
    }

    assert_eq!(
        chain.get_summary().get_total_steps(),
        5,
        "Summary total_steps must be exactly 5"
    );
}

/// PV-54: ChainSummary for a 50-step entity is < 200 tokens
/// and contains the tightest-margin decision (0.001 at step 35).
#[test]
fn pv_54_chain_summary_under_200_tokens_with_tightest_margin() {
    let target_face = EntityRef::new("Face", 5);

    let relevant_indices: Vec<u64> = vec![1, 15, 35, 42, 50];

    let mut replay_log = ReplayLog::new();
    for i in 1..=50u64 {
        let name = if relevant_indices.contains(&i) {
            format!("boolean_step_{}", i)
        } else {
            format!("noise_step_{}", i)
        };
        replay_log.record(make_replay_entry(
            &name,
            i,
            (i as u128 - 1) * 100,
            i as u128 * 100,
        ));
    }

    let mut lineage_events = Vec::new();
    let mut decision_log = DecisionLog::new();

    for (idx, &step) in relevant_indices.iter().enumerate() {
        let op_name = format!("boolean_step_{}", step);
        let op_sig = OpSignature::with_id(&op_name, step);
        let lineage = if idx == 0 {
            Lineage::root(1, op_sig.clone())
        } else {
            let parent = Lineage::root(1, OpSignature::with_id("dummy", 0));
            Lineage::derive(&parent, op_sig.clone())
        };

        if idx == 0 {
            lineage_events.push(LineageEvent::EntityCreated {
                entity_kind: EntityKind::Face,
                lineage,
            });
        } else {
            lineage_events.push(LineageEvent::EntityModified {
                entity_kind: EntityKind::Face,
                old_lineage: Lineage::root(1, OpSignature::with_id("dummy", 0)),
                new_lineage: lineage,
            });
        }

        let margin = if step == 35 { 0.001 } else { 1.0 };
        let tier = if step == 35 {
            DecisionTier::NearBoundary
        } else {
            DecisionTier::Deterministic
        };

        decision_log.record(make_entity_decision(
            step * 100,
            &target_face,
            tier,
            margin,
        ));
    }

    for step in 1..=50u64 {
        if !relevant_indices.contains(&step) {
            let op_name = format!("noise_vertex_{}", step);
            lineage_events.push(LineageEvent::EntityModified {
                entity_kind: EntityKind::Vertex,
                old_lineage: Lineage::root(99, OpSignature::with_id(&op_name, step)),
                new_lineage: Lineage::root(99, OpSignature::with_id(&op_name, step + 1)),
            });
        }
    }

    let summary = query_causal_summary(
        &target_face,
        &replay_log,
        &decision_log,
        &lineage_events,
        &[],
    );

    let token_count = summary.narrative_token_count();
    assert!(
        token_count < 200,
        "ChainSummary narrative must be < 200 tokens, got {} tokens: {:?}",
        token_count,
        summary.get_narrative()
    );

    assert!(
        (summary.get_min_margin() - 0.001).abs() < 1e-10,
        "Min margin must be exactly 0.001 (tightest decision at step 35), got {}",
        summary.get_min_margin()
    );

    assert!(
        summary.get_decision_steps() >= 1,
        "Must have at least 1 decision step (step 35 is NearBoundary)"
    );

    assert_eq!(
        summary.get_total_steps(),
        5,
        "Must have exactly 5 total steps (only relevant ones), got {}",
        summary.get_total_steps()
    );
}

/// PV-54.5: N-ring retention — causal chain includes operations that
/// modified bounding vertices/edges, excludes completely unrelated ops.
///
/// Setup:
/// - 50-step replay log
/// - 5 ops create/modify the target Face (face_op_1/8/22/37/49)
/// - 3 ops modify bounding Vertex entities (vertex_bound_5/15/30)
/// - 42 ops are completely unrelated (solid_noise_*)
///
/// Expected: chain has exactly 8 steps (5 face + 3 vertex N-ring).
/// The 42 unrelated ops must be excluded entirely.
#[test]
fn pv_54_5_chain_retains_nring_excludes_unrelated() {
    let target_face = EntityRef::new("Face", 4);

    let nring_vertex_0 = EntityRef::new("Vertex", 10);
    let nring_vertex_1 = EntityRef::new("Vertex", 11);
    let nring_vertex_2 = EntityRef::new("Vertex", 12);
    let nring_entities = vec![
        nring_vertex_0.clone(),
        nring_vertex_1.clone(),
        nring_vertex_2.clone(),
    ];

    let face_steps: Vec<u64> = vec![1, 8, 22, 37, 49];
    let vertex_steps: Vec<u64> = vec![5, 15, 30];

    let mut replay_log = ReplayLog::new();
    for i in 1..=50u64 {
        let name = if face_steps.contains(&i) {
            format!("face_op_{}", i)
        } else if vertex_steps.contains(&i) {
            format!("vertex_bound_{}", i)
        } else {
            format!("solid_noise_{}", i)
        };
        replay_log.record(make_replay_entry(
            &name,
            i,
            (i as u128 - 1) * 100,
            i as u128 * 100,
        ));
    }

    let mut lineage_events = Vec::new();
    let mut decision_log = DecisionLog::new();

    for (idx, &step) in face_steps.iter().enumerate() {
        let op_name = format!("face_op_{}", step);
        let op_sig = OpSignature::with_id(&op_name, step);
        let lineage = if idx == 0 {
            Lineage::root(1, op_sig.clone())
        } else {
            let parent = Lineage::root(1, OpSignature::with_id("dummy", 0));
            Lineage::derive(&parent, op_sig.clone())
        };

        if idx == 0 {
            lineage_events.push(LineageEvent::EntityCreated {
                entity_kind: EntityKind::Face,
                lineage,
            });
        } else {
            lineage_events.push(LineageEvent::EntityModified {
                entity_kind: EntityKind::Face,
                old_lineage: Lineage::root(1, OpSignature::with_id("dummy", 0)),
                new_lineage: lineage,
            });
        }

        decision_log.record(make_entity_decision(
            step * 100,
            &target_face,
            DecisionTier::Deterministic,
            1.0,
        ));
    }

    for (idx, &step) in vertex_steps.iter().enumerate() {
        let op_name = format!("vertex_bound_{}", step);
        let op_sig = OpSignature::with_id(&op_name, step);
        let parent = Lineage::root(1, OpSignature::with_id("dummy", 0));
        let lineage = Lineage::derive(&parent, op_sig);

        lineage_events.push(LineageEvent::EntityModified {
            entity_kind: EntityKind::Vertex,
            old_lineage: Lineage::root(1, OpSignature::with_id("dummy", 0)),
            new_lineage: lineage,
        });

        let vertex_ref = &nring_entities[idx];
        decision_log.record(make_entity_decision(
            step * 100 + 1,
            vertex_ref,
            DecisionTier::Resolved,
            0.5,
        ));
    }

    for step in 1..=50u64 {
        if !face_steps.contains(&step) && !vertex_steps.contains(&step) {
            let op_name = format!("solid_noise_{}", step);
            lineage_events.push(LineageEvent::EntityModified {
                entity_kind: EntityKind::Solid,
                old_lineage: Lineage::root(99, OpSignature::with_id(&op_name, step)),
                new_lineage: Lineage::root(99, OpSignature::with_id(&op_name, step + 1)),
            });
        }
    }

    let chain = query_causal_chain(
        &target_face,
        &replay_log,
        &decision_log,
        &lineage_events,
        &nring_entities,
    );

    let expected_count = face_steps.len() + vertex_steps.len();
    assert_eq!(
        chain.get_steps().len(),
        expected_count,
        "Chain must have exactly {} steps (5 face + 3 vertex N-ring), got {}",
        expected_count,
        chain.get_steps().len()
    );

    let step_names: Vec<&str> = chain.get_steps().iter()
        .map(|s| s.get_operation().get_name())
        .collect();

    for &idx in &face_steps {
        let expected = format!("face_op_{}", idx);
        assert!(
            step_names.contains(&expected.as_str()),
            "Face op '{}' must be present. Got: {:?}",
            expected,
            step_names
        );
    }

    for &idx in &vertex_steps {
        let expected = format!("vertex_bound_{}", idx);
        assert!(
            step_names.contains(&expected.as_str()),
            "Vertex N-ring op '{}' must be present. Got: {:?}",
            expected,
            step_names
        );
    }

    for name in &step_names {
        assert!(
            !name.starts_with("solid_noise_"),
            "Unrelated op '{}' must NOT be in chain. Got: {:?}",
            name,
            step_names
        );
    }

    let vertex_decisions: Vec<&TracedDecision> = chain.get_steps()
        .iter()
        .flat_map(|s| s.get_decisions())
        .filter(|d| {
            d.get_entity_scope()
                .map(|s| s.get_kind() == "Vertex")
                .unwrap_or(false)
        })
        .collect();
    assert!(
        !vertex_decisions.is_empty(),
        "Chain must include decisions scoped to N-ring vertices"
    );
}
