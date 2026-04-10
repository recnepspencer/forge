use super::*;
use crate::facade::{
    BridgeBulkWorkloadRequest, BridgeBulkWorkloadSegment, BridgeExecutionPolicyClass,
    BridgePolicyCounters, BridgePolicyDeclaration, BridgePolicyDeclarationIdentity,
    BridgePolicyFieldKind, BridgePolicyRejectionKind, BridgePolicyResolution,
    BridgePolicySourceClass, BridgeRequestKind, BridgeRouteErrorKind, BridgeRouteRequest,
};
use crate::input::envelope::TruthBranchIdentity;
use crate::snapshot::{
    BridgeDeliveryIntent, BridgeReplayMode, BridgeTruthViewSelector, HistoricalEvaluationDeclaration,
    TruthSnapshotIdentity,
};

#[test]
fn runtime_admits_canonical_policy_declaration_and_lowers_it() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:authoritative-standard"),
        BridgeRequestKind::Authoritative,
        BridgeExecutionPolicyClass::DeterministicCanonical,
        BridgeDiagnosticsTier::Standard,
        true,
        true,
    );

    let contract = runtime
        .admit_policy_declaration(declaration)
        .expect("deterministic authoritative policy should admit");
    let lowered = runtime.lower_admitted_policy(&contract);
    let provenance = runtime.canonicalize_policy_provenance(&contract, &lowered);

    assert_eq!(
        contract.resolved_execution_class(),
        BridgeExecutionPolicyClass::DeterministicCanonical
    );
    assert_eq!(lowered.execution_class(), contract.resolved_execution_class());
    assert_eq!(lowered.diagnostics_tier(), BridgeDiagnosticsTier::Standard);
    assert!(lowered.route_artifacts());
    assert!(lowered.replay_artifacts());
    assert_eq!(provenance.contract_identity(), contract.contract_identity());
    assert_eq!(provenance.lowered_policy_identity(), lowered.policy_identity());
    assert_eq!(provenance.entries().len(), 4);
}

#[test]
fn runtime_rejects_optimized_authoritative_policy_requests() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:authoritative-optimized"),
        BridgeRequestKind::Authoritative,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Standard,
        false,
        false,
    );

    let rejection = runtime
        .admit_policy_declaration(declaration)
        .expect_err("optimized authoritative policy should fail closed");

    assert_eq!(
        rejection.kind(),
        BridgePolicyRejectionKind::UnsupportedExecutionMode
    );
    assert_eq!(rejection.stage(), crate::facade::BridgePolicyRejectionStage::Validation);
    assert_eq!(rejection.field_kind(), BridgePolicyFieldKind::ExecutionMode);
    assert_eq!(
        rejection.primary_source(),
        BridgePolicySourceClass::RequestDeclared
    );
}

#[test]
fn runtime_rejects_replay_when_baseline_forbids_replay_artifacts() {
    let runtime = runtime(
        BridgeRuntimePolicy::operational().with_replay_artifacts(false),
    );
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:preview-replay-required"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Minimal,
        true,
        false,
    );

    let rejection = runtime
        .admit_policy_declaration(declaration)
        .expect_err("replay requirement should fail when baseline disables replay");

    assert_eq!(rejection.kind(), BridgePolicyRejectionKind::ReplayPolicyConflict);
    assert_eq!(rejection.field_kind(), BridgePolicyFieldKind::ReplayArtifacts);
}

#[test]
fn runtime_rejects_replay_without_route_artifacts() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:preview-replay-without-route-record"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Standard,
        true,
        false,
    );

    let rejection = runtime
        .admit_policy_declaration(declaration)
        .expect_err("replay-capable policy should require route artifacts");

    assert_eq!(rejection.kind(), BridgePolicyRejectionKind::ReplayPolicyConflict);
    assert_eq!(rejection.field_kind(), BridgePolicyFieldKind::ReplayArtifacts);
    assert_eq!(
        rejection.conflicting_source(),
        BridgePolicySourceClass::RequestDeclared
    );
}

#[test]
fn runtime_rejects_replay_with_minimal_diagnostics() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:preview-replay-minimal-diagnostics"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Minimal,
        true,
        true,
    );

    let rejection = runtime
        .admit_policy_declaration(declaration)
        .expect_err("replay-capable policy should require standard diagnostics");

    assert_eq!(rejection.kind(), BridgePolicyRejectionKind::DiagnosticsPolicyConflict);
    assert_eq!(rejection.field_kind(), BridgePolicyFieldKind::DiagnosticsTier);
}

#[test]
fn runtime_narrows_diagnostics_tier_to_baseline() {
    let runtime = runtime(BridgeRuntimePolicy::operational());
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:preview-exhaustive"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Exhaustive,
        false,
        false,
    );

    let contract = runtime
        .admit_policy_declaration(declaration)
        .expect("preview diagnostics should narrow to baseline");

    assert_eq!(contract.resolved_diagnostics_tier(), BridgeDiagnosticsTier::Minimal);
    let diagnostics_entry = contract
        .resolution_entries()
        .iter()
        .find(|entry| entry.field_kind() == BridgePolicyFieldKind::DiagnosticsTier)
        .expect("diagnostics resolution entry should exist");
    assert_eq!(diagnostics_entry.resolution(), BridgePolicyResolution::Narrowed);
    assert_eq!(
        diagnostics_entry.operative_source(),
        BridgePolicySourceClass::RuntimeBaseline
    );
}

#[test]
fn runtime_policy_provenance_is_stable_for_same_inputs() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:preview-stable"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Standard,
        false,
        false,
    );

    let left_contract = runtime
        .admit_policy_declaration(declaration.clone())
        .expect("left declaration should admit");
    let right_contract = runtime
        .admit_policy_declaration(declaration)
        .expect("right declaration should admit");
    let left_lowered = runtime.lower_admitted_policy(&left_contract);
    let right_lowered = runtime.lower_admitted_policy(&right_contract);
    let left = runtime.canonicalize_policy_provenance(&left_contract, &left_lowered);
    let right = runtime.canonicalize_policy_provenance(&right_contract, &right_lowered);

    assert_eq!(left_contract.digest(), right_contract.digest());
    assert_eq!(left_lowered.digest(), right_lowered.digest());
    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn policy_contract_digest_changes_when_resolution_entries_change() {
    let development = runtime(BridgeRuntimePolicy::development());
    let restrictive = runtime(BridgeRuntimePolicy::operational());
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:preview-digest-variance"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Standard,
        false,
        false,
    );

    let permissive_contract = development
        .admit_policy_declaration(declaration.clone())
        .expect("development runtime should admit");
    let restrictive_contract = restrictive
        .admit_policy_declaration(declaration)
        .expect("operational runtime should admit with narrowing");

    assert_ne!(permissive_contract.digest(), restrictive_contract.digest());
    assert_ne!(
        permissive_contract.resolution_entries(),
        restrictive_contract.resolution_entries()
    );
}

#[test]
fn policy_provenance_digest_changes_when_resolution_entries_change() {
    let development = runtime(BridgeRuntimePolicy::development());
    let operational = runtime(BridgeRuntimePolicy::operational());
    let declaration = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:provenance-digest-variance"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Exhaustive,
        false,
        false,
    );

    let left_contract = development
        .admit_policy_declaration(declaration.clone())
        .expect("development runtime should admit");
    let right_contract = operational
        .admit_policy_declaration(declaration)
        .expect("operational runtime should admit");
    let left = development.canonicalize_policy_provenance(
        &left_contract,
        &development.lower_admitted_policy(&left_contract),
    );
    let right = operational.canonicalize_policy_provenance(
        &right_contract,
        &operational.lower_admitted_policy(&right_contract),
    );

    assert_ne!(left.digest(), right.digest());
}

#[test]
fn policy_admission_remains_structurally_distinct_from_truth_view_policy_resolution() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let truth_view = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_snapshot(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-a"),
        ),
        BridgeReplayMode::Enabled,
        BridgeDiagnosticsTier::Standard,
        BridgeDeliveryIntent::PrepareSignalEvaluation,
    );
    let policy = BridgePolicyDeclaration::new(
        BridgePolicyDeclarationIdentity::new("policy:preview-standard"),
        BridgeRequestKind::Preview,
        BridgeExecutionPolicyClass::Optimized,
        BridgeDiagnosticsTier::Standard,
        false,
        false,
    );

    let truth_view_resolution = runtime.resolve_truth_view_policy(&truth_view);
    let contract = runtime
        .admit_policy_declaration(policy)
        .expect("policy declaration should admit independently");

    match truth_view_resolution {
        BridgeTruthViewPolicyResolution::Admitted(resolved) => {
            assert_eq!(resolved.replay_mode(), BridgeReplayMode::Enabled);
        }
        BridgeTruthViewPolicyResolution::Rejected(rejection) => {
            panic!("unexpected truth-view rejection: {}", rejection.detail());
        }
    }
    assert_eq!(contract.resolved_execution_class(), BridgeExecutionPolicyClass::Optimized);
}

#[test]
fn runtime_summarizes_policy_provenance_report_rows_with_semantic_equivalence() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let left = admitted_bundle(
        &runtime,
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:preview-equivalent-left"),
            BridgeRequestKind::Preview,
            BridgeExecutionPolicyClass::Optimized,
            BridgeDiagnosticsTier::Minimal,
            false,
            false,
        ),
    );
    let right = admitted_bundle(
        &runtime,
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:preview-equivalent-right"),
            BridgeRequestKind::Preview,
            BridgeExecutionPolicyClass::Optimized,
            BridgeDiagnosticsTier::Minimal,
            false,
            false,
        ),
    );

    let left_row = runtime.summarize_policy_provenance_row(
        "left",
        &left.0,
        &left.1,
        &left.2,
        &left.3,
    );
    let right_row = runtime.summarize_policy_provenance_row(
        "right",
        &right.0,
        &right.1,
        &right.2,
        &right.3,
    );
    let report = runtime.summarize_policy_provenance_report(vec![left_row.clone(), right_row.clone()]);

    assert_ne!(left_row.policy_digest(), right_row.policy_digest());
    assert_eq!(
        left_row.semantic_policy_digest(),
        right_row.semantic_policy_digest()
    );
    assert_eq!(report.rows().len(), 2);
    assert!(!report.digest().is_empty());
}

#[test]
fn policy_counters_are_canonical_for_same_inputs() {
    let left = BridgePolicyCounters::new(2, 8, 1, 4, 1, 4, 4, 1, 0, 2, 0, 1, 0, 0, 3, 2, 1, 0, 1, 0);
    let right = BridgePolicyCounters::new(2, 8, 1, 4, 1, 4, 4, 1, 0, 2, 0, 1, 0, 0, 3, 2, 1, 0, 1, 0);

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn runtime_projects_route_planning_policy_and_stamps_planned_route() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let (contract, lowered, provenance, replay_bundle) = admitted_bundle(
        &runtime,
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:route-planning"),
            BridgeRequestKind::Preview,
            BridgeExecutionPolicyClass::Optimized,
            BridgeDiagnosticsTier::Standard,
            false,
            false,
        ),
    );
    let route_policy = runtime
        .project_route_planning_policy(&lowered)
        .expect("route policy projection should succeed");
    let planned = runtime
        .plan_committed_patch_with_route_policy(
            BridgeRouteRequest::for_commit("commit-a"),
            &route_policy,
        )
        .expect("route planning under lowered policy should succeed");

    assert_eq!(
        planned.route_planning_policy_digest(),
        Some(route_policy.digest())
    );
    let row = runtime.summarize_policy_provenance_row(
        "route-planning",
        &contract,
        &lowered,
        &provenance,
        &replay_bundle,
    );
    assert_eq!(row.lowered_policy_digest(), lowered.digest());
}

#[test]
fn runtime_rejects_incompatible_route_planning_policy_from_more_permissive_runtime() {
    let permissive = runtime(BridgeRuntimePolicy::development());
    let restrictive = runtime(BridgeRuntimePolicy::operational().with_replay_artifacts(false));
    let (_, lowered, _, _) = admitted_bundle(
        &permissive,
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:replay-required-for-route"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ),
    );

    let error = restrictive
        .project_route_planning_policy(&lowered)
        .expect_err("restrictive runtime should reject incompatible route policy");

    assert_eq!(error.kind(), BridgeRouteErrorKind::RoutePolicyMismatch);
}

#[test]
fn bulk_route_planning_policy_is_carried_by_every_planned_route() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let (_, lowered, _, _) = admitted_bundle(
        &runtime,
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:bulk-route-planning"),
            BridgeRequestKind::Preview,
            BridgeExecutionPolicyClass::Optimized,
            BridgeDiagnosticsTier::Minimal,
            false,
            false,
        ),
    );
    let route_policy = runtime
        .project_route_planning_policy(&lowered)
        .expect("bulk route policy projection should succeed");
    let workload = BridgeBulkWorkloadRequest::new(vec![
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
        BridgeBulkWorkloadSegment::new(BridgeRouteRequest::for_commit("commit-a")),
    ]);

    let plan = runtime
        .plan_bulk_workload_with_route_policy(workload, &route_policy)
        .expect("bulk planning under route policy should succeed");

    assert_eq!(plan.planned_routes().len(), 2);
    for route in plan.planned_routes() {
        assert_eq!(
            route.route_planning_policy_digest(),
            Some(route_policy.digest())
        );
    }
}

#[test]
fn policy_scoped_route_round_trips_through_canonical_replay() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let (_, lowered, _, _) = admitted_bundle(
        &runtime,
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:route-replay-scope"),
            BridgeRequestKind::Preview,
            BridgeExecutionPolicyClass::Optimized,
            BridgeDiagnosticsTier::Standard,
            false,
            true,
        ),
    );
    let route_policy = runtime
        .project_route_planning_policy(&lowered)
        .expect("route replay policy projection should succeed");
    let result = runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch_with_route_policy(
                    BridgeRouteRequest::for_commit("commit-a"),
                    &route_policy,
                )
                .expect("policy scoped route should plan"),
        )
        .expect("policy scoped route should deliver");
    let canonical_record = runtime
        .diagnostics()
        .last_canonical_route_record()
        .expect("canonical route record should be retained");

    let replay = runtime
        .replay_canonical_record(&canonical_record)
        .expect("policy scoped canonical route should replay");

    assert_eq!(
        result.result_summary().route_planning_policy_digest(),
        Some(route_policy.digest())
    );
    assert_eq!(
        canonical_record
            .decode()
            .expect("canonical route record should decode")
            .route_planning_policy_digest(),
        Some(route_policy.digest())
    );
    assert_eq!(replay.route_identity(), result.result_summary().route_identity());
    assert_eq!(
        replay.invalidation_identity(),
        result.result_summary().invalidation_identity()
    );
}

#[test]
fn policy_scoped_route_without_route_artifacts_does_not_retain_canonical_record() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let (_, lowered, _, _) = admitted_bundle(
        &runtime,
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:route-no-retention"),
            BridgeRequestKind::Preview,
            BridgeExecutionPolicyClass::Optimized,
            BridgeDiagnosticsTier::Standard,
            false,
            false,
        ),
    );
    let route_policy = runtime
        .project_route_planning_policy(&lowered)
        .expect("route policy projection should succeed");

    runtime
        .deliver_invalidation(
            runtime
                .plan_committed_patch_with_route_policy(
                    BridgeRouteRequest::for_commit("commit-a"),
                    &route_policy,
                )
                .expect("policy scoped route should plan"),
        )
        .expect("policy scoped route should deliver");

    assert!(runtime.diagnostics().last_canonical_route_record().is_none());
}

fn admitted_bundle(
    runtime: &crate::facade::RuntimeBridge,
    declaration: BridgePolicyDeclaration,
) -> (
    crate::facade::AdmittedBridgePolicyContract,
    crate::facade::LoweredBridgeExecutionPolicy,
    crate::facade::BridgePolicyProvenanceRecord,
    crate::facade::BridgePolicyReplayBundle,
) {
    let contract = runtime
        .admit_policy_declaration(declaration)
        .expect("policy should admit");
    let lowered = runtime.lower_admitted_policy(&contract);
    let provenance = runtime.canonicalize_policy_provenance(&contract, &lowered);
    let replay_bundle = runtime.replay_policy_bundle(&contract, &lowered, &provenance);
    (contract, lowered, provenance, replay_bundle)
}
