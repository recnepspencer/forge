use super::super::history_assertions::last_historical_record;
use super::super::support::{committed_patch_on_branch, registration, snapshot};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;
use worth_harness::facade::{ExecutionProfile, ExecutionRequest, ScenarioPlan};
use worth_harness::runtime::HarnessAdapter;

#[test]
fn bridge_harness_branch_divergence_changes_selected_truth_view_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("development");
    let request = ExecutionRequest::target(
        "branch-head-feature",
        BridgeHarnessTargetId::branch_head(crate::truth_identity_fixtures::truth_branch_fixture(
            "feature",
        )),
    );

    let main_fixture = ScenarioPlan::new(
        "bridge-historical-main-head",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("main"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                "alice",
            )),
    )
    .declare_input("branch-head:main")
    .declare_observation("historical")
    .compile();
    let feature_fixture = ScenarioPlan::new(
        "bridge-historical-feature-head",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch_on_branch(
                crate::truth_identity_fixtures::truth_branch_fixture("feature"),
                crate::truth_identity_fixtures::truth_commit_fixture("commit-f"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-f"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-f"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-f"),
                "frank",
            )),
    )
    .declare_input("branch-head:feature")
    .declare_observation("historical")
    .compile();

    let mut main_session = adapter
        .create_runtime()
        .expect("main bridge harness runtime");
    adapter
        .prepare_runtime(&mut main_session, &profile)
        .expect("main bridge harness prepare");
    adapter
        .load_fixture(&mut main_session, &main_fixture)
        .expect("main bridge harness load fixture");
    let _main_run = adapter
        .execute(
            &mut main_session,
            &main_fixture,
            &ExecutionRequest::target(
                "branch-head-main",
                BridgeHarnessTargetId::branch_head(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                ),
            ),
            &profile,
        )
        .expect("main branch-head execution should succeed");

    let mut feature_session = adapter
        .create_runtime()
        .expect("feature bridge harness runtime");
    adapter
        .prepare_runtime(&mut feature_session, &profile)
        .expect("feature bridge harness prepare");
    adapter
        .load_fixture(&mut feature_session, &feature_fixture)
        .expect("feature bridge harness load fixture");
    let _feature_run = adapter
        .execute(&mut feature_session, &feature_fixture, &request, &profile)
        .expect("feature branch-head execution should succeed");

    let main_record = last_historical_record(&main_session);
    let feature_record = last_historical_record(&feature_session);
    assert_ne!(
        main_record.record_identity(),
        feature_record.record_identity()
    );
    assert_ne!(
        main_record.decision_log().snapshot_identity(),
        feature_record.decision_log().snapshot_identity()
    );
    assert_ne!(
        main_record.decision_log().branch_identity(),
        feature_record.decision_log().branch_identity()
    );
}
