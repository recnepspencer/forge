use crate::facade::tests::policy_phase2::admitted_bundle;
use crate::facade::tests::runtime;
use crate::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgePolicyCounters,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgeRequestKind,
    BridgeRuntimePolicy,
};

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

    let left_row =
        runtime.summarize_policy_provenance_row("left", &left.0, &left.1, &left.2, &left.3);
    let right_row =
        runtime.summarize_policy_provenance_row("right", &right.0, &right.1, &right.2, &right.3);
    let report =
        runtime.summarize_policy_provenance_report(vec![left_row.clone(), right_row.clone()]);

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
    let left =
        BridgePolicyCounters::new(2, 8, 1, 4, 1, 4, 4, 1, 0, 2, 0, 1, 0, 0, 3, 2, 1, 0, 1, 0);
    let right =
        BridgePolicyCounters::new(2, 8, 1, 4, 1, 4, 4, 1, 0, 2, 0, 1, 0, 0, 3, 2, 1, 0, 1, 0);

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
}
