use forge_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest, HarnessAdapter,
    ObservationStatus, ScenarioPlan,
};

use crate::facade::{
    BridgeMergeConsumptionClass, BridgeMergeDenialClass, BridgeMergePrecedenceStage,
    BridgeMergeStructuralAdvisoryDisposition, MergeHistoryDeclarationIdentity,
};
use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;

use super::super::support::{merge_declaration, registration};
use super::support::{merge_fixture, runtime_with_merge};

#[test]
fn harness_fixture_registers_merge_declarations_into_runtime() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-merge-fixture-load",
        BridgeHarnessFixture::new(vec![registration()]).with_merge_declaration(merge_declaration(
            MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:fixture-load"),
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            [
                crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
                crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
            ],
        )),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let profile = ExecutionProfile::development("baseline");

    let mut session = adapter.create_runtime().expect("merge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("merge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("merge harness load fixture");

    let runtime = session
        .runtime
        .as_ref()
        .expect("runtime should be built during fixture load");
    assert_eq!(runtime.merge_registry().contracts().len(), 1);
}

#[test]
fn merge_harness_certification_matrix_reports_candidate_profile_parity() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:certification"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        [
            crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
            crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
        ],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let report = certification_matrix(
        BridgeHarnessAdapter,
        merge_fixture("bridge-merge-certification", declaration.clone()),
        ExecutionRequest::target(
            "merge-execute",
            BridgeHarnessTargetId::merge_execute(declaration.declaration_identity().clone()),
        ),
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("merge certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn merge_harness_parity_proves_truth_is_invariant_across_diagnostics_tiers() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:diagnostics-parity"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        [
            crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
            crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
        ],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let report = parity_suite(
        BridgeHarnessAdapter,
        merge_fixture("bridge-merge-parity", declaration.clone()),
        ExecutionRequest::target(
            "merge-execute",
            BridgeHarnessTargetId::merge_execute(declaration.declaration_identity().clone()),
        ),
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("merge parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
}

#[test]
fn merge_harness_replay_remains_parity_safe_across_candidate_profiles() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:replay-certification"),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        [
            crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
            crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
        ],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let report = parity_suite(
        BridgeHarnessAdapter,
        merge_fixture("bridge-merge-replay-parity", declaration.clone()),
        ExecutionRequest::target(
            "merge-replay",
            BridgeHarnessTargetId::merge_replay(declaration.declaration_identity().clone()),
        ),
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .compare()
    .expect("merge replay parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 1);
}

#[test]
fn merge_harness_topology_rewire_lane_executes_terminal_export_without_json_proof() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:topology-certification"),
        BridgeMergeConsumptionClass::TopologyRewireMerge,
        [
            crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
            crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
        ],
    );
    let adapter = BridgeHarnessAdapter;
    let fixture = merge_fixture("bridge-merge-topology-denial", declaration.clone());
    let request = ExecutionRequest::target(
        "merge-execute",
        BridgeHarnessTargetId::merge_execute(declaration.declaration_identity().clone()),
    );
    let profile = ExecutionProfile::development("baseline");

    let mut session = adapter.create_runtime().expect("merge harness runtime");
    adapter
        .prepare_runtime(&mut session, &profile)
        .expect("merge harness prepare");
    adapter
        .load_fixture(&mut session, &fixture)
        .expect("merge harness load fixture");
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .expect("topology denial execution should succeed");

    assert!(run
        .extensions
        .contains_key("bridge_merge_certification_bundle"));
    assert!(run.extensions.contains_key("bridge_merge_record"));
    assert_eq!(run.target_statuses[0].status, ObservationStatus::Validated);
}

#[test]
fn merge_harness_denial_localizes_stage_without_reopening_continuity() {
    let declaration = merge_declaration(
        MergeHistoryDeclarationIdentity::admit_bridge_owned("merge:causal-denial-certification"),
        BridgeMergeConsumptionClass::PolicyResolvedConflictMerge,
        [
            crate::truth_identity_fixtures::truth_commit_fixture("parent-a"),
            crate::truth_identity_fixtures::truth_commit_fixture("parent-b"),
        ],
    )
    .with_causal_frontier(crate::facade::BridgeMergeCausalFrontierDisposition::Truncated)
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge denial execution should succeed");

    assert_eq!(
        bundle.lowered_packet_set().blocked_stage(),
        Some(BridgeMergePrecedenceStage::CausalFrontierAdmissibility)
    );
    assert_eq!(
        bundle.lowered_packet_set().denial_class(),
        Some(BridgeMergeDenialClass::CausalFrontierTruncated)
    );
    assert!(bundle.continuity_artifact().is_none());
    assert!(bundle.remap_artifact().is_none());
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_explanation_request_count(),
        1
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_replay_request_count(),
        0
    );
}
