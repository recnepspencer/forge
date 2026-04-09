use forge_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest, MutationBatch,
};
use forge_harness::runtime::HarnessAdapter;
use serde_json::json;

use crate::facade::{SnapshotReadRecord, TruthSnapshotIdentity};
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
        exact_control.summary["structural_match_digest"],
        exact_candidate.summary["structural_match_digest"]
    );
    assert_eq!(
        exact_control.extensions["bridge_structural_certification_bundle"]["remap_artifact_digest"],
        exact_candidate.extensions["bridge_structural_certification_bundle"]
            ["remap_artifact_digest"]
    );
    assert_eq!(ambiguous_run.summary["failure_digest"].is_null(), false);
    assert_eq!(
        ambiguous_run.extensions["bridge_structural_certification_bundle"]["ambiguity_report"]
            ["outcome_class"],
        json!("RejectedAmbiguousStructuralMatch")
    );
    assert_eq!(
        ambiguous_run.extensions["bridge_structural_certification_bundle"]["remap_artifact_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        exact_control.summary["counter_snapshot"]["structural_widened_scan_count"],
        json!(0)
    );
    assert_eq!(
        exact_control.summary["counter_snapshot"]["structural_replay_mismatch_count"],
        json!(0)
    );
    assert_eq!(
        no_safe_match_run.summary["outcome_class"],
        json!("RejectedNoStructuralMatch")
    );
    assert_eq!(
        no_safe_match_run.extensions["bridge_structural_certification_bundle"]["failure_digest"],
        no_safe_match_run.summary["failure_digest"]
    );
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

    assert_eq!(
        replay_run.extensions["bridge_structural_certification_bundle"]["structural_reuse_digest"],
        replay_run.summary["structural_reuse_digest"]
    );
    assert_eq!(
        replay_run.extensions["bridge_structural_certification_bundle"]["replay_digest"],
        replay_run.summary["replay_digest"]
    );
    assert_eq!(
        replay_run.summary["counter_snapshot"]["structural_replay_request_count"],
        json!(1)
    );
    assert_eq!(
        replay_run.summary["counter_snapshot"]["structural_replay_mismatch_count"],
        json!(0)
    );
    assert_eq!(
        replay_run.summary["structural_reuse_digest"],
        exact_control.summary["structural_reuse_digest"]
    );
    assert_eq!(
        identity_conflict_run.extensions["bridge_structural_certification_bundle"]
            ["identity_separation_report"]["outcome_class"],
        json!("RejectedIdentityAuthorityConflict")
    );
    assert_eq!(
        identity_conflict_run.summary["failure_digest"].is_null(),
        false
    );
    assert_eq!(
        identity_conflict_run.extensions["bridge_structural_certification_bundle"]
            ["structural_reuse_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        lineage_divergence_run.extensions["bridge_structural_certification_bundle"]
            ["identity_separation_report"]["outcome_class"],
        json!("RejectedLineageStructuralDivergence")
    );
    assert_eq!(
        lineage_divergence_run.summary["counter_snapshot"]["structural_lineage_divergence_count"],
        json!(1)
    );
    assert_eq!(
        lineage_divergence_run.summary["counter_snapshot"]["structural_widened_scan_count"],
        json!(0)
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
                TruthSnapshotIdentity::new("snapshot-unrelated"),
                vec![
                    SnapshotReadRecord::new("entity-1:profile", b"alice".to_vec()),
                    SnapshotReadRecord::new("entity-2:profile", b"alice".to_vec()),
                    SnapshotReadRecord::new(
                        "entity-3:profile",
                        b"shape-mismatch-unrelated".to_vec(),
                    ),
                ],
            ),
        ))
        .push(BridgeHarnessMutation::PublishCommittedPatch(
            committed_patch_on_branch(
                "unrelated",
                "commit-unrelated",
                "patch-unrelated",
                "snapshot-unrelated",
                "name",
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
        branch_run.extensions["bridge_structural_certification_bundle"]["branch_compare_digest"],
        branch_run_after_unrelated.extensions["bridge_structural_certification_bundle"]
            ["branch_compare_digest"]
    );
    assert_eq!(
        branch_run.extensions["bridge_structural_certification_bundle"]["structural_diff_report"]
            ["branch_diff_count"],
        json!(1)
    );
    assert_eq!(
        replay_run.extensions["bridge_structural_certification_bundle"]["replay_digest"],
        replay_run.summary["replay_digest"]
    );
    assert_eq!(
        replay_run.summary["counter_snapshot"]["branch_comparison_diff_count"],
        json!(1)
    );
    assert_eq!(
        replay_run.summary["counter_snapshot"]["branch_comparison_drift_rejection_count"],
        json!(0)
    );
    assert_eq!(
        replay_run.summary["counter_snapshot"]["structural_replay_request_count"],
        json!(1)
    );
    assert_eq!(
        replay_run.summary["branch_compare_digest"],
        branch_run.summary["branch_compare_digest"]
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
    assert_eq!(
        initial_run.extensions["bridge_structural_certification_bundle"]["structural_diff_report"]
            ["branch_diff_count"],
        json!(1)
    );

    let converge_mutation = MutationBatch::new("publish-right-branch-convergence")
        .push(BridgeHarnessMutation::PublishSnapshot(
            SnapshotFixture::new(
                TruthSnapshotIdentity::new("snapshot-c"),
                vec![
                    SnapshotReadRecord::new("entity-1:profile", b"alice".to_vec()),
                    SnapshotReadRecord::new("entity-2:profile", b"alice".to_vec()),
                    SnapshotReadRecord::new(
                        "entity-3:profile",
                        b"shape-mismatch-snapshot-a".to_vec(),
                    ),
                ],
            ),
        ))
        .push(BridgeHarnessMutation::PublishCommittedPatch(
            committed_patch_on_branch(
                "right",
                "commit-right-c",
                "patch-right-c",
                "snapshot-c",
                "name",
            ),
        ));
    adapter
        .apply_mutation_batch(&mut session, &converge_mutation)
        .expect("convergence mutation should apply");

    let converged_run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("converged branch-head comparison should succeed");
    assert_eq!(
        converged_run.extensions["bridge_structural_certification_bundle"]
            ["structural_diff_report"]["branch_diff_count"],
        json!(0)
    );

    let diverge_mutation = MutationBatch::new("publish-right-branch-divergence")
        .push(BridgeHarnessMutation::PublishSnapshot(
            SnapshotFixture::new(
                TruthSnapshotIdentity::new("snapshot-d"),
                vec![
                    SnapshotReadRecord::new("entity-1:profile", b"bob".to_vec()),
                    SnapshotReadRecord::new("entity-2:profile", b"bob".to_vec()),
                    SnapshotReadRecord::new(
                        "entity-3:profile",
                        b"shape-mismatch-snapshot-d".to_vec(),
                    ),
                ],
            ),
        ))
        .push(BridgeHarnessMutation::PublishCommittedPatch(
            committed_patch_on_branch(
                "right",
                "commit-right-d",
                "patch-right-d",
                "snapshot-d",
                "name",
            ),
        ));
    adapter
        .apply_mutation_batch(&mut session, &diverge_mutation)
        .expect("divergence mutation should apply");

    let diverged_run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("diverged branch-head comparison should succeed");
    assert_eq!(
        diverged_run.extensions["bridge_structural_certification_bundle"]["structural_diff_report"]
            ["branch_diff_count"],
        json!(1)
    );
    assert_ne!(
        initial_run.summary["branch_compare_digest"],
        converged_run.summary["branch_compare_digest"]
    );
    assert_ne!(
        converged_run.summary["branch_compare_digest"],
        diverged_run.summary["branch_compare_digest"]
    );
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
