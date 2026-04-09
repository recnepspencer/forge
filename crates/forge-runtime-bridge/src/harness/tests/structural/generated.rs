use forge_harness::facade::{ExecutionProfile, ExecutionRequest, MutationBatch};
use forge_harness::runtime::HarnessAdapter;
use serde_json::json;

use crate::facade::{SnapshotReadRecord, TruthSnapshotIdentity};
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
        (
            "ambiguous",
            ambiguous_target(),
            "RejectedAmbiguousStructuralMatch",
            "ambiguity_report",
        ),
        (
            "no-safe-match",
            no_safe_match_target(),
            "RejectedNoStructuralMatch",
            "failure_digest",
        ),
        (
            "identity-conflict",
            identity_conflict_target(),
            "RejectedIdentityAuthorityConflict",
            "identity_separation_report",
        ),
        (
            "lineage-divergence",
            lineage_divergence_target(),
            "RejectedLineageStructuralDivergence",
            "identity_separation_report",
        ),
    ];

    for (label, target, expected_outcome, expected_bundle_surface) in cases {
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
        assert_eq!(direct_run.summary["outcome_class"], json!(expected_outcome));
        assert_eq!(
            direct_run.extensions["bridge_structural_certification_bundle"]
                ["remap_artifact_digest"],
            serde_json::Value::Null
        );
        assert_eq!(direct_run.summary["failure_digest"].is_null(), false);
        assert_ne!(
            direct_run.extensions["bridge_structural_certification_bundle"]
                [expected_bundle_surface],
            serde_json::Value::Null
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
            expected_branch_diff_count: 1,
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
            expected_branch_diff_count: 1,
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
        GeneratedStep::Observe {
            label: "converged",
            expected_branch_diff_count: 0,
        },
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
            expected_branch_diff_count: 0,
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
            expected_branch_diff_count: 1,
        },
    ];

    let mut observed_digests = Vec::new();

    for step in sequence {
        match step {
            GeneratedStep::Observe {
                label,
                expected_branch_diff_count,
            } => {
                let run = adapter
                    .execute(&mut session, &fixture, &compare_request, &profile)
                    .unwrap_or_else(|error| panic!("{label} compare failed: {error}"));
                let replay = adapter
                    .execute(&mut session, &fixture, &replay_request, &profile)
                    .unwrap_or_else(|error| panic!("{label} replay failed: {error}"));

                assert_eq!(
                    run.extensions["bridge_structural_certification_bundle"]
                        ["structural_diff_report"]["branch_diff_count"],
                    json!(expected_branch_diff_count),
                    "{label} diff count drifted"
                );
                assert_eq!(
                    replay.summary["branch_compare_digest"], run.summary["branch_compare_digest"],
                    "{label} replay digest drifted from compare digest"
                );
                assert_eq!(
                    replay.summary["counter_snapshot"]["branch_comparison_diff_count"],
                    json!(expected_branch_diff_count),
                    "{label} replay counters drifted"
                );
                observed_digests.push((label, run.summary["branch_compare_digest"].clone()));
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
                            TruthSnapshotIdentity::new(snapshot),
                            vec![
                                SnapshotReadRecord::new(
                                    "entity-1:profile",
                                    entity_value.as_bytes().to_vec(),
                                ),
                                SnapshotReadRecord::new(
                                    "entity-2:profile",
                                    entity_value.as_bytes().to_vec(),
                                ),
                                SnapshotReadRecord::new(
                                    "entity-3:profile",
                                    entity3_value.as_bytes().to_vec(),
                                ),
                            ],
                        ),
                    ))
                    .push(BridgeHarnessMutation::PublishCommittedPatch(
                        committed_patch_on_branch(branch, commit, patch, snapshot, "name"),
                    ));
                adapter
                    .apply_mutation_batch(&mut session, &mutation)
                    .unwrap_or_else(|error| panic!("{batch_name} mutation failed: {error}"));
            }
        }
    }

    assert_eq!(observed_digests[0].1, observed_digests[1].1);
    assert_eq!(observed_digests[2].1, observed_digests[3].1);
    assert_ne!(observed_digests[1].1, observed_digests[2].1);
    assert_ne!(observed_digests[3].1, observed_digests[4].1);
}

#[derive(Clone, Copy)]
enum GeneratedStep<'a> {
    Observe {
        label: &'a str,
        expected_branch_diff_count: usize,
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
    assert_ne!(
        direct_run.summary["structural_match_digest"],
        serde_json::Value::Null
    );
    assert_ne!(
        branch_snapshot_run.summary["branch_compare_digest"],
        serde_json::Value::Null
    );
}
