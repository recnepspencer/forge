//! P3.1 acceptance tests — Checkpoint Diffing.
//!
//! PV-33: 10-step chain → diffs correctly identify new decisions at each step.
//! PV-33b: Union of sequential diffs exactly reconstructs the final DecisionLog.
//! PV-34: Identical operation re-run → diff is empty.

use forge_core::tracing::replay::checkpoint_diff::{diff_decision_logs, CheckpointLog};
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionLog, DecisionTier, TracedDecision,
};

fn make_chain_decision(step: u64, decision_num: u64, tier: DecisionTier) -> TracedDecision {
    let id = step * 100 + decision_num;
    TracedDecision::new(
        DecisionId(id),
        DecisionKind::Exact,
        tier,
        1.0 - (step as f64 * 0.05),
        DecisionContext::Classification {
            point: [step as f64, decision_num as f64, 0.0],
            result: format!("Inside_step{}", step),
        },
    )
}

/// PV-33: 10-step Boolean chain → diffs correctly identify new decisions at each step.
///
/// Simulates a 10-step operation chain where each step adds 3 new decisions.
/// Verifies that the checkpoint diff at each step correctly identifies exactly
/// the newly added decisions with no spurious removals or changes.
#[test]
fn pv_33_chain_diffs_identify_new_decisions() {
    let mut checkpoint = CheckpointLog::new();
    let mut log = DecisionLog::new();

    checkpoint.snapshot(&log);

    for step in 1..=10 {
        let span = log.start_span(&format!("boolean_step_{}", step));
        log.record(make_chain_decision(step, 1, DecisionTier::Deterministic));
        log.record(make_chain_decision(step, 2, DecisionTier::Resolved));
        log.record(make_chain_decision(step, 3, DecisionTier::NearBoundary));
        log.end_span(span, step * 1000);

        checkpoint.snapshot(&log);
    }

    assert_eq!(checkpoint.step_count(), 11);

    for step in 1..=10usize {
        let delta = checkpoint
            .delta_at(step)
            .unwrap_or_else(|| panic!("Step {} should have a delta", step));

        assert_eq!(
            delta.get_added().len(),
            3,
            "Step {} should add exactly 3 new decisions",
            step
        );

        assert!(
            delta.get_removed().is_empty(),
            "Step {} should have no removed decisions (chain is append-only)",
            step
        );

        assert!(
            delta.get_changed().is_empty(),
            "Step {} should have no changed decisions (prior decisions remain identical)",
            step
        );

        let step_u64 = step as u64;
        let added_ids: Vec<u64> = delta.get_added().iter().map(|d| d.get_id().0).collect();
        assert!(
            added_ids.contains(&(step_u64 * 100 + 1)),
            "Step {} delta should contain decision {}",
            step,
            step_u64 * 100 + 1
        );
        assert!(
            added_ids.contains(&(step_u64 * 100 + 2)),
            "Step {} delta should contain decision {}",
            step,
            step_u64 * 100 + 2
        );
        assert!(
            added_ids.contains(&(step_u64 * 100 + 3)),
            "Step {} delta should contain decision {}",
            step,
            step_u64 * 100 + 3
        );
    }

    let full_delta = checkpoint
        .delta_between(0, 10)
        .expect("Should have a delta between step 0 and step 10");
    assert_eq!(
        full_delta.get_added().len(),
        30,
        "Full chain should have 30 total new decisions"
    );
    assert!(full_delta.get_removed().is_empty());
    assert!(full_delta.get_changed().is_empty());
}

/// PV-34: Identical operation re-run → diff is empty.
///
/// Runs the same operation twice producing identical DecisionLogs.
/// Verifies that the diff between them is completely empty — no spurious
/// additions, removals, or changes.
#[test]
fn pv_34_identical_rerun_empty_diff() {
    let build_log = || {
        let mut log = DecisionLog::new();
        let span = log.start_span("boolean_union");
        log.record(TracedDecision::new(
            DecisionId(1),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Classification {
                point: [0.5, 0.5, 0.5],
                result: "Inside".to_string(),
            },
        ));
        log.record(TracedDecision::new(
            DecisionId(2),
            DecisionKind::NearBoundary { threshold: 1e-6 },
            DecisionTier::NearBoundary,
            0.1,
            DecisionContext::Tolerance {
                measured: 9e-7,
                threshold: 1e-6,
            },
        ));
        log.record(TracedDecision::new(
            DecisionId(3),
            DecisionKind::PolicyApplied {
                policy: forge_core::PolicyKind::CoincidentGeometry,
                default_used: true,
            },
            DecisionTier::PolicyApplied,
            0.05,
            DecisionContext::Coincidence {
                entity_a: forge_core::EntityRef::new(forge_core::EntityKind::Face, 0),
                entity_b: forge_core::EntityRef::new(forge_core::EntityKind::Face, 1),
            },
        ));
        log.end_span(span, 5000);
        log
    };

    let run_1 = build_log();
    let run_2 = build_log();

    let delta = diff_decision_logs(&run_1, &run_2);

    assert!(
        delta.is_empty(),
        "Identical re-run should produce empty diff, got: {} added, {} removed, {} changed",
        delta.get_added().len(),
        delta.get_removed().len(),
        delta.get_changed().len()
    );
    assert_eq!(delta.total_changes(), 0);

    let mut checkpoint = CheckpointLog::new();
    checkpoint.snapshot(&run_1);
    checkpoint.snapshot(&run_2);

    let checkpoint_delta = checkpoint.delta_at(1).expect("Should have delta at step 1");
    assert!(
        checkpoint_delta.is_empty(),
        "CheckpointLog delta between identical logs should be empty"
    );
}

/// PV-33b: Union of sequential diffs exactly reconstructs the final DecisionLog.
///
/// Collects every `added` decision from each step's delta, then verifies
/// that the union of these sets is exactly identical to the full set of
/// decisions in the final DecisionLog. Zero false positives (no spurious IDs),
/// zero false negatives (no missing IDs).
#[test]
fn pv_33b_union_of_diffs_reconstructs_final_log() {
    let mut checkpoint = CheckpointLog::new();
    let mut log = DecisionLog::new();

    checkpoint.snapshot(&log);

    for step in 1..=10u64 {
        let span = log.start_span(&format!("boolean_step_{}", step));
        log.record(make_chain_decision(step, 1, DecisionTier::Deterministic));
        log.record(make_chain_decision(step, 2, DecisionTier::Resolved));
        log.record(make_chain_decision(step, 3, DecisionTier::NearBoundary));
        log.end_span(span, step * 1000);

        checkpoint.snapshot(&log);
    }

    let mut union_ids: std::collections::BTreeSet<u64> = std::collections::BTreeSet::new();
    for step in 1..=10usize {
        let delta = checkpoint.delta_at(step).expect("Delta should exist");

        assert!(
            delta.get_removed().is_empty(),
            "Append-only chain should never have removals at step {}",
            step
        );
        assert!(
            delta.get_changed().is_empty(),
            "Append-only chain should never have changes at step {}",
            step
        );

        for d in delta.get_added() {
            let inserted = union_ids.insert(d.get_id().0);
            assert!(
                inserted,
                "Decision {} appeared in multiple step deltas — false positive",
                d.get_id().0
            );
        }
    }

    let final_log = checkpoint
        .get_snapshot(10)
        .expect("Final snapshot should exist");
    let final_ids: std::collections::BTreeSet<u64> =
        final_log.decisions().map(|d| d.get_id().0).collect();

    assert_eq!(
        union_ids,
        final_ids,
        "Union of diffs must exactly equal the final DecisionLog.\n\
         Missing (false negatives): {:?}\n\
         Extra (false positives): {:?}",
        final_ids.difference(&union_ids).collect::<Vec<_>>(),
        union_ids.difference(&final_ids).collect::<Vec<_>>()
    );

    assert_eq!(
        union_ids.len(),
        30,
        "Should have exactly 30 decisions total"
    );
}
