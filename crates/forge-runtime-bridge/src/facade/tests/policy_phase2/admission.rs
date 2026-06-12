use crate::facade::tests::runtime;
use crate::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeExecutionPolicyClass,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgePolicyFieldKind,
    BridgePolicyRejectionKind, BridgePolicyRejectionStage, BridgePolicyResolution,
    BridgePolicySourceClass, BridgeRequestKind, BridgeRuntimePolicy,
    BridgeTruthViewPolicyResolution, BridgeTruthViewSelector, HistoricalEvaluationDeclaration,
};
use crate::snapshot::BridgeReplayMode;

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
    assert_eq!(
        lowered.execution_class(),
        contract.resolved_execution_class()
    );
    assert_eq!(lowered.diagnostics_tier(), BridgeDiagnosticsTier::Standard);
    assert!(lowered.route_artifacts());
    assert!(lowered.replay_artifacts());
    assert_eq!(provenance.contract_identity(), contract.contract_identity());
    assert_eq!(
        provenance.lowered_policy_identity(),
        lowered.policy_identity()
    );
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
    assert_eq!(rejection.stage(), BridgePolicyRejectionStage::Validation);
    assert_eq!(rejection.field_kind(), BridgePolicyFieldKind::ExecutionMode);
    assert_eq!(
        rejection.primary_source(),
        BridgePolicySourceClass::RequestDeclared
    );
}

#[test]
fn runtime_rejects_replay_when_baseline_forbids_replay_artifacts() {
    let runtime = runtime(BridgeRuntimePolicy::operational().with_replay_artifacts(false));
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

    assert_eq!(
        rejection.kind(),
        BridgePolicyRejectionKind::ReplayPolicyConflict
    );
    assert_eq!(
        rejection.field_kind(),
        BridgePolicyFieldKind::ReplayArtifacts
    );
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

    assert_eq!(
        rejection.kind(),
        BridgePolicyRejectionKind::ReplayPolicyConflict
    );
    assert_eq!(
        rejection.field_kind(),
        BridgePolicyFieldKind::ReplayArtifacts
    );
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

    assert_eq!(
        rejection.kind(),
        BridgePolicyRejectionKind::DiagnosticsPolicyConflict
    );
    assert_eq!(
        rejection.field_kind(),
        BridgePolicyFieldKind::DiagnosticsTier
    );
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

    assert_eq!(
        contract.resolved_diagnostics_tier(),
        BridgeDiagnosticsTier::Minimal
    );
    let diagnostics_entry = contract
        .resolution_entries()
        .iter()
        .find(|entry| entry.field_kind() == BridgePolicyFieldKind::DiagnosticsTier)
        .expect("diagnostics resolution entry should exist");
    assert_eq!(
        diagnostics_entry.resolution(),
        BridgePolicyResolution::Narrowed
    );
    assert_eq!(
        diagnostics_entry.operative_source(),
        BridgePolicySourceClass::RuntimeBaseline
    );
}

#[test]
fn policy_admission_remains_structurally_distinct_from_truth_view_policy_resolution() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let truth_view = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_snapshot(
            crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
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
    assert_eq!(
        contract.resolved_execution_class(),
        BridgeExecutionPolicyClass::Optimized
    );
}
