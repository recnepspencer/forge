use crate::facade::{
    compare_lineage_records, compare_replay_slices, SignalRuntimePolicy, StageExecutor,
};

#[cfg(feature = "parallel")]
use crate::facade::{NodeEvaluationResult, SignalTransaction};
#[cfg(feature = "parallel")]
use crate::tests::support::{version_ab, ASPECT_A};

use super::fintech_session::fintech_session;
use super::geometry_session::geometry_session;
#[cfg(feature = "parallel")]
use super::geometry_world::{build_geometry_fixture, geometry_evaluator, seed_geometry_baseline};
use super::workflow_truth::{AdversarialWorkflow, WorkflowDomain, WorkflowSeed};

#[cfg(feature = "parallel")]
use super::workflow_truth::trace_adv;
#[cfg(feature = "parallel")]
use super::workflow_truth::ReferenceModel;

#[cfg(feature = "parallel")]
type GeometryTransaction<'a> = SignalTransaction<'a, (), (), (), (), ()>;

#[test]
fn geometry_kernel_adversarial_seed_matrix_keeps_invariants() {
    for (seed, workflow) in [
        (
            WorkflowSeed(7),
            AdversarialWorkflow::FeatureEditRewireRestoreChurn,
        ),
        (
            WorkflowSeed(19),
            AdversarialWorkflow::PartitionScopeCliffSession,
        ),
    ] {
        let _ = geometry_session(
            seed,
            workflow,
            SignalRuntimePolicy::kernel().with_history_limit(8),
            StageExecutor::Serial,
        );
    }
}

#[test]
fn fintech_adversarial_seed_matrix_keeps_invariants() {
    for (seed, workflow) in [
        (
            WorkflowSeed(11),
            AdversarialWorkflow::LateTickCorrectionWithBranchReplay,
        ),
        (
            WorkflowSeed(23),
            AdversarialWorkflow::RiskAlertFlapUnderMemoChurn,
        ),
    ] {
        let _ = fintech_session(
            seed,
            workflow,
            SignalRuntimePolicy::fintech().with_history_limit(8),
            StageExecutor::Serial,
        );
    }
}

#[test]
fn policy_overlap_for_generated_workflows_matches_guaranteed_truth() {
    for (domain, workflow, seed) in [
        (
            WorkflowDomain::GeometryKernel,
            AdversarialWorkflow::FeatureEditRewireRestoreChurn,
            WorkflowSeed(31),
        ),
        (
            WorkflowDomain::Fintech,
            AdversarialWorkflow::LateTickCorrectionWithBranchReplay,
            WorkflowSeed(37),
        ),
    ] {
        let runs = [
            (
                "operational",
                match domain {
                    WorkflowDomain::GeometryKernel => geometry_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::operational().with_history_limit(4),
                        StageExecutor::Serial,
                    ),
                    WorkflowDomain::Fintech => fintech_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::operational().with_history_limit(4),
                        StageExecutor::Serial,
                    ),
                },
            ),
            (
                "development",
                match domain {
                    WorkflowDomain::GeometryKernel => geometry_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::development().with_history_limit(6),
                        StageExecutor::Serial,
                    ),
                    WorkflowDomain::Fintech => fintech_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::development().with_history_limit(6),
                        StageExecutor::Serial,
                    ),
                },
            ),
            (
                "forensic",
                match domain {
                    WorkflowDomain::GeometryKernel => geometry_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::forensic().with_history_limit(8),
                        StageExecutor::Serial,
                    ),
                    WorkflowDomain::Fintech => fintech_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::forensic().with_history_limit(8),
                        StageExecutor::Serial,
                    ),
                },
            ),
        ];

        for pair in runs.windows(2) {
            let (_, (h1, replay1, lineage1)) = &pair[0];
            let (name2, (h2, replay2, lineage2)) = &pair[1];
            let replay_diff = compare_replay_slices(replay1, replay2);
            let lineage_diff = compare_lineage_records(lineage1, lineage2);
            if !replay_diff.is_empty() {
                h2.panic_diff(
                    &SignalRuntimePolicy::development(),
                    "serial",
                    format!("policy overlap drift against {name2}"),
                    replay_diff.mismatches.len(),
                    lineage_diff.mismatches.len(),
                );
            }
            let _ = h1;
        }
    }
}

#[cfg(feature = "parallel")]
#[test]
fn parallel_geometry_hostile_session_matches_serial_truth() {
    trace_adv("[parallel-test] geometry:start");
    let workflow = AdversarialWorkflow::FeatureEditRewireRestoreChurn;
    let seed = WorkflowSeed(41);
    let serial = geometry_session(
        seed,
        workflow,
        SignalRuntimePolicy::development().with_history_limit(8),
        StageExecutor::Serial,
    );
    trace_adv("[parallel-test] geometry:serial-finished");
    let parallel = geometry_session(
        seed,
        workflow,
        SignalRuntimePolicy::development().with_history_limit(8),
        StageExecutor::aggressive_parallel(),
    );
    trace_adv("[parallel-test] geometry:parallel-finished");

    let replay_diff = compare_replay_slices(&serial.1, &parallel.1);
    let lineage_diff = compare_lineage_records(&serial.2, &parallel.2);
    if !replay_diff.is_empty() || !lineage_diff.is_empty() {
        parallel.0.panic_diff(
            &SignalRuntimePolicy::development(),
            "serial-vs-parallel",
            "geometry executor differential drift",
            replay_diff.mismatches.len(),
            lineage_diff.mismatches.len(),
        );
    }
}

#[cfg(feature = "parallel")]
#[test]
fn parallel_fintech_hostile_session_matches_serial_truth() {
    trace_adv("[parallel-test] fintech:start");
    let workflow = AdversarialWorkflow::RiskAlertFlapUnderMemoChurn;
    let seed = WorkflowSeed(53);
    let serial = fintech_session(
        seed,
        workflow,
        SignalRuntimePolicy::development().with_history_limit(8),
        StageExecutor::Serial,
    );
    trace_adv("[parallel-test] fintech:serial-finished");
    let parallel = fintech_session(
        seed,
        workflow,
        SignalRuntimePolicy::development().with_history_limit(8),
        StageExecutor::aggressive_parallel(),
    );
    trace_adv("[parallel-test] fintech:parallel-finished");

    let replay_diff = compare_replay_slices(&serial.1, &parallel.1);
    let lineage_diff = compare_lineage_records(&serial.2, &parallel.2);
    if !replay_diff.is_empty() || !lineage_diff.is_empty() {
        parallel.0.panic_diff(
            &SignalRuntimePolicy::development(),
            "serial-vs-parallel",
            "fintech executor differential drift",
            replay_diff.mismatches.len(),
            lineage_diff.mismatches.len(),
        );
    }
}

#[cfg(feature = "parallel")]
#[test]
fn focused_parallel_branch_restore_and_evaluate_dirty_regression() {
    trace_adv("[parallel-test] focused-regression:start");
    let mut fixture =
        build_geometry_fixture(SignalRuntimePolicy::development().with_history_limit(8));
    let mut model = ReferenceModel::default();
    let (main, main_snapshot) = seed_geometry_baseline(&mut fixture, &mut model);
    trace_adv("[parallel-test] focused-regression:seeded-main");

    let feature = fixture.runtime.create_branch("feature").unwrap();
    fixture.runtime.switch_branch(feature.clone()).unwrap();
    trace_adv("[parallel-test] focused-regression:feature-branch");

    let mut ctx = ();
    fixture
        .runtime
        .transaction(&mut ctx, |tx: &mut GeometryTransaction<'_>| {
            tx.mark_dirty(fixture.source_a, ASPECT_A)?;
            tx.read(fixture.source_a, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(4, 1))
                        .with_output_identity("source-a-4"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    trace_adv("[parallel-test] focused-regression:mutated-feature");

    fixture
        .runtime
        .evaluate_dirty_with_executor(
            &(),
            &geometry_evaluator(&fixture),
            StageExecutor::aggressive_parallel(),
        )
        .unwrap();
    trace_adv("[parallel-test] focused-regression:parallel-evaluated");

    let feature_snapshot = fixture
        .runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    fixture
        .runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .unwrap();
    trace_adv("[parallel-test] focused-regression:feature-restored");

    fixture.runtime.switch_branch(main.clone()).unwrap();
    fixture
        .runtime
        .restore_branch_snapshot(main, &main_snapshot)
        .unwrap();
    trace_adv("[parallel-test] focused-regression:main-restored");

    let replay = fixture.runtime.observe().replay_for_branch(feature.id);
    assert!(
        !replay.frames.is_empty(),
        "parallel branch restore regression should leave observable replay"
    );
}

#[ignore]
#[test]
fn long_geometry_churn_seed_matrix_stays_hard_to_surprise() {
    for seed in [WorkflowSeed(71), WorkflowSeed(89), WorkflowSeed(97)] {
        let _ = geometry_session(
            seed,
            AdversarialWorkflow::FeatureEditRewireRestoreChurn,
            SignalRuntimePolicy::kernel().with_history_limit(12),
            StageExecutor::Serial,
        );
    }
}

#[ignore]
#[cfg(feature = "parallel")]
#[test]
fn long_fintech_parallel_churn_seed_matrix_stays_hard_to_surprise() {
    for seed in [WorkflowSeed(101), WorkflowSeed(131), WorkflowSeed(149)] {
        let _ = fintech_session(
            seed,
            AdversarialWorkflow::RiskAlertFlapUnderMemoChurn,
            SignalRuntimePolicy::fintech().with_history_limit(12),
            StageExecutor::aggressive_parallel(),
        );
    }
}
