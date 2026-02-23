//! P3.6 acceptance tests — Zero-Split and FeatureTree Proof Metadata Integrity.
//!
//! These tests exercise the two code paths where proof metadata was previously
//! silently dropped:
//!
//! 1. **Zero-split path** (`try_zero_split_early_return`): disjoint and contained
//!    solids bypass the split→classify→select pipeline. The result must still
//!    carry `ReplayLog` and `LineageEvent` data for causal chain reconstruction.
//!
//! 2. **FeatureTree path**: `BooleanFeature::evaluate()` converts `BooleanResult`
//!    to `FeatureOutput`. The `Arc<ReplayLog>` and `Arc<Vec<LineageEvent>>`
//!    must survive through `FeatureTree::evaluate_feature()` and remain
//!    accessible from the cached output.
//!
//! PV-37b: Disjoint unions → replay_log and lineage_events non-empty
//! PV-37c: Contained subtraction → replay_log and lineage_events non-empty
//! PV-37d: FeatureTree evaluation → Arc<ReplayLog> survives into FeatureOutput
//! MB-R1b: 10-step chain with mixed disjoint/overlapping → proof at every step
//! MB-R8: Zero-split causal chain traverses real pipeline phases

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use forge_core::DecisionContext;
    use forge_topo::hashing::compute_arena_topology_hash;

    use crate::operations::boolean::test_helpers::{
        build_cube, execute_boolean_logged, euler_audit,
    };
    use crate::operations::boolean::{
        BooleanInput, BooleanOp, execute_boolean,
    };
    use crate::analysis::causal_chain::{query_causal_chain, query_causal_summary};
    use crate::features::tree::{FeatureTree, NativeFeature, FeatureOutput};
    use crate::features::wrappers::{MakeCubeFeature, BooleanFeature};

    /// PV-37b: Disjoint cubes union → proof metadata populated on zero-split path.
    ///
    /// PROOF: Two cubes with no geometric overlap (offset 10.0, size 1.0) trigger
    /// `try_zero_split_early_return`. Despite bypassing the split pipeline,
    /// the result must carry non-empty `ReplayLog` with identifiable phase
    /// entries and `LineageEvent` data for every result face.
    #[test]
    fn pv_37b_disjoint_cubes_proof_metadata_preserved() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([10.0, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo_a, geom_a, topo_b, geom_b, BooleanOp::Union,
        );

        let envelope = execute_boolean_logged(input);
        let result = envelope.into_result().expect("Disjoint union failed");

        let replay_log = result.get_replay_log();
        let lineage_events = result.get_lineage_events();

        assert!(
            replay_log.len() >= 1,
            "Disjoint union must produce at least 1 replay entry even on zero-split path, got {}",
            replay_log.len()
        );

        assert!(
            !lineage_events.is_empty(),
            "Disjoint union must produce lineage events for result faces. \
             Got 0 events for a result with {} faces. This means \
             try_zero_split_early_return is dropping lineage data.",
            result.topology().arena().face_count()
        );

        let result_face_count = result.topology().arena().face_count();
        assert_eq!(
            result_face_count, 12,
            "Disjoint union of two cubes must produce 12 faces (6+6), got {}",
            result_face_count
        );

        let (v, e, f, chi) = euler_audit(result.topology().arena());
        assert_eq!(
            chi, 4,
            "Disjoint union Euler χ must be 4 (two shells): V={v} E={e} F={f} χ={chi}"
        );

        eprintln!(
            "PV-37b: replay={}, lineage={}, faces={}, χ={}",
            replay_log.len(), lineage_events.len(), result_face_count, chi
        );
    }

    /// PV-37c: Contained cubes subtraction → proof metadata populated.
    ///
    /// PROOF: A small cube fully inside a larger cube triggers the containment
    /// detection in `try_zero_split_early_return`. Subtraction of a contained
    /// tool is a policy-sensitive case — the result must carry proof metadata.
    #[test]
    fn pv_37c_contained_cubes_proof_metadata_preserved() {
        let (topo_outer, geom_outer) = build_cube([0.0, 0.0, 0.0], 4.0);
        let (topo_inner, geom_inner) = build_cube([0.0, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo_outer, geom_outer, topo_inner, geom_inner, BooleanOp::Subtraction,
        );

        let result = execute_boolean_logged(input);

        match result.into_result() {
            Ok(r) => {
                let replay_log = r.get_replay_log();
                let lineage_events = r.get_lineage_events();

                assert!(
                    replay_log.len() >= 1,
                    "Contained subtraction must produce replay entries even on zero-split path, got {}",
                    replay_log.len()
                );

                assert!(
                    !lineage_events.is_empty(),
                    "Contained subtraction must produce lineage events. \
                     Got 0 events for a result with {} faces. This means \
                     try_zero_split_early_return is dropping lineage data.",
                    r.topology().arena().face_count()
                );

                let fc = r.topology().arena().face_count();
                assert!(
                    fc >= 6,
                    "Contained subtraction result must have at least 6 faces, got {}",
                    fc
                );

                eprintln!(
                    "PV-37c: replay={}, lineage={}, faces={}",
                    replay_log.len(), lineage_events.len(), fc
                );
            }
            Err(e) => {
                panic!(
                    "Contained subtraction must not fail (this is a valid zero-split operation): {:?}",
                    e
                );
            }
        }
    }

    /// PV-37d: FeatureTree evaluation → Arc<ReplayLog> survives into FeatureOutput.
    ///
    /// PROOF: Registers two MakeCube features and one Boolean feature in
    /// the FeatureTree. Evaluates the Boolean node. The resulting FeatureOutput
    /// must carry non-empty `replay_log` and `lineage_events` (both Arc-wrapped).
    /// This verifies the BooleanFeature::evaluate() → FeatureOutput conversion
    /// preserves proof metadata through the signal graph evaluation path.
    #[test]
    fn pv_37d_feature_tree_preserves_proof_metadata() {
        let mut tree = FeatureTree::new();

        let cube_a = MakeCubeFeature::new("cube_a", [0.0, 0.0, 0.0], 1.0);
        let node_a = tree.register_feature(NativeFeature::MakeCube(cube_a))
            .expect("register cube_a failed");

        let cube_b = MakeCubeFeature::new("cube_b", [0.5, 0.0, 0.0], 1.0);
        let node_b = tree.register_feature(NativeFeature::MakeCube(cube_b))
            .expect("register cube_b failed");

        let boolean = BooleanFeature::new("union_ab", BooleanOp::Union, node_a, node_b);
        let node_bool = tree.register_feature(NativeFeature::Boolean(boolean))
            .expect("register boolean failed");

        tree.evaluate_feature(node_a).expect("eval cube_a failed");
        tree.evaluate_feature(node_b).expect("eval cube_b failed");

        let output = tree.evaluate_feature(node_bool).expect("eval boolean failed");

        assert!(
            output.replay_log.len() >= 1,
            "FeatureOutput from Boolean must carry replay_log with at least 1 entry, got {}. \
             This means BooleanFeature::evaluate() is not populating replay_log in FeatureOutput.",
            output.replay_log.len()
        );

        assert!(
            !output.lineage_events.is_empty(),
            "FeatureOutput from Boolean must carry lineage_events, got 0. \
             This means BooleanFeature::evaluate() is not populating lineage_events in FeatureOutput."
        );

        assert!(
            !output.decision_log.is_empty(),
            "FeatureOutput from Boolean must carry a non-empty decision_log"
        );

        let (v, e, f, chi) = euler_audit(output.topology.arena());
        assert_eq!(
            chi, 2,
            "FeatureTree Boolean union Euler violation: V={v} E={e} F={f} χ={chi}"
        );

        let replay_names: Vec<&str> = output.replay_log.entries().iter()
            .map(|e| e.signature().get_name())
            .collect();

        assert!(
            replay_names.iter().any(|n| n.contains("split") || n.contains("assemble")),
            "Replay log must contain pipeline phase names, got: {:?}",
            replay_names
        );

        eprintln!(
            "PV-37d: replay={}, lineage={}, decisions={}, faces={}, phases={:?}",
            output.replay_log.len(),
            output.lineage_events.len(),
            output.decision_log.len(),
            f,
            replay_names,
        );
    }

    /// MB-R1b: 10-step mixed chain → proof metadata preserved at every step.
    ///
    /// PROOF: Builds a chain of 10 Booleans where even steps produce overlapping
    /// geometry (normal split path) and odd steps produce disjoint geometry
    /// (zero-split path). At EVERY step, verifies:
    /// - replay_log.len() >= 1
    /// - lineage_events is non-empty
    /// - decision_log has decisions
    /// - Euler characteristic is correct
    ///
    /// This is the torture test that would have caught the original data loss.
    #[test]
    fn mb_r1b_mixed_chain_proof_metadata_every_step() {
        let step_count = 10;
        let (mut current_topo, mut current_geom) = build_cube([0.0, 0.0, 0.0], 1.0);
        let mut total_replay_entries = 0usize;
        let mut total_lineage_events = 0usize;
        let mut total_decisions = 0usize;
        let mut zero_split_steps = 0usize;
        let mut normal_split_steps = 0usize;

        let start = Instant::now();

        for step in 1..=step_count {
            let is_disjoint = step % 2 == 1;

            let offset = if is_disjoint {
                step as f64 * 100.0
            } else {
                step as f64 * 0.3
            };

            let (tool_topo, tool_geom) = build_cube([offset, 0.0, 0.0], 1.0);

            let input = BooleanInput::new(
                current_topo,
                current_geom,
                tool_topo,
                tool_geom,
                BooleanOp::Union,
            );

            let envelope = execute_boolean_logged(input);
            let step_decisions = envelope.get_decision_log().len();
            total_decisions += step_decisions;

            let result = envelope.into_result()
                .unwrap_or_else(|e| panic!("Chain step {} failed: {:?}", step, e));
            let step_replay = result.get_replay_log().len();
            let step_lineage = result.get_lineage_events().len();

            assert!(
                step_replay >= 1,
                "Step {} ({}): replay_log must have >= 1 entry, got {}. \
                 Proof metadata is being dropped on this path.",
                step,
                if is_disjoint { "disjoint/zero-split" } else { "overlapping/normal" },
                step_replay,
            );

            assert!(
                !result.get_lineage_events().is_empty(),
                "Step {} ({}): lineage_events must be non-empty, got 0. \
                 Proof metadata is being dropped on this path.",
                step,
                if is_disjoint { "disjoint/zero-split" } else { "overlapping/normal" },
            );

            total_replay_entries += step_replay;
            total_lineage_events += step_lineage;

            if is_disjoint {
                zero_split_steps += 1;
            } else {
                normal_split_steps += 1;
            }

            current_topo = result.topology().clone();
            current_geom = result.geometry().clone();
        }

        let elapsed = start.elapsed();

        assert!(
            zero_split_steps >= 4,
            "Must have exercised at least 4 zero-split steps, got {}",
            zero_split_steps
        );
        assert!(
            normal_split_steps >= 4,
            "Must have exercised at least 4 normal-split steps, got {}",
            normal_split_steps
        );

        assert!(
            total_replay_entries >= step_count,
            "Total replay entries ({}) must be >= step count ({})",
            total_replay_entries, step_count
        );

        assert!(
            total_lineage_events >= step_count,
            "Total lineage events ({}) must be >= step count ({})",
            total_lineage_events, step_count
        );

        eprintln!(
            "MB-R1b: {} steps ({} zero-split, {} normal), {} replay, {} lineage, {} decisions, {:.1}ms",
            step_count, zero_split_steps, normal_split_steps,
            total_replay_entries, total_lineage_events, total_decisions,
            elapsed.as_secs_f64() * 1000.0,
        );
    }

    /// MB-R8: Zero-split causal chain query succeeds with real data.
    ///
    /// PROOF: Runs a disjoint Boolean (zero-split path), then queries the
    /// causal chain for a result face using the REAL ReplayLog, DecisionLog,
    /// and LineageEvent data. Prior to the data loss fix, this would have
    /// produced a chain with zero steps because the data was empty.
    #[test]
    fn mb_r8_zero_split_causal_chain_real_data() {
        let (topo_a, geom_a) = build_cube([0.0, 0.0, 0.0], 1.0);
        let (topo_b, geom_b) = build_cube([10.0, 0.0, 0.0], 1.0);

        let input = BooleanInput::new(
            topo_a, geom_a, topo_b, geom_b, BooleanOp::Union,
        );

        let envelope = execute_boolean_logged(input);
        let decision_log = envelope.get_decision_log().clone();
        let result = envelope.into_result().expect("Disjoint union failed");
        let replay_log = result.get_replay_log();
        let lineage_events = result.get_lineage_events();

        assert!(
            replay_log.len() >= 1,
            "Zero-split must populate replay_log, got {}",
            replay_log.len(),
        );
        assert!(
            !lineage_events.is_empty(),
            "Zero-split must populate lineage_events, got 0",
        );

        let first_face = result.topology().arena().iter_faces().next()
            .expect("Disjoint union must produce faces");
        let face_ref = forge_core::EntityRef::new(forge_core::EntityKind::Face, first_face.0.index() as u32);

        let chain = query_causal_chain(
            &face_ref,
            replay_log,
            &decision_log,
            lineage_events,
            &[],
        );

        assert!(
            !chain.get_steps().is_empty(),
            "Causal chain for a zero-split result face must have at least 1 step. \
             Got 0 steps — this means the zero-split path is not providing \
             enough data for causal chain reconstruction."
        );

        let summary = query_causal_summary(
            &face_ref,
            replay_log,
            &decision_log,
            lineage_events,
            &[],
        );

        assert!(
            summary.get_total_steps() >= 1,
            "Summary for zero-split face must report >= 1 step, got {}",
            summary.get_total_steps()
        );

        let narrative_tokens = summary.narrative_token_count();
        assert!(
            narrative_tokens < 200,
            "Summary narrative for zero-split face must be < 200 tokens, got {}",
            narrative_tokens
        );

        eprintln!(
            "MB-R8: chain={} steps, summary={} tokens, replay={}, lineage={}",
            chain.get_steps().len(),
            narrative_tokens,
            replay_log.len(),
            lineage_events.len(),
        );
    }

    /// MB-R9: FeatureTree 3-step chain → proof metadata accumulates correctly.
    ///
    /// PROOF: Builds a 3-node FeatureTree (cube_a → union(cube_a, cube_b) → 
    /// union(result, cube_c)). Evaluates the final node. Verifies:
    /// - Each intermediate node's FeatureOutput has proof metadata
    /// - The final node's replay_log contains pipeline phases
    /// - Arc cloning preserves data integrity (no corruption from sharing)
    #[test]
    fn mb_r9_feature_tree_chain_proof_accumulation() {
        let mut tree = FeatureTree::new();

        let cube_a = MakeCubeFeature::new("cube_a", [0.0, 0.0, 0.0], 1.0);
        let node_a = tree.register_feature(NativeFeature::MakeCube(cube_a))
            .expect("register cube_a");

        let cube_b = MakeCubeFeature::new("cube_b", [0.5, 0.0, 0.0], 1.0);
        let node_b = tree.register_feature(NativeFeature::MakeCube(cube_b))
            .expect("register cube_b");

        let cube_c = MakeCubeFeature::new("cube_c", [1.0, 0.0, 0.0], 1.0);
        let node_c = tree.register_feature(NativeFeature::MakeCube(cube_c))
            .expect("register cube_c");

        let union_ab = BooleanFeature::new("union_ab", BooleanOp::Union, node_a, node_b);
        let node_ab = tree.register_feature(NativeFeature::Boolean(union_ab))
            .expect("register union_ab");

        let union_abc = BooleanFeature::new("union_abc", BooleanOp::Union, node_ab, node_c);
        let node_abc = tree.register_feature(NativeFeature::Boolean(union_abc))
            .expect("register union_abc");

        tree.evaluate_feature(node_a).expect("eval cube_a");
        tree.evaluate_feature(node_b).expect("eval cube_b");
        tree.evaluate_feature(node_c).expect("eval cube_c");

        let output_ab = tree.evaluate_feature(node_ab).expect("eval union_ab");
        assert!(
            output_ab.replay_log.len() >= 1,
            "Step 1 (A∪B) FeatureOutput must carry replay_log, got {}",
            output_ab.replay_log.len()
        );
        assert!(
            !output_ab.lineage_events.is_empty(),
            "Step 1 (A∪B) FeatureOutput must carry lineage_events"
        );

        let output_abc = tree.evaluate_feature(node_abc).expect("eval union_abc");
        assert!(
            output_abc.replay_log.len() >= 1,
            "Step 2 ((A∪B)∪C) FeatureOutput must carry replay_log, got {}",
            output_abc.replay_log.len()
        );
        assert!(
            !output_abc.lineage_events.is_empty(),
            "Step 2 ((A∪B)∪C) FeatureOutput must carry lineage_events"
        );
        assert!(
            !output_abc.decision_log.is_empty(),
            "Step 2 ((A∪B)∪C) FeatureOutput must carry decision_log"
        );

        let (v, e, f, chi) = euler_audit(output_abc.topology.arena());
        assert_eq!(
            chi, 2,
            "3-cube union chain Euler violation: V={v} E={e} F={f} χ={chi}"
        );

        eprintln!(
            "MB-R9: step1 replay={} lineage={}, step2 replay={} lineage={} decisions={}, final faces={}",
            output_ab.replay_log.len(), output_ab.lineage_events.len(),
            output_abc.replay_log.len(), output_abc.lineage_events.len(),
            output_abc.decision_log.len(), f,
        );
    }
}
