use super::super::super::support::*;

const AUTHORITY_EVIDENCE_CLOSEOUT_DOC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/forge-query/runtime-authoritative-mutation-evidence-closeout.md"
));

#[test]
fn runtime_public_authoritative_mutation_evidence_support_freezes_admitted_families() {
    let workspace = task_runtime()
        .workspace("task.authority-evidence-support")
        .expect("task runtime should open a named workspace");
    let support = workspace.public_authoritative_mutation_evidence_support();
    let matrix = workspace.public_support_matrix();
    let row = matrix
        .row("authoritative-mutation-evidence-certification")
        .expect("authority evidence gate row should exist");

    assert_eq!(
        support.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );
    assert_eq!(
        support.declared_resolved_target_model(),
        "declared-resolved-target-evidence-with-touched-fallout"
    );
    assert_eq!(
        support.existing_truth_binding_families(),
        &[
            "direct_entity_identity".to_string(),
            "direct_relation_identity".to_string(),
        ]
    );
    assert_eq!(
        support.existing_truth_assertion_modes(),
        &[
            "retained_authoritative_assertion".to_string(),
            "backend_verified_assertion".to_string(),
        ]
    );
    assert_eq!(
        support.existing_truth_probe_modes(),
        &["backend_verified_probe".to_string()]
    );
    assert_eq!(
        support.existing_truth_verified_mutation_modes(),
        &[
            "backend_verified_update".to_string(),
            "backend_verified_delete".to_string(),
        ]
    );
    assert_eq!(support.bridge_backed_verification_support_rows().len(), 8);
    assert!(support
        .bridge_backed_verification_support_rows()
        .iter()
        .any(|row| row.operation_family() == "verify_existing"
            && row.target_binding_family() == "direct_entity_identity"));
    assert_eq!(
        support.identity_preserving_update_families(),
        &[
            "direct_entity_identity_update".to_string(),
            "direct_relation_identity_update".to_string(),
        ]
    );
    assert_eq!(
        support.symbolic_target_reference_families(),
        &["same_batch_declared_target".to_string()]
    );
    assert_eq!(
        support.symbolic_aspect_reference_families(),
        &["same_batch_declared_entity_identity".to_string()]
    );
    assert_eq!(
        support.graph_composition_families(),
        &[
            "same_batch_entity_relation_identity_edges".to_string(),
            "mixed_existing_and_symbolic_entity_identity_edges".to_string(),
            "same_batch_symbolic_entity_followup_mutation".to_string(),
            "same_batch_symbolic_relation_followup_mutation".to_string(),
            "same_batch_symbolic_relation_retirement".to_string(),
            "mixed_existing_target_followup_mutation".to_string(),
            "mixed_existing_target_retarget".to_string(),
            "mixed_existing_target_supersession".to_string(),
            "mixed_existing_target_retirement".to_string(),
            "mixed_existing_target_verified_followup_mutation".to_string(),
            "mixed_existing_target_verified_retarget".to_string(),
            "mixed_existing_target_verified_supersession".to_string(),
            "mixed_existing_target_verified_retirement".to_string(),
        ]
    );
    assert!(support
        .naming_mutation_families()
        .iter()
        .any(|family| family == "remove"));
    assert!(support
        .continuity_mutation_families()
        .iter()
        .any(|family| family == "split_existing_target"));
    assert!(support
        .aggregate_evidence_sections()
        .iter()
        .any(|section| section == "aggregate_existing_truth_mode_digest"));
    assert!(support
        .aggregate_evidence_sections()
        .iter()
        .any(|section| section == "aggregate_naming_mutation_digest"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "backend_verification_unsupported"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "backend_probe_unsupported"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "missing_probed_aspect"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "requires_authoritative_lane"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-empty"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-duplicate-symbol"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-unresolved-symbolic-reference"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-symbolic-collection-mismatch"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-existing-target-resolved-target-missing"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-existing-target-collection-mismatch"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-existing-target-retarget-unsupported"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-existing-target-identity-preservation-unavailable"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-existing-target-supersession-unsupported"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-existing-target-backend-verification-unsupported"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-existing-target-missing-asserted-aspect"));
    assert!(support
        .fail_closed_denial_classes()
        .iter()
        .any(|kind| kind == "graph-composition-domain-invariant-denied"));
    assert_eq!(
        row.support_contract_digest(),
        Some(support.support_digest())
    );
}

#[test]
fn runtime_public_authoritative_mutation_evidence_closeout_answers_dependency_contract() {
    let workspace = task_runtime()
        .workspace("task.authority-evidence-closeout")
        .expect("task runtime should open a named workspace");
    let closeout = workspace.public_authoritative_mutation_evidence_closeout();
    let query_support = workspace.public_authoritative_mutation_evidence_support();
    let bridge_support =
        forge_runtime_bridge::facade::RuntimeBridge::public_authoritative_mutation_evidence_support(
        );
    let bridge_closeout =
        forge_runtime_bridge::facade::RuntimeBridge::public_authoritative_mutation_evidence_closeout();

    assert_eq!(
        closeout.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );
    assert_eq!(
        closeout.query_support_digest(),
        query_support.support_digest()
    );
    assert_eq!(
        closeout.bridge_support_digest(),
        bridge_support.support_digest()
    );
    assert_eq!(
        closeout.bridge_closeout_digest(),
        bridge_closeout.closeout_digest()
    );
    assert_eq!(
        query_support.existing_truth_binding_families(),
        bridge_support.existing_truth_binding_families()
    );
    assert_eq!(
        query_support.symbolic_target_reference_families(),
        bridge_support.symbolic_target_reference_families()
    );
    assert!(query_support
        .symbolic_aspect_reference_families()
        .iter()
        .any(|family| family == "same_batch_declared_entity_identity"));
    assert_eq!(
        query_support.naming_mutation_families(),
        bridge_support.naming_mutation_families()
    );
    assert_eq!(
        query_support.continuity_mutation_families(),
        bridge_support.continuity_mutation_families()
    );
    assert!(bridge_support
        .carry_forward_sections()
        .iter()
        .any(|section| section == "replay-safe-request-receipt-digests"));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("existing-truth binding")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("backend-verified assertions")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("aggregate mode evidence")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("existing-truth probes")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("verified updates")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("relation identity")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("verified deletes")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("probe surfaces keep retained assertions")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("attempted-shape summaries")
            && line.contains("declared collections")
            && line.contains("lifecycle families")));
    assert!(closeout
        .safe_to_build_now()
        .iter()
        .any(|line| line.contains("commit atomically at the backend boundary")));
    assert!(closeout.safe_to_build_now().iter().any(
        |line| line.contains("machine-readable by operation family and target-binding family")
    ));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("durable restart")));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("remain fail-closed")));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("identity-preserving relation update families")));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("verified-mutation") && line.contains("probe neighbors")));
    assert!(closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("compatibility runtimes admit them")));
    assert!(bridge_closeout
        .must_not_assume_yet()
        .iter()
        .any(|line| line.contains("existing-truth binding") && line.contains("fail-closed")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("read bridge-backed verified-existing support rows")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("workspace.compose_graph(...)")
            && line.contains("workspace.compose_graph_with_invariant_pack(...)")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("workspace.assert_existing(...)")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("domain_invariant_summary()")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("workspace.probe_existing(...)")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("workspace.bind_existing_relation(...)")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("workspace.update_existing_verified(...)")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("workspace.delete_existing_verified(...)")));
    assert!(closeout
        .migration_guidance()
        .iter()
        .any(|line| line.contains("delete local existing-target rebinding")));
    assert!(closeout
        .required_verification_commands()
        .iter()
        .any(|line| line == "cargo test -p forge-runtime-bridge"));
}

#[test]
fn runtime_public_authoritative_mutation_evidence_closeout_document_matches_certified_contract() {
    let workspace = task_runtime()
        .workspace("task.authority-evidence-closeout-doc")
        .expect("task runtime should open a named workspace");
    let query_support = workspace.public_authoritative_mutation_evidence_support();
    let closeout = workspace.public_authoritative_mutation_evidence_closeout();
    let bridge_closeout =
        forge_runtime_bridge::facade::RuntimeBridge::public_authoritative_mutation_evidence_closeout();

    for line in closeout.safe_to_build_now() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(line));
    }
    for line in closeout.must_not_assume_yet() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(line));
    }
    for line in closeout.migration_guidance() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(line));
    }
    for line in closeout.required_verification_commands() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(line));
    }
    for line in bridge_closeout.safe_to_build_now() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(line));
    }
    for family in query_support.naming_mutation_families() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(family));
    }
    for family in query_support.symbolic_aspect_reference_families() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(family));
    }
    for family in query_support.continuity_mutation_families() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(family));
    }
    for family in query_support.existing_truth_binding_families() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(family));
    }
    for mode in query_support.existing_truth_assertion_modes() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(mode));
    }
    for mode in query_support.existing_truth_probe_modes() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(mode));
    }
    for mode in query_support.existing_truth_verified_mutation_modes() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(mode));
    }
    for row in query_support.bridge_backed_verification_support_rows() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(row.operation_family()));
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(row.target_binding_family()));
    }
    for family in query_support.identity_preserving_update_families() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(family));
    }
    for row in query_support.graph_composition_capability_support_rows() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(row.capability_family()));
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(row.capability_class().as_str()));
    }
    for row in query_support.graph_composition_extension_hook_support_rows() {
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(row.hook_family()));
        assert!(AUTHORITY_EVIDENCE_CLOSEOUT_DOC.contains(row.boundary().as_str()));
    }
}
