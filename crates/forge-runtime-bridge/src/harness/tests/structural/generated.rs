use forge_harness::facade::{ExecutionProfile, ExecutionRequest, MutationBatch};
use forge_harness::runtime::HarnessAdapter;

use crate::facade::{SnapshotReadRecord, SnapshotReadRequest};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessMutation};
use crate::harness::fixtures::SnapshotFixture;

use super::super::support::committed_patch_on_branch;
use super::support::{
    ambiguous_target, branch_compare_target, branch_head_compare_target, branch_head_replay_target,
    direct_profile, exact_target, forensic_profile, identity_conflict_target,
    lineage_divergence_target, no_safe_match_target, structural_fixture,
};

#[test]
fn generated_structural_rejection_matrix_preserves_typed_fail_closed_outcomes() {
    let cases = [
        ("ambiguous", ambiguous_target()),
        ("no-safe-match", no_safe_match_target()),
        ("identity-conflict", identity_conflict_target()),
        ("lineage-divergence", lineage_divergence_target()),
    ];

    for (label, target) in cases {
        let direct_run = super::support::execute_structural_run(
            direct_profile(&format!("direct-{label}")),
            &format!("structural-{label}-direct"),
            target.clone(),
        );
        let forensic_run = super::support::execute_structural_run(
            forensic_profile(&format!("forensic-{label}")),
            &format!("structural-{label}-forensic"),
            target,
        );

        assert_eq!(
            direct_run.summary, forensic_run.summary,
            "{label} summary drifted across diagnostics tiers"
        );
        assert_eq!(
            direct_run.extensions, forensic_run.extensions,
            "{label} extension bundle drifted across diagnostics tiers"
        );
    }
}

#[test]
fn generated_branch_head_oscillation_sequence_remains_local_and_replay_safe() {
    let adapter = BridgeHarnessAdapter;
    let fixture = structural_fixture("bridge-structural-generated-oscillation");
    let compare_request = ExecutionRequest::target(
        "structural-branch-head-compare",
        branch_head_compare_target(),
    );
    let replay_request =
        ExecutionRequest::target("structural-branch-head-replay", branch_head_replay_target());
    let profile = ExecutionProfile::development("baseline");

    let mut session = adapter
        .create_runtime()
        .expect("structural harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("structural harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("structural harness load fixture");

    let sequence = [
        GeneratedStep::Observe {
            label: "initial-diff",
        },
        GeneratedStep::Mutate {
            batch_name: "publish-unrelated-1",
            branch: "unrelated",
            commit: "commit-unrelated-1",
            patch: "patch-unrelated-1",
            snapshot: "snapshot-unrelated-1",
            entity_value: "noise",
            entity3_value: "shape-mismatch-unrelated-1",
        },
        GeneratedStep::Observe {
            label: "after-unrelated-diff",
        },
        GeneratedStep::Mutate {
            batch_name: "publish-right-converged",
            branch: "right",
            commit: "commit-right-converged",
            patch: "patch-right-converged",
            snapshot: "snapshot-c",
            entity_value: "alice",
            entity3_value: "shape-mismatch-snapshot-a",
        },
        GeneratedStep::Observe { label: "converged" },
        GeneratedStep::Mutate {
            batch_name: "publish-unrelated-2",
            branch: "unrelated",
            commit: "commit-unrelated-2",
            patch: "patch-unrelated-2",
            snapshot: "snapshot-unrelated-2",
            entity_value: "noise-2",
            entity3_value: "shape-mismatch-unrelated-2",
        },
        GeneratedStep::Observe {
            label: "after-unrelated-converged",
        },
        GeneratedStep::Mutate {
            batch_name: "publish-right-diverged",
            branch: "right",
            commit: "commit-right-diverged",
            patch: "patch-right-diverged",
            snapshot: "snapshot-d",
            entity_value: "bob",
            entity3_value: "shape-mismatch-snapshot-d",
        },
        GeneratedStep::Observe {
            label: "diverged-again",
        },
    ];

    let mut observed_summaries = Vec::new();

    for step in sequence {
        match step {
            GeneratedStep::Observe { label } => {
                let run = adapter
                    .execute(&mut session, &fixture, &compare_request, &profile)
                    .unwrap_or_else(|error| panic!("{label} compare failed: {error}"));
                let replay = adapter
                    .execute(&mut session, &fixture, &replay_request, &profile)
                    .unwrap_or_else(|error| panic!("{label} replay failed: {error}"));

                assert_ne!(
                    replay.summary, run.summary,
                    "{label} replay evidence missing"
                );
                observed_summaries.push((label, run.summary));
            }
            GeneratedStep::Mutate {
                batch_name,
                branch,
                commit,
                patch,
                snapshot,
                entity_value,
                entity3_value,
            } => {
                let mutation = MutationBatch::new(batch_name)
                    .push(BridgeHarnessMutation::PublishSnapshot(
                        SnapshotFixture::new(
                            crate::truth_identity_fixtures::truth_snapshot_fixture(snapshot),
                            vec![
                                generated_structural_snapshot_record(
                                    "entity-1",
                                    forge_foundational::facade::AspectValue::String(
                                        (entity_value).into(),
                                    ),
                                ),
                                generated_structural_snapshot_record(
                                    "entity-2",
                                    forge_foundational::facade::AspectValue::String(
                                        (entity_value).into(),
                                    ),
                                ),
                                generated_structural_snapshot_record(
                                    "entity-3",
                                    forge_foundational::facade::AspectValue::String(
                                        (entity3_value).into(),
                                    ),
                                ),
                            ],
                        ),
                    ))
                    .push(BridgeHarnessMutation::PublishCommittedPatch(
                        committed_patch_on_branch(
                            crate::truth_identity_fixtures::truth_branch_fixture(branch),
                            crate::truth_identity_fixtures::truth_commit_fixture(commit),
                            crate::truth_identity_fixtures::truth_patch_fixture(patch),
                            crate::truth_identity_fixtures::truth_snapshot_fixture(snapshot),
                            forge_foundational::facade::FieldKey::new("name".to_owned())
                                .expect("valid generated structural field key"),
                        ),
                    ));
                adapter
                    .apply_mutation_batch(&mut session, &mutation)
                    .unwrap_or_else(|error| panic!("{batch_name} mutation failed: {error}"));
            }
        }
    }

    assert_eq!(observed_summaries[0].1, observed_summaries[1].1);
    assert_eq!(observed_summaries[2].1, observed_summaries[3].1);
    assert_ne!(observed_summaries[1].1, observed_summaries[2].1);
    assert_ne!(observed_summaries[3].1, observed_summaries[4].1);
}

#[derive(Clone, Copy)]
enum GeneratedStep<'a> {
    Observe {
        label: &'a str,
    },
    Mutate {
        batch_name: &'a str,
        branch: &'a str,
        commit: &'a str,
        patch: &'a str,
        snapshot: &'a str,
        entity_value: &'a str,
        entity3_value: &'a str,
    },
}

fn generated_structural_snapshot_record(
    entity_identity: &str,
    value: forge_foundational::facade::AspectValue,
) -> SnapshotReadRecord {
    SnapshotReadRecord::for_request(
        &SnapshotReadRequest::for_coarse(
            entity_identity,
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid generated structural aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
        ),
        value,
    )
}

#[test]
fn generated_exact_match_control_and_candidate_runs_preserve_same_certification_bundle() {
    let direct_run = super::support::execute_structural_run(
        direct_profile("direct-exact"),
        "structural-exact-direct",
        exact_target(),
    );
    let forensic_run = super::support::execute_structural_run(
        forensic_profile("forensic-exact"),
        "structural-exact-forensic",
        exact_target(),
    );
    let branch_snapshot_run = super::support::execute_structural_run(
        direct_profile("direct-branch-snapshot"),
        "structural-branch-snapshot",
        branch_compare_target(),
    );

    assert_eq!(direct_run.summary, forensic_run.summary);
    assert_eq!(direct_run.extensions, forensic_run.extensions);
    assert_ne!(direct_run.summary, branch_snapshot_run.summary);
}
