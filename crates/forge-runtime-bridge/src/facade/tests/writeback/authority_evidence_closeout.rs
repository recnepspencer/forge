use crate::facade::RuntimeBridge;

#[test]
fn bridge_public_authoritative_mutation_evidence_support_freezes_admitted_families() {
    let support = RuntimeBridge::public_authoritative_mutation_evidence_support();

    assert!(support
        .carry_forward_sections()
        .iter()
        .any(|item| item == "existing-truth-binding"));
    assert_eq!(
        support.existing_truth_binding_families(),
        &["direct_entity_identity".to_string()]
    );
    assert_eq!(
        support.symbolic_target_reference_families(),
        &["same_batch_declared_target".to_string()]
    );
    assert!(support
        .naming_mutation_families()
        .iter()
        .any(|item| item == "rebind_target"));
    assert!(support
        .continuity_mutation_families()
        .iter()
        .any(|item| item == "split_existing_target"));
    assert!(support
        .aggregate_evidence_sections()
        .iter()
        .any(|item| item == "aggregate_continuity_mutation_digest"));
    assert!(!support.support_digest().is_empty());
}

#[test]
fn bridge_public_authoritative_mutation_evidence_closeout_answers_carry_forward_contract() {
    let support = RuntimeBridge::public_authoritative_mutation_evidence_support();
    let closeout = RuntimeBridge::public_authoritative_mutation_evidence_closeout();

    assert_eq!(closeout.support_digest(), support.support_digest());
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("target, causality, provenance, naming, and continuity")));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("durable restart")));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("existing-truth binding")));
    assert!(closeout
        .required_verification_commands()
        .iter()
        .any(|line| line == "cargo test -p forge-runtime-bridge"));
    assert!(!closeout.closeout_digest().is_empty());
}
