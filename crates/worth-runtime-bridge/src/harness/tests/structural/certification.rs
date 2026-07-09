use worth_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest, MutationBatch,
};
use worth_harness::runtime::HarnessAdapter;

use crate::facade::{SnapshotReadRecord, SnapshotReadRequest};
use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::adapter::BridgeHarnessMutation;
use crate::harness::fixtures::SnapshotFixture;

use super::super::support::committed_patch_on_branch;
use super::support::{
    ambiguous_target, branch_compare_target, branch_head_compare_target, branch_replay_target,
    direct_profile, exact_target, execute_structural_run, forensic_profile,
    identity_conflict_target, lineage_divergence_target, no_safe_match_target, remap_replay_target,
    structural_fixture,
};

#[test]
fn bridge_harness_structural_suite_7_emits_match_and_ambiguity_truth_without_winner_selection() {
    let exact_control = execute_structural_run(
        direct_profile("control"),
        "structural-remap-exact-control",
        exact_target(),
    );
    let exact_candidate = execute_structural_run(
        forensic_profile("candidate"),
        "structural-remap-exact-candidate",
        exact_target(),
    );
    let ambiguous_run = execute_structural_run(
        direct_profile("baseline"),
        "structural-remap-ambiguous",
        ambiguous_target(),
    );
    let no_safe_match_run = execute_structural_run(
        direct_profile("baseline"),
        "structural-remap-no-safe-match",
        no_safe_match_target(),
    );

    assert_eq!(
        exact_control.summary, exact_candidate.summary,
        "diagnostics tier must not change native remap summary export"
    );
    assert_eq!(
        exact_control.extensions, exact_candidate.extensions,
        "diagnostics tier must not change native remap certification export"
    );
    assert_ne!(ambiguous_run.summary, exact_control.summary);
    assert_ne!(no_safe_match_run.summary, exact_control.summary);
}

#[test]
fn bridge_harness_structural_suite_8_preserves_identity_separation_and_replay() {
    let exact_control = execute_structural_run(
        direct_profile("control"),
        "structural-remap-exact-control",
        exact_target(),
    );
    let replay_run = execute_structural_run(
        direct_profile("baseline"),
        "structural-remap-replay",
        remap_replay_target(),
    );
    let identity_conflict_run = execute_structural_run(
        direct_profile("baseline"),
        "structural-remap-identity-conflict",
        identity_conflict_target(),
    );
    let lineage_divergence_run = execute_structural_run(
        direct_profile("baseline"),
        "structural-remap-lineage-divergence",
        lineage_divergence_target(),
    );

    assert_ne!(replay_run.summary, exact_control.summary);
    assert_ne!(replay_run.extensions, exact_control.extensions);
    assert_ne!(identity_conflict_run.summary, exact_control.summary);
    assert_ne!(lineage_divergence_run.summary, exact_control.summary);
    assert_ne!(
        identity_conflict_run.summary,
        lineage_divergence_run.summary
    );
}

#[test]
fn bridge_harness_structural_suite_9_preserves_branch_diff_and_replay_determinism() {
    let adapter = BridgeHarnessAdapter;
    let fixture = structural_fixture("bridge-structural-branch-drift");
    let request = ExecutionRequest::target("structural-branch-compare", branch_compare_target());
    let replay_request =
        ExecutionRequest::target("structural-branch-replay", branch_replay_target());
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
    let branch_run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("initial branch comparison should succeed");

    let unrelated_mutation = MutationBatch::new("publish-unrelated-branch")
        .push(BridgeHarnessMutation::PublishSnapshot(
            SnapshotFixture::new(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-unrelated"),
                vec![
                    certification_structural_snapshot_record(
                        "entity-1",
                        worth_foundational::facade::AspectValue::String(("alice").into()),
                    ),
                    certification_structural_snapshot_record(
                        "entity-2",
                        worth_foundational::facade::AspectValue::String(("alice").into()),
                    ),
                    certification_structural_snapshot_record(
                        "entity-3",
                        worth_foundational::facade::AspectValue::String(
                            ("shape-mismatch-unrelated").into(),
                        ),
                    ),
                ],
            ),
        ))
        .push(BridgeHarnessMutation::PublishCommittedPatch(
            committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("unrelated"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-unrelated"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-unrelated"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-unrelated"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ),
        ));
    adapter
        .apply_mutation_batch(&mut session, &unrelated_mutation)
        .expect("unrelated mutation should apply");

    let branch_run_after_unrelated = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("branch comparison should remain local after unrelated publication");
    let replay_run = adapter
        .execute(&mut session, &fixture, &replay_request, &profile)
        .expect("branch replay should succeed");

    assert_eq!(
        branch_run.summary, branch_run_after_unrelated.summary,
        "unrelated publication must not change branch comparison summary export"
    );
    assert_ne!(
        replay_run.summary, branch_run.summary,
        "replay export must expose replay evidence without changing branch authority"
    );
}

#[test]
fn branch_head_structural_comparison_oscillates_predictably_under_branch_drift() {
    let adapter = BridgeHarnessAdapter;
    let fixture = structural_fixture("bridge-structural-branch-head-oscillation");
    let request = ExecutionRequest::target(
        "structural-branch-head-compare",
        branch_head_compare_target(),
    );
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

    let initial_run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("initial branch-head comparison should succeed");
    let initial_summary = initial_run.summary.clone();

    let converge_mutation = MutationBatch::new("publish-right-branch-convergence")
        .push(BridgeHarnessMutation::PublishSnapshot(
            SnapshotFixture::new(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-c"),
                vec![
                    certification_structural_snapshot_record(
                        "entity-1",
                        worth_foundational::facade::AspectValue::String(("alice").into()),
                    ),
                    certification_structural_snapshot_record(
                        "entity-2",
                        worth_foundational::facade::AspectValue::String(("alice").into()),
                    ),
                    certification_structural_snapshot_record(
                        "entity-3",
                        worth_foundational::facade::AspectValue::String(
                            ("shape-mismatch-snapshot-a").into(),
                        ),
                    ),
                ],
            ),
        ))
        .push(BridgeHarnessMutation::PublishCommittedPatch(
            committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("right"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-right-c"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-right-c"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-c"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ),
        ));
    adapter
        .apply_mutation_batch(&mut session, &converge_mutation)
        .expect("convergence mutation should apply");

    let converged_run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("converged branch-head comparison should succeed");
    let converged_summary = converged_run.summary.clone();

    let diverge_mutation = MutationBatch::new("publish-right-branch-divergence")
        .push(BridgeHarnessMutation::PublishSnapshot(
            SnapshotFixture::new(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-d"),
                vec![
                    certification_structural_snapshot_record(
                        "entity-1",
                        worth_foundational::facade::AspectValue::String(("bob").into()),
                    ),
                    certification_structural_snapshot_record(
                        "entity-2",
                        worth_foundational::facade::AspectValue::String(("bob").into()),
                    ),
                    certification_structural_snapshot_record(
                        "entity-3",
                        worth_foundational::facade::AspectValue::String(
                            ("shape-mismatch-snapshot-d").into(),
                        ),
                    ),
                ],
            ),
        ))
        .push(BridgeHarnessMutation::PublishCommittedPatch(
            committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("right"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-right-d"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-right-d"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-d"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ),
        ));
    adapter
        .apply_mutation_batch(&mut session, &diverge_mutation)
        .expect("divergence mutation should apply");

    let diverged_run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("diverged branch-head comparison should succeed");
    assert_ne!(initial_summary, converged_summary);
    assert_ne!(converged_summary, diverged_run.summary);
}

fn certification_structural_snapshot_record(
    entity_identity: &str,
    value: worth_foundational::facade::AspectValue,
) -> SnapshotReadRecord {
    SnapshotReadRecord::for_request(
        &SnapshotReadRequest::for_coarse(
            entity_identity,
            crate::snapshot::SnapshotReadContract::scalar(
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid certification structural aspect key"),
                worth_foundational::facade::ScalarAspectType::String,
            ),
        ),
        value,
    )
}

#[test]
fn structural_harness_certification_matrix_is_profile_invariant() {
    let fixture = structural_fixture("bridge-structural-certification");
    let request = ExecutionRequest::target("structural-remap-exact", exact_target());

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_profile("baseline"),
    )
    .candidates([forensic_profile("forensic")])
    .certify()
    .expect("structural certification matrix should certify");

    assert!(report.matched);
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn structural_harness_branch_parity_suite_preserves_branch_compare_bundle() {
    let fixture = structural_fixture("bridge-structural-branch-parity");
    let request = ExecutionRequest::target("structural-branch-compare", branch_compare_target());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_profile("baseline"),
    )
    .candidates([forensic_profile("candidate")])
    .compare()
    .expect("structural branch parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);

    let baseline_run = execute_structural_run(
        direct_profile("baseline"),
        "structural-branch-compare-baseline",
        branch_compare_target(),
    );
    let candidate_run = execute_structural_run(
        forensic_profile("candidate"),
        "structural-branch-compare-candidate",
        branch_compare_target(),
    );

    assert_eq!(baseline_run.summary, candidate_run.summary);
    assert_eq!(baseline_run.extensions, candidate_run.extensions);
}
