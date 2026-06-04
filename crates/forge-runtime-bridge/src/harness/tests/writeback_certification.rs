use crate::facade::TruthSnapshotIdentity;
use forge_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest, HarnessAdapter,
    RunRecord, ScenarioPlan,
};

use crate::facade::BridgeRuntimePolicy;
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;

use super::support::{committed_patch, registration, snapshot};

fn writeback_fixture(
    name: &str,
    policy: BridgeRuntimePolicy,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(policy)
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("writeback")
    .declare_observation("writeback")
    .compile()
}

fn direct_host_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(name)
}

fn wrapped_host_profile(name: &str) -> ExecutionProfile {
    ExecutionProfile::development(format!("{name}-wrapped-host"))
        .with_metadata("source_adapter_shape", "wrapped")
}

fn execute_writeback_run(
    fixture_name: &str,
    policy: BridgeRuntimePolicy,
    profile: ExecutionProfile,
    request_name: &str,
    target: BridgeHarnessTargetId,
) -> RunRecord<BridgeHarnessTargetId> {
    let adapter = BridgeHarnessAdapter;
    let fixture = writeback_fixture(fixture_name, policy);
    let mut runtime = adapter.create_runtime().expect("writeback harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("writeback harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("writeback harness load fixture");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target),
            &profile,
        )
        .expect("writeback harness execution")
}

struct WritebackCertificationTerminalLabels {
    certification_fixture_name: String,
    control_fixture_name: String,
    replay_fixture_name: String,
    control_request_name: String,
    replay_request_name: String,
}

impl WritebackCertificationTerminalLabels {
    fn for_target(target: &BridgeHarnessTargetId) -> Self {
        let terminal_target_label = target.to_string();
        Self {
            certification_fixture_name: format!("bridge-{terminal_target_label}"),
            control_fixture_name: format!("bridge-{terminal_target_label}-control"),
            replay_fixture_name: format!("bridge-{terminal_target_label}-replay"),
            control_request_name: format!("{terminal_target_label}-control"),
            replay_request_name: format!("{terminal_target_label}-replay"),
        }
    }
}

fn certify_writeback_target(target: BridgeHarnessTargetId) {
    let terminal_labels = WritebackCertificationTerminalLabels::for_target(&target);
    let fixture = writeback_fixture(
        &terminal_labels.certification_fixture_name,
        BridgeRuntimePolicy::development(),
    );
    let request =
        ExecutionRequest::target(terminal_labels.control_request_name.clone(), target.clone());

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .certify()
    .expect("writeback certification matrix should certify");

    assert!(report.matched);
    assert_eq!(report.cases.len(), 1);

    let control_run = execute_writeback_run(
        &terminal_labels.control_fixture_name,
        BridgeRuntimePolicy::development(),
        direct_host_profile("baseline-direct-host"),
        &terminal_labels.control_request_name,
        target.clone(),
    );
    let replay_run = execute_writeback_run(
        &terminal_labels.replay_fixture_name,
        BridgeRuntimePolicy::development(),
        wrapped_host_profile("candidate"),
        &terminal_labels.replay_request_name,
        target,
    );

    assert_eq!(control_run.summary, replay_run.summary);
    assert_eq!(control_run.extensions, replay_run.extensions);
    assert!(control_run
        .extensions
        .contains_key("bridge_writeback_certification_bundle"));
}

fn compare_writeback_parity(target: BridgeHarnessTargetId) {
    let terminal_labels = WritebackCertificationTerminalLabels::for_target(&target);
    let fixture = writeback_fixture(
        &terminal_labels.certification_fixture_name,
        BridgeRuntimePolicy::development(),
    );
    let request = ExecutionRequest::target(terminal_labels.control_request_name, target);

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        direct_host_profile("baseline-direct-host"),
    )
    .candidates([wrapped_host_profile("candidate")])
    .compare()
    .expect("writeback parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
}

#[test]
fn duplicate_writeback_attempt_bundle_is_replay_safe_and_bounded() {
    compare_writeback_parity(BridgeHarnessTargetId::writeback_duplicate_certification());
    certify_writeback_target(BridgeHarnessTargetId::writeback_duplicate_certification());
}

#[test]
fn writeback_authority_denial_is_typed_and_leaves_zero_authority_residue() {
    certify_writeback_target(BridgeHarnessTargetId::writeback_authority_denial_certification());
}

#[test]
fn bridge_origin_feedback_lane_converges_without_second_authoritative_commit() {
    compare_writeback_parity(BridgeHarnessTargetId::writeback_feedback_loop_certification());
    certify_writeback_target(BridgeHarnessTargetId::writeback_feedback_loop_certification());
}

#[test]
fn writeback_replay_mismatch_is_typed_and_counted() {
    certify_writeback_target(BridgeHarnessTargetId::writeback_replay_mismatch_certification());
}

#[test]
fn extensible_writeback_families_remain_parity_safe_and_family_isolated() {
    certify_writeback_target(BridgeHarnessTargetId::writeback_extensible_family_certification());
}

#[test]
fn multi_family_writeback_admission_boundary_stays_bridge_native() {
    certify_writeback_target(
        BridgeHarnessTargetId::writeback_multi_family_admission_boundary_certification(),
    );
}

#[test]
fn cross_family_replay_and_loop_isolation_remains_family_correct() {
    certify_writeback_target(
        BridgeHarnessTargetId::writeback_cross_family_replay_loop_isolation_certification(),
    );
}

#[test]
fn host_mapper_parity_rejects_shadow_protocol_behavior() {
    certify_writeback_target(BridgeHarnessTargetId::writeback_host_mapper_parity_certification());
}
