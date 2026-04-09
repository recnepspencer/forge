use forge_harness::facade::{
    certification_matrix, parity_suite, ExecutionProfile, ExecutionRequest, ScenarioPlan,
};
use forge_harness::runtime::HarnessAdapter;
use serde_json::json;

use super::support::{merge_declaration, registration};
use crate::facade::{
    BridgeMergeConsumptionClass, BridgeMergeOntologyMappingEntry,
    BridgeMergeOntologyMappingSurface, BridgeMergeStructuralAdvisoryDisposition,
    CanonicalRelationalMergeClass,
};
use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

fn runtime_with_merge(
    declaration: crate::facade::MergeHistoryDeclaration,
) -> crate::facade::RuntimeBridge {
    let source = InMemoryRelationalBridgeSource::default();
    let builder = crate::facade::RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source.clone())
        .with_continuity_lineage_source(source)
        .with_signal_sink(RecordingSignalBridgeSink::default())
        .register_mapping(registration())
        .register_merge(declaration);

    builder
        .build()
        .expect("bridge runtime should build with merge declaration")
}

fn merge_fixture(
    scenario: &str,
    declaration: crate::facade::MergeHistoryDeclaration,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        scenario,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_merge_declaration(declaration),
    )
    .declare_input("merge")
    .declare_observation("merge")
    .compile()
}

fn many_to_one_mapping_declaration(id: &str) -> crate::facade::MergeHistoryDeclaration {
    crate::facade::MergeHistoryDeclaration::new(
        crate::facade::MergeHistoryDeclarationIdentity::new(id),
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        BridgeMergeOntologyMappingSurface::new(
            "rel-merge-v1",
            vec![
                BridgeMergeOntologyMappingEntry::direct_wrapper(
                    CanonicalRelationalMergeClass::AspectReconciliation,
                    BridgeMergeConsumptionClass::AspectReconciliationMerge,
                ),
                BridgeMergeOntologyMappingEntry::direct_wrapper(
                    CanonicalRelationalMergeClass::PolicyResolvedConflict,
                    BridgeMergeConsumptionClass::AspectReconciliationMerge,
                ),
            ],
        ),
        crate::facade::BridgeMergeAuthorityBasis::new(
            crate::facade::BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            format!("merge-artifact:{id}"),
            "rel-merge-v1",
            "schema-policy-v1",
            crate::facade::BridgeMergeParentOrderProof::new(vec![
                crate::facade::TruthCommitIdentity::new("parent-a"),
                crate::facade::TruthCommitIdentity::new("parent-b"),
            ]),
        ),
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent)
}

#[test]
fn ordered_parent_history_remains_deterministic_under_adapter_variation() {
    let declaration = merge_declaration(
        "merge:ordered-parent-determinism",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        vec!["parent-a", "parent-b"],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let left_runtime = runtime_with_merge(declaration.clone());
    let right_runtime = runtime_with_merge(declaration.clone());

    let left_contract = left_runtime
        .admit_merge_history(declaration.clone())
        .expect("left contract should admit");
    let right_contract = right_runtime
        .admit_merge_history(declaration)
        .expect("right contract should admit");
    let left_bundle = left_runtime
        .replay_merge_history(&left_contract)
        .expect("left bundle should replay");
    let right_bundle = right_runtime
        .replay_merge_history(&right_contract)
        .expect("right bundle should replay");

    let report = json!({
        "merge_history_digest": left_contract.digest(),
        "parent_order_report": {
            "left": left_bundle.lowered_packet_set().parent_order_digest_basis().digest(),
            "right": right_bundle.lowered_packet_set().parent_order_digest_basis().digest(),
        },
        "routing_digest": left_bundle.reduced_routing_artifact().digest(),
        "result_bundle_digest": left_bundle.digest(),
        "replay_digest": right_bundle.digest(),
    });

    assert_eq!(
        report["parent_order_report"]["left"],
        report["parent_order_report"]["right"]
    );
    assert_eq!(report["result_bundle_digest"], report["replay_digest"]);
    assert_eq!(
        left_bundle
            .contract()
            .validated_declaration()
            .declaration()
            .ontology_mapping()
            .digest(),
        right_bundle
            .contract()
            .validated_declaration()
            .declaration()
            .ontology_mapping()
            .digest()
    );
}

#[test]
fn merge_ontology_lowering_remains_lossless_under_many_to_one_bridge_class_mapping() {
    let declaration = many_to_one_mapping_declaration("merge:many-to-one-ontology");
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("many-to-one ontology mapping should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("many-to-one ontology mapping should replay");

    let report = json!({
        "merge_history_digest": contract.digest(),
        "merge_ontology_mapping_report": {
            "mapping_digest": contract.validated_declaration().declaration().ontology_mapping().digest(),
            "entry_count": contract.validated_declaration().declaration().ontology_mapping().entries().len(),
            "bridge_class": format!("{:?}", contract.validated_declaration().declaration().bridge_class()),
        },
        "result_bundle_digest": bundle.digest(),
    });

    assert_eq!(
        report["merge_ontology_mapping_report"]["entry_count"],
        json!(2)
    );
    assert_eq!(
        report["merge_ontology_mapping_report"]["bridge_class"],
        json!("AspectReconciliationMerge")
    );
}

#[test]
fn unsupported_merge_classes_fail_without_branch_reconciliation_fallback() {
    let declaration = merge_declaration(
        "merge:topology-denial",
        BridgeMergeConsumptionClass::TopologyRewireMerge,
        vec!["parent-a", "parent-b"],
    );
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("merge bundle should reconstruct");

    let denial_report = json!({
        "merge_support_matrix": {
            "bridge_class": format!("{:?}", contract.validated_declaration().declaration().bridge_class()),
            "outcome_class": format!("{:?}", bundle.reduced_routing_artifact().outcome_class()),
        },
        "merge_denial_stage_report": {
            "blocked_stage": format!("{:?}", bundle.lowered_packet_set().blocked_stage()),
            "denial_class": format!("{:?}", bundle.lowered_packet_set().denial_class()),
        },
        "failure_digest": bundle.explanation_artifact().digest(),
        "diagnostics_digest": bundle.explanation_artifact().digest(),
        "counter_snapshot": {
            "merge_continuity_count": bundle.reduced_routing_artifact().counters().merge_continuity_count(),
            "merge_continuity_denial_count": bundle.reduced_routing_artifact().counters().merge_continuity_denial_count(),
        }
    });

    assert_eq!(
        denial_report["merge_support_matrix"]["outcome_class"],
        json!("Denied")
    );
    assert_eq!(
        denial_report["merge_denial_stage_report"]["blocked_stage"],
        json!("Some(DeletionTopologyGate)")
    );
    assert!(bundle.continuity_artifact().is_none());
    assert!(bundle.remap_artifact().is_none());
    assert_eq!(
        denial_report["counter_snapshot"]["merge_continuity_count"],
        json!(0)
    );
}

#[test]
fn topology_rewire_denial_is_typed_and_keeps_counter_scope_local() {
    let declaration = merge_declaration(
        "merge:topology-rewire-denial",
        BridgeMergeConsumptionClass::TopologyRewireMerge,
        vec!["parent-a", "parent-b"],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("topology rewire declaration should admit");
    let bundle = runtime
        .replay_merge_history(&contract)
        .expect("topology rewire bundle should reconstruct");

    assert_eq!(
        bundle.lowered_packet_set().denial_class(),
        Some(crate::facade::BridgeMergeDenialClass::TopologyRewireGate)
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_topology_rewire_class_count(),
        1
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_history_segment_scan_count(),
        1
    );
    assert_eq!(
        bundle
            .reduced_routing_artifact()
            .counters()
            .merge_widened_scan_count(),
        0
    );
}

#[test]
fn merge_replay_preserves_routing_and_explanation_parity() {
    let declaration = merge_declaration(
        "merge:replay-parity",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        vec!["parent-a", "parent-b"],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let runtime = runtime_with_merge(declaration.clone());
    let contract = runtime
        .admit_merge_history(declaration)
        .expect("merge declaration should admit");
    let original_bundle = runtime
        .replay_merge_history(&contract)
        .expect("original bundle should reconstruct");
    let canonical_record = runtime.canonicalize_merge_record(&original_bundle);
    let replayed_bundle = runtime
        .replay_canonical_merge_record(&canonical_record)
        .expect("canonical replay should reconstruct");

    let parity_bundle = json!({
        "merge_history_digest": contract.digest(),
        "result_bundle_digest": original_bundle.digest(),
        "continuity_digest": original_bundle.continuity_artifact().map(|artifact| artifact.digest()),
        "explanation_digest": original_bundle.explanation_artifact().digest(),
        "replay_digest": replayed_bundle.digest(),
        "counter_snapshot": {
            "merge_history_segment_scan_count": replayed_bundle.reduced_routing_artifact().counters().merge_history_segment_scan_count(),
            "merge_causal_frontier_lookup_count": replayed_bundle.reduced_routing_artifact().counters().merge_causal_frontier_lookup_count(),
            "merge_structural_consult_width": replayed_bundle.reduced_routing_artifact().counters().merge_structural_consult_width(),
        }
    });

    assert_eq!(
        parity_bundle["result_bundle_digest"],
        parity_bundle["replay_digest"]
    );
    assert_eq!(
        original_bundle.explanation_artifact().digest(),
        replayed_bundle.explanation_artifact().digest()
    );
    assert_eq!(
        parity_bundle["counter_snapshot"]["merge_history_segment_scan_count"],
        json!(1)
    );
    assert_eq!(
        replayed_bundle
            .reduced_routing_artifact()
            .counters()
            .merge_replay_request_count(),
        1
    );
}

#[test]
fn harness_fixture_registers_merge_declarations_into_runtime() {
    let adapter = BridgeHarnessAdapter;
    let fixture = ScenarioPlan::new(
        "bridge-merge-fixture-load",
        BridgeHarnessFixture::new(vec![registration()]).with_merge_declaration(merge_declaration(
            "merge:fixture-load",
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            vec!["parent-a", "parent-b"],
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
        "merge:certification",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        vec!["parent-a", "parent-b"],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let report = certification_matrix(
        BridgeHarnessAdapter,
        merge_fixture("bridge-merge-certification", declaration.clone()),
        ExecutionRequest::target(
            "merge-execute",
            format!(
                "merge-execute:{}",
                declaration.declaration_identity().as_str()
            ),
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
        "merge:diagnostics-parity",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        vec!["parent-a", "parent-b"],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let report = parity_suite(
        BridgeHarnessAdapter,
        merge_fixture("bridge-merge-parity", declaration.clone()),
        ExecutionRequest::target(
            "merge-execute",
            format!(
                "merge-execute:{}",
                declaration.declaration_identity().as_str()
            ),
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
        "merge:replay-certification",
        BridgeMergeConsumptionClass::AspectReconciliationMerge,
        vec!["parent-a", "parent-b"],
    )
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let report = parity_suite(
        BridgeHarnessAdapter,
        merge_fixture("bridge-merge-replay-parity", declaration.clone()),
        ExecutionRequest::target(
            "merge-replay",
            format!(
                "merge-replay:{}",
                declaration.declaration_identity().as_str()
            ),
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
fn merge_harness_topology_rewire_lane_emits_canonical_denial_bundle() {
    let declaration = merge_declaration(
        "merge:topology-certification",
        BridgeMergeConsumptionClass::TopologyRewireMerge,
        vec!["parent-a", "parent-b"],
    );
    let adapter = BridgeHarnessAdapter;
    let fixture = merge_fixture("bridge-merge-topology-denial", declaration.clone());
    let request = ExecutionRequest::target(
        "merge-execute",
        format!(
            "merge-execute:{}",
            declaration.declaration_identity().as_str()
        ),
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

    assert_eq!(run.summary["outcome_class"], json!("Denied"));
    assert_eq!(run.summary["blocked_stage"], json!("DeletionTopologyGate"));
    assert_eq!(run.summary["denial_class"], json!("TopologyRewireGate"));
    assert_eq!(
        run.extensions["bridge_merge_certification_bundle"]["counter_snapshot"]
            ["merge_topology_rewire_class_count"],
        json!(1)
    );
    assert_eq!(
        run.extensions["bridge_merge_certification_bundle"]["counter_snapshot"]
            ["merge_widened_scan_count"],
        json!(0)
    );
}

#[test]
fn merge_harness_denial_localizes_stage_without_reopening_continuity() {
    let declaration = merge_declaration(
        "merge:causal-denial-certification",
        BridgeMergeConsumptionClass::PolicyResolvedConflictMerge,
        vec!["parent-a", "parent-b"],
    )
    .with_causal_frontier(crate::facade::BridgeMergeCausalFrontierDisposition::Truncated)
    .with_structural_advisory(BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent);
    let adapter = BridgeHarnessAdapter;
    let fixture = merge_fixture("bridge-merge-causal-denial", declaration.clone());
    let request = ExecutionRequest::target(
        "merge-execute",
        format!(
            "merge-execute:{}",
            declaration.declaration_identity().as_str()
        ),
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
        .expect("merge denial execution should succeed");

    assert_eq!(run.summary["outcome_class"], json!("Denied"));
    assert_eq!(
        run.summary["blocked_stage"],
        json!("CausalFrontierAdmissibility")
    );
    assert_eq!(
        run.summary["denial_class"],
        json!("CausalFrontierTruncated")
    );
    assert_eq!(
        run.extensions["bridge_merge_certification_bundle"]["merge_support_matrix"]
            ["continuity_published"],
        json!(false)
    );
    assert_eq!(
        run.extensions["bridge_merge_certification_bundle"]["merge_support_matrix"]
            ["remap_published"],
        json!(false)
    );
    assert_eq!(
        run.extensions["bridge_merge_certification_bundle"]["counter_snapshot"]
            ["merge_explanation_request_count"],
        json!(1)
    );
    assert_eq!(
        run.extensions["bridge_merge_certification_bundle"]["counter_snapshot"]
            ["merge_replay_request_count"],
        json!(0)
    );
}
