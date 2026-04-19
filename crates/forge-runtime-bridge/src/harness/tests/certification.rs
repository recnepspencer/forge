use forge_harness::facade::{
    certification_matrix, ExecutionProfile, ExecutionRequest, HarnessAdapter, RunRecord,
    ScenarioPlan,
};
use serde_json::json;
use std::sync::Arc;

use super::support::{
    build_runtime, committed_patch, committed_patch_on_branch, field_aspect_registration,
    field_slice_snapshot, merge_declaration, registration, snapshot,
};
use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeContinuityAuthorityBasis,
    BridgeHistoricalLineageAuthority, BridgeLineageContext, BridgeParallelAdmissionClass,
    BridgeParallelAdmissionReason, BridgePreviewSessionDeclaration,
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity, BridgeRequestKind,
    BridgeRouteRequest, BridgeSignalBranchIdentity, BridgeSourceCapability,
    BridgeSourceCapabilitySet, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, BridgeTruthViewSelector, SnapshotReadRecord,
    StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
    StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
    StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
    StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, StructuralTruthViewBasis,
    TruthBranchIdentity, TruthCommitIdentity, TruthSnapshotIdentity,
};
use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::source::{SourceDeclaration, SourceDeclarationIdentity};

fn execute_harness_run(
    fixture: forge_harness::facade::ScenarioFixture<BridgeHarnessFixture>,
    profile: ExecutionProfile,
    request_name: &str,
    target: &str,
) -> RunRecord<String> {
    let adapter = BridgeHarnessAdapter;
    let mut runtime = adapter
        .create_runtime()
        .expect("harness runtime should construct");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("harness prepare should succeed");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("harness fixture should load");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target.to_owned()),
            &profile,
        )
        .expect("harness execution should succeed")
}

fn mixed_stream_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("stream")
    .declare_observation("stream")
    .compile()
}

fn mixed_source_declaration(id: &str) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new(id),
        BridgeTruthViewSelector::historical_commit(
            TruthBranchIdentity::new("analysis"),
            TruthCommitIdentity::new("commit-a"),
        ),
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::HistoricalRead,
            BridgeSourceCapability::BranchRead,
            BridgeSourceCapability::ReplayCompatibleRead,
        ]),
    )
}

fn mixed_source_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_source_declaration(mixed_source_declaration("source:analysis-history"))
            .with_source_adapter_capabilities(BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ]))
            .with_committed_patch(committed_patch_on_branch(
                "analysis",
                "commit-a",
                "patch-a",
                "snapshot-a",
                "name",
            ))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("source")
    .declare_observation("source")
    .compile()
}

fn mixed_structural_snapshot(
    snapshot_identity: &str,
    value: &str,
) -> crate::harness::fixtures::SnapshotFixture {
    crate::harness::fixtures::SnapshotFixture::new(
        TruthSnapshotIdentity::new(snapshot_identity),
        vec![
            SnapshotReadRecord::new("entity-1:profile", value.as_bytes().to_vec()),
            SnapshotReadRecord::new("entity-2:profile", value.as_bytes().to_vec()),
            SnapshotReadRecord::new(
                "entity-3:profile",
                format!("shape-mismatch-{snapshot_identity}").into_bytes(),
            ),
        ],
    )
}

fn mixed_structural_remap_declaration(id: &str) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::advisory_remap(
        StructuralIdentityDeclarationIdentity::new(id),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::TopologyFingerprint,
            "geometry-topology-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    )
}

fn mixed_structural_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_structural_declaration(mixed_structural_remap_declaration(
                "structural:analysis-remap",
            ))
            .with_committed_patch(committed_patch_on_branch(
                "analysis",
                "commit-a",
                "patch-a",
                "snapshot-a",
                "name",
            ))
            .with_committed_patch(committed_patch_on_branch(
                "analysis",
                "commit-b",
                "patch-b",
                "snapshot-b",
                "name",
            ))
            .with_snapshot(mixed_structural_snapshot("snapshot-a", "alice"))
            .with_snapshot(mixed_structural_snapshot("snapshot-b", "bob")),
    )
    .declare_input("structural")
    .declare_observation("structural")
    .compile()
}

fn mixed_merge_fixture(name: &str) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_merge_declaration(merge_declaration(
                "merge:m13-mixed",
                crate::facade::BridgeMergeConsumptionClass::AspectReconciliationMerge,
                vec!["parent-a", "parent-b"],
            ))
            .with_merge_declaration(merge_declaration(
                "merge:m13-topology-denial",
                crate::facade::BridgeMergeConsumptionClass::TopologyRewireMerge,
                vec!["parent-a", "parent-b"],
            )),
    )
    .declare_input("merge")
    .declare_observation("merge")
    .compile()
}

fn mixed_policy_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("policy")
    .declare_observation("policy")
    .compile()
}

fn mixed_speculation_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("speculation")
    .declare_observation("speculation")
    .compile()
}

fn mixed_writeback_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("writeback")
    .declare_observation("writeback")
    .compile()
}

fn continuity_authority(branch: &str, snapshot: &str) -> BridgeHistoricalLineageAuthority {
    continuity_authority_with_successor(branch, snapshot, "entity:0:4:2")
}

fn continuity_authority_with_successor(
    branch: &str,
    snapshot: &str,
    successor: &str,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new(branch),
            TruthSnapshotIdentity::new(snapshot),
        ),
        vec![Arc::from("lineage:test-successor")],
        vec![Arc::from(successor)],
        vec![7],
    )
    .expect("continuity authority should be canonical")
}

fn ambiguous_continuity_authority(
    branch: &str,
    snapshot: &str,
) -> BridgeHistoricalLineageAuthority {
    BridgeHistoricalLineageAuthority::try_new(
        BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new(branch),
            TruthSnapshotIdentity::new(snapshot),
        ),
        vec![
            Arc::from("lineage:test-a"),
            Arc::from("lineage:test-b"),
            Arc::from("lineage:test-c"),
        ],
        vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
        vec![7, 8, 9],
    )
    .expect("ambiguous continuity authority should be canonical")
}

#[test]
fn bridge_certification_matrix_reports_diagnostics_for_candidate_profiles() {
    let fixture = ScenarioPlan::new(
        "bridge-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_continuity_certification_matrix_reports_candidate_profile_parity() {
    let fixture = ScenarioPlan::new(
        "bridge-continuity-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority("user", continuity_authority("main", "snapshot-a"))
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge continuity certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_harness_branch_divergence_changes_continuity_outcome_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let main_fixture = ScenarioPlan::new(
        "bridge-continuity-main",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority(
                "user",
                continuity_authority_with_successor("main", "snapshot-a", "entity:0:4:2"),
            )
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let feature_fixture = ScenarioPlan::new(
        "bridge-continuity-feature",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("feature"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority(
                "user",
                continuity_authority_with_successor("feature", "snapshot-a", "entity:0:5:2"),
            )
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let mut main_runtime = adapter.create_runtime().expect("main harness runtime");
    adapter
        .prepare_runtime(&mut main_runtime, &profile)
        .expect("main harness prepare");
    adapter
        .load_fixture(&mut main_runtime, &main_fixture)
        .expect("main harness load fixture");
    let main_run = adapter
        .execute(&mut main_runtime, &main_fixture, &request, &profile)
        .expect("main harness execute");

    let mut feature_runtime = adapter.create_runtime().expect("feature harness runtime");
    adapter
        .prepare_runtime(&mut feature_runtime, &profile)
        .expect("feature harness prepare");
    adapter
        .load_fixture(&mut feature_runtime, &feature_fixture)
        .expect("feature harness load fixture");
    let feature_run = adapter
        .execute(&mut feature_runtime, &feature_fixture, &request, &profile)
        .expect("feature harness execute");

    assert_ne!(
        main_run.summary["continuity_identity"],
        feature_run.summary["continuity_identity"]
    );
    assert_ne!(
        main_run.extensions["bridge_continuity_record"]["source_branch"],
        feature_run.extensions["bridge_continuity_record"]["source_branch"]
    );
}

#[test]
fn bridge_harness_continuity_certifies_ambiguous_rejection_explicitly() {
    let adapter = BridgeHarnessAdapter;
    let profile = ExecutionProfile::development("baseline");
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());
    let fixture = ScenarioPlan::new(
        "bridge-continuity-ambiguous",
        BridgeHarnessFixture::new(vec![registration()])
            .with_aspect_mapping(field_aspect_registration())
            .with_lineage_context(BridgeLineageContext::new(
                BridgeContinuityAuthorityBasis::new(
                    TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot-a"),
                ),
            ))
            .with_continuity_authority("user", ambiguous_continuity_authority("main", "snapshot-a"))
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();

    let mut runtime = adapter.create_runtime().expect("harness runtime");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("harness prepare");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("harness load fixture");
    let run = adapter
        .execute(&mut runtime, &fixture, &request, &profile)
        .expect("harness execute");

    assert_eq!(
        run.extensions["bridge_continuity_record"]["outcome_classes"][0],
        "RejectedAmbiguousSuccessor"
    );
}

#[test]
fn bridge_historical_certification_matrix_reports_candidate_profile_parity() {
    let fixture = ScenarioPlan::new(
        "bridge-historical-certification",
        BridgeHarnessFixture::new(vec![registration()])
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("history-commit:main:commit-a")
    .declare_observation("historical")
    .compile();
    let request = ExecutionRequest::target(
        "historical-commit-a",
        "history-commit:main:commit-a".to_string(),
    );

    let report = certification_matrix(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([ExecutionProfile::forensic("forensic")])
    .certify()
    .expect("bridge historical certification matrix should succeed");

    assert!(report.matched);
    assert!(report.baseline_diagnostics_summary.is_some());
    assert_eq!(report.cases.len(), 1);
}

#[test]
fn bridge_bulk_certifies_exact_counters_for_parallel_admitted_workload() {
    let source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-b", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    source.insert_snapshot(snapshot("snapshot-b", "bob"));
    let runtime = build_runtime(
        source,
        crate::harness::fixtures::RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let plan = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("parallel-admitted workload should plan");

    assert_eq!(
        plan.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::ParallelPreparationAdmitted
    );
    assert_eq!(
        plan.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::AdmittedOperational
    );
    assert_eq!(plan.packet_set().counters().bulk_packet_count(), 6);
    assert_eq!(plan.execution_plan().counters().bulk_routed_item_count(), 2);
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_normalized_workload_width(),
        13
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_packet_entry_count(),
        6
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_reduction_input_count(),
        4
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_reduction_output_count(),
        4
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_packet_queue_depth_peak(),
        6
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_reducer_input_buffer_peak(),
        4
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_replay_mismatch_count(),
        0
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_parallel_legal_count(),
        1
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_parallel_profitable_count(),
        1
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_parallel_preparation_admitted_count(),
        1
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_parallel_fallback_to_serial_count(),
        0
    );
}

#[test]
fn bridge_bulk_certifies_exact_counters_for_serial_fallback_workload() {
    let source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    source.insert_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"));
    source.insert_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"));
    source.insert_snapshot(snapshot("snapshot-a", "alice"));
    let runtime = build_runtime(
        source,
        crate::harness::fixtures::RecordingSignalBridgeSink::default(),
        vec![registration()],
    );

    let plan = runtime
        .plan_bulk_workload(BridgeBulkWorkloadRequest::new(vec![
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
            BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-b")),
        ]))
        .expect("serial-fallback workload should plan");

    assert_eq!(
        plan.execution_plan().parallel_admission().class(),
        BridgeParallelAdmissionClass::SerialRequired
    );
    assert_eq!(
        plan.execution_plan().parallel_admission().reason(),
        BridgeParallelAdmissionReason::SharedPublicationReductionTarget
    );
    assert_eq!(plan.packet_set().counters().bulk_packet_count(), 5);
    assert_eq!(plan.execution_plan().counters().bulk_routed_item_count(), 2);
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_normalized_workload_width(),
        11
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_packet_entry_count(),
        6
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_reduction_input_count(),
        3
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_reduction_output_count(),
        3
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_packet_queue_depth_peak(),
        5
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_reducer_input_buffer_peak(),
        3
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_replay_mismatch_count(),
        0
    );
    assert_eq!(
        plan.execution_plan().counters().bulk_parallel_legal_count(),
        1
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_parallel_profitable_count(),
        0
    );
    assert_eq!(
        plan.execution_plan()
            .counters()
            .bulk_parallel_fallback_to_serial_count(),
        1
    );
    assert_eq!(plan.execution_plan().planning_failures().len(), 1);
}

#[test]
fn bridge_speculation_promotion_truth_is_invariant_across_diagnostics_tiers() {
    let source = crate::harness::fixtures::InMemoryRelationalBridgeSource::default();
    let baseline_runtime = crate::facade::RuntimeBridge::builder()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source.clone())
        .with_signal_sink(crate::harness::fixtures::RecordingSignalBridgeSink::default())
        .with_policy(crate::facade::BridgeRuntimePolicy::development())
        .register_mapping(registration())
        .build()
        .expect("baseline speculation runtime should build");
    let forensic_runtime = crate::facade::RuntimeBridge::builder()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(crate::harness::fixtures::RecordingSignalBridgeSink::default())
        .with_policy(crate::facade::BridgeRuntimePolicy::forensic())
        .register_mapping(registration())
        .build()
        .expect("forensic speculation runtime should build");

    let baseline_decl = BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new("cert:preview-declaration"),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new("cert:binding"),
            TruthBranchIdentity::new("main"),
            BridgeSignalBranchIdentity::new("signal:cert"),
        ),
        "truth-view:cert",
        "source-capability:cert",
        "request-shape:cert",
        "artifact-schema:cert",
    );
    let forensic_decl = baseline_decl.clone();

    let baseline_admitted = baseline_runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("cert:preview-session"),
            baseline_decl,
        )
        .expect("baseline declaration should admit");
    let forensic_admitted = forensic_runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("cert:preview-session"),
            forensic_decl,
        )
        .expect("forensic declaration should admit");

    let (baseline_active, baseline_execution) =
        baseline_runtime.activate_preview_session(baseline_admitted, 3, 1, 2);
    let (forensic_active, forensic_execution) =
        forensic_runtime.activate_preview_session(forensic_admitted, 3, 1, 2);

    let baseline_proof = baseline_active.promotion_admissibility_proof();
    let forensic_proof = forensic_active.promotion_admissibility_proof();

    let (_baseline_promoted, baseline_promotion) = baseline_runtime
        .promote_preview_session(
            baseline_active,
            &baseline_execution,
            &baseline_proof,
            "commit-boundary:cert",
            "authoritative-artifact:cert",
        )
        .expect("baseline promotion should succeed");
    let (_forensic_promoted, forensic_promotion) = forensic_runtime
        .promote_preview_session(
            forensic_active,
            &forensic_execution,
            &forensic_proof,
            "commit-boundary:cert",
            "authoritative-artifact:cert",
        )
        .expect("forensic promotion should succeed");

    let baseline_replay = baseline_runtime
        .replay_preview_bundle("cert:preview-session")
        .expect("baseline replay should succeed");
    let forensic_replay = forensic_runtime
        .replay_preview_bundle("cert:preview-session")
        .expect("forensic replay should succeed");

    assert_eq!(baseline_execution.digest(), forensic_execution.digest());
    assert_eq!(baseline_promotion.digest(), forensic_promotion.digest());
    assert_eq!(baseline_replay.digest(), forensic_replay.digest());
    assert_eq!(
        baseline_runtime
            .diagnostics()
            .explain_preview_replay_bundle(&baseline_replay)
            .lifecycle_outcome(),
        forensic_runtime
            .diagnostics()
            .explain_preview_replay_bundle(&forensic_replay)
            .lifecycle_outcome()
    );
}

#[test]
fn bridge_m13_mixed_offline_diagnosis_bundle_distinguishes_stream_source_structural_merge_preview_policy_and_writeback_failures(
) {
    let baseline = ExecutionProfile::development("baseline");

    let stream_control = execute_harness_run(
        mixed_stream_fixture("bridge-m13-mixed-stream"),
        baseline.clone(),
        "mixed-stream-routing",
        "stream-routing:commit-a,commit-b",
    );
    let stream_replay = execute_harness_run(
        mixed_stream_fixture("bridge-m13-mixed-stream"),
        baseline.clone(),
        "mixed-stream-replay",
        "stream-replay-audit:commit-a,commit-b",
    );
    let source_control = execute_harness_run(
        mixed_source_fixture("bridge-m13-mixed-source"),
        baseline.clone(),
        "mixed-source-control",
        "source-materialize:source:analysis-history",
    );
    let source_replay = execute_harness_run(
        mixed_source_fixture("bridge-m13-mixed-source"),
        baseline.clone(),
        "mixed-source-replay",
        "source-replay:source:analysis-history",
    );
    let source_hostile = execute_harness_run(
        mixed_source_fixture("bridge-m13-mixed-source"),
        baseline.clone(),
        "mixed-source-hostile",
        "source-reject-unregistered:source:hostile-missing",
    );
    let structural_control = execute_harness_run(
        mixed_structural_fixture("bridge-m13-mixed-structural"),
        baseline.clone(),
        "mixed-structural-control",
        "structural-remap-exact:structural:analysis-remap",
    );
    let structural_replay = execute_harness_run(
        mixed_structural_fixture("bridge-m13-mixed-structural"),
        baseline.clone(),
        "mixed-structural-replay",
        "structural-remap-replay:structural:analysis-remap",
    );
    let structural_hostile = execute_harness_run(
        mixed_structural_fixture("bridge-m13-mixed-structural"),
        baseline.clone(),
        "mixed-structural-hostile",
        "structural-remap-ambiguous:structural:analysis-remap",
    );
    let merge_control = execute_harness_run(
        mixed_merge_fixture("bridge-m13-mixed-merge"),
        baseline.clone(),
        "mixed-merge-control",
        "merge-execute:merge:m13-mixed",
    );
    let merge_replay = execute_harness_run(
        mixed_merge_fixture("bridge-m13-mixed-merge"),
        baseline.clone(),
        "mixed-merge-replay",
        "merge-replay:merge:m13-mixed",
    );
    let merge_hostile = execute_harness_run(
        mixed_merge_fixture("bridge-m13-mixed-merge"),
        baseline.clone(),
        "mixed-merge-hostile",
        "merge-execute:merge:m13-topology-denial",
    );
    let preview_control = execute_harness_run(
        mixed_speculation_fixture("bridge-m13-mixed-preview"),
        baseline.clone(),
        "mixed-preview-control",
        "speculation-discard-certify",
    );
    let policy_control = execute_harness_run(
        mixed_policy_fixture("bridge-m13-mixed-policy"),
        baseline.clone(),
        "mixed-policy-control",
        "policy-provenance-certify",
    );
    let policy_replay = execute_harness_run(
        mixed_policy_fixture("bridge-m13-mixed-policy"),
        ExecutionProfile::development("sections-canonical")
            .with_metadata("policy_builder_load_order", "sections_canonical"),
        "mixed-policy-replay",
        "policy-provenance-certify",
    );
    let policy_hostile = execute_harness_run(
        mixed_policy_fixture("bridge-m13-mixed-policy"),
        baseline.clone(),
        "mixed-policy-hostile",
        "policy-rejection-certify",
    );
    let writeback_control = execute_harness_run(
        mixed_writeback_fixture("bridge-m13-mixed-writeback"),
        baseline.clone(),
        "mixed-writeback-control",
        "writeback-duplicate-certify",
    );
    let writeback_replay = execute_harness_run(
        mixed_writeback_fixture("bridge-m13-mixed-writeback"),
        baseline.clone(),
        "mixed-writeback-replay",
        "writeback-feedback-certify",
    );
    let writeback_hostile = execute_harness_run(
        mixed_writeback_fixture("bridge-m13-mixed-writeback"),
        baseline,
        "mixed-writeback-hostile",
        "writeback-bypass-certify",
    );

    let diagnostics_entrypoint_matrix = json!({
        "stream": true,
        "source": true,
        "structural": true,
        "merge": true,
        "preview": true,
        "policy": true,
        "writeback": true,
    });
    let offline_diagnosis_report = json!([
        {
            "family": "stream",
            "classification": "PressureCheckpointSurface",
            "reason": "pressure/checkpoint evidence carried by stream certification bundle",
            "digest": stream_control.extensions["bridge_stream_certification_bundle"]["checkpoint_digest"],
        },
        {
            "family": "source",
            "classification": source_hostile.extensions["bridge_source_rejection"]["failure_kind"],
            "reason": "source capability/open rejection remains typed",
            "digest": source_hostile.summary["failure_digest"],
        },
        {
            "family": "structural",
            "classification": structural_hostile.extensions["bridge_structural_certification_bundle"]["ambiguity_report"]["outcome_class"],
            "reason": "structural ambiguity is mechanically separated from remap replay",
            "digest": structural_hostile.summary["failure_digest"],
        },
        {
            "family": "merge",
            "classification": merge_hostile.summary["denial_class"],
            "reason": "merge denial stays topology-gated and family-local",
            "digest": merge_hostile.summary["failure_digest"],
        },
        {
            "family": "preview",
            "classification": "PreviewResidueSurface",
            "reason": "preview discard residue proof remains offline-evaluable",
            "digest": preview_control.summary["preview_lifecycle_digest"],
        },
        {
            "family": "policy",
            "classification": policy_hostile.extensions["bridge_policy_certification_bundle"]["policy_matrix"]["rows"][0]["failure_kind"],
            "reason": "policy mismatch remains typed without route/runtime logs",
            "digest": policy_hostile.summary["failure_digest"],
        },
        {
            "family": "writeback",
            "classification": writeback_hostile.extensions["bridge_writeback_certification_bundle"]["bypass_rejection"]["failure_kind"],
            "reason": "writeback rejection remains authority-boundary local",
            "digest": writeback_hostile.summary["failure_digest"],
        }
    ]);
    let mixed_bundle = json!({
        "digests": {
            "stream": stream_control.summary["stream_digest"],
            "source": source_control.summary["source_contract_digest"],
            "structural": structural_control.summary["structural_match_digest"],
            "merge": merge_control.summary["result_bundle_digest"],
            "preview": preview_control.summary["preview_lifecycle_digest"],
            "policy": policy_control.summary["policy_digest"],
            "writeback": writeback_control.summary["repeated_bundle_digest"],
        },
        "typed_failure_summary": {
            "source": source_hostile.extensions["bridge_source_rejection"]["failure_kind"],
            "structural": structural_hostile.extensions["bridge_structural_certification_bundle"]["ambiguity_report"]["outcome_class"],
            "merge": merge_hostile.summary["denial_class"],
            "policy": policy_hostile.extensions["bridge_policy_certification_bundle"]["policy_matrix"]["rows"][0]["failure_kind"],
            "writeback": writeback_hostile.extensions["bridge_writeback_certification_bundle"]["bypass_rejection"]["failure_kind"],
        },
        "diagnostics_entrypoint_matrix": diagnostics_entrypoint_matrix,
        "offline_diagnosis_report": offline_diagnosis_report,
    });

    assert_eq!(
        stream_control.summary["stream_digest"],
        stream_replay.summary["stream_digest"]
    );
    assert_eq!(
        source_control.summary["truth_view_digest"],
        source_replay.summary["truth_view_digest"]
    );
    assert_eq!(
        structural_control.summary["structural_reuse_digest"],
        structural_replay.summary["structural_reuse_digest"]
    );
    assert_eq!(
        merge_control.summary["result_bundle_digest"],
        merge_replay.summary["replay_digest"]
    );
    assert_eq!(policy_control.summary, policy_replay.summary);
    assert_eq!(
        writeback_replay.extensions["bridge_writeback_certification_bundle"]
            ["feedback_origin_matrix"]["restart_replay_matrix"]
            ["replay_equivalent_to_live_feedback"],
        json!(true)
    );
    assert_eq!(
        writeback_control.extensions["bridge_writeback_certification_bundle"]
            ["duplicate_authority_matrix"]["boundedness_proof"]["loop_converged"],
        json!(true)
    );
    assert_eq!(
        writeback_replay.extensions["bridge_writeback_certification_bundle"]
            ["feedback_origin_matrix"]["boundedness_proof"]["feedback_converged"],
        json!(true)
    );

    assert_eq!(
        mixed_bundle["typed_failure_summary"]["source"],
        json!("SourceContractMismatch")
    );
    assert_eq!(
        mixed_bundle["typed_failure_summary"]["structural"],
        json!("RejectedAmbiguousStructuralMatch")
    );
    assert_eq!(
        mixed_bundle["typed_failure_summary"]["merge"],
        json!("TopologyRewireGate")
    );
    assert_eq!(
        mixed_bundle["typed_failure_summary"]["policy"],
        json!("UnsupportedExecutionMode")
    );
    assert_eq!(
        mixed_bundle["typed_failure_summary"]["writeback"],
        json!("PreviewWritebackRejected")
    );
    assert_ne!(
        mixed_bundle["typed_failure_summary"]["source"],
        mixed_bundle["typed_failure_summary"]["writeback"]
    );
    assert_eq!(
        mixed_bundle["diagnostics_entrypoint_matrix"]["stream"],
        json!(true)
    );
    assert_eq!(
        mixed_bundle["diagnostics_entrypoint_matrix"]["writeback"],
        json!(true)
    );
    assert_eq!(offline_diagnosis_report.as_array().map(Vec::len), Some(7));
    assert_eq!(
        preview_control.summary["failure_digest"],
        serde_json::Value::Null
    );
    assert_eq!(
        preview_control.extensions["bridge_speculation_certification_bundle"]
            ["discard_residue_report"]["authoritative_residue_count"],
        json!(0)
    );
}
